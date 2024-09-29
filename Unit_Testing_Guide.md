# Unit Testing

Each file (`client.rs`, `methods.rs`, `server.rs`, and `methods.rs`) needs to have a thorough set of tests. 

An example of unit testing is the following:
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}
```

The proper way to do unit testing is to check if each function is doing its job with a various of cases, it could be useful to test for edge cases too.

For example, another unit test could be to test what happens when you add the max integer with a 1, and assert the overflow occurs.

## Example

For the `server.rs` file:
```rust
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
```

To test the `handle_cookie` function, we would to check that it returns a valid uuid and if it was added properly to sessions variable.

To test the `run` function, we would need to refactor the code to add an identifier to each thread and research if there is a way to check that each thread has been created successfully. We could test for other cases, for example
- What would happen if we send 10 requests at the same time? 
- Is the ThreadPool working as it is supposed to? 
- Are we handling the errors correctly? 
- Can we force an error to check if the Connection failed is being displayed correctly?

# Keywords
To successfully made a unit test, we need to use the following keywords:
- `assert(expr, message)` this would panic if the expression is false, therefore failing the test. We can customize the panic error (message), however this is optional.
- `assert_eq!(left, right, message)` this would panic if the left and right expressions aren't equal. We can customize the panic error (message), however this is optional.
- `assert_ne!(left, right, message)` this would panic if the left and right expressions are equal. We can customize the panic error (message), however this is optional.

For more information regarding this keywords, you can check the official documentation for each keyword:
- [assert](https://doc.rust-lang.org/std/macro.assert.html)
- [assert_eq!](https://doc.rust-lang.org/std/macro.assert_eq.html)
- [assert_ne!](https://doc.rust-lang.org/std/macro.assert_ne.html)
