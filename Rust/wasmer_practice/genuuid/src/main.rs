use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    println!("{}", uuid);
    Ok(())
}
