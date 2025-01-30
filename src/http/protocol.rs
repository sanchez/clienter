use std::str::FromStr;

#[derive(Debug, PartialEq)]
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
