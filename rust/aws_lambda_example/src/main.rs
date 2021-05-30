use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(_event: Value, _context: Context) -> Result<Value, Error> {
    Ok(json!({ "random": random() }))
}

fn random() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

