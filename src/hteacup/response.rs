use super::HttpStatus;
use super::VERSION;
use std::collections::HashMap;

pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub content: Vec<u8>,
    raw: Option<Vec<u8>>,
    is_raw: bool,
}

impl HttpResponse {
    pub fn new<B: AsRef<[u8]>>(
        status: HttpStatus,
        content: B,
        headers: Option<HashMap<String, String>>,
    ) -> Self {
        let mut headers = headers.unwrap_or(HashMap::new());
        let content = content.as_ref();
        headers.insert("Content-Length".to_string(), content.len().to_string());
        headers.insert(
            "Server".to_string(),
            format!("HTeaPot/{}", VERSION).to_string(),
        );
        HttpResponse {
            status,
            headers,
            content: content.to_owned(),
            raw: None,
            is_raw: false,
        }
    }

    pub fn new_raw(raw: Vec<u8>) -> Self {
        HttpResponse {
            status: HttpStatus::IAmATeapot,
            headers: HashMap::new(),
            content: vec![],
            raw: Some(raw),
            is_raw: true,
        }
    }

    pub fn is_raw(&self) -> bool {
        self.is_raw
    }

    pub fn psrse_raw(&mut self) {
        if !self.is_raw() {
            return;
        };
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        if self.is_raw() {
            return self.raw.clone().unwrap();
        }
        let mut headers_text = String::new();
        for (key, value) in self.headers.iter() {
            headers_text.push_str(&format!("{}: {}\r\n", key, value));
        }
        let response_header = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n",
            self.status as u16,
            self.status.to_string(),
            headers_text
        );
        let mut response = Vec::new();
        response.extend_from_slice(response_header.as_bytes());
        response.append(&mut self.content.clone());
        response.push(0x0D); // Carriage Return
        response.push(0x0A); // Line Feed
        response
    }
}
