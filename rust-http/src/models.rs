use std::collections::HashMap;
use std::net::TcpStream;

// Struct to represent an HTTP request
#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub _headers: Vec<String>,
    pub body: String,
    pub cookie: Option<String>,
}

// Struct to represent a client
pub struct Client {
    pub stream: TcpStream,
}

// Main server struct with session management
pub struct Server {
    pub sessions: HashMap<String, String>,
}