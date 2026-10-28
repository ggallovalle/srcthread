# Why srcthread has no PC daemon (v1)

Earlier sketches copied herdr’s `daemon` + NDJSON Unix socket as the PC spine.
That was the wrong analogy for this product. This note records why v1 drops
the daemon crate and treats the **lib as the only engine**.

## What we compared

| Product | Long-lived process? | Why |
| --- | --- | --- |
| **herdr** | Required server | Owns live PTYs, agents, and layout that must survive client disconnect. The server *is* the product. |
| **fff** | Host process only | Warm path/content index lives inside nvim or the MCP stdio child. No shared index daemon. |
| **mise** | No core daemon | One-shot CLI loads config each run; heavy work lands in XDG cache/data. Optional tool sidecars are unrelated. |

See also [02-herdr-socket.md](02-herdr-socket.md) (pattern we studied),
[01-fff-index.md](01-fff-index.md), [07-mise-cli-structure.md](07-mise-cli-structure.md),
[08-herdr-single-binary.md](08-herdr-single-binary.md).

## What srcthread actually owns

- **Source of truth** — files in a git-tracked **store** (threads, anchors,
  excerpts, refs, notes). Mergeable on disk; not a live session graph.
- **Mirrors** — regenerable upstream clones under
  `$XDG_CACHE_HOME/srcthread/mirrors/` (env override later). Not in the store.
- **Clients** — CLI (short-lived), nvim, desktop, MCP stdio, Android (UniFFI).
  Desktop / nvim / MCP / Android are already long-lived hosts that can embed
  the lib (fff-shaped).

There is no herdr-like “must keep OS processes alive across disconnect”
requirement for the core research-thread flow.

## When a daemon *would* earn its keep

Revisit only if product needs show up in use:

1. **Shared warm index** across short-lived CLI invocations *and* across
   nvim ↔ CLI ↔ MCP without each host warming alone.
2. **Background FS watchers** independent of any UI/MCP process.
3. **Always-on peer receive** (Localsend-like listener).
4. **Mirror fetch locking** so concurrent clients do not race clones.

Until then, an in-process **cold walk** over the store is fine (no `rg` —
keeps Android viable). Long-lived hosts may keep an optional **in-process**
RAM index later (fff), not a central socket server.

## v1 process model

- **`srcthread` lib** — sole domain logic and (eventually) file store I/O.
- **Every surface embeds the lib** (CLI opens per invocation; desktop/nvim/MCP
  hold it for process lifetime; Android via UniFFI).
- **No** `srcthread-daemon` crate, **no** `srcthread daemon` subcommand, **no**
  required NDJSON socket.
- Single PC binary remains (CLI + `desktop` + `mcp` libraries as subcommands).

Build order becomes: **lib → CLI → nvim → desktop → Android → MCP** (daemon
removed from the critical path).
