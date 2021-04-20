mod sentry_data;
use crate::sentry_data::EventData;
use std::env;

fn get_authorization_header_for_sentry() -> String {
    match env::var("SENTRY_AUTH_TOKEN") {
        Ok(val) => format!("Bearer {}", val),
        Err(_e) => "".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://sentry.io/api/0/issues/1197906478/events/";
    let data = reqwest::Client::new()
        .get(url)
        .header("Authorization", get_authorization_header_for_sentry())
        .send()
        .await?
        .json::<Vec<EventData>>()
        // .text()
        .await?;
    println!("{:?}", data);
    Ok(())
}
