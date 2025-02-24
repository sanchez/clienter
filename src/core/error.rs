/// Represents possible errors that can occur during HTTP operations.
#[derive(Debug, PartialEq)]
pub enum HttpError {
    /// The provided URI is invalid or cannot be parsed
    InvalidUri,
    /// Failed to establish a TCP connection to the server
    ConnectionFailed,
    /// An unexpected error occurred during the operation
    UnknownError,
}
