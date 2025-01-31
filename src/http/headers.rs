use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct HttpHeaders {
    data: HashMap<String, String>,
}

impl HttpHeaders {
    pub fn new() -> Self {
        HttpHeaders {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn set_host(&mut self, host: String) {
        self.insert("Host".to_string(), host);
    }

    pub fn set_user_agent(&mut self, user_agent: String) {
        self.insert("User-Agent".to_string(), user_agent);
    }

    pub fn set_accept(&mut self, accept: String) {
        self.insert("Accept".to_string(), accept);
    }

    pub fn set_accept_language(&mut self, accept_language: String) {
        self.insert("Accept-Language".to_string(), accept_language);
    }

    pub fn set_accept_encoding(&mut self, accept_encoding: String) {
        self.insert("Accept-Encoding".to_string(), accept_encoding);
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.data.iter()
    }
}

impl Default for HttpHeaders {
    fn default() -> Self {
        HttpHeaders {
            data: HashMap::from([
                ("User-Agent".to_string(), "Clienter/1.0".to_string()),
                ("Accept-Language".to_string(), "en-US".to_string()),
                ("Accept-Encoding".to_string(), "gzip".to_string()),
                ("Connection".to_string(), "keep-alive".to_string()),
                ("Upgrade-Insecure-Requests".to_string(), "1".to_string()),
                ("Sec-Fetch-Dest".to_string(), "document".to_string()),
            ]),
        }
    }
}

impl From<HashMap<String, String>> for HttpHeaders {
    fn from(data: HashMap<String, String>) -> Self {
        HttpHeaders { data }
    }
}

// impl Iterator for HttpHeaders {
//     type Item = (String, String);

//     fn next(&mut self) -> Option<Self::Item> {
//         self.data.iter().next().map(|(k, v)| (k.clone(), v.clone()))
//     }
// }

impl IntoIterator for HttpHeaders {
    type Item = (String, String);
    type IntoIter = std::collections::hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
