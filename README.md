# srcthread

Personal research-thread tool. Local-first: file-canonical stores, shared
source mirrors, thin clients (CLI, nvim, desktop, Android, MCP) that embed the
engine lib in-process. No PC daemon — see
[`research/09-no-daemon.md`](research/09-no-daemon.md).

Private personal project for now.

## Crates / apps

| Name | Path | Description |
| --- | --- | --- |
| `srcthread` | [`crates/srcthread`](crates/srcthread) | Engine lib (store + facade) |
| `srcthread-cli` | [`crates/srcthread-cli`](crates/srcthread-cli) | **Sole PC binary** `srcthread` |
| `srcthread-mcp` | [`crates/srcthread-mcp`](crates/srcthread-mcp) | MCP **library** (via `srcthread mcp`) |
| desktop | [`apps/desktop`](apps/desktop) | Rune UI **library** (`srcthread desktop`) |
| android | [`apps/android`](apps/android) | Kotlin Compose UI |
| `srcthread` (kt) | [`ktpackages/srcthread`](ktpackages/srcthread) | Compose `SrcThread` + UniFFI |
| nvim | [`nvim`](nvim) | Neovim plugin |

PC distribution follows herdr’s single-binary idea for CLI + desktop + MCP
(`research/08-herdr-single-binary.md`), without a socket daemon.

## In-repo store (dogfood)

This repository nests a store at [`srcthread/`](srcthread/) — same layout a
normal project would use. The exemplar thread is **How is Deno.env.get
implemented?** (anchors, excerpts, refs, notes against
`denoland/deno @ 6e9f66f5…`).

## Quick start

```sh
mise install
mise run check
cargo run -p srcthread-cli -- hello
cargo run -p srcthread-cli -- desktop   # needs Wayland/GPU; default feature
```

## Install (released builds)

PC via packslip (after a tagged release):

```toml
[tools]
"packslip:github.com/ggallovalle/srcthread" = {
  version = "0.1.0",
  variant = "desktop",
}
```

Omit `variant` for the cli-only build (`--no-default-features`). Optional
postinstall:

```toml
postinstall = """
  srcthread integration install cursor --user && \
  srcthread desktop install-xdg
"""
```

Agent MCP wiring and FreeDesktop helpers:

```sh
srcthread integration install cursor --user   # or --local
srcthread integration status
srcthread desktop install-xdg                 # Linux; --dev for ship-badge
```

Android: download `srcthread-*-android.apk` from the GitHub release.

Cut a release locally: `scripts/publish-release.sh <version>` (see
[plans/06-packaging.md](plans/06-packaging.md)). The GitHub Actions release
workflow is checked in but deactivated.

## Docs

- [CONTEXT.md](CONTEXT.md) — glossary (thread, anchor, excerpt, …)
- [PLAN.md](PLAN.md) — product strap + build order
- [research/](research/) — notes behind decisions
- [plans/](plans/) — phased roadmap and surface sketches

## License

MIT — see [LICENSE](LICENSE).
