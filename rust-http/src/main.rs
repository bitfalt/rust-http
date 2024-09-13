use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    headers: Vec<String>,
    body: String,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Servidor escuchando en http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_lines = Vec::new();

    // Leer las líneas de la solicitud sin consumir buf_reader
    for line in buf_reader.by_ref().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        request_lines.push(line);
    }

    // Extraer el método y la ruta de la primera línea del request
    let first_line = &request_lines[0];
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let method = parts[0].to_string();
    let path = parts[1].to_string();

    // Crear el struct HttpRequest
    let mut request = HttpRequest {
        method,
        path,
        headers: request_lines[1..].to_vec(), // Agregar las cabeceras
        body: String::new(),
    };

    // Leer el cuerpo si es POST o PUT
    if request.method == "POST" || request.method == "PUT" {
        if let Some(content_length_line) = request.headers.iter().find(|line| line.starts_with("Content-Length")) {
            let content_length: usize = content_length_line
                .split(": ")
                .nth(1)
                .unwrap()
                .trim()
                .parse()
                .unwrap();

            if content_length > 0 {
                buf_reader.take(content_length as u64).read_to_string(&mut request.body).unwrap();
            }
        }
    }

    println!("Request: {:#?}", request);

    let response = match request.method.as_str() {
        "GET" => format!("HTTP/1.1 200 OK\r\n\r\nGET recibido en la ruta: {}", request.path),
        "POST" => format!("HTTP/1.1 200 OK\r\n\r\nPOST recibido con cuerpo: {}", request.body),
        "PUT" => format!("HTTP/1.1 200 OK\r\n\r\nPUT recibido con cuerpo: {}", request.body),
        "DELETE" => format!("HTTP/1.1 200 OK\r\n\r\nDELETE recibido en la ruta: {}", request.path),
        "UPDATE" => "HTTP/1.1 200 OK\r\n\r\nUPDATE recibido".to_string(),
        _ => "HTTP/1.1 405 Method Not Allowed\r\n\r\nMétodo no soportado".to_string(),
    };    

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
