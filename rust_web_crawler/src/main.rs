use reqwest::blocking::Response;
use serde_json::{Value};

fn main() -> eyre::Result<()>{
    let response: Response = reqwest::blocking::get("https://app.hyiromori.com/health")?;
    if response.status().is_success() {
        let body: std::string::String = response.text()?;
        println!("body: {:?}", body);
        let v: Value = serde_json::from_str(&body)?;
        println!("Random: {:?}", v["data"]);
    } else if response.status().is_server_error() {
        println!("Server error");
    } else {
        println!("Something else happened. Status: {:?}", response.status());
    }

    Ok(())
}
