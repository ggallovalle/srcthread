# Releasing srcthread

Releases are cut with `mise run release:publish` on a clean `main`
checkout. That task builds/uploads assets, then dispatches
[`.github/workflows/packslip-sign.yml`](.github/workflows/packslip-sign.yml)
for keyless packslip (no `pubkey` for mise). The full
[`.github/workflows/release.yml`](.github/workflows/release.yml) matrix
stays deactivated.

Design notes: [plans/06-packaging.md](plans/06-packaging.md),
[research/09-mise-packaging-plugin.md](research/09-mise-packaging-plugin.md).

## Prerequisites

- Clean git tree on `main`, push access, `gh` authenticated
- Version in `crates/srcthread-cli/Cargo.toml` (and sibling crates) already
  bumped to the version you will publish — the script refuses a mismatch
- Optional:
  - `ANDROID_HOME` / `ANDROID_SDK_ROOT` for the release APK
  - Extra `rustup` targets if you want more than the host triple

## Bump the version

Rust crates share `[workspace.package] version`; Android `versionName` /
`versionCode` stay in lockstep via:

```sh
mise run release:bump 0.1.1          # or: patch | minor | major
```

Write GitHub release notes at `release-notes/<version>.md` (required;
publish refuses to run without a non-empty file). Start from
[release-notes/TEMPLATE.md](release-notes/TEMPLATE.md).

Commit on `main` before publishing. Confirm what publish will require:

```sh
cargo metadata --no-deps --format-version 1 | python3 -c '
import json, sys
d = json.load(sys.stdin)
print(next(p["version"] for p in d["packages"] if p["name"] == "srcthread-cli"))
'
```

## Dry-run (recommended)

Builds assets under `dist/` only — no tag, no `gh release`, no packslip.

```sh
git checkout main && git pull
mise run release:publish 0.1.0 --dry-run --host-only
ls -lh dist/
```

Useful flags:

| Flag | Effect |
| --- | --- |
| `--dry-run` | Build artifacts only |
| `--host-only` | Only the current rustc host triple |
| `--skip-desktop` | Skip the `desktop` Cargo-feature tarball |
| `--skip-android` | Skip the APK (also skipped if no Android SDK) |

## Publish

```sh
mise run release:publish 0.1.0 --host-only   # or omit --host-only
```

The task will:

1. Regenerate usage KDL, shell completions, and man
2. Build platform tarballs (`default` and `desktop` when possible)
3. Build `srcthread-<ver>-android.apk` when the SDK is present
4. Write `dist/SHASUMS256.txt`
5. Create annotated tag `v<ver>`, push it, and `gh release create`
6. Dispatch `packslip-sign.yml` for that tag and wait until it is green

Cross-OS triples are skipped when this host cannot build them (e.g. no
Darwin archives from Linux). For a full matrix, run publish once per OS
and attach missing assets with `gh release upload`, then re-run
packslip-sign (or `gh workflow run packslip-sign.yml -f tag=…`).

## What ships

**Tarball** (`srcthread-<ver>-<triple>.tar.gz`, plus `-desktop` variant):

```text
srcthread/
  bin/srcthread
  man/man1/srcthread.1
  README.md
  LICENSE
```

- Omit suffix / `default` → `--no-default-features` (cli + mcp, no rune)
- `-desktop` → Cargo `desktop` feature

**Separate release assets** (not inside the archive): usage KDL, bash /
zsh / fish / powershell completions, checksums, optional APK and
packslip sigstore.

Layout template: [packaging/release.toml](packaging/release.toml).

## After the release — install

PC via packslip:

```toml
[tools]
"packslip:github.com/ggallovalle/srcthread" = {
  version = "0.1.0",
  variant = "desktop",
}
# postinstall = """
#   srcthread integration install cursor --user && \
#   srcthread desktop install-xdg
# """
```

Omit `variant` for the cli-only build. Android: download
`srcthread-<ver>-android.apk` from the GitHub release.

## Lower-level tasks

Prefer `release:publish`. For debugging a single step:

| Task | Role |
| --- | --- |
| `mise run render:usage [--dist]` | usage KDL + completions + man |
| `mise run release:tarball [triple] [default\|desktop]` | One archive under `dist/` |
| `mise run release:checksums` | `dist/SHASUMS256.txt` |

## Checklist

1. Version bumped and on `main`
2. `release-notes/<version>.md` written
3. `mise run check-ci` (or rely on green CI on `main`)
4. Dry-run with `--host-only`
5. Real publish (`mise run release:publish` — waits on packslip-sign)
6. Spot-check mise install (no `pubkey`) and the APK
