# herdr: one distributable binary

How [herdr](https://github.com/herdrdev/herdr) ships so a single CLI
suffices — the pattern srcthread should follow for PC distribution.

## What herdr does

- **One Rust binary** named `herdr`. No separate `herdrd` in the install.
- The same binary is both automation CLI and interactive client.
- **`herdr server`** (bare) runs the headless daemon in-process.
- **`herdr server stop`** (and friends) talk to a running server over the
  socket API.
- Other subcommands are thin socket clients (or local install helpers).
- Plugins / agents use the same NDJSON socket the CLI uses.

Cite: <https://herdr.dev/docs/cli-reference/>, herdr `Cargo.toml` (single
package binary, no second `[[bin]]` for the server).

## Implications for srcthread

| Surface | Distributable shape |
| --- | --- |
| Engine | `srcthread` lib crate |
| Daemon | `srcthread-daemon` **library**; entered via `srcthread daemon` |
| Desktop | `srcthread-desktop` **library**; entered via `srcthread desktop` |
| MCP | library stub; entered via `srcthread mcp` |
| CLI | **only** user-facing binary: `srcthread` |

The product name stays **`srcthread`** — never a trailing-`d` daemon binary
(`srcthreadd` and friends are wrong).

Android stays a separate APK (UniFFI embed) — outside the PC single-binary
story.

Dev/tooling bins (e.g. `uniffi-bindgen`) are fine; they are not the product
install.
