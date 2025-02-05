use super::headers::HttpHeaders;
use super::method::HttpMethod;
use super::uri::Uri;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub uri: Uri,
    pub headers: HttpHeaders,

    pub timeout: Option<std::time::Duration>,
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

    pub fn get_request_line(&self) -> String {
        let uri = format!("/{}", self.uri.get_encoded_path());
        let version = self.uri.protocol.get_http_version();
        format!("{} {} {}", self.method, uri, version)
    }
}
