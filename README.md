# Update Night MCP

MCP server for the Update Night catalog. Gives AI assistants tools to search and explore AI dev tools, agent frameworks, skills, and MCP servers.

## Tools

**search** -- search the catalog by query string. Optionally filter by kind (`tool`, `skill`, `mcp`) and set a result limit. Returns entries with name, tagline, pricing, and install snippets.

**get_entry** -- fetch a single catalog entry by kind and slug. Returns full details including description, pricing, install snippet, and homepage URL.

**list_by_category** -- list entries by kind and category slug (e.g. `agent-framework`, `llm`, `rag`, `vector-db`). Returns entries sorted by publish date.

**list_news** -- fetch recent news items from the Update Night news timeline. Accepts a `days` parameter (default 7). Returns titles, summaries, sources, and timestamps.

## Build

```
git clone https://github.com/amajorai/updatenight-mcp
cd updatenight-mcp
cargo build --release
```

Binary output: `target/release/updatenight-mcp` (or `updatenight-mcp.exe` on Windows).

Move the binary somewhere on your PATH, for example:

```
# macOS / Linux
sudo mv target/release/updatenight-mcp /usr/local/bin/updatenight-mcp

# Windows (run as admin, or pick any directory on PATH)
move target\release\updatenight-mcp.exe C:\Windows\System32\updatenight-mcp.exe
```

## Authentication

On first run the server starts a device authorization flow, prints a verification URL and code to stderr, and opens your browser. Once approved, the token is saved and subsequent runs start immediately.

Token location: `~/.config/updatenight/mcp-config.json` (Linux/macOS) or `%APPDATA%\updatenight\mcp-config.json` (Windows).

## Installation by host

Replace `/usr/local/bin/updatenight-mcp` with the actual path to your binary in all configs below.

### Claude Desktop

Config file:
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`
- Linux: `~/.config/claude-desktop/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "updatenight": {
      "command": "/usr/local/bin/updatenight-mcp"
    }
  }
}
```

Quit and relaunch Claude Desktop after editing.

### Claude Code

```bash
claude mcp add updatenight /usr/local/bin/updatenight-mcp
```

To add it at project scope (committed to `.mcp.json` for team sharing):

```bash
claude mcp add --scope project updatenight /usr/local/bin/updatenight-mcp
```

### Cursor

Config file:
- Project: `.cursor/mcp.json`
- Global: `~/.cursor/mcp.json`

```json
{
  "mcpServers": {
    "updatenight": {
      "command": "/usr/local/bin/updatenight-mcp"
    }
  }
}
```

Quit and reopen Cursor after editing.

### Windsurf

Config file:
- macOS/Linux: `~/.codeium/windsurf/mcp_config.json`
- Windows: `%USERPROFILE%\.codeium\windsurf\mcp_config.json`

```json
{
  "mcpServers": {
    "updatenight": {
      "command": "/usr/local/bin/updatenight-mcp"
    }
  }
}
```

### VS Code (GitHub Copilot)

Config file: `.vscode/mcp.json` in your project root (commit to share with your team).

Note: VS Code uses `servers` not `mcpServers`, and requires an explicit `"type"` field.

```json
{
  "servers": {
    "updatenight": {
      "type": "stdio",
      "command": "/usr/local/bin/updatenight-mcp"
    }
  }
}
```

MCP tools are only available in Agent mode in VS Code Copilot Chat.

### Cline (VS Code extension)

Open the Cline panel, click the MCP Servers icon, go to the Configure tab, and click "Configure MCP Servers". Add to the JSON that opens:

```json
{
  "mcpServers": {
    "updatenight": {
      "command": "/usr/local/bin/updatenight-mcp"
    }
  }
}
```

### Continue.dev

Config file: `~/.continue/config.yaml` (macOS/Linux) or `%USERPROFILE%\.continue\config.yaml` (Windows).

```yaml
mcpServers:
  - name: Update Night
    command: /usr/local/bin/updatenight-mcp
    type: stdio
```

Or drop a JSON file at `.continue/mcpServers/updatenight.json` in your project:

```json
{
  "mcpServers": {
    "updatenight": {
      "command": "/usr/local/bin/updatenight-mcp"
    }
  }
}
```

### Zed

Config file:
- macOS: `~/.zed/settings.json`
- Linux: `~/.config/zed/settings.json`
- Windows: `%APPDATA%\Zed\settings.json`
- Project: `.zed/settings.json`

Note: Zed uses `context_servers` not `mcpServers`.

```json
{
  "context_servers": {
    "updatenight": {
      "command": "/usr/local/bin/updatenight-mcp",
      "args": []
    }
  }
}
```

### Amazon Q Developer

Config file:
- Global: `~/.aws/amazonq/mcp.json`
- Project: `.amazonq/mcp.json`

```json
{
  "mcpServers": {
    "updatenight": {
      "command": "/usr/local/bin/updatenight-mcp"
    }
  }
}
```

### Gemini CLI

Config file:
- Global: `~/.gemini/settings.json`
- Project: `.gemini/settings.json`

```json
{
  "mcpServers": {
    "updatenight": {
      "command": "/usr/local/bin/updatenight-mcp"
    }
  }
}
```

### OpenAI Codex CLI

Config file: `~/.codex/config.toml` (global) or `.codex/config.toml` (project).

Note: Codex uses TOML, not JSON.

```toml
[mcp_servers.updatenight]
command = "/usr/local/bin/updatenight-mcp"
args = []
```

### Raycast

Open Raycast, run "Manage MCP Servers", press Cmd+N to add a new server, or open the config file directly at `~/Library/Application Support/com.raycast.macos/extensions/EvanZhouDev.mcp/mcp.json`:

```json
{
  "mcpServers": {
    "updatenight": {
      "command": "/usr/local/bin/updatenight-mcp"
    }
  }
}
```

## Configuration

Set `UPDATENIGHT_API_URL` to point at a different API host. Defaults to `https://server.updatenight.com`.

## Related

- [Update Night CLI](https://github.com/amajorai/updatenight-cli) -- terminal UI for browsing the catalog
- [Update Night Skill](https://github.com/amajorai/updatenight-skill) -- Claude Code skill for browsing the catalog from any AI agent
