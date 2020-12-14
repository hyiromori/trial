use std::error::Error as StdError;
use lambda_runtime::{error::HandlerError, lambda, Context};
use lambda_container::{hello, HelloEvent, HelloOutput};

fn handler(event: HelloEvent, _context: Context) -> Result<HelloOutput, HandlerError> {
    Ok(hello(event))
}

fn main() -> Result<(), Box<dyn StdError>> {
    lambda!(handler);
    Ok(())
}
