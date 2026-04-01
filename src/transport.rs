//! Pluggable transport trait for OSC communication.
//!
//! Implemented by [`UdpTransport`](crate::udp::UdpTransport) for direct OSC over UDP.
//! External code can implement this trait to route through a daemon, proxy, or mock.

use std::time::Duration;

use crate::error::Result;
use crate::osc::Arg;

/// Transport for sending and receiving OSC messages.
pub trait Transport: Send + Sync {
    /// Send a fire-and-forget message.
    fn send(&self, address: &str, args: &[Arg]) -> Result<()>;

    /// Send a message and wait for a response with a timeout.
    fn query_timeout(&self, address: &str, args: &[Arg], timeout: Duration) -> Result<Vec<Arg>>;
}
