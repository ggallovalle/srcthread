# herdr: one distributable binary

How [herdr](https://github.com/herdrdev/herdr) ships so a single CLI
suffices — the packaging pattern srcthread still follows for PC distribution
(**without** copying herdr’s required socket server; see
[09-no-daemon.md](09-no-daemon.md)).

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
| Desktop | desktop **library**; entered via `srcthread desktop` |
| MCP | library; entered via `srcthread mcp` |
| CLI | **only** user-facing binary: `srcthread` |

We take the **one product binary** lesson, not the **mandatory server**
lesson. Do not add a trailing-`d` daemon binary or a `srcthread daemon`
subcommand for v1.

Android stays a separate APK (UniFFI embed) — outside the PC single-binary
story.

Dev/tooling bins (e.g. `uniffi-bindgen`) are fine; they are not the product
install.
