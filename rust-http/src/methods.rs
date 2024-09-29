use std::fs;
use std::path::Path;
use serde_json;
use std::collections::HashMap;
use crate::models::HttpResponse;

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

// Function to handle GET requests
pub fn handle_get(path: &str) -> HttpResponse {
    println!("Handling GET request for path: {}", path);
    let file_path = format!(".{}", path);
    if Path::new(&file_path).exists() {
        match fs::read_to_string(&file_path) {
            Ok(contents) => {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                HttpResponse::new(200, headers, Some(contents))
            },
            Err(_) => HttpResponse::new(500, HashMap::new(), Some("Failed to read file".to_string())),
        }
    } else {
        HttpResponse::new(404, HashMap::new(), Some("File not found".to_string()))
    }
}

// Function to handle POST requests
pub fn handle_post(path: &str, json_body: Option<&serde_json::Value>) -> HttpResponse {
    println!("Handling POST request for path: {}", path);
    if let Some(data) = json_body {
        let file_path = format!(".{}", path);
        match serde_json::to_string_pretty(data) {
            Ok(json_string) => match fs::write(&file_path, json_string) {
                Ok(_) => HttpResponse::new(201, HashMap::new(), Some("File created successfully".to_string())),
                Err(_) => HttpResponse::new(500, HashMap::new(), Some("Failed to create file".to_string())),
            },
            Err(_) => HttpResponse::new(400, HashMap::new(), Some("Invalid JSON data".to_string())),
        }
    } else {
        HttpResponse::new(400, HashMap::new(), Some("Missing JSON body".to_string()))
    }
}

// Function to handle PUT requests
pub fn handle_put(path: &str, json_body: Option<&serde_json::Value>) -> HttpResponse {
    println!("Handling PUT request for path: {}", path);
    if let Some(data) = json_body {
        let file_path = format!(".{}", path);
        match serde_json::to_string_pretty(data) {
            Ok(json_string) => match fs::write(&file_path, json_string) {
                Ok(_) => HttpResponse::new(200, HashMap::new(), Some("File updated successfully".to_string())),
                Err(_) => HttpResponse::new(500, HashMap::new(), Some("Failed to update file".to_string())),
            },
            Err(_) => HttpResponse::new(400, HashMap::new(), Some("Invalid JSON data".to_string())),
        }
    } else {
        HttpResponse::new(400, HashMap::new(), Some("Missing JSON body".to_string()))
    }
}

// Function to handle DELETE requests
pub fn handle_delete(path: &str) -> HttpResponse {
    println!("Handling DELETE request for path: {}", path);
    let file_path = format!(".{}", path);
    if Path::new(&file_path).exists() {
        match fs::remove_file(&file_path) {
            Ok(_) => HttpResponse::new(200, HashMap::new(), Some("File deleted successfully".to_string())),
            Err(_) => HttpResponse::new(500, HashMap::new(), Some("Failed to delete file".to_string())),
        }
    } else {
        HttpResponse::new(404, HashMap::new(), Some("File not found".to_string()))
    }
}

// Function to handle PATCH requests
pub fn handle_patch(path: &str, json_body: Option<&serde_json::Value>) -> HttpResponse {
    println!("Handling PATCH request for path: {}", path);
    if let Some(data) = json_body {
        let file_path = format!(".{}", path);
        if Path::new(&file_path).exists() {
            match fs::read_to_string(&file_path) {
                Ok(existing_content) => {
                    let existing_json: serde_json::Value = serde_json::from_str(&existing_content).unwrap_or(serde_json::json!({}));
                    let mut updated_json = existing_json;
                    if let serde_json::Value::Object(obj) = &mut updated_json {
                        if let serde_json::Value::Object(patch) = data {
                            obj.extend(patch.clone());
                        }
                    }
                    match serde_json::to_string_pretty(&updated_json) {
                        Ok(json_string) => match fs::write(&file_path, json_string) {
                            Ok(_) => HttpResponse::new(200, HashMap::new(), Some("File patched successfully".to_string())),
                            Err(_) => HttpResponse::new(500, HashMap::new(), Some("Failed to patch file".to_string())),
                        },
                        Err(_) => HttpResponse::new(400, HashMap::new(), Some("Invalid JSON data".to_string())),
                    }
                },
                Err(_) => HttpResponse::new(500, HashMap::new(), Some("Failed to read file".to_string())),
            }
        } else {
            HttpResponse::new(404, HashMap::new(), Some("File not found".to_string()))
        }
    } else {
        HttpResponse::new(400, HashMap::new(), Some("Missing JSON body".to_string()))
    }
}

// Function to handle unsupported methods
pub fn handle_method_not_allowed() -> HttpResponse {
    HttpResponse::new(405, HashMap::new(), Some("Method not allowed".to_string()))
}