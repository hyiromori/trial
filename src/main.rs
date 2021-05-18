use std::env;

use crate::sentry_data::EventData;
use crate::user_info::show_summary_data;

mod sentry_data;
mod user_info;

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

    println!();
    let ip_address_list: Vec<Option<String>> = data.iter().map(|d| {
        match &d.user.ip_address {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }).collect();
    show_summary_data(String::from("IP Addresses"), &ip_address_list);

    println!();
    let platform_list: Vec<Option<String>> = data
        .iter()
        .map(|d| Some(String::from(&d.platform)))
        .collect();
    show_summary_data(String::from("Platform"), &platform_list);

    println!();
    Ok(())
}
