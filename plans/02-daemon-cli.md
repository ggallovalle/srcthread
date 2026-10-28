# Plan: daemon + CLI

## Daemon (`srcthreadd`)

- Owns open stores, optional RAM index, mirror cache paths.
- Listens on a Unix socket; NDJSON `{id,method,params}` /
  `{id,result|error}` (herdr-shaped).
- Autostart from CLI when socket missing.

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
`usage-rs` (not clap).

```text
srcthread hello
srcthread doctor                         # alias: dr
srcthread daemon start|stop|status       # alias: d; bare → status
srcthread store init|list|use|path       # bare → path
srcthread thread list|show|new|edit|rm   # alias: t; bare → list
srcthread anchor add|list|show|open      # alias: a
srcthread excerpt add|show
srcthread ref add|list
srcthread mirror add|list|path|fetch     # bare → list
srcthread search threads|anchors <query>
srcthread sync git [push|pull|status]    # bare git → status
srcthread sync peer discover|send|receive
```

Hello-world: `hello` prints via daemon socket if up, else in-process lib.
Other commands stub until the lib catches up.
