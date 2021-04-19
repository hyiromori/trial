use std::env;
use serde::{Deserialize};

fn get_authorization_header_for_sentry() -> String {
    match env::var("SENTRY_AUTH_TOKEN") {
        Ok(val) => format!("Bearer {}", val),
        Err(_e) => "".to_string(),
    }
}

#[derive(Deserialize, Debug)]
struct Tag {
    key: String,
    value: String,
}

#[derive(Deserialize, Debug)]
struct UserData {
    #[allow(non_snake_case)]
    isStaff: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct User {
    username: Option<String>,
    name: Option<String>,
    ip_address: Option<String>,
    email: Option<String>,
    data: Option<UserData>,
    id: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Event {
    r#type: Option<String>,
}

// Ref: https://docs.sentry.io/api/events/list-an-issues-events/
#[derive(Deserialize, Debug)]
struct EventData {
    #[allow(non_snake_case)]
    eventID: String,
    tags: Vec<Tag>,
    #[allow(non_snake_case)]
    dateCreated: String,
    user: User,
    message: String,
    id: String,
    platform: String,
    event: Option<Event>,
    #[allow(non_snake_case)]
    groupID: String,
    title: String,
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
