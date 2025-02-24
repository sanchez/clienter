//! HTTP client implementation for making HTTP requests.
//!
//! This module provides a simple HTTP client that can be used to make HTTP requests
//! over TCP connections. It supports custom headers and timeout configuration.
//!
//! # Example
//! ```
//! use clienter::{HttpClient, HttpMethod};
//!
//! let client = HttpClient::new();
//! let request = client.request(HttpMethod::GET, "http://example.com");
//! let response = client.send(&request).expect("Failed to send request");
//! ```

use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};

use super::{HttpError, HttpHeaders, HttpMethod, HttpRequest, HttpResponse, Uri};

/// A configurable HTTP client for making HTTP requests.
///
/// The client supports setting custom headers and connection timeout.
pub struct HttpClient {
    /// Optional timeout duration for connections
    pub timeout: Option<std::time::Duration>,
    /// Default headers to be included in every request
    pub headers: HttpHeaders,
}

impl HttpClient {
    /// Creates a new HTTP client with default configuration.
    ///
    /// # Returns
    /// A new `HttpClient` instance with no timeout and empty default headers.
    pub fn new() -> Self {
        HttpClient {
            timeout: None,
            headers: HttpHeaders::default(),
        }
    }

    /// Creates a new HTTP request with the specified method and URI.
    ///
    /// # Parameters
    /// * `method` - The HTTP method to use for the request
    /// * `uri` - The target URI, which can be any type that can be converted into a `Uri`
    ///
    /// # Returns
    /// A new `HttpRequest` instance configured with the specified method and URI.
    pub fn request<T>(&self, method: HttpMethod, uri: T) -> HttpRequest
    where
        T: Into<Uri>,
    {
        HttpRequest::new(method, uri)
    }

    /// Sends an HTTP request and returns the response.
    ///
    /// This method will:
    /// 1. Establish a TCP connection to the server
    /// 2. Send the request line and headers
    /// 3. Read and parse the response
    ///
    /// # Parameters
    /// * `request` - The `HttpRequest` to send
    ///
    /// # Returns
    /// A `Result` containing either the `HttpResponse` or an `HttpError`
    pub fn send(&self, request: &HttpRequest) -> Result<HttpResponse, HttpError> {
        let handler = request.uri.protocol.get_handler();
        handler(self, request)
    }
}
