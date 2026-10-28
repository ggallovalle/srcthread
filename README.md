# srcthread

Personal research-thread tool. Local-first: file-canonical stores, a warm PC
daemon, thin clients (CLI, nvim, desktop, Android, MCP).

Private personal project for now.

## Crates / apps

| Name | Path | Description |
| --- | --- | --- |
| `srcthread` | [`crates/srcthread`](crates/srcthread) | Engine lib (store + facade) |
| `srcthread-daemon` | [`crates/srcthread-daemon`](crates/srcthread-daemon) | Socket daemon **library** |
| `srcthread-cli` | [`crates/srcthread-cli`](crates/srcthread-cli) | **Sole PC binary** `srcthread` |
| `srcthread-mcp` | [`crates/srcthread-mcp`](crates/srcthread-mcp) | MCP **library** (via `srcthread mcp`) |
| desktop | [`apps/desktop`](apps/desktop) | Rune UI **library** (`srcthread desktop`) |
| android | [`apps/android`](apps/android) | Kotlin Compose + UniFFI |
| nvim | [`nvim`](nvim) | Neovim plugin |

PC distribution follows herdr: one binary covers CLI, daemon, and desktop
(`research/08-herdr-single-binary.md`).

## Quick start

```sh
mise install
mise run check
cargo run -p srcthread-cli -- hello
cargo run -p srcthread-cli -- daemon status
cargo run -p srcthread-cli -- desktop   # needs Wayland/GPU; default feature
```

## Docs

- [research/](research/) — notes behind decisions
- [plans/](plans/) — roadmap and surface sketches

## License

MIT — see [LICENSE](LICENSE).
