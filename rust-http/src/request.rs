// Struct to represent an HTTP request
#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub _headers: Vec<String>,
    pub body: String,
    pub cookie: Option<String>,
}
