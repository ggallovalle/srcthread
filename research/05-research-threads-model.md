# Research-threads model (settled)

Design decisions from grilling. Name: **srcthread**.

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
- **Optional derived index** (RAM in daemon; SQLite only if needed later) —
  never the merge source of truth. See [01-fff-index.md](01-fff-index.md).
- **Multi-store**: many independent store directories.
- Sync: **git** when remote/not nearby; **Localsend-like peer** for explicit
  nearby transfer of a store or selected threads. See
  [03-localsend.md](03-localsend.md).

## Process model

PC: **daemon + NDJSON socket** owns open stores and warm index
([02-herdr-socket.md](02-herdr-socket.md)). Android embeds the Rust lib
(UniFFI) without requiring the daemon.

## Build order

1. lib (`srcthread`)
2. daemon + socket (`srcthreadd`)
3. CLI
4. nvim
5. desktop (rune)
6. Android (Kotlin + Gradle + UniFFI)
7. MCP (last)

Personal-first; greenfield (not journal / journalit / ghq).
