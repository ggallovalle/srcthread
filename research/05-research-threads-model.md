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

- **File-canonical git store** (Obsidian-vault shaped): mergeable on disk.
  Intended to nest in normal project repos via an owned folder marker
  (mise-shaped; exact layout still open).
- **Optional derived index** (in-process in long-lived hosts; SQLite only if
  needed later) — never the merge source of truth. See
  [01-fff-index.md](01-fff-index.md), [09-no-daemon.md](09-no-daemon.md).
- **Multi-store**: many independent store directories.
- **Mirrors**: `$XDG_CACHE_HOME/srcthread/mirrors/…`, overridable via env
  (not store config).
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
