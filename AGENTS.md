# srcthread

Personal research-thread tool (local-first). Primary target: Linux Wayland
Arch. Repo may stay private; treat history as if it could become public.

## Application stack

- Rust + `tokio` — async engine and daemon I/O.
- `usage-rs` — typed CLIs from Rust structs/enums for every CLI surface.
  Prefer over clap or plain `env::args_os()`.
- `rune` — GPU desktop UI (wgpu/winit), plain Rust views; opened via
  `srcthread desktop`.
- UniFFI — bound inside `ktpackages/srcthread`; Android UI uses Compose
  `SrcThread` only (no direct UniFFI).

The `srcthread` crate is the only source-of-truth engine. Daemon, CLI, nvim,
desktop, Android, and MCP call it (in-process or over the socket).

## Single PC binary (herdr-shaped)

`srcthread` is the **only** user-facing PC executable. Daemon, desktop, and
MCP are libraries entered as subcommands (`srcthread daemon`,
`srcthread desktop`, `srcthread mcp`). See
`research/08-herdr-single-binary.md`. Do not add a second product `[[bin]]`
for the daemon or GUI.

## Layout

- `research/` — notes behind decisions
- `plans/` — phased roadmap and surface sketches
- `crates/srcthread` — lib facade + store
- `crates/srcthread-daemon` — socket daemon **library**
- `crates/srcthread-cli` — sole PC binary (`src/cli/` mise-shaped)
- `crates/srcthread-mcp` — MCP **library** (stdio stub)
- `apps/desktop` — rune desktop **library**
- `ktpackages/srcthread` — Kotlin Android library; UniFFI + `SrcThread` facade
- `apps/android` — Kotlin + Jetpack Compose UI (depends on kt package)
- `nvim/` — Neovim plugin (picker / open-at-line)
- `packaging/linux/` — FreeDesktop `.desktop` + app icon (`-dev` ship badge
  via `shellrc/zsh`)
- Gradle root + `detekt.yaml` sit next to `Cargo.toml`

## Commands

- `mise run check`
- `mise run lint`
- `mise run lint-all`
- `mise run check-ci` — what GitHub Actions runs (no rune desktop feature)

CI covers the Rust libs + CLI with `--no-default-features` (skip rune / Android
SDK). Local full GUI: `cargo build -p srcthread-cli` (default `desktop`
feature).

## Git work

Do the git work across `gh stack` which is for stacked prs that let you break a
large change into a chain of pull requests that build on each other.

```sh
gh stack --help
# View current stack
gh stack view
# Start a new stack targeting your default branch
gh stack init
# Or turn an existing set of branches into a stack
gh stack init branch1 branch2 branch3
# Make changes and commit, then add a branch to the stack
gh stack add branch4
# Push all branches and create/update PRs on GitHub
gh stack submit
# Keep your local in sync with remote
gh stack sync
```

Name the branches and messages for what they represent based on conventional
commit 1.0.0.

## This repo may stay private

Everything pushed can become public in history. Before committing, keep these
out:

- secrets, tokens and credential file contents;
- home folder and temp paths, machine names and tailnet hostnames;
- session IDs, chat or Slack links, and quotes from private sessions, chats or
  calls;
- teammates' names attached to their plans, opinions or internal discussions,
  and private repos or branches;
- quotes of, or citations to, Anomaly's internal guides and rules (Abattoir).
  Say what the code does and why on its own terms;
- agent handoff notes (`REVIEW-TO-PARENT.md` and similar), raw evidence, logs
  and benchmark dumps;
