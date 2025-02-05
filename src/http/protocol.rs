use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Protocol {
    HTTP,
    HTTPS,
}

impl FromStr for Protocol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http" => Ok(Protocol::HTTP),
            "https" => Ok(Protocol::HTTPS),
            _ => Err(()),
        }
    }
}

impl Protocol {
    pub fn get_default_port(&self) -> u16 {
        match self {
            Protocol::HTTP => 80,
            Protocol::HTTPS => 443,
        }
    }

    pub fn get_http_version(&self) -> &'static str {
        match self {
            Protocol::HTTP => "HTTP/1.1",
            Protocol::HTTPS => "HTTP/2",
        }
    }
}
