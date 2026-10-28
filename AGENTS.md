# srcthread

Personal research-thread tool (local-first). Primary target: Linux Wayland
Arch. Repo may stay private; treat history as if it could become public.

## Application stack

- Rust + `tokio` — async engine and daemon I/O.
- `usage-rs` — typed CLIs from Rust structs/enums (prefer over clap).
- `rune` later — GPU desktop UI (wgpu/winit), plain Rust views.
- UniFFI later — bind the Rust lib into the Kotlin/Gradle Android app.

The `srcthread` crate is the only source-of-truth engine. Daemon, CLI, nvim,
desktop, Android, and MCP call it (in-process or over the socket).

## Layout

- `research/` — notes behind decisions
- `plans/` — phased roadmap and surface sketches
- `crates/srcthread` — lib facade + store
- `crates/srcthread-daemon` — Unix-socket daemon (`srcthreadd`)
- `crates/srcthread-cli` — thin CLI over the socket (mise-shaped `src/cli/`)
- `crates/srcthread-mcp` — MCP tools for agents
- `apps/desktop` — rune desktop app
- `apps/android` — Kotlin + Gradle UI; UniFFI to Rust later
- `nvim/` — Neovim plugin (picker / open-at-line)

## Commands

- `mise run check`
- `mise run lint`
- `mise run lint-all`
- `mise run check-ci` — what GitHub Actions runs

CI covers the Rust workspace only (skip android/desktop GUI matrix).

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
