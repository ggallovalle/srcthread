#!/usr/bin/env bash
# Cut a GitHub release manually: PC tarballs (+ optional desktop variant),
# usage/completions assets, Android APK, checksums, and packslip bundle.
#
# Usage:
#   scripts/publish-release.sh <version> [--dry-run] [--skip-android] [--skip-desktop] [--host-only]
#
# Env:
#   PACKSLIP_KEY   Path to a packslip secret key (from `packslip keygen`).
#                  If unset, packslip signing is skipped with a note.
#   HOST_TRIPLE    Override rustc host triple for --host-only builds.
#
# Deactivated GHA: `.github/workflows/release.yml` (workflow_dispatch only).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION=""
DRY_RUN=0
SKIP_ANDROID=0
SKIP_DESKTOP=0
HOST_ONLY=0

usage() {
	sed -n '2,16p' "$0" | sed 's/^# \?//'
	exit "${1:-0}"
}

while [[ $# -gt 0 ]]; do
	case "$1" in
	-h | --help) usage 0 ;;
	--dry-run) DRY_RUN=1 ;;
	--skip-android) SKIP_ANDROID=1 ;;
	--skip-desktop) SKIP_DESKTOP=1 ;;
	--host-only) HOST_ONLY=1 ;;
	-*)
		echo "unknown flag: $1" >&2
		usage 1
		;;
	*)
		if [[ -n "$VERSION" ]]; then
			echo "unexpected argument: $1" >&2
			usage 1
		fi
		VERSION="$1"
		;;
	esac
	shift
done

if [[ -z "$VERSION" ]]; then
	echo "version required (e.g. 0.1.0)" >&2
	usage 1
fi
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+([.-].+)?$ ]]; then
	echo "version must look like semver (got: $VERSION)" >&2
	exit 1
fi

TAG="v${VERSION}"
HOST_TRIPLE="${HOST_TRIPLE:-$(rustc -vV | awk '/^host:/{print $2}')}"

echo "==> publish $TAG (dry_run=$DRY_RUN host_only=$HOST_ONLY)"

if [[ "$DRY_RUN" -eq 0 ]]; then
	if [[ -n "$(git status --porcelain)" ]]; then
		echo "working tree is dirty; commit or stash first" >&2
		exit 1
	fi
	command -v gh >/dev/null || {
		echo "gh is required" >&2
		exit 1
	}
	gh auth status >/dev/null
fi

pkg_version="$(cargo metadata --no-deps --format-version 1 |
	python3 -c "import json,sys; d=json.load(sys.stdin); print([p['version'] for p in d['packages'] if p['name']=='srcthread-cli'][0])")"
if [[ "$pkg_version" != "$VERSION" ]]; then
	echo "Cargo.toml version is $pkg_version but publish asked for $VERSION" >&2
	echo "bump crates/srcthread-cli (and workspace) version first" >&2
	exit 1
fi

rm -rf dist
mkdir -p dist

echo "==> release assets (usage, completions, man)"
DIST=1 ./scripts/release-assets.sh

build_one() {
	local triple="$1" variant="$2"
	./scripts/build-tarball.sh "$triple" "$variant"
}

if [[ "$HOST_ONLY" -eq 1 ]]; then
	echo "==> host-only tarball ($HOST_TRIPLE)"
	build_one "$HOST_TRIPLE" default
	if [[ "$SKIP_DESKTOP" -eq 0 ]]; then
		build_one "$HOST_TRIPLE" desktop || {
			echo "warning: desktop variant build failed; continuing without it" >&2
		}
	fi
else
	echo "==> platform tarballs"
	triples=(
		x86_64-unknown-linux-gnu
		aarch64-unknown-linux-gnu
		x86_64-apple-darwin
		aarch64-apple-darwin
	)
	for triple in "${triples[@]}"; do
		# Skip foreign triples that cannot build on this host without cross.
		case "$triple" in
		"$HOST_TRIPLE") ;;
		*-apple-darwin)
			[[ "$(uname -s)" == Darwin ]] || {
				echo "skip $triple (not on macOS)"
				continue
			}
			;;
		*-linux-gnu)
			[[ "$(uname -s)" == Linux ]] || {
				echo "skip $triple (not on Linux)"
				continue
			}
			# Cross-arch Linux needs a toolchain; skip if rustup target missing.
			if ! rustup target list --installed 2>/dev/null | grep -qx "$triple"; then
				echo "skip $triple (rustup target not installed)"
				continue
			fi
			;;
		esac
		build_one "$triple" default
		if [[ "$SKIP_DESKTOP" -eq 0 ]]; then
			build_one "$triple" desktop || echo "warning: desktop build failed for $triple" >&2
		fi
	done
fi

if [[ "$SKIP_ANDROID" -eq 0 ]]; then
	echo "==> Android release APK"
	if [[ -z "${ANDROID_HOME:-}${ANDROID_SDK_ROOT:-}" ]]; then
		echo "ANDROID_HOME/ANDROID_SDK_ROOT unset; skip APK (pass --skip-android to silence)" >&2
	else
		./gradlew :apps:android:assembleRelease
		apk="$(find apps/android/build/outputs/apk/release -name '*.apk' | head -1)"
		if [[ -z "$apk" ]]; then
			echo "assembleRelease produced no APK" >&2
			exit 1
		fi
		cp -f "$apk" "dist/srcthread-${VERSION}-android.apk"
		ls -lh "dist/srcthread-${VERSION}-android.apk"
	fi
fi

./scripts/checksums.sh

if [[ "$DRY_RUN" -eq 1 ]]; then
	echo "==> dry-run complete; artifacts in dist/"
	ls -lh dist/
	exit 0
fi

echo "==> tag $TAG"
if git rev-parse "$TAG" >/dev/null 2>&1; then
	echo "tag $TAG already exists" >&2
	exit 1
fi
git tag -a "$TAG" -m "$TAG"
git push origin "refs/tags/$TAG"

echo "==> GitHub release"
shopt -s nullglob
assets=(dist/*)
gh release create "$TAG" "${assets[@]}" \
	--title "$TAG" \
	--notes "PC: packslip install. Android: attach \`srcthread-${VERSION}-android.apk\`." \
	--verify-tag

echo "==> packslip"
PACKSLIP_BIN="${PACKSLIP_BIN:-}"
if [[ -z "$PACKSLIP_BIN" ]]; then
	if command -v packslip >/dev/null 2>&1; then
		PACKSLIP_BIN="$(command -v packslip)"
	elif [[ -x "${HOME}/.local/share/mise/installs/packslip/1.11.1/packslip" ]]; then
		PACKSLIP_BIN="${HOME}/.local/share/mise/installs/packslip/1.11.1/packslip"
	fi
fi

if [[ -z "${PACKSLIP_KEY:-}" ]]; then
	echo "PACKSLIP_KEY unset — skipping packslip.sigstore.json"
	echo "Generate a key with: mise exec packslip -- packslip keygen"
	echo "Or dispatch .github/workflows/release.yml after enabling packslip there."
elif [[ -z "$PACKSLIP_BIN" ]]; then
	echo "packslip CLI not found — skipping packslip.sigstore.json"
else
	url_base="https://github.com/ggallovalle/srcthread/releases/download/${TAG}"
	create_args=(
		--project github.com/ggallovalle/srcthread
		--version "$VERSION"
		--bin srcthread
		--url-base "$url_base"
		--out dist
		--key "$PACKSLIP_KEY"
		--no-log
		--resource "cli-spec/usage=asset:dist/srcthread.usage.kdl"
		--resource "completion/bash=asset:dist/srcthread.bash"
		--resource "completion/zsh=asset:dist/srcthread.zsh"
		--resource "completion/fish=asset:dist/srcthread.fish"
		--resource "completion/powershell=asset:dist/srcthread.powershell"
		--resource "man@linux=archive:srcthread/man/man1/srcthread.1"
		--resource "man@darwin=archive:srcthread/man/man1/srcthread.1"
	)
	artifact_args=()
	for f in dist/srcthread-"${VERSION}"-*.tar.gz; do
		base="$(basename "$f")"
		if [[ "$base" == *"-desktop.tar.gz" ]]; then
			artifact_args+=("${f}@desktop")
		else
			artifact_args+=("$f")
		fi
	done
	"$PACKSLIP_BIN" create "${create_args[@]}" "${artifact_args[@]}"
	gh release upload "$TAG" dist/packslip.sigstore.json --clobber
fi

cat <<EOF

Published $TAG.

Install (PC):

  [tools]
  "packslip:github.com/ggallovalle/srcthread" = { version = "${VERSION}", variant = "desktop" }

Optional postinstall:

  postinstall = "srcthread integration install cursor --user && srcthread desktop install-xdg"

Android: download srcthread-${VERSION}-android.apk from the release.
EOF
