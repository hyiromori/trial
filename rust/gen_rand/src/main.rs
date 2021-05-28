use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn random() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

fn main() {
    println!("Random: {}", random());
}
