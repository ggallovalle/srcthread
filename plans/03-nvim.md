# Plan: Neovim plugin

fff-inspired UX: fast pickers over store contents, open files at the pinned
anchor location.

## Hello-world

- `:SrcthreadHello` / Threads / ThreadNew via thin Lua over the `srcthread`
  CLI (in-process lib under the hood).

## Later

- Pick thread / pick anchor; open mirror path at line.
- Optional in-process warm index later (fff-shaped inside a long-lived host);
  not a PC daemon socket.
