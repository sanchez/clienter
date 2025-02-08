//! HTTP method definitions according to RFC 7231.
//!
//! This module provides an enumeration of standard HTTP methods used in HTTP/1.1 requests.

/// Represents standard HTTP methods as defined in RFC 7231.
///
/// # Examples
///
/// ```
/// use clienter::HttpMethod;
///
/// let method = HttpMethod::GET;
/// assert_eq!(method.to_string(), "GET");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum HttpMethod {
    /// The GET method requests transfer of a current selected representation
    /// for the target resource.
    GET,
    /// The POST method requests that the target resource process the
    /// representation enclosed in the request according to its semantics.
    POST,
    /// The PUT method requests that the state of the target resource be
    /// created or replaced with the state defined by the representation
    /// enclosed in the request.
    PUT,
    /// The DELETE method requests that the origin server remove the
    /// association between the target resource and its current functionality.
    DELETE,
    /// The PATCH method requests that a set of changes described in the
    /// request entity be applied to the target resource.
    PATCH,
    /// The HEAD method is identical to GET except that the server MUST NOT
    /// send a message body in the response.
    HEAD,
    /// The OPTIONS method requests information about the communication options
    /// available for the target resource.
    OPTIONS,
    /// The CONNECT method establishes a tunnel to the server identified by
    /// the target resource.
    CONNECT,
    /// The TRACE method performs a message loop-back test along the path to
    /// the target resource.
    TRACE,
}

/// Implements string representation for HTTP methods.
///
/// This implementation allows converting an HttpMethod variant into its
/// canonical uppercase string representation.
impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::PATCH => "PATCH",
            Self::HEAD => "HEAD",
            Self::OPTIONS => "OPTIONS",
            Self::CONNECT => "CONNECT",
            Self::TRACE => "TRACE",
        };
        f.write_str(s)
    }
}
