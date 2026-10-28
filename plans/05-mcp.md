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
- optional `search` against an in-process index in the MCP process

Embed the `srcthread` lib in the MCP stdio process (fff-shaped warm host).
No PC daemon socket.
