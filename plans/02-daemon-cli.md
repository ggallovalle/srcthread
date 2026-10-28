# Plan: daemon + CLI

## Single binary (herdr-shaped)

PC install is **one** binary: `srcthread`. Daemon and desktop are libraries
entered via subcommands — see `research/08-herdr-single-binary.md`.

```text
srcthread daemon              # run the socket server (foreground)
srcthread daemon stop|status
srcthread desktop             # rune GPU UI
srcthread mcp                 # stdio MCP stub
```

No second binary in the distributable — not a trailing-`d` daemon name, not a
separate desktop executable.

## Daemon (library `srcthread-daemon`)

- Owns open stores, optional RAM index, mirror cache paths.
- Listens on a Unix socket; NDJSON `{id,method,params}` /
  `{id,result|error}` (herdr-shaped).
- Autostart from other CLI commands when socket missing (later).

### Socket methods (sketch)

| Method | Role |
| --- | --- |
| `ping` | liveness |
| `server.stop` | shut down |
| `store.list` / `store.open` / `store.close` | multi-store |
| `thread.list` / `thread.get` / `thread.create` | threads |
| `anchor.list` / `anchor.add` / `anchor.resolve` | anchors |
| `excerpt.get` / `note.get` / `note.set` | offline bodies |
| `ref.list` / `ref.add` | external refs |
| `search.paths` | optional index query |
| `sync.git` / `sync.peer` | later |

## CLI surface

Binary name: `srcthread`. Prefer thin socket client; global `--store`,
`-C`, `-q`, `-v`, `-y`. Layout follows mise
(`research/07-mise-cli-structure.md`): one module per top-level command
under `crates/srcthread-cli/src/cli/`, nested groups as directories,
`usage-rs` (not clap / not `env::args_os()`).

```text
srcthread hello
srcthread doctor                         # alias: dr
srcthread daemon [--socket PATH]         # alias: d; bare → run server
srcthread daemon stop|status
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

Hello-world: `hello` prints via daemon socket if up, else in-process lib.
Other store/thread commands stub until the lib catches up.
