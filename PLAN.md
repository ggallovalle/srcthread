# PLAN — srcthread

Living strap for settled product shape. Phased work lives under [`plans/`](plans/).
Glossary: [`CONTEXT.md`](CONTEXT.md). Decision notes: [`research/`](research/).

## Strap

**srcthread** is a personal, local-first **research-thread** tool: file-canonical
**stores** (git-friendly, multi-store, nestable in normal project repos),
**shared mirrors** in XDG cache (env-overridable path; never inside the store),
and thin surfaces (CLI, nvim, desktop, Android, MCP) that all embed the
**`srcthread` lib in-process**.

**No PC daemon in v1.** Herdr’s socket server was the wrong spine for a
mergeable file vault — see [research/09-no-daemon.md](research/09-no-daemon.md).
Optional warm indexes belong in long-lived hosts (desktop / nvim / MCP) later,
fff-style; cold walk is enough until then.

PC install stays **one binary** (`srcthread`) with library subcommands
`desktop` and `mcp` ([research/08-herdr-single-binary.md](research/08-herdr-single-binary.md)).

## Build order

1. Lib + file store
2. CLI
3. Neovim
4. Desktop (rune)
5. Android (Compose + UniFFI)
6. MCP

## Roadmap pointers

| Phase | Plan |
| --- | --- |
| Lib + store | [plans/01-lib-store.md](plans/01-lib-store.md) |
| CLI | [plans/02-cli.md](plans/02-cli.md) |
| nvim | [plans/03-nvim.md](plans/03-nvim.md) |
| Desktop + Android | [plans/04-desktop-android.md](plans/04-desktop-android.md) |
| MCP | [plans/05-mcp.md](plans/05-mcp.md) |
