use std::{io::Read, net::TcpStream};

pub struct StreamBuffer {
    stream: TcpStream,
}

impl StreamBuffer {
    pub fn new(stream: TcpStream) -> Self {
        StreamBuffer { stream }
    }

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
}
