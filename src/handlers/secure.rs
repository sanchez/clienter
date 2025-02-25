use crate::{HttpClient, HttpError, HttpRequest, HttpResponse};
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};

// This has been super useful: https://tls12.xargs.org/#client-hello/annotated

fn generate_random_bytes(len: usize) -> Vec<u8> {
    (0..len)
        .map(|_| {
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            (nanos & 0xff) as u8
        })
        .collect()
}

fn push_u16(buf: &mut Vec<u8>, value: u16) {
    buf.push((value >> 8) as u8);
    buf.push((value & 0xff) as u8);
}

fn calculate_client_hello_extensions(request: &HttpRequest) -> Vec<u8> {
    let server_name = calculate_client_hello_extensions_server_name(request);
    let status_request = calculate_client_hello_extensions_status_request(request);

    let mut extensions: Vec<u8> = vec![];
    let total_length = server_name.len() + status_request.len();
    push_u16(&mut extensions, total_length as u16);

    extensions.extend_from_slice(&server_name);
    extensions.extend_from_slice(&status_request);

    extensions
}

fn calculate_client_hello_extensions_server_name(request: &HttpRequest) -> Vec<u8> {
    let hostname = request.uri.hostname.as_bytes();
    let hostname_length = hostname.len() as u16;

    let mut extensions: Vec<u8> = vec![
        0x00, 0x00, // Server Name Indication (SNI) extension
    ];

    push_u16(&mut extensions, hostname_length + 5); // number of hostname bytes to follow
    push_u16(&mut extensions, hostname_length + 3); // number of list entry bytes to follow
    extensions.push(0x00); // list entry is type DNS Hostname

    // Hostname stuff
    push_u16(&mut extensions, hostname_length); // length of hostname
    extensions.extend_from_slice(hostname);

    extensions
}

fn calculate_client_hello_extensions_status_request(request: &HttpRequest) -> Vec<u8> {
    vec![
        0x00, 0x05, // Status Request extension
        0x00, 0x05, // 5 bytes of status request follows
        0x01, // OCSP
        0x00, 0x00, // responder id information
        0x00, 0x00, // request extension information
    ]
}

fn handshake_client_hello(stream: &mut TcpStream, request: &HttpRequest) -> Result<(), HttpError> {
    let random_bytes = generate_random_bytes(32);

    let header = [
        // Record header
        0x16, // Handshake record
        0x03, 0x01, // TLS 1.0 (for initial handshake)
        0x00, 0xa5, // 0xA5 (165) bytes of handshake message follows
        //
        // Handshake record
        0x01, // ClientHello
        0x00, 0x00, 0xa1, // 0xA1 (161) bytes of handshake message follows
        //
        // Client Version
        0x03, 0x03, // Protocol version "3,3" (TLS 1.2)
        //
        // Random
        // TODO: Need to make this truly random
        0xE5, 0xD7, 0xFC, 0x4F, 0xAE, 0xAD, 0x37, 0xD6, 0x6B, 0x1F, 0x23, 0x2C, 0x1B, 0xC5, 0x04,
        0x5B, 0xB2, 0x6C, 0xD1, 0xD5, 0x69, 0x24, 0xB9, 0x69, 0x2D, 0x35, 0xC1, 0x9C, 0x8A, 0x1F,
        0xA9, 0xB4, //
        //
        // Session
        0x00, // Session ID
        //
        // Cipher Suites
        0x00, 0x02, // 2 cipher suites
        0x00, 0x2f, // TLS_RSA_WITH_AES_128_CBC_SHA
        0x00, 0x35, // TLS_RSA_WITH_AES_256_CBC_SHA
        //
        // Compression Methods
        0x01, // 1 compression method
        0x00, // No compression
        //
        // Extensions
        0x00, 0x2b, // 43 bytes of extensions
    ];

    Ok(())
}

pub fn handle_https(client: &HttpClient, request: &HttpRequest) -> Result<HttpResponse, HttpError> {
    let addr = request
        .uri
        .get_addr()
        .to_socket_addrs()
        .map_err(|_| HttpError::InvalidUri)?
        .next()
        .ok_or(HttpError::InvalidUri)?;

    println!("Connecting to {:?}", addr);

    let mut stream = match client.timeout {
        Some(x) => TcpStream::connect_timeout(&addr, x),
        None => TcpStream::connect(addr),
    }
    .map_err(|_| HttpError::ConnectionFailed)?;

    handshake_client_hello(&mut stream)?;

    todo!()
}
