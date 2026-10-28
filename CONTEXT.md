# srcthread — product context

Short glossary so humans and agents share the same nouns. Schemas (exact TOML
fields, id formats) are **not** locked yet; this is meaning and relationships.
Settled process/store shape: see [PLAN.md](PLAN.md) and
[research/09-no-daemon.md](research/09-no-daemon.md).

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
| **Notes** | Your prose for the investigation | `notes.md` (or similar) in the thread dir |
| **Anchor** | A **pointer** into mirrored source: repo @ commit + path + optional line/span/symbol | small typed file (e.g. toml) |
| **Excerpt** | **Frozen text** copied around an anchor so you still have the body offline if the mirror moves or network is gone | markdown/text body (+ optional meta) |
| **Ref** | Something **outside** the mirror: PR, issue, doc URL, chat/session link, or a pasted transcript | small typed file; optional pasted body |

Mental model:

- **Anchor** = “open this place in the editor / show agents this location.”
- **Excerpt** = “keep this slice of source even without the repo.”
- **Ref** = “remember this external thing (and maybe a paste of it).”
- **Notes** = “what I think / how the pieces connect.”

## Mirrors (not owned by a thread)

**Shared upstream clones** on the machine. Threads **pin** revision + path; they
do **not** each get a private full clone.

- Live under **`$XDG_CACHE_HOME/srcthread/mirrors/…`** (regenerable cache).
- Override path via **environment variable** (not a store config field) —
  exact name TBD (`SRCTHREAD_MIRRORS_DIR` is the likely shape).
- Never commit mirrors into a store (avoids git submodule pain).

Related idea: [opensrc](research/04-opensrc-mirrors.md) is mirrors-only;
srcthread adds threads on top.

## Store

A **directory** of research files, typically git-tracked for persistence and
sync. There can be **many** stores (personal vault, per-project research, …).

Intent (layout still being grilled):

- An **owned folder** that can sit **inside a normal project repo**, e.g.
  `srcthread/config.toml` as marker + `srcthread/threads/…`, mise-shaped so
  the tool can live among `Cargo.toml` / source without owning the whole tree.
- Or a **standalone vault** whose job is only research.
- Discovery: walk parents for the marker; `-C` / `--store`; user-level registry
  under XDG config later.

**Files are canonical.** Any search index is derived (cold walk first; optional
in-process warm index in long-lived hosts later — not a PC daemon).

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

## IDs (direction, schema open)

Human **slug** + short **id-looking suffix** (e.g. `deno-env-get-01HK…`) —
readable paths, collision-safe. Exact alphabet / ULID-vs-other and per-item
file schemas are the next design frontier after this context doc.

## What this is not

- Not a general notes app (Obsidian/Notion stay for messy journals).
- Not a clone manager (ghq stays the clone habit if you want it).
- Not “have the sources nearby” alone — that is only the **mirror** half;
  the product is remembering the **investigation**.
