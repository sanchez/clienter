use crate::{HttpClient, HttpError, HttpRequest, HttpResponse};
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};

pub fn handle_http(client: &HttpClient, request: &HttpRequest) -> Result<HttpResponse, HttpError> {
    let addr = request
        .uri
        .get_addr()
        .to_socket_addrs()
        .map_err(|_| HttpError::InvalidUri)?
        .next()
        .ok_or(HttpError::InvalidUri)?;

    let mut stream = match client.timeout {
        Some(x) => TcpStream::connect_timeout(&addr, x),
        None => TcpStream::connect(addr),
    }
    .map_err(|_| HttpError::ConnectionFailed)?;

    let request_line = request.get_request_line();
    write!(stream, "{}\r\n", request_line).map_err(|_| HttpError::UnknownError)?;

    let headers = client.headers.combine(&request.headers);
    for (key, value) in headers.iter() {
        write!(stream, "{}: {}\r\n", *key, *value).map_err(|_| HttpError::UnknownError)?;
    }

    write!(stream, "\r\n\r\n").map_err(|_| HttpError::UnknownError)?;
    stream.flush().map_err(|_| HttpError::UnknownError)?;

    let response = HttpResponse::build(stream).map_err(|_| HttpError::UnknownError)?;

    Ok(response)
}
