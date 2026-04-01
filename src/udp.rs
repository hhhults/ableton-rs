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
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use rosc::{OscMessage, OscPacket, OscType};

use crate::error::{Error, Result};
use crate::osc::Arg;
use crate::transport::Transport;

type ResponseSlot = Arc<Mutex<Option<Vec<Arg>>>>;
type WaiterEntry = (Arc<Condvar>, ResponseSlot);

/// A listener message: (osc_address, args).
pub type ListenerMessage = (String, Vec<Arg>);

/// Shared state between the sender and the receiver thread.
struct Inner {
    /// Pending waiters: address → FIFO queue of (condvar, response slot).
    /// Multiple concurrent queries to the same address each get their own slot.
    waiters: Mutex<HashMap<String, VecDeque<WaiterEntry>>>,
    /// Listener channels: prefix → sender.
    /// Messages whose address starts with the prefix (and have no pending waiter)
    /// are forwarded to the listener channel.
    listeners: Mutex<Vec<(String, mpsc::Sender<ListenerMessage>)>>,
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
            listeners: Mutex::new(Vec::new()),
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

    /// Register a listener for unsolicited OSC messages matching a prefix.
    /// Returns a receiver that yields `(address, args)` for each matching message.
    /// Messages that have a pending query waiter are NOT forwarded to listeners
    /// (query responses take priority).
    pub fn register_listener(&self, prefix: &str) -> mpsc::Receiver<ListenerMessage> {
        let (tx, rx) = mpsc::channel();
        self.inner
            .listeners
            .lock()
            .unwrap()
            .push((prefix.to_string(), tx));
        rx
    }
}

impl Transport for UdpTransport {
    fn register_listener(&self, prefix: &str) -> Option<mpsc::Receiver<ListenerMessage>> {
        Some(self.register_listener(prefix))
    }

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

    /// Optimized batch: register all waiters and send all queries while holding the
    /// waiter lock (ensures FIFO ordering matches send ordering), then collect responses.
    /// All queries land in AbletonOSC's socket buffer before the next tick, so they're
    /// all processed in one ~100ms cycle instead of one cycle each.
    fn batch_query_timeout(
        &self,
        queries: &[(String, Vec<Arg>)],
        timeout: Duration,
    ) -> Result<Vec<Vec<Arg>>> {
        if queries.is_empty() {
            return Ok(Vec::new());
        }

        let mut slots: Vec<(String, Arc<Condvar>, ResponseSlot)> = Vec::with_capacity(queries.len());

        // Register all waiters and send all queries atomically
        {
            let mut waiters = self.inner.waiters.lock().unwrap();
            for (address, args) in queries {
                let cvar = Arc::new(Condvar::new());
                let slot: ResponseSlot = Arc::new(Mutex::new(None));
                waiters
                    .entry(address.clone())
                    .or_default()
                    .push_back((cvar.clone(), slot.clone()));
                slots.push((address.clone(), cvar, slot));

                // Send immediately (UDP send_to is non-blocking)
                let msg = OscPacket::Message(OscMessage {
                    addr: address.clone(),
                    args: args.iter().cloned().map(|a| a.into_osc()).collect(),
                });
                if let Ok(buf) = rosc::encoder::encode(&msg) {
                    let _ = self.socket.send_to(&buf, &self.send_addr);
                }
            }
        }

        // Collect all responses
        let deadline = Instant::now() + timeout;
        let mut results = Vec::with_capacity(slots.len());

        for (address, cvar, slot) in &slots {
            let mut guard = slot.lock().unwrap();
            loop {
                if guard.is_some() {
                    break;
                }
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    // Clean up remaining waiters
                    let mut waiters = self.inner.waiters.lock().unwrap();
                    if let Some(queue) = waiters.get_mut(address) {
                        queue.retain(|(c, _)| !Arc::ptr_eq(c, cvar));
                        if queue.is_empty() {
                            waiters.remove(address);
                        }
                    }
                    return Err(Error::Timeout {
                        address: address.clone(),
                    });
                }
                let (new_guard, _) = cvar.wait_timeout(guard, remaining).unwrap();
                guard = new_guard;
            }
            results.push(guard.take().unwrap());
        }

        Ok(results)
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
            } else {
                // No pending waiter — check listeners
                drop(waiters); // release waiter lock before acquiring listener lock
                let listeners = inner.listeners.lock().unwrap();
                for (prefix, tx) in listeners.iter() {
                    if msg.addr.starts_with(prefix.as_str()) {
                        let _ = tx.send((msg.addr.clone(), args.clone()));
                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_inner() -> Inner {
        Inner {
            waiters: Mutex::new(HashMap::new()),
            listeners: Mutex::new(Vec::new()),
        }
    }

    fn make_osc_message(addr: &str, args: Vec<OscType>) -> OscPacket {
        OscPacket::Message(OscMessage {
            addr: addr.to_string(),
            args,
        })
    }

    #[test]
    fn dispatch_to_waiter() {
        let inner = make_inner();
        let cvar = Arc::new(Condvar::new());
        let slot: ResponseSlot = Arc::new(Mutex::new(None));
        inner.waiters.lock().unwrap()
            .entry("/live/test".into())
            .or_default()
            .push_back((cvar.clone(), slot.clone()));

        dispatch_packet(&inner, make_osc_message("/live/test", vec![OscType::Int(42)]));

        let result = slot.lock().unwrap();
        assert!(result.is_some());
        assert_eq!(result.as_ref().unwrap()[0], Arg::Int(42));
    }

    #[test]
    fn dispatch_to_listener_when_no_waiter() {
        let inner = make_inner();
        let (tx, rx) = mpsc::channel();
        inner.listeners.lock().unwrap().push(("/live/".into(), tx));

        dispatch_packet(&inner, make_osc_message("/live/track/get/volume", vec![OscType::Int(0), OscType::Float(0.75)]));

        let (addr, args) = rx.recv().unwrap();
        assert_eq!(addr, "/live/track/get/volume");
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], Arg::Int(0));
        assert_eq!(args[1], Arg::Float(0.75));
    }

    #[test]
    fn waiter_takes_priority_over_listener() {
        let inner = make_inner();

        // Register both a waiter and a listener for the same address
        let cvar = Arc::new(Condvar::new());
        let slot: ResponseSlot = Arc::new(Mutex::new(None));
        inner.waiters.lock().unwrap()
            .entry("/live/track/get/volume".into())
            .or_default()
            .push_back((cvar.clone(), slot.clone()));

        let (tx, rx) = mpsc::channel();
        inner.listeners.lock().unwrap().push(("/live/".into(), tx));

        dispatch_packet(&inner, make_osc_message("/live/track/get/volume", vec![OscType::Float(0.5)]));

        // Waiter should get it
        assert!(slot.lock().unwrap().is_some());
        // Listener should NOT get it
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn listener_prefix_matching() {
        let inner = make_inner();
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        inner.listeners.lock().unwrap().push(("/live/track/".into(), tx1));
        inner.listeners.lock().unwrap().push(("/live/song/".into(), tx2));

        dispatch_packet(&inner, make_osc_message("/live/track/get/volume", vec![]));

        assert!(rx1.try_recv().is_ok()); // matches /live/track/
        assert!(rx2.try_recv().is_err()); // doesn't match /live/song/
    }

    #[test]
    fn unmatched_message_dropped() {
        let inner = make_inner();
        // No waiters, no listeners — should not panic
        dispatch_packet(&inner, make_osc_message("/unknown/address", vec![OscType::Int(1)]));
    }

    #[test]
    fn bundle_dispatches_all_messages() {
        let inner = make_inner();
        let (tx, rx) = mpsc::channel();
        inner.listeners.lock().unwrap().push(("/live/".into(), tx));

        let bundle = OscPacket::Bundle(rosc::OscBundle {
            timetag: rosc::OscTime { seconds: 0, fractional: 0 },
            content: vec![
                make_osc_message("/live/a", vec![]),
                make_osc_message("/live/b", vec![]),
            ],
        });
        dispatch_packet(&inner, bundle);

        let msgs: Vec<_> = rx.try_iter().collect();
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].0, "/live/a");
        assert_eq!(msgs[1].0, "/live/b");
    }
}
