mod auth;
mod config;
mod tools;

use rmcp::{ServiceExt, transport::stdio};
use tools::UpdateNightMcp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let client = reqwest::Client::new();
    let token = auth::ensure_token(&client).await?;

    let service = UpdateNightMcp { client, token };
    let server = service.serve(stdio()).await?;
    server.waiting().await?;

    Ok(())
}
