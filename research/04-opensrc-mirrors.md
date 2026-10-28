# opensrc: shallow clone cache for agents

Notes on [vercel-labs/opensrc](https://github.com/vercel-labs/opensrc) /
[opensrc.sh](https://opensrc.sh) as a related (but narrower) mirror cache.

## What opensrc does

- Gives coding agents a **local path to package/repo sources**.
- First use fetches (shallow clone / cache); later calls return the cached
  path instantly (`opensrc path …` → stdout path for `rg`/`cat`/etc.).
- Registries and GitHub-style refs; progress on stderr so command
  substitution stays clean.

Cites:

- <https://opensrc.sh>
- <https://github.com/vercel-labs/opensrc>

## Implications for srcthread

- **Shared mirror cache** is the same *kind* of idea: local clones agents and
  UIs can jump into without re-fetching.
- opensrc is **mirrors-only**. srcthread also needs **research threads**,
  anchors, excerpts, refs, and notes pinned into those mirrors.
- Do not conflate “have the source nearby” with “remember the investigation.”
