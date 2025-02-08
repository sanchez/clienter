//! HTTP status codes as defined in RFC 7231, 6585, and others.
//!
//! This module provides a type-safe enumeration of HTTP status codes along with
//! helpful methods for classification and conversion.

use std::fmt::Display;

/// Represents an HTTP status code.
///
/// The enum variants are named with their numerical value appended to make them unique
/// and easily identifiable. For example, `Ok200` represents HTTP 200 OK status.
///
/// # Categories
/// - 1xx: Informational responses
/// - 2xx: Successful responses
/// - 3xx: Redirection responses
/// - 4xx: Client error responses
/// - 5xx: Server error responses
#[derive(Debug, PartialEq)]
pub enum StatusCode {
    /// 100 Continue
    Continue100,
    /// 101 Switching Protocols
    SwitchingProtocols101,
    /// 102 Processing
    Processing102,
    /// 103 Early Hints
    EarlyHints103,

    /// 200 OK
    Ok200,
    /// 201 Created
    Created201,
    /// 202 Accepted
    Accepted202,
    /// 203 Non-Authoritative Information
    NonAuthoritativeInformation203,
    /// 204 No Content
    NoContent204,
    /// 205 Reset Content
    ResetContent205,
    /// 206 Partial Content
    PartialContent206,
    /// 207 Multi-Status
    MultiStatus207,
    /// 208 Already Reported
    AlreadyReported208,
    /// 226 IM Used
    ImUsed226,

    /// 300 Multiple Choices
    MultipleChoices300,
    /// 301 Moved Permanently
    MovedPermanently301,
    /// 302 Found
    Found302,
    /// 303 See Other
    SeeOther303,
    /// 304 Not Modified
    NotModified304,
    /// 305 Use Proxy
    UseProxy305,
    /// 307 Temporary Redirect
    TemporaryRedirect307,
    /// 308 Permanent Redirect
    PermanentRedirect308,

    /// 400 Bad Request
    BadRequest400,
    /// 401 Unauthorized
    Unauthorized401,
    /// 402 Payment Required
    PaymentRequired402,
    /// 403 Forbidden
    Forbidden403,
    /// 404 Not Found
    NotFound404,
    /// 405 Method Not Allowed
    MethodNotAllowed405,
    /// 406 Not Acceptable
    NotAcceptable406,
    /// 407 Proxy Authentication Required
    ProxyAuthenticationRequired407,
    /// 408 Request Timeout
    RequestTimeout408,
    /// 409 Conflict
    Conflict409,
    /// 410 Gone
    Gone410,
    /// 411 Length Required
    LengthRequired411,
    /// 412 Precondition Failed
    PrecondiditionFailed412,
    /// 413 Payload Too Large
    PayloadTooLarge413,
    /// 414 URI Too Long
    UriTooLong414,
    /// 415 Unsupported Media Type
    UnsupportedMediaType415,
    /// 416 Range Not Satisfiable
    RangeNotSatisfiable416,
    /// 417 Expectation Failed
    ExpectationFailed417,
    /// 421 Misdirected Request
    MisdirectedRequest421,
    /// 422 Unprocessable Entity
    UnprocessableEntity422,
    /// 423 Locked
    Locked423,
    /// 424 Failed Dependency
    FailedDependency424,
    /// 425 Too Early
    TooEarly425,
    /// 426 Upgrade Required
    UpgradeRequired426,
    /// 428 Precondition Required
    PreconditionRequired428,
    /// 429 Too Many Requests
    TooManyRequests429,
    /// 431 Request Header Fields Too Large
    RequestHeaderFieldsTooLarge431,
    /// 451 Unavailable For Legal Reasons
    UnavailableForLegalReasons451,

    /// 500 Internal Server Error
    InternalServerError500,
    /// 501 Not Implemented
    NotImplemented501,
    /// 502 Bad Gateway
    BadGateway502,
    /// 503 Service Unavailable
    ServiceUnavailable503,
    /// 504 Gateway Timeout
    GatewayTimeout504,
    /// 505 HTTP Version Not Supported
    HttpVersionNotSupported505,
    /// 506 Variant Also Negotiates
    VariantAlsoNegotiates506,
    /// 507 Insufficient Storage
    InsufficientStorage507,
    /// 508 Loop Detected
    LoopDetected508,
    /// 510 Not Extended
    NotExtended510,
    /// 511 Network Authentication Required
    NetworkAuthenticationRequired511,
}

impl StatusCode {
    /// Determines if the status code represents a successful response (2xx range).
    ///
    /// # Returns
    /// `true` if the status code is in the 2xx range, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use crate::http::StatusCode;
    ///
    /// let status = StatusCode::Ok200;
    /// assert!(status.is_success());
    /// ```
    pub fn is_success(&self) -> bool {
        match self {
            StatusCode::Ok200 => true,
            StatusCode::Created201 => true,
            StatusCode::Accepted202 => true,
            StatusCode::NonAuthoritativeInformation203 => true,
            StatusCode::NoContent204 => true,
            StatusCode::ResetContent205 => true,
            StatusCode::PartialContent206 => true,
            StatusCode::MultiStatus207 => true,
            StatusCode::AlreadyReported208 => true,
            StatusCode::ImUsed226 => true,
            _ => false,
        }
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = &'static str;

    /// Attempts to convert a u16 into a StatusCode.
    ///
    /// # Arguments
    /// * `status_code` - The numerical status code to convert
    ///
    /// # Returns
    /// * `Ok(StatusCode)` if the conversion succeeds
    /// * `Err("Unknown status code")` if the status code is not recognized
    ///
    /// # Example
    /// ```
    /// use crate::http::StatusCode;
    ///
    /// let status = StatusCode::try_from(200).unwrap();
    /// assert_eq!(status, StatusCode::Ok200);
    /// ```
    fn try_from(status_code: u16) -> Result<Self, Self::Error> {
        match status_code {
            100 => Ok(StatusCode::Continue100),
            101 => Ok(StatusCode::SwitchingProtocols101),
            102 => Ok(StatusCode::Processing102),
            103 => Ok(StatusCode::EarlyHints103),

            200 => Ok(StatusCode::Ok200),
            201 => Ok(StatusCode::Created201),
            202 => Ok(StatusCode::Accepted202),
            203 => Ok(StatusCode::NonAuthoritativeInformation203),
            204 => Ok(StatusCode::NoContent204),
            205 => Ok(StatusCode::ResetContent205),
            206 => Ok(StatusCode::PartialContent206),
            207 => Ok(StatusCode::MultiStatus207),
            208 => Ok(StatusCode::AlreadyReported208),
            226 => Ok(StatusCode::ImUsed226),

            300 => Ok(StatusCode::MultipleChoices300),
            301 => Ok(StatusCode::MovedPermanently301),
            302 => Ok(StatusCode::Found302),
            303 => Ok(StatusCode::SeeOther303),
            304 => Ok(StatusCode::NotModified304),
            305 => Ok(StatusCode::UseProxy305),
            307 => Ok(StatusCode::TemporaryRedirect307),
            308 => Ok(StatusCode::PermanentRedirect308),

            400 => Ok(StatusCode::BadRequest400),
            401 => Ok(StatusCode::Unauthorized401),
            402 => Ok(StatusCode::PaymentRequired402),
            403 => Ok(StatusCode::Forbidden403),
            404 => Ok(StatusCode::NotFound404),
            405 => Ok(StatusCode::MethodNotAllowed405),
            406 => Ok(StatusCode::NotAcceptable406),
            407 => Ok(StatusCode::ProxyAuthenticationRequired407),
            408 => Ok(StatusCode::RequestTimeout408),
            409 => Ok(StatusCode::Conflict409),
            410 => Ok(StatusCode::Gone410),
            411 => Ok(StatusCode::LengthRequired411),
            412 => Ok(StatusCode::PrecondiditionFailed412),
            413 => Ok(StatusCode::PayloadTooLarge413),
            414 => Ok(StatusCode::UriTooLong414),
            415 => Ok(StatusCode::UnsupportedMediaType415),
            416 => Ok(StatusCode::RangeNotSatisfiable416),
            417 => Ok(StatusCode::ExpectationFailed417),
            421 => Ok(StatusCode::MisdirectedRequest421),
            422 => Ok(StatusCode::UnprocessableEntity422),
            423 => Ok(StatusCode::Locked423),
            424 => Ok(StatusCode::FailedDependency424),
            425 => Ok(StatusCode::TooEarly425),
            426 => Ok(StatusCode::UpgradeRequired426),
            428 => Ok(StatusCode::PreconditionRequired428),
            429 => Ok(StatusCode::TooManyRequests429),
            431 => Ok(StatusCode::RequestHeaderFieldsTooLarge431),
            451 => Ok(StatusCode::UnavailableForLegalReasons451),

            500 => Ok(StatusCode::InternalServerError500),
            501 => Ok(StatusCode::NotImplemented501),
            502 => Ok(StatusCode::BadGateway502),
            503 => Ok(StatusCode::ServiceUnavailable503),
            504 => Ok(StatusCode::GatewayTimeout504),
            505 => Ok(StatusCode::HttpVersionNotSupported505),
            506 => Ok(StatusCode::VariantAlsoNegotiates506),
            507 => Ok(StatusCode::InsufficientStorage507),
            508 => Ok(StatusCode::LoopDetected508),
            510 => Ok(StatusCode::NotExtended510),
            511 => Ok(StatusCode::NetworkAuthenticationRequired511),
            _ => Err("Unknown status code"),
        }
    }
}

impl Display for StatusCode {
    /// Formats the status code as a string in the format "{code} {reason}".
    ///
    /// # Example
    /// ```
    /// use crate::http::StatusCode;
    ///
    /// let status = StatusCode::Ok200;
    /// assert_eq!(status.to_string(), "200 OK");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::Continue100 => write!(f, "100 Continue"),
            StatusCode::SwitchingProtocols101 => write!(f, "101 Switching Protocols"),
            StatusCode::Processing102 => write!(f, "102 Processing"),
            StatusCode::EarlyHints103 => write!(f, "103 Early Hints"),

            StatusCode::Ok200 => write!(f, "200 OK"),
            StatusCode::Created201 => write!(f, "201 Created"),
            StatusCode::Accepted202 => write!(f, "202 Accepted"),
            StatusCode::NonAuthoritativeInformation203 => {
                write!(f, "203 Non-Authoritative Information")
            }
            StatusCode::NoContent204 => write!(f, "204 No Content"),
            StatusCode::ResetContent205 => write!(f, "205 Reset Content"),
            StatusCode::PartialContent206 => write!(f, "206 Partial Content"),
            StatusCode::MultiStatus207 => write!(f, "207 Multi-Status"),
            StatusCode::AlreadyReported208 => write!(f, "208 Already Reported"),
            StatusCode::ImUsed226 => write!(f, "226 IM Used"),

            StatusCode::MultipleChoices300 => write!(f, "300 Multiple Choices"),
            StatusCode::MovedPermanently301 => write!(f, "301 Moved Permanently"),
            StatusCode::Found302 => write!(f, "302 Found"),
            StatusCode::SeeOther303 => write!(f, "303 See Other"),
            StatusCode::NotModified304 => write!(f, "304 Not Modified"),
            StatusCode::UseProxy305 => write!(f, "305 Use Proxy"),
            StatusCode::TemporaryRedirect307 => write!(f, "307 Temporary Redirect"),
            StatusCode::PermanentRedirect308 => write!(f, "308 Permanent Redirect"),

            StatusCode::BadRequest400 => write!(f, "400 Bad Request"),
            StatusCode::Unauthorized401 => write!(f, "401 Unauthorized"),
            StatusCode::PaymentRequired402 => write!(f, "402 Payment Required"),
            StatusCode::Forbidden403 => write!(f, "403 Forbidden"),
            StatusCode::NotFound404 => write!(f, "404 Not Found"),
            StatusCode::MethodNotAllowed405 => write!(f, "405 Method Not Allowed"),
            StatusCode::NotAcceptable406 => write!(f, "406 Not Acceptable"),
            StatusCode::ProxyAuthenticationRequired407 => {
                write!(f, "407 Proxy Authentication Required")
            }
            StatusCode::RequestTimeout408 => write!(f, "408 Request Timeout"),
            StatusCode::Conflict409 => write!(f, "409 Conflict"),
            StatusCode::Gone410 => write!(f, "410 Gone"),
            StatusCode::LengthRequired411 => write!(f, "411 Length Required"),
            StatusCode::PrecondiditionFailed412 => write!(f, "412 Precondition Failed"),
            StatusCode::PayloadTooLarge413 => write!(f, "413 Payload Too Large"),
            StatusCode::UriTooLong414 => write!(f, "414 URI Too Long"),
            StatusCode::UnsupportedMediaType415 => write!(f, "415 Unsupported Media Type"),
            StatusCode::RangeNotSatisfiable416 => write!(f, "416 Range Not Satisfiable"),
            StatusCode::ExpectationFailed417 => write!(f, "417 Expectation Failed"),
            StatusCode::MisdirectedRequest421 => write!(f, "421 Misdirected Request"),
            StatusCode::UnprocessableEntity422 => write!(f, "422 Unprocessable Entity"),
            StatusCode::Locked423 => write!(f, "423 Locked"),
            StatusCode::FailedDependency424 => write!(f, "424 Failed Dependency"),
            StatusCode::TooEarly425 => write!(f, "425 Too Early"),
            StatusCode::UpgradeRequired426 => write!(f, "426 Upgrade Required"),
            StatusCode::PreconditionRequired428 => write!(f, "428 Precondition Required"),
            StatusCode::TooManyRequests429 => write!(f, "429 Too Many Requests"),
            StatusCode::RequestHeaderFieldsTooLarge431 => {
                write!(f, "431 Request Header Fields Too Large")
            }
            StatusCode::UnavailableForLegalReasons451 => {
                write!(f, "451 Unavailable For Legal Reasons")
            }
            StatusCode::InternalServerError500 => write!(f, "500 Internal Server Error"),
            StatusCode::NotImplemented501 => write!(f, "501 Not Implemented"),
            StatusCode::BadGateway502 => write!(f, "502 Bad Gateway"),
            StatusCode::ServiceUnavailable503 => write!(f, "503 Service Unavailable"),
            StatusCode::GatewayTimeout504 => write!(f, "504 Gateway Timeout"),
            StatusCode::HttpVersionNotSupported505 => write!(f, "505 HTTP Version Not Supported"),
            StatusCode::VariantAlsoNegotiates506 => write!(f, "506 Variant Also Negotiates"),
            StatusCode::InsufficientStorage507 => write!(f, "507 Insufficient Storage"),
            StatusCode::LoopDetected508 => write!(f, "508 Loop Detected"),
            StatusCode::NotExtended510 => write!(f, "510 Not Extended"),
            StatusCode::NetworkAuthenticationRequired511 => {
                write!(f, "511 Network Authentication Required")
            }
        }
    }
}
