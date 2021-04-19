// use std::env;
use std::collections::HashMap;

// ISSUE_ID="$1"
// URL="https://sentry.io/api/0/issues/${ISSUE_ID}/events/?full=true"
// curl -H "${AUTHORIZATION_HEADER}" "${URL}"

// fn get_sentry_auth_token() -> String {
//     match env::var("SENTRY_AUTH_TOKEN") {
//         Ok(val) => val,
//         Err(_e) => "".to_string(),
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
