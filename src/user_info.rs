use crate::sentry_data::EventData;

#[derive(Debug)]
struct SummaryData {
    count: i32,
    value: String,
}

#[derive(Debug)]
pub struct UserInfo {
    pub ip_addresses: Vec<String>,
}

pub fn get_user_info_summary(event_data_list: &Vec<EventData>) -> Vec<String> {
    event_data_list
        .into_iter()
        .map(|data| -> String {
            match &data.user.ip_address {
                Some(val) => String::from(val),
                None => "".to_string(),
            }
        })
        .collect()
}
