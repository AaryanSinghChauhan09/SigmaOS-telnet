//! Session model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState { Connecting, Negotiating, Connected, Closed }

#[derive(Debug, Clone)]
pub struct TelnetSession {
    pub host: String,
    pub port: u16,
    pub state: SessionState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telnet_session() {
        let s = TelnetSession { host: "example.com".into(), port: 23, state: SessionState::Connecting };
        assert_eq!(s.port, 23);
    }
}
