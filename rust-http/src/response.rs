use std::collections::HashMap;

// Struct ro represent an HTTP response
#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(status_code: u16, headers: HashMap<String, String>, body: Option<String>) -> Self {
        HttpResponse { status_code, headers, body }
    }

    pub fn to_string(&self) -> String {
        let status_text = match self.status_code {
            100 => "Continue",
            101 => "Switching Protocols",
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            204 => "No Content",
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            304 => "Not Modified",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "Unknown Status",
        };
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, status_text);
        
        // Create a mutable copy of headers
        let mut headers = self.headers.clone();
        
        // Add Content-Length header if there's a body
        if let Some(body) = &self.body {
            headers.entry("Content-Length".to_string())
                .or_insert_with(|| body.len().to_string());
        }
        
        // Add headers to the response
        for (key, value) in headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        response.push_str("\r\n");
        if let Some(body) = &self.body {
            response.push_str(body);
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_http_response_new() {
        let status_code = 200;
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let body = Some(r#"{"message": "Hello, world!"}"#.to_string());
    
        // Create a new HttpResponse instance
        let response = HttpResponse::new(status_code, headers.clone(), body.clone());
    
        // Assert: verify the response values
        assert_eq!(response.status_code, status_code);
        assert_eq!(response.headers, headers);
        assert_eq!(response.body, body);
    }

    #[test]
    fn test_http_response_to_string_with_body() {
        // Arrange
        let status_code = 200;
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let body = Some(r#"{"message": "Hello, world!"}"#.to_string());

        // Create a new HttpResponse instance
        let response = HttpResponse::new(status_code, headers, body.clone());
        let response_string = response.to_string();

        // Assert: verify key parts of the response
        assert!(response_string.contains("HTTP/1.1 200 OK"));
        assert!(response_string.contains("Content-Type: application/json"));
        assert!(response_string.contains(&format!("Content-Length: {}", body.clone().unwrap().len())));
        assert!(response_string.contains(r#"{"message": "Hello, world!"}"#));
    }

    #[test]
    fn test_http_response_to_string_without_body() {
        let status_code = 204;
        let headers = HashMap::new();
        let body = None;

        // Create a new HttpResponse instance
        let response = HttpResponse::new(status_code, headers, body);
        let response_string = response.to_string();

        // Assert: verify key parts of the response
        let expected_response = "HTTP/1.1 204 No Content\r\n\r\n";
        assert_eq!(response_string, expected_response);
    }
}