use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("OSC send/recv: {0}")]
    Io(#[from] std::io::Error),

    #[error("OSC decode: {0}")]
    OscDecode(String),

    #[error("query timed out: {address}")]
    Timeout { address: String },

    #[error("Ableton returned error: {0}")]
    Ableton(String),

    #[error("unexpected response for {address}: expected {expected} args, got {got}")]
    BadResponse {
        address: String,
        expected: usize,
        got: usize,
    },

    #[error("parameter not found: {0}")]
    ParamNotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;
