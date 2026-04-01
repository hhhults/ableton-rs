//! UDP transport — direct OSC communication over UDP.
//!
//! Uses a single socket for both send and receive, bound to an ephemeral port.
//! AbletonOSC responds to the sender's address:port, so multiple transports
//! can coexist without port conflicts.
//!
//! Supports concurrent queries to the same OSC address via a FIFO waiter queue
//! (AbletonOSC processes requests sequentially, so responses arrive in order).

use std::collections::{HashMap, VecDeque};
use std::net::UdpSocket;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use rosc::{OscMessage, OscPacket, OscType};

use crate::error::{Error, Result};
use crate::osc::Arg;
use crate::transport::Transport;

type ResponseSlot = Arc<Mutex<Option<Vec<Arg>>>>;
type WaiterEntry = (Arc<Condvar>, ResponseSlot);

/// Shared state between the sender and the receiver thread.
struct Inner {
    /// Pending waiters: address → FIFO queue of (condvar, response slot).
    /// Multiple concurrent queries to the same address each get their own slot.
    waiters: Mutex<HashMap<String, VecDeque<WaiterEntry>>>,
}

/// Direct OSC over UDP. Thread-safe, supports concurrent queries.
pub struct UdpTransport {
    socket: Arc<UdpSocket>,
    send_addr: String,
    inner: Arc<Inner>,
}

impl UdpTransport {
    /// Connect to AbletonOSC at the given host and send port.
    pub fn new(host: &str, send_port: u16) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(Some(Duration::from_millis(100)))?;
        let socket = Arc::new(socket);

        let inner = Arc::new(Inner {
            waiters: Mutex::new(HashMap::new()),
        });

        let recv_socket = socket.clone();
        let inner2 = inner.clone();
        std::thread::Builder::new()
            .name("ableton-osc-recv".into())
            .spawn(move || recv_loop(recv_socket, inner2))?;

        Ok(Self {
            socket,
            send_addr: format!("{host}:{send_port}"),
            inner,
        })
    }

    /// Connect with default settings (localhost:11000).
    pub fn connect() -> Result<Self> {
        Self::new("127.0.0.1", 11000)
    }
}

impl Transport for UdpTransport {
    fn send(&self, address: &str, args: &[Arg]) -> Result<()> {
        let msg = OscPacket::Message(OscMessage {
            addr: address.to_string(),
            args: args.iter().cloned().map(|a| a.into_osc()).collect(),
        });
        let buf = rosc::encoder::encode(&msg)
            .map_err(|e| Error::OscDecode(format!("{e:?}")))?;
        self.socket.send_to(&buf, &self.send_addr)?;
        Ok(())
    }

    fn query_timeout(
        &self,
        address: &str,
        args: &[Arg],
        timeout: Duration,
    ) -> Result<Vec<Arg>> {
        let cvar = Arc::new(Condvar::new());
        let slot: ResponseSlot = Arc::new(Mutex::new(None));

        // Register waiter in the FIFO queue for this address
        {
            let mut waiters = self.inner.waiters.lock().unwrap();
            waiters
                .entry(address.to_string())
                .or_default()
                .push_back((cvar.clone(), slot.clone()));
        }

        // Send the query
        self.send(address, args)?;

        // Wait for response
        let deadline = Instant::now() + timeout;
        let mut guard = slot.lock().unwrap();
        loop {
            if guard.is_some() {
                break;
            }
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                // Timeout — remove our entry from the queue
                let mut waiters = self.inner.waiters.lock().unwrap();
                if let Some(queue) = waiters.get_mut(address) {
                    queue.retain(|(c, _)| !Arc::ptr_eq(c, &cvar));
                    if queue.is_empty() {
                        waiters.remove(address);
                    }
                }
                return Err(Error::Timeout {
                    address: address.to_string(),
                });
            }
            let (new_guard, _) = cvar.wait_timeout(guard, remaining).unwrap();
            guard = new_guard;
        }

        Ok(guard.take().unwrap())
    }
}

fn recv_loop(socket: Arc<UdpSocket>, inner: Arc<Inner>) {
    let mut buf = [0u8; 65536];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, _src)) => {
                if let Ok((_, packet)) = rosc::decoder::decode_udp(&buf[..size]) {
                    dispatch_packet(&inner, packet);
                }
            }
            Err(ref e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut => {}
            Err(_) => {
                break;
            }
        }
    }
}

fn dispatch_packet(inner: &Inner, packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            let args: Vec<Arg> = msg.args.into_iter().map(osc_type_to_arg).collect();
            let mut waiters = inner.waiters.lock().unwrap();
            if let Some(queue) = waiters.get_mut(&msg.addr) {
                // FIFO: wake the oldest waiter for this address
                if let Some((cvar, slot)) = queue.pop_front() {
                    let mut slot = slot.lock().unwrap();
                    *slot = Some(args);
                    cvar.notify_one();
                }
                if queue.is_empty() {
                    waiters.remove(&msg.addr);
                }
            }
        }
        OscPacket::Bundle(bundle) => {
            for p in bundle.content {
                dispatch_packet(inner, p);
            }
        }
    }
}

fn osc_type_to_arg(t: OscType) -> Arg {
    match t {
        OscType::Int(v) => Arg::Int(v),
        OscType::Float(v) => Arg::Float(v),
        OscType::String(v) => Arg::String(v),
        OscType::Double(v) => Arg::Double(v),
        OscType::Long(v) => Arg::Long(v),
        OscType::Bool(v) => Arg::Int(v as i32),
        _ => Arg::Nil,
    }
}
