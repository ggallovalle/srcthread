# Plan: Packaging and releases

Ship the PC binary via **packslip** (variants, completions, man) and cut
releases with a **manual** publish script that also attaches the Android
APK. No mise tool plugin. Decision note:
[research/09-mise-packaging-plugin.md](../research/09-mise-packaging-plugin.md).

## Product side effects (CLI)

| Command | Role |
| --- | --- |
| `srcthread integration install\|uninstall\|status --user\|--local` | Register `srcthread mcp` in agent configs |
| `srcthread desktop install-xdg [--dev]` | FreeDesktop launcher + icon |

Mise consumers can call these from a tool `postinstall` string.

## Scripts

| Script | Role |
| --- | --- |
| `scripts/release-assets.sh` | usage KDL + completions + `man/man1/srcthread.1` |
| `scripts/build-tarball.sh [triple] [default\|desktop]` | Platform archive under `dist/` |
| `scripts/checksums.sh` | `dist/SHASUMS256.txt` |
| `scripts/publish-release.sh <version>` | Tag, build PC + APK, `gh release`, packslip |

Useful flags: `--dry-run`, `--host-only`, `--skip-android`,
`--skip-desktop`. Set `PACKSLIP_KEY` for local packslip signing.

Archive layout:

```text
srcthread/
  bin/srcthread
  man/man1/srcthread.1
  README.md
  LICENSE
```

Variants: omit suffix = `--no-default-features`; `-desktop` = Cargo
`desktop` feature. Completions and usage KDL are **separate** release
assets (not inside the tarball). Man is inside the archive for packslip
`man@linux` / `man@darwin`.

## CI

`.github/workflows/release.yml` mirrors the publish path but stays
**deactivated** (`workflow_dispatch` only). Do not enable tag-push
triggers until the manual script is retired.

## Consumer install

```toml
[tools]
"packslip:github.com/ggallovalle/srcthread" = {
  version = "0.1.0",
  variant = "desktop",
}
```

Android: download `srcthread-*-android.apk` from the GitHub release.
