use std::fs;
use std::path::Path;
use serde_json::Value;
use std::collections::HashMap;
use crate::response::HttpResponse;

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
                HttpResponse::new(500, HashMap::new(), Some(serde_json::json!({
                    "status_code": 500,
                    "message": "Failed to read file"
                }).to_string()))
            },
        }
    } else {
        HttpResponse::new(404, HashMap::new(), Some(serde_json::json!({
            "status_code": 404,
            "message": "File not found"
        }).to_string()))
    }
}

// Function to handle POST requests
pub fn handle_post(id: &str, json_body: Option<&serde_json::Value>) -> HttpResponse {
    println!("Handling POST request for user with ID: {}", id);

    if let Some(data) = json_body {
        // Check if the JSON body is a valid object
        if !data.is_object() {
            return HttpResponse::new(400, HashMap::new(), Some(serde_json::json!({
                "status_code": 400,
                "message": "Invalid JSON data: must be an object"
            }).to_string()));
        }

        // Construir la ruta completa usando la carpeta 'files' y el ID como nombre del archivo
        let file_path = format!("./files/{}.json", id);
        let path_parent = Path::new(&file_path).parent();

        // Crear el directorio padre si no existe
        if let Some(parent) = path_parent {
            if let Err(e) = fs::create_dir_all(parent) {
                println!("Failed to create directory: {}", e);
                return HttpResponse::new(500, HashMap::new(), Some(serde_json::json!({
                    "status_code": 500,
                    "message": "Failed to create directory"
                }).to_string()));
            }
        }

        // Convertir el cuerpo JSON a un string formateado y escribirlo en el archivo
        match serde_json::to_string_pretty(data) {
            Ok(json_string) => match fs::write(&file_path, json_string) {
                Ok(_) => HttpResponse::new(201, HashMap::new(), Some(serde_json::json!({
                    "status_code": 201,
                    "message": "File created successfully"
                }).to_string())),
                Err(e) => {
                    println!("Failed to create file: {}", e);
                    HttpResponse::new(500, HashMap::new(), Some(serde_json::json!({
                        "status_code": 500,
                        "message": format!("Failed to create file: {}", e)
                    }).to_string()))
                },
            },
            Err(e) => {
                println!("Failed to serialize JSON: {}", e);
                HttpResponse::new(500, HashMap::new(), Some(serde_json::json!({
                    "status_code": 500,
                    "message": "Failed to serialize JSON"
                }).to_string()))
            },
        }
    } else {
        HttpResponse::new(400, HashMap::new(), Some(serde_json::json!({
            "status_code": 400,
            "message": "Missing JSON body"
        }).to_string()))
    }
}

// Function to handle PUT requests
pub fn handle_put(id: &str, json_body: Option<&serde_json::Value>) -> HttpResponse {
    println!("Handling PUT request for user with ID: {}", id);
    
    // Construir la ruta del archivo dentro de la carpeta `files`
    let file_path = format!("./files/{}.json", id);

    // Verificar si el cuerpo JSON está presente
    if let Some(data) = json_body {
        // Verificar si el JSON es un objeto
        if !data.is_object() {
            return HttpResponse::new(400, HashMap::new(), Some(serde_json::json!({
                "status_code": 400,
                "message": "Invalid JSON data: must be an object"
            }).to_string()));
        }

        // Verificar si el archivo existe antes de intentar actualizarlo
        if Path::new(&file_path).exists() {
            // Convertir el cuerpo JSON a un string formateado y escribirlo en el archivo
            match serde_json::to_string_pretty(data) {
                Ok(json_string) => match fs::write(&file_path, json_string) {
                    Ok(_) => HttpResponse::new(200, HashMap::new(), Some(serde_json::json!({
                        "status_code": 200,
                        "message": "File updated successfully"
                    }).to_string())),
                    Err(e) => {
                        println!("Failed to update file: {}", e);
                        HttpResponse::new(500, HashMap::new(), Some(serde_json::json!({
                            "status_code": 500,
                            "message": format!("Failed to update file: {}", e)
                        }).to_string()))
                    },
                },
                Err(e) => {
                    println!("Failed to serialize JSON: {}", e);
                    HttpResponse::new(500, HashMap::new(), Some(serde_json::json!({
                        "status_code": 500,
                        "message": "Failed to serialize JSON"
                    }).to_string()))
                },
            }
        } else {
            HttpResponse::new(404, HashMap::new(), Some(serde_json::json!({
                "status_code": 404,
                "message": "File not found"
            }).to_string()))
        }
    } else {
        HttpResponse::new(400, HashMap::new(), Some(serde_json::json!({
            "status_code": 400,
            "message": "Missing JSON body"
        }).to_string()))
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
            Ok(_) => HttpResponse::new(200, HashMap::new(), Some(serde_json::json!({
                "status_code": 200,
                "message": "File deleted successfully"
            }).to_string())),
            Err(e) => {
                println!("Failed to delete file: {}", e);
                HttpResponse::new(500, HashMap::new(), Some(serde_json::json!({
                    "status_code": 500,
                    "message": "Failed to delete file"
                }).to_string()))
            },
        }
    } else {
        HttpResponse::new(404, HashMap::new(), Some(serde_json::json!({
            "status_code": 404,
            "message": "File not found"
        }).to_string()))
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
                            return HttpResponse::new(500, HashMap::new(), Some(serde_json::json!({
                                "status_code": 500,
                                "message": "Failed to parse existing file"
                            }).to_string()));
                        }
                    };

                    // Verificar si el JSON existente y el patch son objetos
                    if let (Value::Object(ref mut obj), Value::Object(ref patch)) = (&mut existing_json, data) {
                        // Verificar si todas las claves del patch existen en el objeto original
                        for key in patch.keys() {
                            if !obj.contains_key(key) {
                                return HttpResponse::new(400, HashMap::new(), Some(serde_json::json!({
                                    "status_code": 400,
                                    "message": format!("Key '{}' does not exist in the original JSON", key)
                                }).to_string()));
                            }
                        }

                        // Extender el JSON original solo si todas las claves existen
                        obj.extend(patch.clone());

                        // Convertir el JSON actualizado a string y escribirlo en el archivo
                        match serde_json::to_string_pretty(&existing_json) {
                            Ok(json_string) => match fs::write(&file_path, json_string) {
                                Ok(_) => HttpResponse::new(200, HashMap::new(), Some(serde_json::json!({
                                    "status_code": 200,
                                    "message": "File patched successfully"
                                }).to_string())),
                                Err(e) => {
                                    println!("Failed to write updated file: {}", e);
                                    HttpResponse::new(500, HashMap::new(), Some(serde_json::json!({
                                        "status_code": 500,
                                        "message": "Failed to patch file"
                                    }).to_string()))
                                },
                            },
                            Err(e) => {
                                println!("Failed to serialize updated JSON: {}", e);
                                HttpResponse::new(400, HashMap::new(), Some(serde_json::json!({
                                    "status_code": 400,
                                    "message": "Invalid JSON data"
                                }).to_string()))
                            },
                        }
                    } else {
                        HttpResponse::new(400, HashMap::new(), Some(serde_json::json!({
                            "status_code": 400,
                            "message": "Existing data and patch must be JSON objects"
                        }).to_string()))
                    }
                },
                Err(e) => {
                    println!("Failed to read file: {}", e);
                    HttpResponse::new(500, HashMap::new(), Some(serde_json::json!({
                        "status_code": 500,
                        "message": "Failed to read file"
                    }).to_string()))
                },
            }
        } else {
            HttpResponse::new(404, HashMap::new(), Some(serde_json::json!({
                "status_code": 404,
                "message": "File not found"
            }).to_string()))
        }
    } else {
        HttpResponse::new(400, HashMap::new(), Some(serde_json::json!({
            "status_code": 400,
            "message": "Missing JSON body"
        }).to_string()))
    }
}

// Function to handle unsupported methods
pub fn handle_method_not_allowed() -> HttpResponse {
    HttpResponse::new(405, HashMap::new(), Some("Method not allowed".to_string()))
}

#[cfg(test)]
mod tests {
    // Import everything out of scope form tests
    use super::*;
    

    #[test]
    fn test_handle_get_successfully() {
        let file = "get";

        let response = handle_get(file);
        // Assert the response was successful
        assert_eq!(response.status_code, 200, "Status code should be 200");

        let file_path = format!("files/{}.json", file);
        let file_contents = fs::read_to_string(&file_path).expect("Failed to read file");

        // Assert the file returned is the same
        assert_eq!(response.body, Some(file_contents), "File contents should be the same");

    }

    #[test]
    fn test_handle_get_file_not_found() {
        let file = "notfound";

        let response = handle_get(file);

        // Assert the response gave 404
        assert_eq!(response.status_code, 404, "Status code should be 404");
    }

    #[test]
    fn test_handle_post_successfully() {
        let id = "test_post";
        let json_body = serde_json::json!({
            "key": "value",
            "number": 42
        });

        let response = handle_post(id, Some(&json_body));

        assert_eq!(response.status_code, 201, "Status code should be 201");
        
        let file_path = format!("files/{}.json", id);
        assert!(Path::new(&file_path).exists(), "File should be created");

        let file_contents = fs::read_to_string(&file_path).expect("Failed to read file");
        let saved_json: Value = serde_json::from_str(&file_contents).expect("Failed to parse JSON");
        assert_eq!(saved_json, json_body, "Saved JSON should match the input");

        // Clean up: remove the test file
        fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_handle_post_invalid_json_data() {
        let id = "test_invalid_json";
        let invalid_json = serde_json::Value::String("This is not a valid JSON object".to_string());

        let response = handle_post(id, Some(&invalid_json));

        assert_eq!(response.status_code, 400, "Status code should be 400");
    }

    #[test]
    fn test_handle_post_missing_json() {
        let id = "test_missing_json";
        let response = handle_post(id, None);

        assert_eq!(response.status_code, 400, "Status code should be 400");
    }

    #[test]
    fn test_handle_post_existing_file() {
        let id = "test_existing_file";
        let json_body = serde_json::json!({"key": "value"});

        // Create a file first
        let _ = handle_post(id, Some(&json_body));

        // Try to create the same file again
        let response = handle_post(id, Some(&json_body));

        assert_eq!(response.status_code, 201, "Status code should be 201");
        
        // Clean up: remove the test file
        let file_path = format!("./files/{}.json", id);
        fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_handle_put_successfully() {
        let id = "test_put_success";
        let initial_json = serde_json::json!({"key": "initial_value"});
        let updated_json = serde_json::json!({"key": "updated_value"});

        // Create a file first
        let _ = handle_post(id, Some(&initial_json));

        // Update the file
        let response = handle_put(id, Some(&updated_json));

        assert_eq!(response.status_code, 200, "Status code should be 200");


        let file_path = format!("./files/{}.json", id);
        assert!(Path::new(&file_path).exists(), "File should exist");

        let file_contents = fs::read_to_string(&file_path).expect("Failed to read file");
        let saved_json: Value = serde_json::from_str(&file_contents).expect("Failed to parse JSON");
        assert_eq!(saved_json, updated_json, "Saved JSON should match the updated input");

        // Clean up: remove the test file
        fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_handle_put_invalid_json_data() {
        let id = "test_put_invalid_json";
        let invalid_json = serde_json::Value::String("This is not a valid JSON object".to_string());

        let response = handle_put(id, Some(&invalid_json));

        assert_eq!(response.status_code, 400, "Status code should be 400");
    }

    #[test]
    fn test_handle_put_file_not_found() {
        let id = "test_put_not_found";
        let json_body = serde_json::json!({"key": "value"});

        let response = handle_put(id, Some(&json_body));

        assert_eq!(response.status_code, 404, "Status code should be 404");
    }

    #[test]
    fn test_handle_put_missing_json() {
        let id = "test_put_missing_json";
        let response = handle_put(id, None);

        assert_eq!(response.status_code, 400, "Status code should be 400");
        assert!(response.body.unwrap().contains("Missing JSON body"), "Response should mention missing JSON body");
    }

    #[test]
    fn test_handle_put_empty_json_object() {
        let id = "test_put_empty_json";
        let initial_json = serde_json::json!({"key": "value"});
        let empty_json = serde_json::json!({});

        // Create a file first
        let _ = handle_post(id, Some(&initial_json));

        // Update with empty JSON
        let response = handle_put(id, Some(&empty_json));

        assert_eq!(response.status_code, 200, "Status code should be 200");


        let file_path = format!("./files/{}.json", id);
        let file_contents = fs::read_to_string(&file_path).expect("Failed to read file");
        let saved_json: Value = serde_json::from_str(&file_contents).expect("Failed to parse JSON");
        assert_eq!(saved_json, empty_json, "Saved JSON should be an empty object");

        // Clean up: remove the test file
        fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_handle_delete_successfully() {
        let id = "test_delete";
        let initial_json = serde_json::json!({"key": "value"});

        // Create a file first
        let _ = handle_post(id, Some(&initial_json));

        // Delete the file
        let response = handle_delete(id);

        assert_eq!(response.status_code, 200, "Status code should be 200");


        let file_path = format!("./files/{}.json", id);
        assert!(!Path::new(&file_path).exists(), "File should not exist after deletion");
    }

    #[test]
    fn test_handle_delete_file_not_found() {
        let id = "nonexistent_file";
        let response = handle_delete(id);

        assert_eq!(response.status_code, 404, "Status code should be 404");
    }

    #[test]
    fn test_handle_patch_successfully() {
        let id = "test_patch";
        let initial_json = serde_json::json!({"key1": "value1", "key2": "value2"});
        let patch_json = serde_json::json!({"key2": "new_value2"});

        // Create a file first
        let _ = handle_post(id, Some(&initial_json));

        // Patch the file
        let response = handle_patch(id, Some(&patch_json));

        assert_eq!(response.status_code, 200, "Status code should be 200");


        let file_path = format!("./files/{}.json", id);
        let file_contents = fs::read_to_string(&file_path).expect("Failed to read file");
        let saved_json: Value = serde_json::from_str(&file_contents).expect("Failed to parse JSON");
        assert_eq!(saved_json, serde_json::json!({"key1": "value1", "key2": "new_value2"}), "Saved JSON should reflect the patch");

        // Clean up: remove the test file
        fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_handle_patch_file_not_found() {
        let id = "nonexistent_file";
        let patch_json = serde_json::json!({"key": "value"});

        let response = handle_patch(id, Some(&patch_json));

        assert_eq!(response.status_code, 404, "Status code should be 404");
    }

    #[test]
    fn test_handle_patch_invalid_json() {
        let id = "test_patch_invalid";
        let initial_json = serde_json::json!({"key": "value"});
        let invalid_json: Value = serde_json::from_str("{invalid_json}").unwrap_or(Value::Null);

        // Create a file first
        let _ = handle_post(id, Some(&initial_json));

        // Attempt to patch with invalid JSON
        let response = handle_patch(id, Some(&invalid_json));

        assert_eq!(response.status_code, 400, "Status code should be 400");

        // Clean up: remove the test file
        let file_path = format!("./files/{}.json", id);
        fs::remove_file(file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_handle_unallowed_method() {
        let response = handle_method_not_allowed();
        
        assert_eq!(response.status_code, 405, "Status code should be 405");
    }
}