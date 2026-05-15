# updatenight-mcp

MCP server for the Update Night catalog. Gives AI assistants tools to search and explore AI dev tools, skills, and MCP servers.

## Tools

**search** -- search the catalog by query string. Optionally filter by kind (`tool`, `skill`, `mcp`) and set a result limit. Returns entries with name, tagline, pricing, and install snippets.

**get_entry** -- fetch a single catalog entry by kind and slug. Returns full details including description, pricing, install snippet, and homepage URL.

**list_by_category** -- list entries by kind and category slug (e.g. `agent-framework`, `llm`, `rag`, `vector-db`). Returns entries sorted by publish date.

**list_news** -- fetch recent news items from the Update Night news timeline. Accepts a `days` parameter (default 7). Returns titles, summaries, sources, and timestamps.

## Authentication

On first run the server starts a device authorization flow. It prints a URL and user code to stderr and opens the browser. Once you approve, the token is saved to `~/.config/updatenight/mcp-config.json` (Linux/macOS) or `%APPDATA%\updatenight\mcp-config.json` (Windows) and the server starts normally on subsequent runs.

## Building

```
cargo build --release
```

The binary will be at `target/release/updatenight-mcp` (or `updatenight-mcp.exe` on Windows).

## Claude Desktop setup

Add this to your Claude Desktop config (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "updatenight": {
      "command": "/path/to/updatenight-mcp"
    }
  }
}
```

## Configuration

Set `UPDATENIGHT_API_URL` to point at a different API host (defaults to `https://server.updatenight.com`).
