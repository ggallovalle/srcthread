# Releasing srcthread

Releases are **manual**. Run `scripts/publish-release.sh` on a clean
`main` checkout. The GitHub Actions workflow
(`.github/workflows/release.yml`) is checked in but deactivated
(`workflow_dispatch` only) — do not turn on tag-push until the script is
retired.

Design notes: [plans/06-packaging.md](plans/06-packaging.md),
[research/09-mise-packaging-plugin.md](research/09-mise-packaging-plugin.md).

## Prerequisites

- Clean git tree on `main`, push access, `gh` authenticated
- Version in `crates/srcthread-cli/Cargo.toml` (and sibling crates) already
  bumped to the version you will publish — the script refuses a mismatch
- Optional:
  - `PACKSLIP_KEY` — **required** for a real publish (must be
    `export`ed). Path to a packslip secret key (`packslip keygen`)
  - `packslip` on `PATH` (or mise / `PACKSLIP_BIN`)
  - `ANDROID_HOME` / `ANDROID_SDK_ROOT` for the release APK
  - Extra `rustup` targets if you want more than the host triple

## Bump the version

Rust crates share `[workspace.package] version`; Android `versionName` /
`versionCode` stay in lockstep via:

```sh
scripts/bump-version.py 0.1.1          # or: patch | minor | major
```

Commit on `main` before publishing. Confirm what publish will require:

```sh
cargo metadata --no-deps --format-version 1 | python3 -c '
import json, sys
d = json.load(sys.stdin)
print(next(p["version"] for p in d["packages"] if p["name"] == "srcthread-cli"))
'
```

## Dry-run (recommended)

Builds assets under `dist/` only — no tag, no `gh release`.

```sh
git checkout main && git pull
scripts/publish-release.sh 0.1.0 --dry-run --host-only
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
export PACKSLIP_KEY=/path/to/packslip.key   # required; assign alone is not enough

scripts/publish-release.sh 0.1.0
# first machine / smoke: add --host-only
```

The script will:

1. Regenerate usage KDL, shell completions, and man
2. Build platform tarballs (`default` and `desktop` when possible)
3. Build `srcthread-<ver>-android.apk` when the SDK is present
4. Write `dist/SHASUMS256.txt`
5. Create annotated tag `v<ver>`, push it, and `gh release create` with
   everything in `dist/`
6. Run `packslip create` and upload `packslip.sigstore.json` (fails if
   `PACKSLIP_KEY` is missing or not a file)

Cross-OS triples are skipped when this host cannot build them (e.g. no
Darwin archives from Linux). For a full matrix, run the script once per
OS (or enable the deactivated workflow later) and attach any missing
assets with `gh release upload`.

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

## Lower-level scripts

Prefer `publish-release.sh`. For debugging a single step:

| Script | Role |
| --- | --- |
| `scripts/release-assets.sh` | usage KDL + completions + man |
| `scripts/build-tarball.sh [triple] [default\|desktop]` | One archive under `dist/` |
| `scripts/checksums.sh` | `dist/SHASUMS256.txt` |

## Checklist

1. Version bumped and on `main`
2. `mise run check-ci` (or rely on green CI on `main`)
3. Dry-run with `--host-only`
4. Real publish with `export PACKSLIP_KEY=...`
5. Spot-check the GitHub release assets and a packslip / APK install
