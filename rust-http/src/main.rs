use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use log::{error, info};
use env_logger;
use uuid::Uuid;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

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
            let mut server_lock = server.lock().unwrap(); // Lock the server for exclusive access
            let session_id = server_lock.handle_cookie(&request);
            drop(server_lock); // Release the lock once done

            // Handle request based on method
            let response = match request.method.as_str() {
                "GET" => handle_get(&request.path),
                "POST" => handle_post(&request.body),
                "PUT" => handle_put(&request.body),
                "DELETE" => handle_delete(&request.path),
                "PATCH" => handle_patch(&request.body),
                _ => handle_method_not_allowed(),
            };

            // Add Set-Cookie header if session ID is new
            let full_response = format!(
                "{}\r\nSet-Cookie: sessionId={}; Path=/\r\n\r\n",
                response, session_id
            );

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

// Function to handle GET requests (prints path for debugging)
fn handle_get(path: &str) -> String {
    println!("Handling GET request for path: {}", path); // Log the request path
    format!("HTTP/1.1 200 OK\r\n\r\nGET request received for path: {}", path)
}

// Function to handle POST requests
fn handle_post(body: &str) -> String {
    format!("HTTP/1.1 200 OK\r\n\r\nPOST received with body: {}", body)
}

// Function to handle PUT requests
fn handle_put(body: &str) -> String {
    format!("HTTP/1.1 200 OK\r\n\r\nPUT received with body: {}", body)
}

// Function to handle DELETE requests (prints path for debugging)
fn handle_delete(path: &str) -> String {
    println!("Handling DELETE request for path: {}", path); // Log the request path
    format!("HTTP/1.1 200 OK\r\n\r\nDELETE request received for path: {}", path)
}

// Function to handle PATCH requests
fn handle_patch(body: &str) -> String {
    format!("HTTP/1.1 200 OK\r\n\r\nPATCH received with body: {}", body)
}

// Function to handle unsupported methods
fn handle_method_not_allowed() -> String {
    "HTTP/1.1 405 Method Not Allowed\r\n\r\nMethod not allowed".to_string()
}

// Fixed Thread Pool Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_thread_creation(){
        let pool = ThreadPool::new(4);
        let (sender, receiver) = channel();
        // Enviar 10 tareas para verificar la creación y reutilización de hilos
        for i in 0..10 {
            let sender = sender.clone();
            pool.execute(move || {
                thread::sleep(Duration::from_secs(1));
                println!("Tarea {} ejecutada en un hilo.", i);
                sender.send(()).expect("Error al enviar el mensaje");
            });
        }
        // Espera a que todas las tareas terminen
        for _ in 0..10 {
            receiver.recv().expect("Error al recibir el mensaje");
        }
        println!("Prueba de creación de hilos completada.");
    }
    #[test]
    fn test_pool_saturation() {
        let pool = ThreadPool::new(4);
        let (sender, receiver) = channel();

        for i in 0..10 {
            let sender = sender.clone();
            pool.execute(move || {
                println!("Ejecutando tarea {}...", i);
                thread::sleep(Duration::from_secs(1)); // Simula trabajo de 1 segundo
                sender.send(()).expect("Error al enviar el mensaje");
            });
        }
        for _ in 0..10 {
            receiver.recv().expect("Error al recibir el mensaje");
        }

        println!("Prueba de saturación completada.");
    }

    #[test]
    fn test_response_time_under_load() {
        let pool = ThreadPool::new(4);
        let (sender, receiver) = channel();
        let start = std::time::Instant::now();

        for i in 0..50 {
            let sender = sender.clone();
            pool.execute(move || {
                thread::sleep(Duration::from_millis(100)); 
                sender.send(()).expect("Error al enviar el mensaje");
            });
        }

        for _ in 0..50 {
            receiver.recv().expect("Error al recibir el mensaje");
        }

        let duration = start.elapsed();
        println!("Prueba bajo carga completada en {:?}", duration);
    }

    #[test]
    fn test_error_handling() {
        let pool = ThreadPool::new(4);
        let (sender, receiver) = channel();

        for i in 0..4 {
            let sender = sender.clone();
            pool.execute(move || {
                if i == 2 {
                    panic!("Error controlado en la tarea {}", i); // Introduce un error controlado
                }
                println!("Tarea {} ejecutada.", i);
                sender.send(()).expect("Error al enviar el mensaje");
            });
        }
        // Observa si los hilos continúan operando después del error
        for _ in 0..4 {
            if receiver.recv().is_err() {
                println!("Se capturó un error.");
            }
        }

        println!("Prueba de manejo de errores completada.");
    }
}

fn main() {
    // Initialize logger
    env_logger::init();

    // Use Arc and Mutex to share the server across threads
    let server = Arc::new(Mutex::new(Server::new()));

    if let Err(e) = Server::run(server) {
        error!("Server error: {}", e);
    }
}
