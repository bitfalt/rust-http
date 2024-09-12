use std::{
    io::{BufRead, BufReader, Write},  // Asegúrate de importar Write para poder usar el método `write`
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }    
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", request);

    let method = &request[0];
    
    let response = if method.starts_with("GET") {
        "HTTP/1.1 200 OK\r\n\r\nGET recibido"
    } else if method.starts_with("POST") {
        "HTTP/1.1 200 OK\r\n\r\nPOST recibido"
    } else if method.starts_with("PUT") {
        "HTTP/1.1 200 OK\r\n\r\nPUT recibido"
    } else if method.starts_with("DELETE") {
        "HTTP/1.1 200 OK\r\n\r\nDELETE recibido"
    } else if method.starts_with("PATCH") {
        "HTTP/1.1 200 OK\r\n\r\nPATCH recibido"
    } else {
        "HTTP/1.1 405 Method Not Allowed\r\n\r\nMétodo no soportado"
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

