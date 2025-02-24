//! HTTP Response handling module.
//!
//! This module provides functionality for parsing and handling HTTP responses
//! received from a server over a TCP connection.

use std::net::TcpStream;

use crate::{
    internal::StreamBuffer,
    utils::{triple_split, tuple_split},
};

use super::{HttpHeaders, StatusCode};

/// Represents an HTTP response received from a server.
///
/// This struct contains the parsed status code, headers, and maintains a buffer
/// for reading the response body.
pub struct HttpResponse {
    /// The HTTP status code of the response
    pub status: StatusCode,
    /// The HTTP headers included in the response
    pub headers: HttpHeaders,

    /// Internal buffer for reading response data
    buffer: StreamBuffer,
}

/// Errors that can occur while parsing an HTTP response.
#[derive(Debug, PartialEq)]
pub enum ResponseError {
    /// The status line was malformed or could not be parsed
    InvalidStatusLine,
    /// One or more headers were malformed or could not be parsed
    InvalidHeader,
    /// The response body could not be read or parsed
    InvalidBody,
}

impl HttpResponse {
    /// Builds a new HttpResponse from a TCP stream.
    ///
    /// This method reads and parses the status line and headers from the stream.
    /// The body can be read later using the `body()` or `body_as_string()` methods.
    ///
    /// # Arguments
    /// * `stream` - A TcpStream connected to the server
    ///
    /// # Returns
    /// * `Ok(HttpResponse)` if parsing was successful
    /// * `Err(ResponseError)` if any parsing errors occurred
    pub fn build(stream: TcpStream) -> Result<Self, ResponseError> {
        let mut buffer = StreamBuffer::new(stream);

        let status_line = buffer
            .read_line()
            .map_err(|_| ResponseError::InvalidStatusLine)?;
        let (_http_version, status, _) =
            triple_split(&status_line, " ").ok_or(ResponseError::InvalidStatusLine)?;
        let status = status
            .parse::<u16>()
            .map_err(|_| ResponseError::InvalidStatusLine)?;
        let status = status
            .try_into()
            .map_err(|_| ResponseError::InvalidStatusLine)?;

        let mut headers = HttpHeaders::new();

        loop {
            let line = buffer
                .read_line()
                .map_err(|_| ResponseError::InvalidHeader)?;
            let line = line.trim();

            if line.is_empty() {
                break;
            }

            let (key, value) = tuple_split(line, ":").ok_or(ResponseError::InvalidHeader)?;
            let key = key.trim();
            let value = value.trim();
            headers.insert(key.to_string(), value.to_string());
        }

        // Check for a Content-Length header to set the total bytes to read
        if let Some(content_length) = headers.get("Content-Length") {
            if let Ok(content_length) = content_length.parse::<usize>() {
                buffer.set_total_bytes(content_length);
            }
        }

        Ok(HttpResponse {
            status,
            headers,
            buffer,
        })
    }

    /// Reads the response body as a vector of bytes.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` containing the raw body data
    /// * `Err(ResponseError)` if the body cannot be read
    pub fn body(&mut self) -> Result<Vec<u8>, ResponseError> {
        self.buffer
            .read_all()
            .map_err(|_| ResponseError::InvalidBody)
    }

    /// Reads the response body and converts it to a String.
    ///
    /// # Returns
    /// * `Ok(String)` containing the body as a UTF-8 string
    /// * `Err(ResponseError)` if the body cannot be read or is not valid UTF-8
    pub fn body_as_string(&mut self) -> Result<String, ResponseError> {
        self.buffer
            .read_all_string()
            .map_err(|_| ResponseError::InvalidBody)
    }
}
