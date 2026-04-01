//! # ableton
//!
//! Low-latency Rust client for controlling Ableton Live via
//! [AbletonOSC](https://github.com/ideoforms/AbletonOSC).
//!
//! Communicates over UDP/OSC on ports 11000 (send) / 11001 (receive).
//!
//! ## Quick start
//!
//! ```rust,no_run
//! use ableton::{Session, Note};
//!
//! let session = Session::connect()?;
//! session.set_tempo(120.0)?;
//!
//! let track = session.create_midi_track(-1)?;
//! track.set_name("Lead")?;
//!
//! // Load an instrument
//! session.load_instrument(track.track_idx, "Analog")?;
//!
//! // Create a clip with notes
//! let clip = track.create_clip(0, 4.0)?;
//! clip.set_name("melody")?;
//! clip.set_looping(true)?;
//! clip.add_notes(&[
//!     Note::new(60, 0.0, 1.0, 100),
//!     Note::new(64, 1.0, 1.0, 100),
//!     Note::new(67, 2.0, 1.0, 100),
//!     Note::new(72, 3.0, 1.0, 100),
//! ])?;
//! clip.fire()?;
//!
//! session.play()?;
//! # Ok::<(), ableton::Error>(())
//! ```

pub mod clip;
pub mod compiler;
pub mod device;
pub mod error;
pub mod live;
pub mod osc;
pub mod session;
pub mod track;
pub mod transport;
pub mod udp;

pub use clip::{Clip, Note};
pub use compiler::{CompileResult, IrPatch};
pub use device::{Device, Param};
pub use error::{Error, Result};
pub use live::{LiveSession, UpdateKind, UpdateResult};
pub use osc::{Arg, OscClient};
pub use session::{ReturnTrack, Scene, Session};
pub use track::Track;
pub use transport::Transport;
