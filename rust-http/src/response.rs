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
