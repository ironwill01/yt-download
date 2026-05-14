pub mod requestcore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    requestcore::request::request::run().await?;
    Ok(())
}