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
      thread.toml             # title, tags, created/updated
      notes.md                # prose
      anchors/
        <anchor-id>.toml      # repo, sha, path, line/span/symbol
      excerpts/
        <excerpt-id>.txt      # frozen body (+ sidecar toml meta)
      refs/
        <ref-id>.toml         # url / kind / optional pasted body path
# mirrors live outside the vault (XDG cache); store only pins
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
