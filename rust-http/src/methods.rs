use std::fs;
use std::path::Path;
use serde_json::Value;
use std::collections::HashMap;
use crate::models::HttpResponse;

impl HttpResponse {
    pub fn new(status_code: u16, headers: HashMap<String, String>, body: Option<String>) -> Self {
        HttpResponse { status_code, headers, body }
    }

    pub fn to_string(&self) -> String {
        let status_text = match self.status_code {
            100 => "Continue",
            101 => "Switching Protocols",
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            204 => "No Content",
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            304 => "Not Modified",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "Unknown Status",
        };
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, status_text);
        
        // Create a mutable copy of headers
        let mut headers = self.headers.clone();
        
        // Add Content-Length header if there's a body
        if let Some(body) = &self.body {
            headers.entry("Content-Length".to_string())
                .or_insert_with(|| body.len().to_string());
        }
        
        // Add headers to the response
        for (key, value) in headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        response.push_str("\r\n");
        if let Some(body) = &self.body {
            response.push_str(body);
        }
        response
    }
}

// Function to handle GET requests
pub fn handle_get(id: &str) -> HttpResponse {
    println!("Handling GET request for user with ID: {}", id);
    
    // Construir la ruta del archivo dentro de la carpeta `files`
    let file_path = format!("./files/{}.json", id);

    // Verificar si el archivo existe
    if Path::new(&file_path).exists() {
        // Intentar leer el contenido del archivo
        match fs::read_to_string(&file_path) {
            Ok(contents) => {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                HttpResponse::new(200, headers, Some(contents))
            },
            Err(e) => {
                println!("Failed to read file: {}", e);
                HttpResponse::new(500, HashMap::new(), Some("Failed to read file".to_string()))
            },
        }
    } else {
        HttpResponse::new(404, HashMap::new(), Some("File not found".to_string()))
    }
}

// Function to handle POST requests
pub fn handle_post(id: &str, json_body: Option<&serde_json::Value>) -> HttpResponse {
    println!("Handling POST request for user with ID: {}", id);

    if let Some(data) = json_body {
        // Construir la ruta completa usando la carpeta 'files' y el ID como nombre del archivo
        let file_path = format!("./files/{}.json", id);
        let path_parent = Path::new(&file_path).parent();

        // Crear el directorio padre si no existe
        if let Some(parent) = path_parent {
            if let Err(e) = fs::create_dir_all(parent) {
                println!("Failed to create directory: {}", e);
                return HttpResponse::new(500, HashMap::new(), Some("Failed to create directory".to_string()));
            }
        }

        // Convertir el cuerpo JSON a un string formateado y escribirlo en el archivo
        match serde_json::to_string_pretty(data) {
            Ok(json_string) => match fs::write(&file_path, json_string) {
                Ok(_) => HttpResponse::new(201, HashMap::new(), Some("File created successfully".to_string())),
                Err(e) => {
                    println!("Failed to create file: {}", e);
                    HttpResponse::new(500, HashMap::new(), Some(format!("Failed to create file: {}", e)))
                },
            },
            Err(_) => HttpResponse::new(400, HashMap::new(), Some("Invalid JSON data".to_string())),
        }
    } else {
        HttpResponse::new(400, HashMap::new(), Some("Missing JSON body".to_string()))
    }
}

// Function to handle PUT requests
pub fn handle_put(id: &str, json_body: Option<&serde_json::Value>) -> HttpResponse {
    println!("Handling PUT request for user with ID: {}", id);
    
    // Construir la ruta del archivo dentro de la carpeta `files`
    let file_path = format!("./files/{}.json", id);

    // Verificar si el cuerpo JSON está presente
    if let Some(data) = json_body {
        // Verificar si el archivo existe antes de intentar actualizarlo
        if Path::new(&file_path).exists() {
            // Convertir el cuerpo JSON a un string formateado y escribirlo en el archivo
            match serde_json::to_string_pretty(data) {
                Ok(json_string) => match fs::write(&file_path, json_string) {
                    Ok(_) => HttpResponse::new(200, HashMap::new(), Some("File updated successfully".to_string())),
                    Err(e) => {
                        println!("Failed to update file: {}", e);
                        HttpResponse::new(500, HashMap::new(), Some("Failed to update file".to_string()))
                    },
                },
                Err(_) => HttpResponse::new(400, HashMap::new(), Some("Invalid JSON data".to_string())),
            }
        } else {
            HttpResponse::new(404, HashMap::new(), Some("File not found".to_string()))
        }
    } else {
        HttpResponse::new(400, HashMap::new(), Some("Missing JSON body".to_string()))
    }
}

// Function to handle DELETE requests
pub fn handle_delete(id: &str) -> HttpResponse {
    println!("Handling DELETE request for user with ID: {}", id);
    
    // Construye la ruta del archivo dentro de la carpeta `files`
    let file_path = format!("./files/{}.json", id);

    // Verifica si el archivo existe
    if Path::new(&file_path).exists() {
        // Intenta eliminar el archivo
        match fs::remove_file(&file_path) {
            Ok(_) => HttpResponse::new(200, HashMap::new(), Some("File deleted successfully".to_string())),
            Err(e) => {
                println!("Failed to delete file: {}", e);
                HttpResponse::new(500, HashMap::new(), Some("Failed to delete file".to_string()))
            },
        }
    } else {
        HttpResponse::new(404, HashMap::new(), Some("File not found".to_string()))
    }
}

// Function to handle PATCH requests
pub fn handle_patch(id: &str, json_body: Option<&Value>) -> HttpResponse {
    println!("Handling PATCH request for user with ID: {}", id);

    // Construir la ruta del archivo dentro de la carpeta `files`
    let file_path = format!("./files/{}.json", id);

    // Verificar si el cuerpo JSON está presente
    if let Some(data) = json_body {
        // Verificar si el archivo existe antes de intentar actualizarlo
        if Path::new(&file_path).exists() {
            // Leer el contenido existente del archivo
            match fs::read_to_string(&file_path) {
                Ok(existing_content) => {
                    // Intentar parsear el contenido existente como JSON
                    let mut existing_json: Value = match serde_json::from_str(&existing_content) {
                        Ok(json) => json,
                        Err(e) => {
                            println!("Failed to parse existing JSON: {}", e);
                            return HttpResponse::new(500, HashMap::new(), Some("Failed to parse existing file".to_string()));
                        }
                    };

                    // Verificar si el JSON existente y el patch son objetos
                    if let (Value::Object(ref mut obj), Value::Object(ref patch)) = (&mut existing_json, data) {
                        // Verificar si todas las claves del patch existen en el objeto original
                        for key in patch.keys() {
                            if !obj.contains_key(key) {
                                return HttpResponse::new(400, HashMap::new(), Some(format!("Key '{}' does not exist in the original JSON", key)));
                            }
                        }

                        // Extender el JSON original solo si todas las claves existen
                        obj.extend(patch.clone());

                        // Convertir el JSON actualizado a string y escribirlo en el archivo
                        match serde_json::to_string_pretty(&existing_json) {
                            Ok(json_string) => match fs::write(&file_path, json_string) {
                                Ok(_) => HttpResponse::new(200, HashMap::new(), Some("File patched successfully".to_string())),
                                Err(e) => {
                                    println!("Failed to write updated file: {}", e);
                                    HttpResponse::new(500, HashMap::new(), Some("Failed to patch file".to_string()))
                                },
                            },
                            Err(e) => {
                                println!("Failed to serialize updated JSON: {}", e);
                                HttpResponse::new(400, HashMap::new(), Some("Invalid JSON data".to_string()))
                            },
                        }
                    } else {
                        HttpResponse::new(400, HashMap::new(), Some("Existing data and patch must be JSON objects".to_string()))
                    }
                },
                Err(e) => {
                    println!("Failed to read file: {}", e);
                    HttpResponse::new(500, HashMap::new(), Some("Failed to read file".to_string()))
                },
            }
        } else {
            HttpResponse::new(404, HashMap::new(), Some("File not found".to_string()))
        }
    } else {
        HttpResponse::new(400, HashMap::new(), Some("Missing JSON body".to_string()))
    }
}

// Function to handle unsupported methods
pub fn handle_method_not_allowed() -> HttpResponse {
    HttpResponse::new(405, HashMap::new(), Some("Method not allowed".to_string()))
}