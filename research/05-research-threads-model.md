# Research-threads model (settled)

Design decisions from grilling. Name: **srcthread**. Glossary:
[`CONTEXT.md`](../CONTEXT.md). Strap: [`PLAN.md`](../PLAN.md).

## Primary object

A **research thread** — a named investigation you reopen later. Not “a note”
and not “a clone” as the atom.

## What hangs off a thread

- **Anchors** — repo @ sha + path + line/span or symbol
- **Excerpts** — frozen offline text around an anchor
- **Refs** — PR/issue/doc/chat URLs or pasted transcripts
- **Notes** — your prose
- **Shared mirrors** — global clone cache; threads pin revision + path, they
  do not own full private clones

Offline bar: open thread, read notes/excerpts, jump anchors in already-mirrored
repos at pinned SHAs. Fresh fetch can need network.

## Store and sync

- **File-canonical git store**: nestable owned folder
  `srcthread/config.toml` + `threads/…` (mise-shaped).
- **Derived index**: v1 = in-process **cold walk** (no `rg`/`fd`). Optional
  later RAM index in long-lived hosts; SQLite only if needed. Never merge SoT.
  See [01-fff-index.md](01-fff-index.md), [09-no-daemon.md](09-no-daemon.md).
- **Multi-store**: many independent store directories.
- **Mirrors**: `[mirrors] id = { git = url }` in store `config.toml`; clones
  under `$XDG_CACHE_HOME/srcthread/mirrors/…` or **`SRCTHREAD_MIRRORS_DIR`**.
  Pins (sha/path) on anchors.
- **Lint / schema**: lenient load; `srcthread lint [--fix]`; schemars JSON
  Schema day one; no vault `schema_version`
  ([10-schema-evolution-mise.md](10-schema-evolution-mise.md)).
- **User registry**: `$XDG_CONFIG_HOME/srcthread/stores.toml` (not a store).
- **Typed files**: thread.toml / AnchorMeta / refs / excerpt frontmatter —
  see `CONTEXT.md` and HANDOFF settled Q28–Q34.
- Sync: **git** when remote/not nearby; **Localsend-like peer** for explicit
  nearby transfer of a store or selected threads. See
  [03-localsend.md](03-localsend.md).

## Process model

PC: **no required daemon**. Every surface embeds the lib in-process
([09-no-daemon.md](09-no-daemon.md)). Android already embeds via UniFFI.

## Build order

1. lib (`srcthread`)
2. CLI
3. nvim
4. desktop (rune)
5. Android (Kotlin + Gradle + UniFFI)
6. MCP (last)

Personal-first; greenfield (not journal / journalit / ghq).
