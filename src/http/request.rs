use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

use super::headers::HttpHeaders;
use super::method::HttpMethod;
use super::response::HttpResponse;
use super::uri::Uri;

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub uri: Uri,
    pub headers: HttpHeaders,

    pub timeout: Option<std::time::Duration>,
}

#[derive(Debug, PartialEq)]
pub enum RequestError {
    InvalidUri,
    ConnectionFailed,
    InternalError,
}

impl HttpRequest {
    pub fn new<T>(method: HttpMethod, uri: T) -> Self
    where
        T: Into<Uri>,
    {
        HttpRequest {
            method,
            uri: uri.into(),
            headers: HttpHeaders::default(),

            timeout: None,
        }
    }

    pub fn execute(&self) -> Result<HttpResponse, RequestError> {
        let addr = self
            .uri
            .get_addr()
            .to_socket_addrs()
            .map_err(|_| RequestError::InvalidUri)?
            .next()
            .ok_or(RequestError::InvalidUri)?;

        let mut stream = match self.timeout {
            Some(x) => TcpStream::connect_timeout(&addr, x),
            None => TcpStream::connect(addr),
        }
        .map_err(|_| RequestError::ConnectionFailed)?;

        /* Example Raw HTTP Request:
            GET / HTTP/2
            Host: httpbin.org
            User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:134.0) Gecko/20100101 Firefox/134.0
            Accept: text/html,application/xhtml+xml,application/xml;q=0.9
            Accept-Language: en-US,en;q=0.5
            Accept-Encoding: gzip, deflate, br, zstd
            Connection: keep-alive
            Upgrade-Insecure-Requests: 1
            Sec-Fetch-Dest: document
            Sec-Fetch-Mode: navigate
            Sec-Fetch-Site: none
            Sec-Fetch-User: ?1
            Sec-GPC: 1
            Priority: u=0, i
        */

        let uri = format!("/{}", self.uri.get_encoded_path());
        println!("Before");
        let method = self.method.to_string();
        println!("Method: {}", method);
        println!("After");

        write!(stream, "{} {} HTTP/2\r\n", self.method, uri)
            .map_err(|_| RequestError::InternalError)?;

        for (key, value) in self.headers.iter() {
            write!(stream, "{}: {}\r\n", *key, *value).map_err(|_| RequestError::InternalError)?;
        }

        write!(stream, "\r\n\r\n").map_err(|_| RequestError::InternalError)?;
        stream.flush().map_err(|_| RequestError::InternalError)?;

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_request() {
        let request = HttpRequest::new(HttpMethod::GET, "http://httpbin.org/anything");

        let response = request.execute().unwrap();
        assert_eq!(response.status, super::super::StatusCode::Ok200);
    }
}
