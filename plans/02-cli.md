# Plan: CLI

## Single binary (herdr-shaped, no daemon)

PC install is **one** binary: `srcthread`. Desktop and MCP are libraries
entered via subcommands — see `research/08-herdr-single-binary.md` and
`research/09-no-daemon.md`.

```text
srcthread desktop             # rune GPU UI
srcthread mcp                 # stdio MCP
```

No second product binary; no `srcthread daemon` / `srcthread-daemon` crate.

## Process model

Every command opens the **`srcthread` lib in-process** (file store when that
exists; hello-world uses in-memory). Long-lived hosts (desktop, nvim via CLI
or later embed, MCP) may keep a warm in-process index later; CLI stays
one-shot.

## CLI surface

Binary name: `srcthread`. Global `--store`, `-C`, `-q`, `-v`, `-y`. Layout
follows mise (`research/07-mise-cli-structure.md`): one module per top-level
command under `crates/srcthread-cli/src/cli/`, nested groups as directories,
`usage-rs` (not clap / not `env::args_os()`).

```text
srcthread hello
srcthread doctor                         # alias: dr
srcthread desktop                        # rune UI (feature `desktop`)
srcthread store init|list|use|path       # bare → path
srcthread thread list|show|new|edit|rm   # alias: t; bare → list
srcthread anchor add|list|show|open      # alias: a
srcthread excerpt add|show
srcthread ref add|list
srcthread mirror add|list|path|fetch     # bare → list
srcthread search threads|anchors <query>
srcthread sync git [push|pull|status]    # bare git → status
srcthread sync peer discover|send|receive
srcthread mcp
```

Hello-world: `hello` / `thread list` / `thread new` call the in-process lib.
Other store/thread commands stub until the lib catches up.
