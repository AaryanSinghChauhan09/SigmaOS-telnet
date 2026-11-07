//! Telnet command model.
use crate::option::TelnetOption;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandCode { Do, Dont, Will, Wont, Subcommand }

#[derive(Debug, Clone)]
pub struct TelnetCommand {
    pub code: CommandCode,
    pub option: TelnetOption,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telnet_command() {
        let c = TelnetCommand { code: CommandCode::Do, option: TelnetOption::Echo };
        assert_eq!(c.code, CommandCode::Do);
    }
}
