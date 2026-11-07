//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Binary,
    NoEcho,
    Escape(char),
}

#[derive(Debug, Clone)]
pub struct TelnetOptions {
    pub mode: Mode,
    pub host: String,
    pub port: u16,
}

impl Default for TelnetOptions {
    fn default() -> Self { Self { mode: Mode::Default, host: String::new(), port: 23 } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(TelnetOptions::default().mode, Mode::Default); }
}
