use std::collections::HashMap;
use std::option::Option;

#[derive(Debug)]
pub struct SummaryData {
    count: i32,
    value: String,
}

pub fn show_summary_data(title: String, data: &Vec<Option<String>>) -> () {
    println!("[{}]", title);
    get_summary_data(data).iter().for_each(|summary| {
        println!("{:>3}: {}", summary.count, summary.value);
    });
}

pub fn get_summary_data(data: &Vec<Option<String>>) -> Vec<SummaryData> {
    let mut summaries: Vec<SummaryData> = data
        .iter()
        .map(|data| -> String {
            match &data {
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

    summaries.sort_by(|a, b| b.count.cmp(&a.count)); // DESC
    summaries
}
