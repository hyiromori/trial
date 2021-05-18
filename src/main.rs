mod sentry_data;
mod user_info;

use crate::sentry_data::EventData;
use crate::user_info::get_user_info_summary;
use std::env;

fn get_authorization_header_for_sentry() -> String {
    // https://sentry.io/settings/account/api/auth-tokens/
    match env::var("SENTRY_AUTH_TOKEN") {
        Ok(val) => format!("Bearer {}", val),
        Err(_err) => "".to_string(),
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
        .await?;
    let user_info = get_user_info_summary(&data);
    println!("{:?}", user_info);
    Ok(())
}
