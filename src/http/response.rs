use std::net::TcpStream;

use crate::{
    internal::StreamBuffer,
    utils::{triple_split, tuple_split},
};

use super::{HttpHeaders, StatusCode};

pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HttpHeaders,

    buffer: StreamBuffer,
}

#[derive(Debug, PartialEq)]
pub enum ResponseError {
    InvalidStatusLine,
    InvalidHeader,
    InvalidBody,
}

impl HttpResponse {
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

        Ok(HttpResponse {
            status,
            headers,
            buffer,
        })
    }

    pub fn body(&mut self) -> Result<Vec<u8>, ResponseError> {
        self.buffer
            .read_all()
            .map_err(|_| ResponseError::InvalidBody)
    }

    pub fn body_as_string(&mut self) -> Result<String, ResponseError> {
        self.buffer
            .read_all_string()
            .map_err(|_| ResponseError::InvalidBody)
    }
}
