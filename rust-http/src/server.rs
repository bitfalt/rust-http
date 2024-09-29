use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::models::{Server, Client, HttpRequest};
use std::net::TcpListener;
use threadpool::ThreadPool;
use log::{error, info};


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