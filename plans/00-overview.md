# Roadmap overview

Phases follow the settled build order. Each phase ships a hello-world then
deepens behavior; CI stays Rust-workspace-only until GUI/Android matrices
are intentional.

Product strap: [PLAN.md](../PLAN.md). No PC daemon:
[research/09-no-daemon.md](../research/09-no-daemon.md).

| Phase | Deliverable | Plan |
| --- | --- | --- |
| 0 | Workspace + tooling + research | (this tree) |
| 1 | Lib + file store | [01-lib-store.md](01-lib-store.md) |
| 2 | CLI | [02-cli.md](02-cli.md) |
| 3 | Neovim | [03-nvim.md](03-nvim.md) |
| 4 | Desktop + Android | [04-desktop-android.md](04-desktop-android.md) |
| 5 | MCP | [05-mcp.md](05-mcp.md) |
| 6 | Packaging / packslip releases | [06-packaging.md](06-packaging.md) |

## Phase goals (short)

1. **Lib** — open store, CRUD threads/anchors/notes; in-memory for tests.
2. **CLI** — full surface sketched; leaves call the in-process lib (stubs OK).
3. **nvim** — pick thread/anchor, open at line (fff-inspired UX).
4. **Desktop / Android** — rune browser; phone capture + read offline.
5. **MCP** — agents read anchors/excerpts without web fetch.
6. **Packaging** — packslip PC releases (variants, completions, man),
   `integration` / `desktop install-xdg`, manual publish + Android APK.

Sync (git + peer) and optional in-process RAM index land after the store
format is stable; see research notes.
