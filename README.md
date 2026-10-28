# srcthread

Personal research-thread tool. Local-first: file-canonical stores, a warm PC
daemon, thin clients (CLI, nvim, desktop, Android, MCP).

Private personal project for now.

## Crates / apps

| Name | Path | Description |
| --- | --- | --- |
| `srcthread` | [`crates/srcthread`](crates/srcthread) | Engine lib (store + facade) |
| `srcthread-daemon` | [`crates/srcthread-daemon`](crates/srcthread-daemon) | Long-lived socket daemon |
| `srcthread-cli` | [`crates/srcthread-cli`](crates/srcthread-cli) | Thin CLI client |
| `srcthread-mcp` | [`crates/srcthread-mcp`](crates/srcthread-mcp) | MCP server for agents |
| desktop | [`apps/desktop`](apps/desktop) | rune desktop UI (later) |
| android | [`apps/android`](apps/android) | Kotlin + UniFFI (later) |
| nvim | [`nvim`](nvim) | Neovim plugin |

## Quick start

```sh
mise install
mise run check
```

## Docs

- [research/](research/) — notes behind decisions
- [plans/](plans/) — roadmap and surface sketches

## License

MIT — see [LICENSE](LICENSE).
