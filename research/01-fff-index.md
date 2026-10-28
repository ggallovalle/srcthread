# fff: long-lived in-process index

Notes on how [dmtrKovalenko/fff](https://github.com/dmtrKovalenko/fff) keeps
search fast without shelling out to `rg`/`fzf` on every query.

## What fff does

FFF is a file-search library (Neovim, MCP, Rust/C/Node SDKs). A long-lived
process owns warm state:

- **Path index in RAM** — scanned once, then queried in-process.
- **Content bigram prefilter** — inverted bigram bitsets narrow grep
  candidates; not a full-text IR engine.
- **Background `notify` watchers** — incremental add/modify/remove; overflow
  or event loss triggers a full rescan, not constant walks.
- **LMDB for frecency / history only** — ranking scores, not the path or
  content catalog.
- Explicit contrast with Tantivy-class engines: FFF **does not persist an
  inverted index on disk** (rebuild/warm in the long-lived process).

Primary cite: <https://github.com/dmtrKovalenko/fff>

Useful internals (same repo): `file_picker.rs`, `scan.rs`,
`watcher/background_watcher.rs`, `index/bigram_filter.rs`, `dbs/frecency.rs`.

## Implications for srcthread

1. **Files stay canonical** — the git store is source of truth; any index is
   derived.
2. **Optional RAM index in long-lived hosts** — desktop / nvim / MCP embed the
   lib (fff-shaped); **no** shared PC index daemon for v1
   ([09-no-daemon.md](09-no-daemon.md)). Not a committed SQLite catalog.
3. **Path index first** — threads/anchors useful before content warm-up.
4. **Event-driven updates** — store mutations (and FS watchers on store dirs)
   feed the host’s index; rare rebuild beats periodic full walks.
5. **Cold walk / `rg`** is fine for short-lived CLI until a host index exists.
