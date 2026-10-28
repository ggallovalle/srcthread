# mise CLI structure

How [jdx/mise](https://github.com/jdx/mise) lays out its CLI — the pattern
srcthread should follow (and already shares a parser with: **usage-rs**).

Refs: [`src/cli/`][gh-cli], [architecture][arch],
[usage-rs migration PR][pr-usage].

## Layout

```text
src/cli/
  mod.rs              # root Cli + Commands enum + global flags
  install.rs          # leaf command (one file)
  doctor/             # command group
    mod.rs
    …
  cache/
    mod.rs            # optional nested subcommand
    clear.rs
    path.rs
    …
  sync/
    mod.rs
    node.rs
    python.rs
    …
```

Rules of thumb:

1. **One module per top-level command** — not a single `commands.rs`.
2. **Nested verbs** → directory: `mod.rs` holds the group +
   `Subcommands` enum; each verb is its own file.
3. **`mod.rs` only wires** — `Commands` lists every top-level variant;
   dispatch is `usage_rs::Run` / async `run()`.
4. **Short visible aliases** where muscle memory matters
   (`install` → `i`, `bootstrap` → `bs`, `doctor` → `dr`).
5. **Optional nested command** when a bare group is useful
   (`mise cache` with no subcommand prints the cache path).
6. **Rich `///` docs** + `verbatim_doc_comment` so `--help` teaches.
7. **Global flags on the root `Cli`** (`-C`, `-q`, `-v`, `-y`, …),
   not redeclared on every leaf.

## Parser

mise moved from clap to **usage-rs** ([PR #12221][pr-usage]). Patterns:

```rust
#[derive(usage_rs::Cli)]
pub struct Cli {
    #[usage(subcommand)]
    pub command: Option<Commands>,
    #[usage(short='C', long, global=true, …)]
    pub cd: Option<PathBuf>,
    #[usage(short='q', long, global=true, …)]
    pub quiet: bool,
    // …
}

#[derive(usage_rs::Subcommands)]
pub enum Commands {
    #[usage(visible_alias = "i")]
    Install(install::Install),
    Cache(cache::Cache),
    // …
}
```

Leaf:

```rust
#[derive(Debug, usage_rs::Args)]
#[usage(visible_alias = "i", verbatim_doc_comment)]
pub struct Install { /* fields */ }
```

Group with optional child (`cache`):

```rust
#[derive(usage_rs::Args)]
pub struct Cache {
    #[usage(subcommand)]
    command: Option<Commands>,
}
```

## UX lessons for srcthread

| mise | srcthread analogue |
| --- | --- |
| Flat top-level verbs users type daily | Prefer `srcthread ls` / `new` over deep nests when common |
| Groups for resource areas (`cache`, `sync`) | `store`, `sync`, `mirror` as groups |
| Aliases | `thread list` → `ls`, `doctor` → `dr` |
| Global `-C` / quiet / yes | Global `--store` / `-C`, `-q`, `-v`, `-y` |
| Bare group does something sensible | `srcthread store` → print active store path |
| usage-rs | Already our stack (not clap) |

## Proposed srcthread tree

```text
crates/srcthread-cli/src/cli/
  mod.rs                 # Cli + Commands + globals
  hello.rs
  doctor.rs
  store/{mod,init,list,use_cmd,path}.rs
  thread/{mod,list,show,new,edit,rm}.rs
  anchor/{mod,add,list,show,open}.rs
  excerpt/{mod,add,show}.rs
  r#ref/{mod,add,list}.rs
  mirror/{mod,add,list,path,fetch}.rs
  search/{mod,threads,anchors}.rs
  sync/
    mod.rs
    git/{mod,push,pull,status}.rs
    peer/{mod,discover,send,receive}.rs
```

Keep implementing leaves as stubs until the lib catches up; the **shape**
of the tree matters more than filling every body.

[gh-cli]: https://github.com/jdx/mise/tree/main/src/cli
[arch]: https://mise.jdx.dev/architecture.html
[pr-usage]: https://github.com/jdx/mise/pull/12221
