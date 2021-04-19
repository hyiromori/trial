use std::env;

fn get_sentry_auth_token() -> String {
    match env::var("SENTRY_AUTH_TOKEN") {
        Ok(val) => val,
        Err(_e) => "".to_string(),
    }
}

fn main() {
    let authorization_header = format!("Authorization: Bearer {}", get_sentry_auth_token());
    println!("{}", authorization_header);
}
