//! Telnet option model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionState { Yes, No }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelnetOption { Echo, SuppressGoAhead, Status, Timing, TerminalType, WindowSize }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telnet_option() {
        assert_eq!(OptionState::Yes, OptionState::Yes);
    }
}
