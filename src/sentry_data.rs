use serde::Deserialize;

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
pub struct EventData {
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
