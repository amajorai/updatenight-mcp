import { App } from "@modelcontextprotocol/ext-apps";

interface SearchItem {
  entryId: string;
  score: number;
  kind: "tool" | "skill" | "mcp";
  slug: string;
  name: string;
  tagline: string;
  pricing: "free" | "paid" | "freemium" | null;
}

interface SearchResponse {
  items: SearchItem[];
  total: number;
  query: string;
  blurred: boolean;
}

function escape(str: string): string {
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

const KIND_COLORS: Record<string, string> = {
  tool: "#3b82f6",
  skill: "#10b981",
  mcp: "#8b5cf6",
};

const PRICING_LABELS: Record<string, { label: string; color: string }> = {
  free: { label: "Free", color: "#10b981" },
  paid: { label: "Paid", color: "#3b82f6" },
  freemium: { label: "Freemium", color: "#8b5cf6" },
};

function badge(text: string, color: string): string {
  return `<span style="background:${color}22;color:${color};padding:2px 8px;border-radius:4px;font-size:11px;font-weight:600;text-transform:uppercase;letter-spacing:0.5px">${text}</span>`;
}

function renderCard(item: SearchItem): string {
  const color = KIND_COLORS[item.kind] ?? "#6b7280";
  const pricing = item.pricing ? PRICING_LABELS[item.pricing] : null;
  const url = `https://updatenight.com/${item.kind}/${item.slug}`;

  return `
    <a href="${url}" target="_blank" rel="noopener noreferrer" class="card">
      <div style="display:flex;gap:6px;align-items:center;margin-bottom:10px;flex-wrap:wrap">
        ${badge(item.kind, color)}
        ${pricing ? `<span style="color:${pricing.color};font-size:12px;font-weight:500">${pricing.label}</span>` : ""}
      </div>
      <div style="font-weight:600;font-size:14px;color:#fff;margin-bottom:5px;line-height:1.3">${escape(item.name)}</div>
      <div style="font-size:12px;color:#777;line-height:1.5">${escape(item.tagline ?? "")}</div>
    </a>
  `;
}

function render(data: SearchResponse): void {
  const el = document.getElementById("container");
  if (!el) return;

  if (data.blurred) {
    el.innerHTML = `<div class="empty"><div style="font-size:20px;margin-bottom:10px">🔒</div><div>Sign in to search the catalog</div></div>`;
    return;
  }

  if (!data.items?.length) {
    el.innerHTML = `<div class="empty">No results for <strong>"${escape(data.query ?? "")}"</strong></div>`;
    return;
  }

  el.innerHTML = `
    <div style="color:#555;font-size:12px;margin-bottom:14px">
      ${data.total} result${data.total !== 1 ? "s" : ""} for
      <span style="color:#aaa">"${escape(data.query)}"</span>
    </div>
    <div class="grid">${data.items.map(renderCard).join("")}</div>
  `;
}

// Inject styles
const style = document.createElement("style");
style.textContent = `
  * { box-sizing: border-box; }
  body {
    margin: 0;
    padding: 16px;
    background: #0a0a0a;
    color: #fff;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
    font-size: 14px;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
    gap: 10px;
  }
  .card {
    display: block;
    background: #111;
    border: 1px solid #1e1e1e;
    border-radius: 8px;
    padding: 14px;
    text-decoration: none;
    color: inherit;
    transition: border-color 0.15s, background 0.15s;
  }
  .card:hover {
    border-color: #333;
    background: #161616;
  }
  .empty {
    text-align: center;
    padding: 48px 24px;
    color: #555;
    font-size: 14px;
  }
`;
document.head.appendChild(style);

// Show initial state
const container = document.getElementById("container");
if (container) {
  container.innerHTML = `<div class="empty">Waiting for search results…</div>`;
}

// Connect to host
const app = new App({ name: "Update Night Search", version: "1.0.0" });

app.ontoolresult = (result) => {
  const textContent = result.content?.find(
    (c): c is { type: "text"; text: string } => c.type === "text"
  );
  if (!textContent?.text) return;
  try {
    render(JSON.parse(textContent.text) as SearchResponse);
  } catch {
    // malformed response
  }
};

app.connect();
