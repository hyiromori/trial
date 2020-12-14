use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct HelloEvent {}

#[derive(Debug, Serialize)]
pub struct HelloOutput {
    message: String
}

pub fn hello(_event: HelloEvent) -> HelloOutput {
    HelloOutput {
        // message: format!("Hello, {}!", event.name)
        message: format!("Hello, Rust on Docker!!")
    }
}
