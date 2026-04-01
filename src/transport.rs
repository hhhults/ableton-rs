//! Pluggable transport trait for OSC communication.
//!
//! Implemented by [`UdpTransport`](crate::udp::UdpTransport) for direct OSC over UDP.
//! External code can implement this trait to route through a daemon, proxy, or mock.

use std::sync::mpsc;
use std::time::Duration;

use crate::error::Result;
use crate::osc::Arg;
use crate::udp::ListenerMessage;

/// Transport for sending and receiving OSC messages.
pub trait Transport: Send + Sync {
    /// Send a fire-and-forget message.
    fn send(&self, address: &str, args: &[Arg]) -> Result<()>;

    /// Send a message and wait for a response with a timeout.
    fn query_timeout(&self, address: &str, args: &[Arg], timeout: Duration) -> Result<Vec<Arg>>;

    /// Register a listener for unsolicited OSC messages matching a prefix.
    /// Returns None if the transport doesn't support listeners (e.g. daemon proxy).
    fn register_listener(&self, _prefix: &str) -> Option<mpsc::Receiver<ListenerMessage>> {
        None
    }

    /// Send multiple queries at once and collect all responses.
    /// Default implementation sends sequentially. UdpTransport overrides with
    /// an optimized version that sends all queries in one batch so AbletonOSC
    /// processes them all in a single tick (~100ms total instead of ~100ms each).
    fn batch_query_timeout(
        &self,
        queries: &[(String, Vec<Arg>)],
        timeout: Duration,
    ) -> Result<Vec<Vec<Arg>>> {
        queries
            .iter()
            .map(|(addr, args)| self.query_timeout(addr, args, timeout))
            .collect()
    }
}
