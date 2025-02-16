//! URI handling for HTTP requests
//!
//! This module provides functionality for parsing and handling URIs (Uniform Resource Identifiers).
//! URIs are used to identify resources in HTTP requests.
//!
//! # Examples
//!
//! ```
//! use clienter::Uri;
//!
//! // Parse a basic HTTP URL
//! let uri: Uri = "http://example.com/path".parse().unwrap();
//! assert_eq!(uri.hostname, "example.com");
//! assert_eq!(uri.path, "path");
//!
//! // Create from string with explicit port
//! let uri: Uri = "https://localhost:8080/api".parse().unwrap();
//! assert_eq!(uri.get_addr(), "localhost:8080");
//! ```

use std::{fmt::Debug, str::FromStr};

use crate::utils;

/// Represents a URI with protocol, hostname, optional port, and path components.
///
/// # Examples
///
/// ```
/// use clienter::Uri;
///
/// let uri: Uri = "http://api.example.com:8080/v1/users".parse().unwrap();
/// assert_eq!(uri.get_addr(), "api.example.com:8080");
/// assert_eq!(uri.get_encoded_path(), "v1/users");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Uri {
    pub protocol: super::protocol::Protocol,
    pub hostname: String,
    pub port: Option<u16>,
    pub path: String,
}

/// Possible errors that can occur when parsing a URI
#[derive(Debug, PartialEq)]
pub enum UriError {
    Empty,
    InvalidProtocol,
    InvalidHostname,
    InvalidPort,
}

impl Uri {
    /// Returns the address string in the format "hostname:port".
    /// If port is not specified, uses the default port for the protocol.
    ///
    /// # Examples
    ///
    /// ```
    /// use clienter::Uri;
    ///
    /// let uri: Uri = "http://example.com".parse().unwrap();
    /// assert_eq!(uri.get_addr(), "example.com:80"); // Default HTTP port
    ///
    /// let uri: Uri = "https://example.com:443".parse().unwrap();
    /// assert_eq!(uri.get_addr(), "example.com:443");
    /// ```
    pub fn get_addr(&self) -> String {
        match self.port {
            Some(port) => format!("{}:{}", self.hostname, port),
            None => format!("{}:{}", self.hostname, self.protocol.get_default_port()),
        }
    }

    /// Returns the path with proper URL encoding.
    /// Encodes spaces as "%20" and percent signs as "%25".
    ///
    /// # Examples
    ///
    /// ```
    /// use clienter::Uri;
    ///
    /// let uri: Uri = "http://example.com/path with spaces".parse().unwrap();
    /// assert_eq!(uri.get_encoded_path(), "path%20with%20spaces");
    ///
    /// let uri: Uri = "http://example.com/50%discount".parse().unwrap();
    /// assert_eq!(uri.get_encoded_path(), "50%25discount");
    /// ```
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

        if hostname.is_empty() {
            return Err(UriError::InvalidHostname);
        }

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

        // Test default protocol
        let uri = "localhost/path".parse::<Uri>().unwrap();
        assert_eq!(uri.protocol, super::super::protocol::Protocol::HTTP);
        assert_eq!(uri.hostname, "localhost");
        assert_eq!(uri.port, None);
        assert_eq!(uri.path, "path");

        // Test with HTTPS and default port
        let uri = "https://api.example.com/v1/users".parse::<Uri>().unwrap();
        assert_eq!(uri.protocol, super::super::protocol::Protocol::HTTPS);
        assert_eq!(uri.hostname, "api.example.com");
        assert_eq!(uri.port, None);
        assert_eq!(uri.path, "v1/users");

        // Test empty path
        let uri = "http://localhost:8080".parse::<Uri>().unwrap();
        assert_eq!(uri.path, "");
    }

    #[test]
    fn test_uri_errors() {
        assert_eq!("".parse::<Uri>(), Err(UriError::Empty));
        assert_eq!(
            "invalid://host".parse::<Uri>(),
            Err(UriError::InvalidProtocol)
        );
        assert_eq!("http://:80".parse::<Uri>(), Err(UriError::InvalidHostname));
        assert_eq!(
            "http://localhost:invalid".parse::<Uri>(),
            Err(UriError::InvalidPort)
        );
    }
}
