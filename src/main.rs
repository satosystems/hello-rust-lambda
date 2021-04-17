use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(move |e, c| handler(e, c))).await?;
    Ok(())
}

async fn handler(event: CustomEvent, _context: Context) -> Result<CustomOutput, Error> {
    Ok(CustomOutput {
        message: format!("Helli, {} {}!", event.given_name, event.family_name),
    })
}
