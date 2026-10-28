# Plan: MCP (last)

Ship MCP after lib, daemon, CLI, and at least one human UI path work.

## Why last

Agents are consumers of a stable store + socket. Building MCP first invites
rework every time the thread model shifts.

## Tools (sketch)

Expose read-heavy tools so agents use local anchors/excerpts instead of
web-fetching GitHub:

- `list_threads` / `get_thread`
- `list_anchors` / `resolve_anchor` (path, sha, line/span)
- `get_excerpt` — frozen offline body
- `get_note` / `list_refs`
- optional `search` against the daemon index

Writes (`add_anchor`, `add_note`) can follow once read tools prove useful.

## Process

Prefer attaching to `srcthread daemon` (same NDJSON) so MCP shares the warm index
with nvim/CLI. Standalone embed of the lib is a fallback for CI/tests.
