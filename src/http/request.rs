//! HTTP Request module that provides functionality for creating and managing HTTP requests.
//!
//! This module contains the core `HttpRequest` struct and its implementations for
//! handling HTTP requests in a type-safe manner.

use super::headers::HttpHeaders;
use super::method::HttpMethod;
use super::uri::Uri;

/// Represents an HTTP request with its components.
///
/// # Fields
/// * `method` - The HTTP method (GET, POST, etc.)
/// * `uri` - The target URI of the request
/// * `headers` - HTTP headers associated with the request
/// * `timeout` - Optional timeout duration for the request
#[derive(Debug, PartialEq, Clone)]
pub struct HttpRequest {
    /// The HTTP method to be used for this request
    pub method: HttpMethod,
    /// The target URI for this request
    pub uri: Uri,
    /// Headers to be sent with this request
    pub headers: HttpHeaders,
    /// Optional timeout duration for this request
    pub timeout: Option<std::time::Duration>,
}

impl HttpRequest {
    /// Creates a new HTTP request with the specified method and URI.
    ///
    /// # Arguments
    /// * `method` - The HTTP method to use
    /// * `uri` - The target URI, which will be converted into a Uri type
    ///
    /// # Returns
    /// A new HttpRequest instance with default headers and no timeout
    pub fn new<T>(method: HttpMethod, uri: T) -> Self
    where
        T: Into<Uri>,
    {
        HttpRequest {
            method,
            uri: uri.into(),
            headers: HttpHeaders::default(),
            timeout: None,
        }
    }

    /// Generates the request line for the HTTP request.
    ///
    /// # Returns
    /// A String containing the formatted request line in the format:
    /// "{METHOD} /{PATH} {HTTP_VERSION}"
    pub fn get_request_line(&self) -> String {
        let uri = format!("/{}", self.uri.get_encoded_path());
        let version = self.uri.protocol.get_http_version();
        format!("{} {} {}", self.method, uri, version)
    }
}
