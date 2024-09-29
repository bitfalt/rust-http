use std::fs;
use std::path::Path;
use serde_json;

// Function to handle GET requests
pub fn handle_get(path: &str) -> String {
    println!("Handling GET request for path: {}", path);
    let file_path = format!(".{}", path);
    if Path::new(&file_path).exists() {
        match fs::read_to_string(&file_path) {
            Ok(contents) => format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", contents),
            Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\nFailed to read file".to_string(),
        }
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\nFile not found".to_string()
    }
}

// Function to handle POST requests
pub fn handle_post(path: &str, json_body: Option<&serde_json::Value>) -> String {
    println!("Handling POST request for path: {}", path);
    if let Some(data) = json_body {
        // Process the JSON data
        let file_path = format!(".{}", path);
        match serde_json::to_string_pretty(data) {
            Ok(json_string) => match fs::write(&file_path, json_string) {
                Ok(_) => "HTTP/1.1 201 Created\r\n\r\nFile created successfully".to_string(),
                Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\nFailed to create file".to_string(),
            },
            Err(_) => "HTTP/1.1 400 Bad Request\r\n\r\nInvalid JSON data".to_string(),
        }
    } else {
        "HTTP/1.1 400 Bad Request\r\n\r\nMissing JSON body".to_string()
    }
}

// Function to handle PUT requests
pub fn handle_put(path: &str, json_body: Option<&serde_json::Value>) -> String {
    println!("Handling PUT request for path: {}", path);
    if let Some(data) = json_body {
        // Process the JSON data
        let file_path = format!(".{}", path);
        match serde_json::to_string_pretty(data) {
            Ok(json_string) => match fs::write(&file_path, json_string) {
                Ok(_) => "HTTP/1.1 200 OK\r\n\r\nFile updated successfully".to_string(),
                Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\nFailed to update file".to_string(),
            },
            Err(_) => "HTTP/1.1 400 Bad Request\r\n\r\nInvalid JSON data".to_string(),
        }
    } else {
        "HTTP/1.1 400 Bad Request\r\n\r\nMissing JSON body".to_string()
    }
}

// Function to handle DELETE requests
pub fn handle_delete(path: &str) -> String {
    println!("Handling DELETE request for path: {}", path);
    let file_path = format!(".{}", path);
    if Path::new(&file_path).exists() {
        match fs::remove_file(&file_path) {
            Ok(_) => "HTTP/1.1 200 OK\r\n\r\nFile deleted successfully".to_string(),
            Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\nFailed to delete file".to_string(),
        }
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\nFile not found".to_string()
    }
}

// Function to handle PATCH requests
pub fn handle_patch(path: &str, json_body: Option<&serde_json::Value>) -> String {
    println!("Handling PATCH request for path: {}", path);
    if let Some(data) = json_body {
        // Process the JSON data
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
                            Ok(_) => "HTTP/1.1 200 OK\r\n\r\nFile patched successfully".to_string(),
                            Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\nFailed to patch file".to_string(),
                        },
                        Err(_) => "HTTP/1.1 400 Bad Request\r\n\r\nInvalid JSON data".to_string(),
                    }
                },
                Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\nFailed to read file".to_string(),
            }
        } else {
            "HTTP/1.1 404 Not Found\r\n\r\nFile not found".to_string()
        }
    } else {
        "HTTP/1.1 400 Bad Request\r\n\r\nMissing JSON body".to_string()
    }
}

// Function to handle unsupported methods
pub fn handle_method_not_allowed() -> String {
    "HTTP/1.1 405 Method Not Allowed\r\n\r\nMethod not allowed".to_string()
}