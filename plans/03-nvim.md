# Plan: Neovim plugin

fff-inspired UX: fast pickers over a warm daemon, open files at the pinned
line without re-indexing the world on every keystroke.

## Goals

- Pick a **thread**, then an **anchor** (or search across open store).
- **Open at line** — `edit` + cursor (already stubbed as `open_anchor`).
- Talk to `srcthread daemon` over the socket (fallback: in-process later if needed).

## Commands (planned)

- `:SrcthreadHello` — smoke notify
- `:SrcthreadThreads` — picker of threads
- `:SrcthreadAnchors` — anchors in current / selected thread
- `:SrcthreadOpen` — resolve selection to path@line

## Non-goals early

- Reimplement fff's full content index inside Lua
- Shipping a separate binary from the plugin tree
