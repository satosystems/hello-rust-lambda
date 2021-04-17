use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CustomEvent {
    family_name: String,
    given_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn StdError>> {
    lambda!(handler);
    Ok(())
}

fn handler(event: CustomEvent, _context: Context) -> Result<CustomOutput, HandlerError> {
    Ok(CustomOutput {
        message: format!("Helli, {} {}!", event.given_name, event.family_name),
    })
}
