use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct UserData {
    #[allow(non_snake_case)]
    pub isStaff: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub username: Option<String>,
    pub name: Option<String>,
    pub ip_address: Option<String>,
    pub email: Option<String>,
    pub data: Option<UserData>,
    pub id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub r#type: Option<String>,
}

// Ref: https://docs.sentry.io/api/events/list-an-issues-events/
#[derive(Deserialize, Debug)]
pub struct EventData {
    #[allow(non_snake_case)]
    pub eventID: String,
    pub tags: Vec<Tag>,
    #[allow(non_snake_case)]
    pub dateCreated: String,
    pub user: User,
    message: String,
    pub id: String,
    pub platform: String,
    pub event: Option<Event>,
    #[allow(non_snake_case)]
    pub groupID: String,
    pub title: String,
}
