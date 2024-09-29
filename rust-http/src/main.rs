use std::sync::{Arc, Mutex};
use std::env;
use log::error;
use env_logger;
use rust_http::server::Server;

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
