use rmcp::{
    handler::server::wrapper::Parameters,
    model::{Implementation, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router, ServerHandler,
};
use serde::Deserialize;

use crate::auth::api_base;

#[derive(Clone)]
pub struct UpdateNightMcp {
    pub client: reqwest::Client,
    pub token: String,
}

impl UpdateNightMcp {
    async fn get(&self, path: &str) -> String {
        match self
            .client
            .get(format!("{}{path}", api_base()))
            .bearer_auth(&self.token)
            .send()
            .await
        {
            Ok(r) => r.text().await.unwrap_or_else(|e| e.to_string()),
            Err(e) => format!("Error: {e}"),
        }
    }

    async fn post(&self, path: &str, body: serde_json::Value) -> String {
        match self
            .client
            .post(format!("{}{path}", api_base()))
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await
        {
            Ok(r) => r.text().await.unwrap_or_else(|e| e.to_string()),
            Err(e) => format!("Error: {e}"),
        }
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchParams {
    pub q: String,
    pub kind: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetEntryParams {
    pub kind: String,
    pub slug: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListByCategoryParams {
    pub kind: String,
    pub category: String,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListNewsParams {
    pub days: Option<u32>,
}

#[tool_router]
impl UpdateNightMcp {
    #[tool(description = "Search the Update Night catalog for AI dev tools, skills, and MCP servers. Returns matching entries with name, tagline, pricing, and install snippets.")]
    async fn search(&self, Parameters(p): Parameters<SearchParams>) -> String {
        self.post(
            "/api/search",
            serde_json::json!({
                "q": p.q,
                "kind": p.kind,
                "limit": p.limit.unwrap_or(10),
            }),
        )
        .await
    }

    #[tool(description = "Get a single catalog entry by kind (tool|skill|mcp) and slug. Returns full details including description, pricing, install snippet, and links.")]
    async fn get_entry(&self, Parameters(p): Parameters<GetEntryParams>) -> String {
        self.get(&format!("/api/entries/{}/{}", p.kind, p.slug)).await
    }

    #[tool(description = "List catalog entries by kind (tool|skill|mcp) and category slug (e.g. agent-framework, llm, rag). Returns entries sorted by publish date.")]
    async fn list_by_category(&self, Parameters(p): Parameters<ListByCategoryParams>) -> String {
        self.get(&format!(
            "/api/entries?kind={}&category={}&limit={}",
            p.kind,
            p.category,
            p.limit.unwrap_or(12)
        ))
        .await
    }

    #[tool(description = "List recent news items from the Update Night news timeline. Returns titles, summaries, sources, and timestamps.")]
    async fn list_news(&self, Parameters(p): Parameters<ListNewsParams>) -> String {
        self.get(&format!("/api/news?days={}", p.days.unwrap_or(7))).await
    }
}

#[tool_handler(router = UpdateNightMcp::tool_router())]
impl ServerHandler for UpdateNightMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: Default::default(),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "updatenight".into(),
                version: "1.0.0".into(),
                title: None,
                description: None,
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "Search and explore the Update Night catalog of AI dev tools, skills, and MCP servers.".into(),
            ),
        }
    }
}
