//! Provides buffered reading functionality for TCP streams.
//!
//! This module implements line-by-line and complete content reading
//! capabilities over TCP connections.

use std::{io::Read, net::TcpStream};

/// A buffered reader for TCP streams that provides convenient reading operations.
///
/// # Examples
///
/// ```
/// use std::net::TcpStream;
///
/// let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
/// let mut buffer = StreamBuffer::new(stream);
///
/// // Read a line
/// let line = buffer.read_line().unwrap();
/// ```
pub struct StreamBuffer {
    stream: TcpStream,
}

impl StreamBuffer {
    /// Creates a new StreamBuffer from a TcpStream.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream to wrap
    pub fn new(stream: TcpStream) -> Self {
        StreamBuffer { stream }
    }

    /// Reads a single line from the stream until a newline character is encountered.
    ///
    /// The returned string has whitespace trimmed from both ends and does not include
    /// the newline character.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The line that was read
    /// * `Err(std::io::Error)` - If an I/O error occurs during reading
    pub fn read_line(&mut self) -> Result<String, std::io::Error> {
        let mut buffer = String::new();

        let mut buf = [0x00; 1];
        loop {
            let bytes = self.stream.read(&mut buf)?;
            if bytes == 0 {
                break;
            }

            let c = buf[0] as char;
            if c == '\n' {
                break;
            }

            buffer.push(c);
        }

        Ok(buffer.trim().to_string())
    }

    /// Reads all remaining bytes from the stream into a vector.
    ///
    /// This method will read until EOF is reached.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - The bytes that were read
    /// * `Err(std::io::Error)` - If an I/O error occurs during reading
    pub fn read_all(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        self.stream.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /// Reads all remaining data from the stream as a UTF-8 string.
    ///
    /// This method will read until EOF is reached and attempt to decode
    /// the bytes as UTF-8.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The decoded string
    /// * `Err(std::io::Error)` - If an I/O error occurs during reading
    ///                           or if the data is not valid UTF-8
    pub fn read_all_string(&mut self) -> Result<String, std::io::Error> {
        let mut buffer = String::new();
        self.stream.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
