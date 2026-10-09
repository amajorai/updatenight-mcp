mod auth;
mod config;
mod security;
mod tools;

use rmcp::{transport::stdio, ServiceExt};
use tools::UpdateNightMcp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = security::http_client()?;
    let url = security::http_url(&auth::api_base())?;
    anyhow::ensure!(
        url.path() == "/" && url.query().is_none() && url.fragment().is_none(),
        "API URL must be an origin"
    );
    let base = url.origin().ascii_serialization();
    let token = auth::ensure_token(&client, &base).await?;

    let service = UpdateNightMcp {
        client,
        token,
        base,
    };
    let server = service.serve(stdio()).await?;
    server.waiting().await?;

    Ok(())
}
