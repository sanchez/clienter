use std::str::FromStr;

use super::{HttpClient, HttpError, HttpRequest, HttpResponse};

/// Represents HTTP protocol versions
///
/// Supports both HTTP and HTTPS protocols, providing functionality
/// for protocol-specific operations like default ports and HTTP versions.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Protocol {
    /// Standard HTTP protocol
    HTTP,
    /// Secure HTTPS protocol
    HTTPS,
}

impl FromStr for Protocol {
    type Err = ();

    /// Converts a string to a Protocol enum
    ///
    /// # Arguments
    /// * `s` - A string slice that should be either "http" or "https"
    ///
    /// # Returns
    /// * `Ok(Protocol)` - If the string matches either "http" or "https"
    /// * `Err(())` - If the string doesn't match any known protocol
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http" => Ok(Protocol::HTTP),
            "https" => Ok(Protocol::HTTPS),
            _ => Err(()),
        }
    }
}

impl Protocol {
    /// Returns the default port number for the protocol
    ///
    /// # Returns
    /// * 80 for HTTP
    /// * 443 for HTTPS
    pub fn get_default_port(&self) -> u16 {
        match self {
            Protocol::HTTP => 80,
            Protocol::HTTPS => 443,
        }
    }

    /// Returns the HTTP version string for the protocol
    ///
    /// # Returns
    /// * "HTTP/1.1" for HTTP
    /// * "HTTP/2" for HTTPS
    pub fn get_http_version(&self) -> &'static str {
        match self {
            Protocol::HTTP => "HTTP/1.1",
            Protocol::HTTPS => "HTTP/2",
        }
    }

    pub fn get_handler(
        &self,
    ) -> impl Fn(&HttpClient, &HttpRequest) -> Result<HttpResponse, HttpError> {
        match self {
            Protocol::HTTP => crate::handlers::handle_http,
            Protocol::HTTPS => crate::handlers::handle_https,
        }
    }
}
