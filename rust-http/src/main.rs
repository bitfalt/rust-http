use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::env;
use log::{error, info};
use env_logger;
use uuid::Uuid;
use threadpool::ThreadPool;
use serde_json;

// Struct to represent an HTTP request
#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    _headers: Vec<String>,
    body: String,
    cookie: Option<String>,
}

// Struct to represent a client
struct Client {
    stream: TcpStream,
}

// Main server struct with session management
struct Server {
    sessions: HashMap<String, String>,
}

impl Server {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    fn handle_cookie(&mut self, request: &HttpRequest) -> String {
        if let Some(cookie) = &request.cookie {
            if let Some(session_data) = self.sessions.get(cookie) {
                println!("Existing session for cookie: {} -> {}", cookie, session_data);
                return cookie.clone(); // Return the existing session ID
            }
        }

        // If no valid session, create a new one
        let session_id = Uuid::new_v4().to_string();
        self.sessions.insert(session_id.clone(), "user_data".to_string());
        println!("New session created: {}", session_id);

        // Return the new session ID and set it in the Set-Cookie header
        session_id
    }

    fn run(server: Arc<Mutex<Server>>) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        info!("Server running on port 8080");

        // Create a thread pool with 4 threads
        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server_clone = Arc::clone(&server);
                    pool.execute(move || {
                        let mut client = Client { stream };
                        client.handle(server_clone);
                    });
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }

        Ok(())
    }
}

impl Client {
    // Handle the client connection
    fn handle(&mut self, server: Arc<Mutex<Server>>) {
        if let Some(request) = self.parse_request() {
            // Handle the session cookie
            let mut server_lock = server.lock().unwrap();
            let session_id = server_lock.handle_cookie(&request);
            drop(server_lock);

            // Parse JSON body if present
            let json_body = if !request.body.is_empty() {
                serde_json::from_str(&request.body).ok()
            } else {
                None
            };

            // Handle request based on method
            let response = match request.method.as_str() {
                "GET" => handle_get(&request.path),
                "POST" => handle_post(&request.path, json_body.as_ref()),
                "PUT" => handle_put(&request.path, json_body.as_ref()),
                "DELETE" => handle_delete(&request.path),
                "PATCH" => handle_patch(&request.path, json_body.as_ref()),
                _ => handle_method_not_allowed(),
            };

            // Add Set-Cookie header if session ID is new
            // let full_response = format!(
            //     "{}\r\nSet-Cookie: sessionId={}; Path=/\r\n\r\n",
            //     response, session_id
            // );

            let full_response = response;

            // Send the response back to the client
            if let Err(e) = self.send_response(&full_response) {
                eprintln!("Failed to send response: {}", e);
            }

            // Log the response
            println!("Sent Response: {}", full_response);
        }
    }

    // Parse the incoming request and extract cookie if available
    fn parse_request(&mut self) -> Option<HttpRequest> {
        let mut buffer = [0; 1024];
        let bytes_read = match self.stream.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return None;
            }
        };

        let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut headers_and_body = request_str.split("\r\n\r\n");

        let header_part = headers_and_body.next().unwrap_or_default();
        if header_part.is_empty() {
            // Malformed request: No headers
            eprintln!("Malformed request: No headers.");
            return None;
        }

        let body_part = headers_and_body.next().unwrap_or_default().to_string();

        let mut header_lines = header_part.lines();
        let request_line = header_lines.next().unwrap_or_default();

        let mut request_parts = request_line.split_whitespace();
        let method = request_parts.next().unwrap_or("").to_string();
        if method.is_empty() {
            // Malformed request: No HTTP method
            eprintln!("Malformed request: No HTTP method.");
            return None;
        }

        let path = request_parts.next().unwrap_or("").to_string();
        let _headers: Vec<String> = header_lines.map(|h| h.to_string()).collect();

        // Extract cookie from headers if present
        let cookie_header = _headers.iter().find(|h| h.starts_with("Cookie"));
        let cookie = cookie_header.and_then(|h| {
            h.split('=').nth(1).map(|c| c.trim().to_string()) // Extract the sessionId value
        });

        Some(HttpRequest {
            method,
            path,
            _headers,
            body: body_part,
            cookie, // Include the cookie if available
        })
    }

    // Send the response back to the client
    fn send_response(&mut self, response: &str) -> std::io::Result<()> {
        self.stream.write_all(response.as_bytes())?;
        self.stream.flush()
    }
}

use std::fs;
use std::path::Path;

// Function to handle GET requests
fn handle_get(path: &str) -> String {
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
fn handle_post(path: &str, json_body: Option<&serde_json::Value>) -> String {
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
fn handle_put(path: &str, json_body: Option<&serde_json::Value>) -> String {
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
fn handle_delete(path: &str) -> String {
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
fn handle_patch(path: &str, json_body: Option<&serde_json::Value>) -> String {
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
fn handle_method_not_allowed() -> String {
    "HTTP/1.1 405 Method Not Allowed\r\n\r\nMethod not allowed".to_string()
}

fn main() {
    // Initialize logger
    env_logger::init();

    // Use Arc and Mutex to share the server across threads
    let server = Arc::new(Mutex::new(Server::new()));

    println!("Current working directory: {:?}", env::current_dir().unwrap());

    if let Err(e) = Server::run(server) {
        error!("Server error: {}", e);
    }
}
