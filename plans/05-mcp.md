# Plan: MCP

Uses the official Rust SDK
[`modelcontextprotocol/rust-sdk`](https://github.com/modelcontextprotocol/rust-sdk)
(`rmcp` on crates.io). Entered via `srcthread mcp` (no separate binary).

## Hello-world tools

| Tool | Kind | Role |
| --- | --- | --- |
| `hello` | query | greeting token |
| `list_threads` | query | thread ids |
| `create_thread` | mutation | create a thread id |

Transport: stdio (`rmcp::transport::stdio`).

## Later

Expose read-heavy tools so agents use local anchors/excerpts instead of
web-fetching GitHub:

- `get_thread` / `list_anchors` / `resolve_anchor`
- `get_excerpt` / `get_note` / `list_refs`
- optional `search` against the daemon index

Prefer attaching to `srcthread daemon` (same NDJSON) so MCP shares the warm
index with nvim/CLI. Standalone embed of the lib remains the hello-world path.
