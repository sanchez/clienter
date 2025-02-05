use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};

use super::{HttpHeaders, HttpMethod, HttpRequest, HttpResponse, Uri};

pub struct HttpClient {
    pub timeout: Option<std::time::Duration>,
    pub headers: HttpHeaders,
}

#[derive(Debug, PartialEq)]
pub enum HttpError {
    InvalidUri,
    ConnectionFailed,

    UnknownError,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            timeout: None,
            headers: HttpHeaders::default(),
        }
    }

    pub fn request<T>(&self, method: HttpMethod, uri: T) -> HttpRequest
    where
        T: Into<Uri>,
    {
        HttpRequest::new(method, uri)
    }

    pub fn send(&self, request: &HttpRequest) -> Result<HttpResponse, HttpError> {
        let addr = request
            .uri
            .get_addr()
            .to_socket_addrs()
            .map_err(|_| HttpError::InvalidUri)?
            .next()
            .ok_or(HttpError::InvalidUri)?;

        let mut stream = match self.timeout {
            Some(x) => TcpStream::connect_timeout(&addr, x),
            None => TcpStream::connect(addr),
        }
        .map_err(|_| HttpError::ConnectionFailed)?;

        let request_line = request.get_request_line();
        write!(stream, "{}\r\n", request_line).map_err(|_| HttpError::UnknownError)?;

        let headers = self.headers.combine(&request.headers);
        for (key, value) in headers.iter() {
            write!(stream, "{}: {}\r\n", *key, *value).map_err(|_| HttpError::UnknownError)?;
        }

        write!(stream, "\r\n\r\n").map_err(|_| HttpError::UnknownError)?;
        stream.flush().map_err(|_| HttpError::UnknownError)?;

        let response = HttpResponse::build(stream).map_err(|_| HttpError::UnknownError)?;

        Ok(response)
    }
}
