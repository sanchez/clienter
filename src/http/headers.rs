//! HTTP headers implementation for managing request and response headers.
//!
//! This module provides a container for HTTP headers with convenience methods
//! for setting common headers and combining header sets.
//!
//! # Example
//! ```
//! use clienter::HttpHeaders;
//!
//! let mut headers = HttpHeaders::new();
//! headers.set_user_agent("MyApp/1.0".to_string());
//! headers.set_accept("text/html".to_string());
//! ```

use std::collections::HashMap;

/// A container for HTTP headers that provides convenient methods for
/// managing and manipulating HTTP header fields.
#[derive(Debug, PartialEq, Clone)]
pub struct HttpHeaders {
    /// Internal storage for header key-value pairs
    data: HashMap<String, String>,
}

impl HttpHeaders {
    /// Creates a new empty headers container.
    pub fn new() -> Self {
        HttpHeaders {
            data: HashMap::new(),
        }
    }

    /// Combines two header sets, with the other set taking precedence for duplicate keys.
    ///
    /// # Parameters
    /// * `other` - Another headers container to merge with this one
    ///
    /// # Returns
    /// A new `HttpHeaders` instance containing the merged headers
    pub fn combine(&self, other: &HttpHeaders) -> HttpHeaders {
        let mut data = self.data.clone();
        for (key, value) in other.data.iter() {
            data.insert(key.clone(), value.clone());
        }
        HttpHeaders { data }
    }

    /// Inserts a header key-value pair into the container.
    ///
    /// # Parameters
    /// * `key` - The header field name
    /// * `value` - The header field value
    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Retrieves the value of a header by its key.
    ///
    /// # Parameters
    /// * `key` - The header field name to look up
    ///
    /// # Returns
    /// An Option containing a reference to the header value if it exists
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// Sets the Host header.
    pub fn set_host(&mut self, host: String) {
        self.insert("Host".to_string(), host);
    }

    /// Sets the User-Agent header.
    pub fn set_user_agent(&mut self, user_agent: String) {
        self.insert("User-Agent".to_string(), user_agent);
    }

    /// Sets the Accept header.
    pub fn set_accept(&mut self, accept: String) {
        self.insert("Accept".to_string(), accept);
    }

    /// Sets the Accept-Language header.
    pub fn set_accept_language(&mut self, accept_language: String) {
        self.insert("Accept-Language".to_string(), accept_language);
    }

    /// Sets the Accept-Encoding header.
    pub fn set_accept_encoding(&mut self, accept_encoding: String) {
        self.insert("Accept-Encoding".to_string(), accept_encoding);
    }

    /// Returns an iterator over the header key-value pairs.
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.data.iter()
    }
}

/// Provides default headers commonly used in HTTP requests.
impl Default for HttpHeaders {
    fn default() -> Self {
        HttpHeaders {
            data: HashMap::from([
                ("User-Agent".to_string(), "Clienter/1.0 (Rust)".to_string()),
                ("Accept".to_string(), "*/*".to_string()),
                ("Accept-Language".to_string(), "en-US".to_string()),
                ("Accept-Encoding".to_string(), "gzip".to_string()),
                ("Connection".to_string(), "keep-alive".to_string()),
                ("Upgrade-Insecure-Requests".to_string(), "1".to_string()),
                ("Sec-Fetch-Dest".to_string(), "document".to_string()),
                ("Host".to_string(), "localhost".to_string()),
            ]),
        }
    }
}

/// Allows creation of HttpHeaders from a HashMap.
impl From<HashMap<String, String>> for HttpHeaders {
    fn from(data: HashMap<String, String>) -> Self {
        HttpHeaders { data }
    }
}

/// Enables iteration over header key-value pairs.
impl IntoIterator for HttpHeaders {
    type Item = (String, String);
    type IntoIter = std::collections::hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
