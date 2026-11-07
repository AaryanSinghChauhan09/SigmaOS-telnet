//! SigmaOS-telnet — Telnet client model.

pub mod util;
pub mod options;

pub use util::{Telnet, Session};
pub use options::{TelnetOptions, Mode};
