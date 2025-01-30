use super::method::HttpMethod;

#[derive(Debug, PartialEq)]
pub struct Request {
    method: HttpMethod,
}
