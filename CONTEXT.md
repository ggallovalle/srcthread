# srcthread — product context

Short glossary so humans and agents share the same nouns. On-disk format and
field lists are settled (see [PLAN.md](PLAN.md),
[research/09-no-daemon.md](research/09-no-daemon.md),
[research/10-schema-evolution-mise.md](research/10-schema-evolution-mise.md)).
Dogfood exemplar: in-repo store at [`srcthread/`](srcthread/) (Deno.env.get
thread).

## Problem in one line

Capture a **research investigation** (notes + pinned places in source + offline
snippets + external links) so you and coding agents can reopen it later without
re-hunting GitHub or Notion — including offline, on PC and Android.

## Primary object

### Research thread

A **named investigation** you reopen later. Not “a note” and not “a clone.”

Example: “how is `Deno.env.get` implemented?” — one thread that gathers the
Rust op, the JS wrapper, interesting PRs, and your prose.

Everything else hangs off a thread.

## What hangs off a thread

| Thing | Meaning | Rough on-disk idea |
| --- | --- | --- |
| **Notes** | Your prose for the investigation | `notes.md` in the thread dir |
| **Anchor** | Pin: `mirror` + `sha` + `path`; optional `line` / `end_line` / `symbol` / `title` | `anchors/<id>.toml` **or** `.md` with same TOML in `+++` frontmatter; body may embed excerpt fences (info-string: lang) |
| **Excerpt** | Frozen text around an anchor | Standalone `excerpts/<id>.md` (frontmatter: id, `anchor_id`, title?) and/or fences in an anchor `.md`. Shared parser |
| **Ref** | Outside the mirror: PR / issue / doc / chat / other | `refs/<id>.toml` — kind, url?, title?, body_path? |

Mental model:

- **Anchor** = “open this place in the editor / show agents this location.”
- **Excerpt** = “keep this slice of source even without the repo.” Same
  markdown shape whether embedded in an anchor file or under `excerpts/`.
- **Ref** = “remember this external thing (and maybe a paste of it).”
- **Notes** = “what I think / how the pieces connect.”

## Mirrors (not owned by a thread)

**Shared upstream clones** on the machine. Threads **pin** revision + path; they
do **not** each get a private full clone.

- Live under **`$XDG_CACHE_HOME/srcthread/mirrors/…`** (regenerable cache).
- Override with **`SRCTHREAD_MIRRORS_DIR`** (env only; not a store config
  field).
- Never commit mirrors into a store (avoids git submodule pain).

Related idea: [opensrc](research/04-opensrc-mirrors.md) is mirrors-only;
srcthread adds threads on top.

## Store

A **directory** of research files, typically git-tracked for persistence and
sync. There can be **many** stores (personal vault, per-project research, …).

Settled layout:

```text
srcthread/
  config.toml                 # marker; [store] meta + [mirrors] id→git
  threads/<slug>-<shortid>/
    thread.toml
    notes.md
    anchors/                  # *.toml and/or *.md (TOML frontmatter)
    excerpts/                 # standalone *.md (optional if embedded)
    refs/
```

Example `config.toml` shape (grilling):

```toml
[store]
name = "my-project research"
description = "…"
authors = ["…"]
license = "MIT"
tags = ["deno", "perms"]
# edition / min_version optional later

[mirrors]
# repo identity only — pinned sha/path live on anchors
deno = { git = "https://github.com/denoland/deno" }
```

Nestable in a normal project. Discovery: `--store` → walk from `-C`/cwd for
`srcthread/config.toml` → `$XDG_CONFIG_HOME/srcthread/stores.toml` registry.
Revitalize: resolve `[mirrors]` ids + scan anchor pins → fetch into XDG /
`SRCTHREAD_MIRRORS_DIR`.

**thread.toml:** id, title, tags, created, updated — prose in `notes.md`.
**refs:** kind + url?/title?/body_path?. **excerpts:** id + anchor_id +
optional title; fenced body.

Config evolution is mise-shaped (**no** integer `schema_version` in the
vault). **JSON Schemas are day one**: schemars-derived from the Rust types
for store config, anchors, threads, refs (editors, lint, “did you mean”) —
[research/10-schema-evolution-mise.md](research/10-schema-evolution-mise.md).

**Files are canonical.** Search/list is an in-process **cold walk** of the
store (no shelling out to `rg`/`fd` — that would pull an external binary and
break Android). Optional in-process warm index in long-lived hosts later
(fff-shaped) — not a PC daemon.

## Sync (high level)

- **Git** when devices are not nearby (store = mergeable vault).
- **Localsend-like** explicit peer send/receive when nearby.
- v1 conflicts: git merge + pick at thread grain (no Automerge day one).

## Surfaces (all call the same lib)

| Surface | Role |
| --- | --- |
| **CLI** `srcthread` | Daily driver; sole PC binary; also enters `desktop` / `mcp` |
| **nvim** | Pick thread/anchor, open at line |
| **Desktop** | Rune GPU UI (`srcthread desktop`) |
| **Android** | Full client via UniFFI + Compose (capture + read offline) |
| **MCP** | Agents read anchors/excerpts without web fetch (`srcthread mcp`) |

No required PC daemon: every surface embeds [`srcthread`](crates/srcthread)
in-process.

## IDs

Human **slug** + **10-char Crockford ULID** suffix (time-sortable, e.g.
`deno-env-get-01HK3K6N8R`) — readable paths, collision-safe.

## What this is not

- Not a general notes app (Obsidian/Notion stay for messy journals).
- Not a clone manager (ghq stays the clone habit if you want it).
- Not “have the sources nearby” alone — that is only the **mirror** half;
  the product is remembering the **investigation**.
