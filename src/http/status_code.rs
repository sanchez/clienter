use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum StatusCode {
    Continue100,
    SwitchingProtocols101,
    Processing102,
    EarlyHints103,

    Ok200,
    Created201,
    Accepted202,
    NonAuthoritativeInformation203,
    NoContent204,
    ResetContent205,
    PartialContent206,
    MultiStatus207,
    AlreadyReported208,
    ImUsed226,

    MultipleChoices300,
    MovedPermanently301,
    Found302,
    SeeOther303,
    NotModified304,
    UseProxy305,
    TemporaryRedirect307,
    PermanentRedirect308,

    BadRequest400,
    Unauthorized401,
    PaymentRequired402,
    Forbidden403,
    NotFound404,
    MethodNotAllowed405,
    NotAcceptable406,
    ProxyAuthenticationRequired407,
    RequestTimeout408,
    Conflict409,
    Gone410,
    LengthRequired411,
    PrecondiditionFailed412,
    PayloadTooLarge413,
    UriTooLong414,
    UnsupportedMediaType415,
    RangeNotSatisfiable416,
    ExpectationFailed417,
    MisdirectedRequest421,
    UnprocessableEntity422,
    Locked423,
    FailedDependency424,
    TooEarly425,
    UpgradeRequired426,
    PreconditionRequired428,
    TooManyRequests429,
    RequestHeaderFieldsTooLarge431,
    UnavailableForLegalReasons451,

    InternalServerError500,
    NotImplemented501,
    BadGateway502,
    ServiceUnavailable503,
    GatewayTimeout504,
    HttpVersionNotSupported505,
    VariantAlsoNegotiates506,
    InsufficientStorage507,
    LoopDetected508,
    NotExtended510,
    NetworkAuthenticationRequired511,
}

impl StatusCode {
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
