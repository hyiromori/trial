use crate::sentry_data::EventData;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SummaryData {
    count: i32,
    value: String,
}
impl SummaryData {
    fn increment(&mut self) {
        self.count = self.count + 1;
    }
}

#[derive(Debug)]
pub struct UserInfo {
    pub ip_addresses: Vec<String>,
}

pub fn get_ip_addresses_summary(event_data_list: &Vec<EventData>) -> Vec<SummaryData> {
    let mut ip_addresses_summary: Vec<SummaryData> = event_data_list
        .iter()
        .map(|data| -> String {
            match &data.user.ip_address {
                Some(val) => String::from(val),
                None => "".to_string(),
            }
        })
        .filter(|ip_address| ip_address != &"".to_string())
        .fold(HashMap::new(), |mut acc, ip_address| {
            let count = acc.entry(ip_address).or_insert(0);
            *count += 1;
            acc
        })
        .iter()
        .map(|(ip_address, count)| -> SummaryData {
            SummaryData {
                value: String::from(ip_address),
                count: *count,
            }
        })
        .collect();

    ip_addresses_summary.sort_by(|a, b| b.count.cmp(&a.count)); // DESC
    ip_addresses_summary
}
