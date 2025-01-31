use std::{fmt::Debug, str::FromStr};

use crate::utils;

#[derive(Debug, PartialEq)]
pub struct Uri {
    pub protocol: super::protocol::Protocol,
    pub hostname: String,
    pub port: Option<u16>,
    pub path: String,
}

#[derive(Debug, PartialEq)]
pub enum UriError {
    Empty,
    InvalidProtocol,
    InvalidHostname,
    InvalidPort,
}

impl Uri {
    pub fn get_addr(&self) -> String {
        match self.port {
            Some(port) => format!("{}:{}", self.hostname, port),
            None => format!("{}:{}", self.hostname, self.protocol.get_default_port()),
        }
    }

    pub fn get_encoded_path(&self) -> String {
        self.path.replace("%", "%25").replace(" ", "%20")
    }
}

impl FromStr for Uri {
    type Err = UriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(UriError::Empty);
        }

        let (protocol, s) = match utils::tuple_split(s, "://") {
            Some(x) => x,
            None => ("http", s),
        };

        let protocol = protocol
            .parse::<super::protocol::Protocol>()
            .map_err(|_| UriError::InvalidProtocol)?;

        let (hostname, path) = if s.contains('/') {
            utils::tuple_split(s, "/").ok_or(UriError::InvalidHostname)?
        } else {
            (s, "")
        };

        let (hostname, port) = if hostname.contains(':') {
            utils::tuple_split_parse::<String, u16>(hostname, ":")
                .map(|(hostname, port)| (hostname, Some(port)))
                .ok_or(UriError::InvalidPort)?
        } else {
            (String::from(hostname), None)
        };

        Ok(Uri {
            protocol,
            hostname,
            port,
            path: String::from(path),
        })
    }
}

impl From<String> for Uri {
    fn from(s: String) -> Self {
        s.parse().unwrap()
    }
}

impl From<&str> for Uri {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_from_str() {
        let uri = "http://localhost:8080/hello/world".parse::<Uri>().unwrap();
        assert_eq!(uri.protocol, super::super::protocol::Protocol::HTTP);
        assert_eq!(uri.hostname, "localhost");
        assert_eq!(uri.port, Some(8080));
        assert_eq!(uri.path, "hello/world");
    }
}
