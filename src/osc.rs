//! OSC argument types and client abstraction.
//!
//! [`OscClient`] wraps a pluggable [`Transport`](crate::transport::Transport),
//! providing a uniform API whether backed by direct UDP or a daemon proxy.

use std::sync::{mpsc, Arc};
use std::time::Duration;

use rosc::OscType;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::transport::Transport;
use crate::udp::{ListenerMessage, UdpTransport};

/// A single OSC argument value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// Thread-safe OSC client backed by a pluggable transport.
///
/// Clone is cheap (Arc'd transport).
#[derive(Clone)]
pub struct OscClient {
    transport: Arc<dyn Transport>,
    pub default_timeout: Duration,
}

impl OscClient {
    /// Connect to AbletonOSC via direct UDP (localhost:11000).
    pub fn connect() -> Result<Self> {
        Self::udp("127.0.0.1", 11000)
    }

    /// Connect to AbletonOSC at the given host and port via UDP.
    pub fn udp(host: &str, send_port: u16) -> Result<Self> {
        let transport = UdpTransport::new(host, send_port)?;
        Ok(Self {
            transport: Arc::new(transport),
            default_timeout: Duration::from_secs(2),
        })
    }

    /// Create a client backed by any transport.
    pub fn from_transport(transport: impl Transport + 'static) -> Self {
        Self {
            transport: Arc::new(transport),
            default_timeout: Duration::from_secs(2),
        }
    }

    /// Send a fire-and-forget message.
    pub fn send(&self, address: &str, args: &[Arg]) -> Result<()> {
        self.transport.send(address, args)
    }

    /// Send a message and wait for a response.
    pub fn query(&self, address: &str, args: &[Arg]) -> Result<Vec<Arg>> {
        self.transport.query_timeout(address, args, self.default_timeout)
    }

    /// Send a message and wait for a response with custom timeout.
    pub fn query_timeout(
        &self,
        address: &str,
        args: &[Arg],
        timeout: Duration,
    ) -> Result<Vec<Arg>> {
        self.transport.query_timeout(address, args, timeout)
    }

    /// Send multiple queries at once and collect all responses.
    /// With UdpTransport, all queries are batched into one AbletonOSC tick cycle.
    pub fn batch_query(
        &self,
        queries: &[(String, Vec<Arg>)],
    ) -> Result<Vec<Vec<Arg>>> {
        self.transport
            .batch_query_timeout(queries, self.default_timeout)
    }

    /// Register a listener for unsolicited OSC messages matching a prefix.
    /// Returns None if the underlying transport doesn't support listeners.
    pub fn register_listener(&self, prefix: &str) -> Option<mpsc::Receiver<ListenerMessage>> {
        self.transport.register_listener(prefix)
    }

    /// Send multiple queries with custom timeout.
    pub fn batch_query_timeout(
        &self,
        queries: &[(String, Vec<Arg>)],
        timeout: Duration,
    ) -> Result<Vec<Vec<Arg>>> {
        self.transport.batch_query_timeout(queries, timeout)
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
        assert_eq!(Arg::Float(3.7).as_i32(), Some(3));
        assert_eq!(Arg::Int(42).as_f32(), Some(42.0));
    }

    #[test]
    fn arg_serde_roundtrip() {
        let args = vec![
            Arg::Int(42),
            Arg::Float(3.14),
            Arg::String("hello".into()),
            Arg::Nil,
        ];
        let json = serde_json::to_string(&args).unwrap();
        let parsed: Vec<Arg> = serde_json::from_str(&json).unwrap();
        assert_eq!(args, parsed);
    }
}
