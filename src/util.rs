//! Telnet model — remote terminal protocol client.

#[derive(Debug, Clone)]
pub struct Session {
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub connected: bool,
}

#[derive(Debug, Clone)]
pub struct Telnet {
    pub escape: char,
    pub binary: bool,
    pub echo: bool,
}

impl Default for Telnet {
    fn default() -> Self { Self { escape: '^', binary: false, echo: true } }
}

impl Telnet {
    pub fn new() -> Self { Self::default() }

    pub fn connect(&self, host: &str, port: u16) -> Session {
        Session { host: host.to_string(), port, user: None, connected: true }
    }

    pub fn send(&self, _sess: &Session, data: &[u8]) -> usize {
        data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn connect_send() {
        let t = Telnet::new();
        let s = t.connect("localhost", 23);
        assert_eq!(t.send(&s, b"test"), 4);
    }
}
