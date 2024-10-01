use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::request::HttpRequest;
use crate::client::Client;
use std::net::{TcpStream, TcpListener};
use threadpool::ThreadPool;
use log::{error, info};
use std::thread;
use std::io::{Write, Read};
use std::time::Duration;

// Main server struct with session management
pub struct Server {
    pub sessions: HashMap<String, String>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
    pub fn handle_cookie(&mut self, request: &HttpRequest) -> String {
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

    pub fn run(server: Arc<Mutex<Server>>) -> Result<(), Box<dyn std::error::Error>> {
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


// Fixed Thread Pool Tests
#[cfg(test)]
mod tests {
    use super::*;
    //use crate::request::HttpRequest;

    #[test]
    fn test_new_session_creation_without_cookie() {
        // New server
        let mut server = Server::new();

        // Request without cookie 
        let request = HttpRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            _headers: vec![],
            body: "".to_string(),
            cookie: None,
        };

        // Generate cookie
        let session_id = server.handle_cookie(&request);

        // Verify that the new session has been created
        assert!(server.sessions.contains_key(&session_id));
        assert_eq!(server.sessions.get(&session_id).unwrap(), "user_data");
    }
    #[test]
    fn test_new_session_creation_existing_cookie() {
        // New server
        let mut server = Server::new();
        
        // Manual Session
        server.sessions.insert("abc".to_string(), "user_data".to_string());

        // Request with cookie 
        let request = HttpRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            _headers: vec![],
            body: "".to_string(),
            cookie: Some("abc".to_string()),
        };

        // Handle cookie
        let session = server.handle_cookie(&request);

        assert_eq!(session, "abc", "Cookie should be abc");
    }

    #[test]
    fn test_server_run_single_connection() {
        let server = Arc::new(Mutex::new(Server::new()));
        let server_clone = Arc::clone(&server);
    
        // Execute on a thread
        std::thread::spawn(move || {
            Server::run(server_clone).unwrap();
        });
    
        //Connects with the server
        match std::net::TcpStream::connect("127.0.0.1:8080") {
            Ok(mut stream) => {
                stream.write(b"GET /get HTTP/1.1\r\n\r\n").unwrap();
    
                let mut buffer = [0; 512];
                let bytes_read = stream.read(&mut buffer).unwrap();
                // Verify that has read something from connection
                assert!(bytes_read > 0);
            }
            Err(e) => {
                panic!("Failed to connect to the server: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_server_run_multiple_connections() {
        // Creates a server with shared sessions
        let server = Arc::new(Mutex::new(Server::new()));

        // Runs the server on a thread
        let server_clone = Arc::clone(&server);
        let handle = thread::spawn(move || {
            Server::run(server_clone).unwrap();
        });

        // Creates multiple simulated conections
        let clients: Vec<_> = (0..4)
            .map(|_| {
                thread::spawn(|| {
                    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
                    stream.write(b"GET /get HTTP/1.1\r\n\r\n").unwrap();

                    let mut buffer = [0; 512];
                    stream.read(&mut buffer).unwrap();
                    assert!(buffer.starts_with(b"HTTP/1.1"));
                })
            })
            .collect();

        // Esperar a que todas las conexiones terminen
        for client in clients {
            client.join().unwrap();
        }

        // Detener el servidor
        handle.join().unwrap();
    }
}