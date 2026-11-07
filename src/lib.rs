//! SigmaOS-telnet — telnet protocol model.
//!
//! Models the telnet protocol: options negotiation, commands (DO/DONT/WILL/WONT),
//! connection state, and terminal handling. Deterministic and std-only.

pub mod option;
pub mod command;
pub mod session;
pub mod terminal;

pub use option::{TelnetOption, OptionState};
pub use command::{TelnetCommand, CommandCode};
pub use session::{TelnetSession, SessionState};
pub use terminal::{TerminalType, TerminalMode};
