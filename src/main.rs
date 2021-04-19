use std::env;
use serde::{Deserialize};

fn get_authorization_header_for_sentry() -> String {
    match env::var("SENTRY_AUTH_TOKEN") {
        Ok(val) => format!("Bearer {}", val),
        Err(_e) => "".to_string(),
    }
}

#[derive(Deserialize, Debug)]
struct Data {
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://sentry.io/api/0/issues/1197906478/events/";
    let data = reqwest::Client::new()
        .get(url)
        .header("Authorization", get_authorization_header_for_sentry())
        .send()
        .await?
        .json::<Vec<Data>>()
        .await?;
    println!("{:?}", data);
    Ok(())
}
