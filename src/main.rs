use std::env;

fn get_sentry_auth_token() -> String {
    match env::var("SENTRY_AUTH_TOKEN") {
        Ok(val) => format!("Bearer {}", val),
        Err(_e) => "".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "https://sentry.io/api/0/issues/1197906478/events/";
    let resp = client.get(url).header("Authorization", get_sentry_auth_token())
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
