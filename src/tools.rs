use rmcp::{
    RoleServer,
    handler::server::{
        tool::ToolCallContext,
        wrapper::Parameters,
    },
    model::{
        Annotated, Annotations, CallToolRequestParams, CallToolResult, Implementation,
        ListResourcesResult, ListToolsResult, Meta, PaginatedRequestParams, RawResource,
        ReadResourceRequestParams, ReadResourceResult, ResourceContents, ServerCapabilities,
        ServerInfo, Tool,
    },
    schemars, service::RequestContext, tool, tool_router, ServerHandler,
};
use serde::Deserialize;

use crate::auth::api_base;

const SEARCH_UI_HTML: &str = include_str!("ui/search.html");

const SEARCH_UI_URI: &str = "ui://updatenight/search";

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

impl ServerHandler for UpdateNightMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: Default::default(),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
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

    fn get_tool(&self, name: &str) -> Option<Tool> {
        Self::tool_router().get(name).cloned()
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, rmcp::ErrorData> {
        let mut tools = Self::tool_router().list_all();

        for tool in &mut tools {
            if tool.name == "search" {
                let mut meta = Meta::new();
                meta.insert(
                    "ui".to_string(),
                    serde_json::json!({ "resourceUri": SEARCH_UI_URI }),
                );
                tool.meta = Some(meta);
            }
        }

        Ok(ListToolsResult::with_all_items(tools))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let router = Self::tool_router();
        let ctx = ToolCallContext::new(self, request, context);
        router.call(ctx).await
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, rmcp::ErrorData> {
        let resource = Annotated {
            raw: RawResource {
                uri: SEARCH_UI_URI.to_string(),
                name: "Search Results UI".to_string(),
                title: Some("Update Night Search".to_string()),
                description: Some(
                    "Interactive card grid for Update Night catalog search results".to_string(),
                ),
                mime_type: Some("text/html;profile=mcp-app".to_string()),
                size: None,
                icons: None,
                meta: None,
            },
            annotations: None::<Annotations>,
        };

        Ok(ListResourcesResult::with_all_items(vec![resource]))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, rmcp::ErrorData> {
        if request.uri != SEARCH_UI_URI {
            return Err(rmcp::ErrorData::resource_not_found(
                format!("unknown resource: {}", request.uri),
                None,
            ));
        }

        Ok(ReadResourceResult {
            contents: vec![ResourceContents::TextResourceContents {
                uri: SEARCH_UI_URI.to_string(),
                mime_type: Some("text/html;profile=mcp-app".to_string()),
                text: SEARCH_UI_HTML.to_string(),
                meta: None,
            }],
        })
    }
}
