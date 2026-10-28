# Plan: lib + store format

The `srcthread` crate is the only engine. CLI/UIs/MCP/Android call it
in-process.

## Store sketch (file-canonical)

A store is a directory (typically git-tracked), nestable in a normal project:

```text
srcthread/                    # owned folder (mise-shaped marker)
  config.toml                 # id, name, schema version
  threads/
    <thread-id>/
      thread.toml             # id, title, tags, created, updated
      notes.md                # prose
      anchors/
        <id>.toml             # mirror, sha, path; line/end_line/symbol/title?
        <id>.md               # +++ TOML frontmatter + optional excerpt fences
      excerpts/
        <id>.md               # frontmatter: id, anchor_id, title?; fenced body
      refs/
        <ref-id>.toml         # kind, url?, title?, body_path?
# mirrors: [mirrors] in config.toml → clone URLs; pins (sha/path) on anchors
# clone bytes: $XDG_CACHE_HOME/srcthread/mirrors or SRCTHREAD_MIRRORS_DIR
# one markdown excerpt parser for embedded + standalone
# schemars JSON Schema per typed surface (day one); no vault schema_version
```

Standalone vaults use the same layout at their root (or as the repo contents).

## Lib surface (early)

- `Srcthread::open(path)` / `open_memory()`
- `list_threads`, `get_thread`, `create_thread`
- `list_anchors`, `add_anchor`, `open_anchor` (resolve path+line)
- `add_note` / read notes; excerpt and ref helpers
- Errors via `thiserror`; serde for toml/json borders

## Non-goals for v1

- Committed SQLite as source of truth
- Per-thread private full clones
- PC daemon / NDJSON socket
- CRDT library day-one (git merge + explicit conflicts)
