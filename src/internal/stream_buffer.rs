//! Provides buffered reading functionality for TCP streams.
//!
//! This module implements line-by-line and complete content reading
//! capabilities over TCP connections.

use std::{
    io::{ErrorKind, Read},
    net::TcpStream,
};

/// A buffered reader for TCP streams that provides convenient reading operations.
///
/// # Examples
///
/// ```ignore
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
    bytes_read: usize,
    total_bytes: Option<usize>,
}

impl StreamBuffer {
    /// Creates a new StreamBuffer from a TcpStream.
    ///
    /// # Arguments
    ///
    /// * `stream` - The TCP stream to wrap
    pub fn new(stream: TcpStream) -> Self {
        StreamBuffer {
            stream,
            bytes_read: 0,
            total_bytes: None,
        }
    }

    /// Sets the total number of bytes expected to be read from the stream.
    ///
    /// This is useful when you know the content length in advance and want to
    /// prevent reading beyond the expected data size.
    ///
    /// # Arguments
    ///
    /// * `total_bytes` - The total number of bytes that should be read from the stream
    pub fn set_total_bytes(&mut self, total_bytes: usize) {
        self.total_bytes = Some(total_bytes);
    }

    /// Reads a single byte from the stream.
    ///
    /// This is an internal helper method that maintains the bytes_read count
    /// while reading individual bytes from the underlying TCP stream.
    ///
    /// # Returns
    ///
    /// * `Ok(u8)` - The byte that was read
    /// * `Err(std::io::Error)` - If an I/O error occurs during reading
    fn get_byte(&mut self) -> Result<u8, std::io::Error> {
        // If we have already read past the max, no need to keep going
        if let Some(total_bytes) = self.total_bytes {
            if self.bytes_read >= total_bytes {
                return Err(std::io::Error::new(
                    ErrorKind::UnexpectedEof,
                    "End of file reached",
                ));
            }
        }

        let mut buf = [0x00; 1];
        self.stream.read_exact(&mut buf)?;
        self.bytes_read += 1;
        Ok(buf[0])
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

        loop {
            let c = match self.get_byte() {
                Ok(byte) => byte as char,
                Err(err) if err.kind() == ErrorKind::UnexpectedEof => break,
                Err(err) => return Err(err),
            };

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
        // If we know the length of the data, we only need to read that much and can close out the connection early
        if let Some(total_bytes) = self.total_bytes {
            let mut buffer = vec![0; total_bytes];
            self.stream.read_exact(&mut buffer)?;
            return Ok(buffer);
        }

        // We don't know how many bytes are left, we need to keep reading
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
        let bytes = self.read_all()?;
        let s = std::str::from_utf8(&bytes)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?
            .to_owned();
        Ok(s)
    }
}
