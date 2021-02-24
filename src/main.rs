use models::WkResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.wanikani.com/v2/summary")
        .header(
            "Authorization",
            "Bearer <key here>",
        )
        .send()
        .await?;


    let models = response.json::<WkResponse>().await?;
    println!("{:?}", models);

    Ok(())
}
