mod methods;
mod request;
mod response;
mod status;

use std::collections::HashMap;

pub use self::methods::HttpMethod;
pub use self::request::HttpRequest;
pub use self::response::HttpResponse;
pub use self::status::HttpStatus;

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl HttpRequest {
    pub fn new(method: HttpMethod, path: &str) -> HttpRequest {
        let path = path.to_string();
        HttpRequest {
            method,
            path,
            args: HashMap::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn arg(&mut self, key: &str, value: &str) -> &mut HttpRequest {
        self.args.insert(key.to_string(), value.to_string());
        return self;
    }

    pub fn header(&mut self, key: &str, value: &str) -> &mut HttpRequest {
        self.headers.insert(key.to_string(), value.to_string());
        return self;
    }

    pub fn body(&mut self, body: String) -> &mut HttpRequest {
        self.body = body;
        return self;
    }

    pub fn to_string(&self) -> String {
        let path = if self.args.is_empty() {
            self.path.clone()
        } else {
            let mut path = self.path.clone();
            path.push('?');
            for (k, v) in self.args.iter() {
                path.push_str(format!("{}={}&", k, v).as_str());
            }
            if path.ends_with('&') {
                path.pop();
            }
            path
        };
        let mut result: String = format!("{} {} HTTP/1.1\n", self.method.to_str(), path);
        for (k, v) in self.headers.iter() {
            result.push_str(format!("{}: {}\n", k, v).as_str());
        }
        result.push('\n');
        result.push_str(self.body.as_str());
        result
    }

    pub fn brew(self) -> Result<HttpResponse, &'static str> {
        Err("the batimamaste borra este error luego")
    }
}

pub fn brew(request: HttpRequest) -> Result<HttpResponse, &'static str> {
    return request.brew();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_new() {
        let request = HttpRequest::new(HttpMethod::GET, "/example");
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path, "/example");
        assert!(request.args.is_empty());
        assert!(request.headers.is_empty());
        assert_eq!(request.body, "");
    }

    #[test]
    fn test_http_request_arg() {
        let mut request = HttpRequest::new(HttpMethod::POST, "/submit");
        request.arg("key", "value");
        assert_eq!(request.args.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_http_request_header() {
        let mut request = HttpRequest::new(HttpMethod::GET, "/data");
        request.header("Content-Type", "application/json");
        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_http_request_body() {
        let mut request = HttpRequest::new(HttpMethod::POST, "/upload");
        request.body("Test body content".to_string());
        assert_eq!(request.body, "Test body content");
    }

    #[test]
    fn test_http_request_to_string() {
        let mut request = HttpRequest::new(HttpMethod::POST, "/resource");
        request
            .header("Content-Type", "application/json")
            .body("{\"data\":\"test\"}".to_string());

        let request_string = request.to_string();
        assert!(request_string.contains("POST /resource HTTP/1.1"));
        assert!(request_string.contains("Content-Type: application/json"));
        assert!(request_string.contains("{\"data\":\"test\"}"));
    }

    #[test]
    fn test_http_request_to_string_with_args() {
        let mut request = HttpRequest::new(HttpMethod::POST, "/resource");
        request
            .header("Content-Type", "application/json")
            .arg("key", "value")
            .body("{\"data\":\"test\"}".to_string());

        let request_string = request.to_string();
        assert!(request_string.contains("POST /resource?key=value HTTP/1.1"));
        assert!(request_string.contains("Content-Type: application/json"));
        assert!(request_string.contains("{\"data\":\"test\"}"));
    }
}
