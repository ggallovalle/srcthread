# mise packaging: packslip + `srcthread integration`

Ship the binary the way mise does (packslip for variants, completions,
man). Wire coding agents the way herdr does (`integration
install|uninstall|status` in the product CLI). Avoid a fat mise tool
plugin that edits user configs from Lua.

## Verdict

| Concern | Owner |
| --- | --- |
| Binary download, checksum, lockfile | **packslip** |
| Desktop vs cli build matrix | packslip **`variant`** (`desktop` vs default) |
| Completions + usage KDL + man | packslip **resources** (not inside the zip for completions) |
| Agent MCP / hooks registration | **`srcthread integration`** (Rust, herdr-shaped) |
| FreeDesktop `.desktop` / icons | **`srcthread desktop …`** helper (or `integration`-adjacent), not Lua |
| Optional mise sugar | `postinstall` calling those helpers, or a thin plugin later |

A custom `packaging/mise-srcthread/` plugin is **optional**. Prefer
`packslip:github.com/OWNER/srcthread` once releases are signed. Keep the
subdir in-repo only if we need non-packslip install or postinstall glue
before packslip is ready.

## Packslip path (completions, man, variants)

mise [v2026.10.21](https://github.com/jdx/mise/releases/tag/v2026.10.21)
shows the shape:

- Platform archive holds `bin/mise`, man, README/LICENSE — **not** shell
  completions.
- Completions + `mise.usage.kdl` are **separate release assets**.
- `packslip.sigstore.json` lists artifacts (per OS/arch/libc/format) and
  resources: man from archive, completions/cli-spec from assets.
- Users install with `mise use packslip:github.com/jdx/mise`. Completions
  follow the active version under `mise activate`, or
  `mise completion zsh --tool mise --install`.
- Alternate builds use the [`variant`][packslip-variant] tool option.

[packslip-variant]: https://mise.jdx.dev/dev-tools/backends/packslip.html#variant

For us:

```toml
[tools]
"packslip:github.com/OWNER/srcthread" = {
  version = "0.1.0",
  variant = "desktop",
}
# omit variant → default (cli + mcp, no rune)
```

Today only **`desktop`** is a Cargo feature; MCP is always linked. Do not
advertise `features = ["mcp"]` as a compile switch. Agent wiring is
orthogonal (below).

Scaled-down release assets (see also the inventory in the prior section
of this note / mise tag above):

```text
srcthread-0.1.0-linux-x64.tar.gz            # default
srcthread-0.1.0-linux-x64-desktop.tar.gz    # variant=desktop
… arm64 / macos …
srcthread.usage.kdl
srcthread.bash / .zsh / .fish / .ps1
SHASUMS256.txt
packslip.sigstore.json
```

Archive root: `srcthread/bin/srcthread` (+ optional `man/`, LICENSE,
README). One compression for v1 is enough.

Release scripts: mise’s `scripts/build-tarball.sh` + `scripts/release.sh`
shape — gather artifacts, checksums, upload usage/completions, then
packslip.

## Agent integrations: herdr CLI shape, our payload

herdr:

```text
herdr integration install|uninstall|status
herdr integration install <TARGET>   # pi, claude, cursor, opencode, …
```

What herdr installs is **agent-state hooks** (shell/TS/JS plugins under
each agent’s config dir), not MCP servers — no `mcpServers` edits in
`herdr/src/integration`. The valuable pattern is the **surface**:

- Subcommands on the **product binary** (tested Rust, not a mise plugin).
- Per-target install that is **idempotent** and reversible.
- `status` that reports path + installed / missing / outdated.
- Embedded assets + version stamps; refuse or warn if the agent isn’t
  present; preserve unrelated user config.
- Enum of targets with experimental extras kept separate.

srcthread’s payload is different: register **`srcthread mcp`** (stdio) in
each agent’s MCP config (and later skills / hooks if we want them). Same
CLI:

```text
srcthread integration install cursor
srcthread integration install claude
srcthread integration uninstall opencode
srcthread integration status
```

Suggested first targets (match earlier sketch): `cursor`, `claude`,
`opencode`, `codex`. Grow the set like herdr when paths are known.

Implementation sketch (mirrors herdr layout, smaller):

```text
crates/srcthread-cli/src/cli/integration.rs   # usage-rs command
crates/srcthread/src/integration/             # or under cli crate
  mod.rs / targets.rs / mcp_config.rs / status.rs
```

Idempotent JSON/TOML merge under a namespaced key (e.g. server id
`srcthread`); uninstall removes only our entry. Point `command` at the
`srcthread` on `PATH` (or absolute path from `std::env::current_exe`
when invoked as a postinstall).

### How mise users trigger it

Packslip does not edit agent configs. Options:

1. **Manual** (always): `srcthread integration install cursor`.
2. **Tool `postinstall`** in `mise.toml`:

   ```toml
   [tools]
   "packslip:github.com/OWNER/srcthread" = {
     version = "0.1.0",
     variant = "desktop",
     postinstall = """
       srcthread integration install cursor && \
       srcthread integration install claude
     """,
   }
   ```

3. Later: a thin plugin whose only job is to read an `agents = […]`
   option and call the same binary — **not** reimplement merges in Lua.

Prefer (1)+(2) until the agent list is stable. mise docs discourage
user-wide config edits from plugin `PostInstall`; calling our own CLI
keeps that logic testable.

Desktop XDG files (`packaging/linux/*.desktop`) can follow the same
idea: `srcthread desktop install-xdg` / uninstall, invoked from
`postinstall` when `variant = "desktop"`, rather than a Lua XDG writer.

## Packslip skills (optional later)

packslip can also declare **skills**; consumers run
`mise skills sync --dir .agents/skills`. That is complementary to
`srcthread integration` (MCP server registration ≠ skill files). Use
skills when we have agent-facing docs to ship per release; keep MCP
wiring in `integration`.

## What not to do

- Put completions only inside the platform tarball and expect packslip
  Tab completion to work — declare them as resources/assets.
- Drive agent MCP installs from Lua with ad-hoc JSON surgery.
- Use a mise tool plugin as the primary public install path once
  packslip releases exist.
- Confuse herdr’s hook integrations with MCP: same CLI verbs, different
  files on disk.

## Open choices

1. Default vs `desktop` artifact naming in the packslip manifest
   (`variant` field on artifacts).
2. Whether `postinstall` should auto-detect installed agents (herdr
   `integration status` / recommendations style) or only install an
   explicit list.
3. Absolute vs `PATH`-relative `command` in MCP configs when mise shims
   / multiple versions are in play.
4. When to add `packaging/mise-srcthread/` at all — only if we need a
   non-packslip fallback or declarative `agents = […]` before
   `postinstall` strings feel enough.
