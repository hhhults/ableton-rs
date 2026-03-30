//! Low-level OSC transport over UDP.
//!
//! Sends messages to AbletonOSC on port 11000, receives responses on port 11001.
//! Uses a background thread to receive responses and dispatch them to waiters.

use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use rosc::{OscMessage, OscPacket, OscType};

use crate::error::{Error, Result};

/// A single OSC argument value.
#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Int(i32),
    Float(f32),
    String(String),
    Double(f64),
    Long(i64),
    Nil,
}

impl Arg {
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Arg::Int(v) => Some(*v),
            Arg::Float(v) => Some(*v as i32),
            Arg::Double(v) => Some(*v as i32),
            Arg::Long(v) => Some(*v as i32),
            _ => None,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Arg::Float(v) => Some(*v),
            Arg::Int(v) => Some(*v as f32),
            Arg::Double(v) => Some(*v as f32),
            Arg::Long(v) => Some(*v as f32),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Arg::Double(v) => Some(*v),
            Arg::Float(v) => Some(*v as f64),
            Arg::Int(v) => Some(*v as f64),
            Arg::Long(v) => Some(*v as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Arg::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        self.as_i32().map(|v| v != 0)
    }

    pub fn into_osc(self) -> OscType {
        match self {
            Arg::Int(v) => OscType::Int(v),
            Arg::Float(v) => OscType::Float(v),
            Arg::String(v) => OscType::String(v),
            Arg::Double(v) => OscType::Double(v),
            Arg::Long(v) => OscType::Long(v),
            Arg::Nil => OscType::Nil,
        }
    }
}

impl From<i32> for Arg {
    fn from(v: i32) -> Self { Arg::Int(v) }
}
impl From<f32> for Arg {
    fn from(v: f32) -> Self { Arg::Float(v) }
}
impl From<f64> for Arg {
    fn from(v: f64) -> Self { Arg::Float(v as f32) }
}
impl From<&str> for Arg {
    fn from(v: &str) -> Self { Arg::String(v.to_string()) }
}
impl From<String> for Arg {
    fn from(v: String) -> Self { Arg::String(v) }
}
impl From<bool> for Arg {
    fn from(v: bool) -> Self { Arg::Int(v as i32) }
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

type ResponseSlot = Arc<Mutex<Option<Vec<Arg>>>>;

/// Shared state between the sender and the receiver thread.
struct Inner {
    /// Pending waiters: address → (condvar, response slot).
    waiters: Mutex<HashMap<String, (Arc<Condvar>, ResponseSlot)>>,
}

/// Low-level OSC client. Thread-safe, clone-cheap.
#[derive(Clone)]
pub struct OscClient {
    send_socket: Arc<UdpSocket>,
    send_addr: String,
    inner: Arc<Inner>,
    pub default_timeout: Duration,
}

impl OscClient {
    /// Connect to AbletonOSC.
    pub fn new(host: &str, send_port: u16, recv_port: u16) -> Result<Self> {
        let recv_socket = UdpSocket::bind(format!("{host}:{recv_port}"))?;
        recv_socket.set_read_timeout(Some(Duration::from_millis(100)))?;

        let send_socket = UdpSocket::bind("0.0.0.0:0")?;

        let inner = Arc::new(Inner {
            waiters: Mutex::new(HashMap::new()),
        });

        // Spawn receiver thread
        let inner2 = inner.clone();
        std::thread::Builder::new()
            .name("ableton-osc-recv".into())
            .spawn(move || recv_loop(recv_socket, inner2))?;

        Ok(Self {
            send_socket: Arc::new(send_socket),
            send_addr: format!("{host}:{send_port}"),
            inner,
            default_timeout: Duration::from_secs(2),
        })
    }

    /// Connect with default ports (11000/11001).
    pub fn connect() -> Result<Self> {
        Self::new("127.0.0.1", 11000, 11001)
    }

    /// Send a fire-and-forget message.
    pub fn send(&self, address: &str, args: &[Arg]) -> Result<()> {
        let msg = OscPacket::Message(OscMessage {
            addr: address.to_string(),
            args: args.iter().cloned().map(|a| a.into_osc()).collect(),
        });
        let buf = rosc::encoder::encode(&msg)
            .map_err(|e| Error::OscDecode(format!("{e:?}")))?;
        self.send_socket.send_to(&buf, &self.send_addr)?;
        Ok(())
    }

    /// Send a message and wait for a response.
    pub fn query(&self, address: &str, args: &[Arg]) -> Result<Vec<Arg>> {
        self.query_timeout(address, args, self.default_timeout)
    }

    /// Send a message and wait for a response with custom timeout.
    pub fn query_timeout(
        &self,
        address: &str,
        args: &[Arg],
        timeout: Duration,
    ) -> Result<Vec<Arg>> {
        let cvar = Arc::new(Condvar::new());
        let slot: ResponseSlot = Arc::new(Mutex::new(None));

        // Register waiter
        {
            let mut waiters = self.inner.waiters.lock().unwrap();
            waiters.insert(address.to_string(), (cvar.clone(), slot.clone()));
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
                // Cleanup waiter
                self.inner.waiters.lock().unwrap().remove(address);
                return Err(Error::Timeout {
                    address: address.to_string(),
                });
            }
            let (new_guard, _timeout_result) = cvar.wait_timeout(guard, remaining).unwrap();
            guard = new_guard;
        }

        // Cleanup waiter
        self.inner.waiters.lock().unwrap().remove(address);

        Ok(guard.take().unwrap())
    }
}

fn recv_loop(socket: UdpSocket, inner: Arc<Inner>) {
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
                // Socket closed or fatal error — exit thread
                break;
            }
        }
    }
}

fn dispatch_packet(inner: &Inner, packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            let args: Vec<Arg> = msg.args.into_iter().map(osc_type_to_arg).collect();
            let waiters = inner.waiters.lock().unwrap();
            if let Some((cvar, slot)) = waiters.get(&msg.addr) {
                let mut slot = slot.lock().unwrap();
                *slot = Some(args);
                cvar.notify_one();
            }
        }
        OscPacket::Bundle(bundle) => {
            for p in bundle.content {
                dispatch_packet(inner, p);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg_conversions() {
        assert_eq!(Arg::from(42).as_i32(), Some(42));
        assert_eq!(Arg::from(3.14f32).as_f32(), Some(3.14));
        assert_eq!(Arg::from("hello").as_str(), Some("hello"));
        assert_eq!(Arg::from(true).as_bool(), Some(true));
        assert_eq!(Arg::from(false).as_bool(), Some(false));
    }

    #[test]
    fn arg_cross_conversions() {
        // Float → i32
        assert_eq!(Arg::Float(3.7).as_i32(), Some(3));
        // Int → f32
        assert_eq!(Arg::Int(42).as_f32(), Some(42.0));
    }
}
