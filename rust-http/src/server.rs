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


// Fixed Thread Pool Tests
#[cfg(test)]
mod tests {
    use threadpool::ThreadPool;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;


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