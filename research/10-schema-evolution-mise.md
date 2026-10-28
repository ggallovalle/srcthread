# Schema evolution the mise way

How [jdx/mise](https://github.com/jdx/mise) evolves `mise.toml` without a
config `schema_version`, and what srcthread should copy for
`srcthread/config.toml`, anchors, and store lint.

Primary refs in the mise tree:

- Docs: `docs/configuration.md` (`min_version`, `[_]`)
- Unknown keys: `src/config/config_file/unknown_key.rs` + `serde_ignored`
- Declarative schema: `crates/mise-config` (`MiseTomlDecl` → `schema/mise.json`)
- Deprecations: `deprecated!` / `deprecated_at!`, settings rename aliases
- Migrations: `src/migrate.rs` (on-disk **state paths**, not toml shape)

## The short answer

mise does **not** version the config file format with an integer
`schema_version`. The **running binary** is the schema. Old files keep
loading; new keys appear over time; unknown keys usually **warn and skip**;
breaking needs are gated with **`min_version`** (required *tool* semver), not
a bump inside every vault.

srcthread should follow that for store/thread files: **additive serde types,
lenient load, strict lint, optional later `min_version`.**

## What mise actually does

### 1. Additive fields in Rust

New capabilities = new optional fields on the config structs the current
release understands. Old files without those fields deserialize fine (defaults
/ `Option`). There is no “migrate every mise.toml to vN” rewriter for ordinary
evolution.

### 2. Unknown keys: warn, don’t die (usually)

Top-level `mise.toml` parse uses **`serde_ignored`**: keys the struct does not
know are **skipped** and surface a warning with path + line/column and a
fuzzy “did you mean …?” from the embedded JSON schema
(`unknown_key.rs`).

Stricter pockets exist: **`[settings]`** uses `deny_unknown_fields` (unknown
setting → hard fail). That split matters — “project config” stays
forward-compatible; “engine knobs” stay strict.

### 3. Deprecations with a time window

Old spellings keep deserializing for a while:

- Schema marks `deprecated`
- Runtime `deprecated!` / `deprecated_at!` warn based on the **running mise
  version**
- Settings get **rename aliases** at merge (old key → new path; new wins if
  both set)
- Some legacy fields remain on structs and map into the modern name

Removal is scheduled (`deprecated_warn_at` / `deprecated_remove_at` style for
settings), not “schema_version 3 deletes the key tomorrow.”

### 4. `min_version` — gate the **binary**, not a schema integer

From `docs/configuration.md`:

```toml
min_version = "2026.1.0"
# or
min_version = { hard = "2025.6.0", soft = "2026.1.0" }
```

Meaning: “this file needs at least this **mise release**.” Hard → error +
upgrade hint; soft → warn. Still enforced when the rest of the file fails to
parse (peek the raw TOML). That is the escape hatch when a new construct is
not safely ignorable.

### 5. Escape hatch `[_]`

```toml
[_]
my_sidecar = true
```

mise **never reads** `[_]`. Users/tools can stash private data without
tripping unknown-key policy. Useful when another tool cohabits the file.

### 6. Generated JSON schema (editors / “did you mean”)

`MiseTomlDecl` drives both parsing and `schema/mise.json` (render task).
LSP/editors get completions and deprecation text; runtime warnings reuse the
same known-key set. Parser and schema are kept honest with a conformance
corpus (with documented gaps for intentional forward-compat looseness).

**This is not a `schema_version` field.** The JSON Schema is an artifact of
the **current binary/decls** (published / shipped for editors and
`schemastore`). Files on disk stay unversioned integers; the schema URL in
an editor settings file pins “validate against this release’s shape.”

### 7. What `migrate.rs` is *not*

`src/migrate.rs` moves **machine state directories** (trusted configs, install
markers), not “rewrite all mise.toml to schema v2.” Config shape change ≠
data-dir migration.

## Anti-pattern: integer `schema_version` in every vault

| Approach | Cost |
| --- | --- |
| `schema_version = N` in each store | Every consumer must branch; old tools hard-fail or need N migration scripts; git noise on bump |
| mise-style (binary = schema) | Old tools ignore new keys (warn); new tools read old files; breakages opt-in via `min_version` |

A schema integer is useful for **opaque binary blobs** or lockfiles. For
**human-edited, mergeable TOML/Markdown vaults**, it fights the product:
personal + project stores will sit unmaintained; a bump shouldn’t brick them.

## Implications for srcthread

### Store `config.toml`

Settled direction (grilling): Cargo/mise-shaped single file —

```toml
[store]
name = "…"
description = "…"
authors = ["…"]
license = "MIT"
tags = ["…"]

[mirrors]
deno = { git = "https://github.com/denoland/deno" }
```

Evolution tips copied from mise:

1. **No `schema_version`.** Optional later: `min_version = "0.2.0"` meaning
   “need srcthread ≥ 0.2” if we introduce a non-ignorable construct.
2. **`[store]` / `[mirrors]`**: unknown keys **warn + skip** on everyday
   commands (lenient).
3. **`srcthread lint`**: same parse, but unknown/deprecated/missing-required
   become **diagnostics** (and `--fix` only for safe autofixes).
4. Optional **`[_]`** (or `[store.custom]`) for user sidecar data we never
   interpret.
5. Emit warnings through **`tracing`**; users raise noise with
   `SRCTHREAD_LOG=` (and a subscriber in the CLI). Don’t fail `thread list`
   because someone typo’d a mirror key.

### Anchors / excerpts / threads

Same policy at a smaller grain:

- One serde type for anchor meta (whole `.toml` or `+++` TOML frontmatter).
- New optional fields appear over time; unknown frontmatter keys warn.
- Annotated fence info-strings: unknown attrs warn; body still loads.
- `lint` walks the store cold and reports; normal open/list stays best-effort.

### What *would* justify `min_version`

Only when ignoring a new construct would **silently corrupt meaning** (e.g.
a new mirror source kind that old binaries would treat as missing and
re-fetch wrong). Prefer additive enums with a clear “unknown variant → warn
and skip that pin” before reaching for `min_version`.

### What *would* justify a real migration command

Rare, explicit, user-driven — e.g. `srcthread lint --fix` renames a
deprecated field, or a one-shot `srcthread migrate` for on-disk layout moves
(like mise’s state-dir migrate). Not an automatic bump stamped into every
commit.

## Recommended srcthread checklist

| Mechanism | v1? | Notes |
| --- | --- | --- |
| Additive optional fields | yes | Default path |
| `serde_ignored` (or equivalent) + tracing warn | yes | Commands lenient |
| `srcthread lint [--fix]` | yes | Strict surface |
| Deprecation aliases | when needed | Keep old key → map to new |
| `min_version` | later | Gate binary, not schema int |
| `[_]` escape hatch | nice | Forward-compat for humans/tools |
| **JSON Schema** (schemars from Rust types, day one) | **yes** | store + anchor + thread + ref (…); see below |
| Integer `schema_version` in vault files | **no** | Avoid for vault files |

## JSON Schema for srcthread (wanted — day one, not deferred)

Schemas are cheap: derive from the same Rust types `serde` uses (e.g.
[`schemars`](https://crates.io/crates/schemars), already pulled in via
`rmcp`). Do **not** wait for a later phase.

Ship multiple small schemas alongside the types they describe — mise’s
“one big mise.toml schema” pattern, applied per typed surface:

| Artifact (example) | Covers |
| --- | --- |
| `schema/srcthread.store.json` | `srcthread/config.toml` (`[store]`, `[mirrors]`, …) |
| `schema/srcthread.anchor.json` | `anchors/*.toml` and `+++` TOML frontmatter on `*.md` |
| `schema/srcthread.thread.json` | `thread.toml` |
| `schema/srcthread.ref.json` | `refs/*.toml` |
| excerpt meta (if any) | standalone/embedded excerpt frontmatter |

Pipeline:

1. **Source of truth** — Rust structs with `Serialize` / `Deserialize` /
   `JsonSchema`.
2. **Generate** — `mise run render:schema` (or a tiny bin) writes the JSON
   files into `schema/`; keep them committed so editors and CI don’t need
   a build to validate.
3. **Consumers** — editor path globs; `srcthread lint` known-keys / “did
   you mean”; optional embed of the same JSON for offline lint.
4. **Evolution** — additive optional properties in the next derive; runtime
   still warn/skip unknown keys. **No** integer `schema_version` inside
   vault files; editors pin a schema path/URL for a **srcthread release**.

Association: prefer **path globs** (`**/srcthread/config.toml` → store
schema; `**/anchors/*.toml` → anchor schema) over a nonstandard `$schema`
key inside TOML, unless we later find a clean convention.

## Tie-ins

- Product strap: [PLAN.md](../PLAN.md), glossary [CONTEXT.md](../CONTEXT.md)
- Lenient commands + lint: grilling Q26
- Store layout / mirrors in config: grilling Q25′
- Index cold walk (no `rg`): [01-fff-index.md](01-fff-index.md),
  [09-no-daemon.md](09-no-daemon.md)
