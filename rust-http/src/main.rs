use std::sync::{Arc, Mutex};
use std::env;
use log::error;
use env_logger;

use rust_http::models::Server;

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

    println!("Current working directory: {:?}", env::current_dir().unwrap());

    if let Err(e) = Server::run(server) {
        error!("Server error: {}", e);
    }
}
