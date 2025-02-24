//! HTTP module providing core functionality for HTTP protocol communication.
//!
//! This module contains all the essential components needed to construct and handle
//! HTTP requests and responses, including headers, methods, URIs, and status codes.

/// Client implementation for making HTTP requests
mod client;
pub use client::HttpClient;

mod error;
pub use error::HttpError;

/// HTTP headers management
mod headers;
pub use headers::HttpHeaders;

/// HTTP methods (GET, POST, etc.)
mod method;
pub use method::HttpMethod;

/// Protocol definitions (HTTP/1.1, HTTP/2)
mod protocol;
pub use protocol::Protocol;

/// HTTP request structure and builder
mod request;
pub use request::HttpRequest;

/// HTTP response handling
mod response;
pub use response::HttpResponse;

/// HTTP status codes and categories
mod status_code;
pub use status_code::StatusCode;

/// URI parsing and manipulation
mod uri;
pub use uri::Uri;
