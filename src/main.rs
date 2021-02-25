use api::Api;

mod api;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api: Api = Api::new("");
    let summary = api.summary().await?;
    let user = api.user().await?;

    println!("{:?}", summary);
    println!("{:?}", user);

    Ok(())
}
