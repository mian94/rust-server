// src/service.rs

pub fn get_hello() -> String {
    "Hello, World!".to_string()
}

pub fn post_echo(body: hyper::body::Bytes) -> String {
    format!("Echo: {}", String::from_utf8_lossy(&body))
}