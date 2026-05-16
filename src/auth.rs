use std::time::Duration;

use reqwest::Client;
use serde::Deserialize;
use tokio::time::sleep;

use crate::config::{self, Config};

const CLIENT_ID: &str = "updatenight-mcp";

#[derive(Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: Option<String>,
    interval: Option<u64>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    error: Option<String>,
}

pub fn api_base() -> String {
    std::env::var("UPDATENIGHT_API_URL")
        .unwrap_or_else(|_| "https://server.updatenight.com".to_string())
}

pub async fn ensure_token(client: &Client) -> anyhow::Result<String> {
    let cfg = config::load();
    if let Some(token) = cfg.access_token {
        return Ok(token);
    }
    device_auth_flow(client).await
}

async fn device_auth_flow(client: &Client) -> anyhow::Result<String> {
    let base = api_base();

    let resp: DeviceCodeResponse = {
        let url = format!("{base}/api/auth/device/code");
        let body = serde_json::json!({ "client_id": CLIENT_ID });
        let mut attempt = 0u32;
        loop {
            match client.post(&url).json(&body).send().await {
                Ok(r) => break r.error_for_status()?.json::<DeviceCodeResponse>().await?,
                Err(e) if e.is_connect() && attempt < 15 => {
                    attempt += 1;
                    eprintln!("Waiting for server to be ready... ({attempt}/15)");
                    sleep(Duration::from_secs(2)).await;
                }
                Err(e) => return Err(e.into()),
            }
        }
    };

    let url = resp
        .verification_uri_complete
        .as_deref()
        .unwrap_or(&resp.verification_uri);

    eprintln!("\nUpdate Night MCP - Authentication required");
    eprintln!("Visit: {url}");
    eprintln!("Code:  {}\n", resp.user_code);

    let _ = open::that(url);

    let interval = Duration::from_secs(resp.interval.unwrap_or(5));

    loop {
        sleep(interval).await;

        let tr: TokenResponse = client
            .post(format!("{base}/api/auth/device/token"))
            .json(&serde_json::json!({
                "grant_type": "urn:ietf:params:oauth:grant-type:device_code",
                "device_code": resp.device_code,
                "client_id": CLIENT_ID,
            }))
            .send()
            .await?
            .json()
            .await?;

        if let Some(token) = tr.access_token {
            config::save(&Config {
                access_token: Some(token.clone()),
            });
            eprintln!("Authorized.\n");
            return Ok(token);
        }

        match tr.error.as_deref() {
            Some("authorization_pending") | None => continue,
            Some("slow_down") => sleep(Duration::from_secs(5)).await,
            Some(e) => return Err(anyhow::anyhow!("Auth error: {e}")),
        }
    }
}
