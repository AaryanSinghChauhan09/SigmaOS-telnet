//! Terminal model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalType { Vt100, Xterm, Ansi, Dumb }

#[derive(Debug, Clone)]
pub struct TerminalMode {
    pub ttype: TerminalType,
    pub columns: u16,
    pub rows: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_mode() {
        let tm = TerminalMode { ttype: TerminalType::Vt100, columns: 80, rows: 24 };
        assert_eq!(tm.columns, 80);
    }
}
