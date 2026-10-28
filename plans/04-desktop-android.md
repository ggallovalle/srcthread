# Plan: desktop + Android

## Desktop (rune library)

- `apps/desktop` — GPU UI on Wayland/Arch first; **library**, not a binary.
- Launch: `srcthread desktop` (CLI feature `desktop`, on by default locally).
- Embed the `srcthread` lib in-process (no PC daemon).
- First screens: hello → thread list → thread detail → open anchor in editor.

## Android (Kotlin Compose + UniFFI)

- Gradle root + `detekt.yaml` live at the repo root (next to `Cargo.toml`).
- `ktpackages/srcthread` — Android library: UniFFI bindgen/NDK on `preBuild`,
  Compose-aware `SrcThread` / `SrcThreadFfi` facade (hides callbacks).
- `apps/android` — Compose UI only; depends on `:ktpackages:srcthread` (not
  UniFFI directly). Namespace / `applicationId`: `dev.kbroom.srcthread`.
- Capture notes/refs/excerpts on the go; sync later via git or peer transfer.
- CI does **not** require Android SDK in the default matrix.

## Shared rules

- One Rust engine; UIs do not reimplement store logic.
- File-canonical store format identical across PC and phone.
- PC distributable is still just `srcthread` (see
  `research/08-herdr-single-binary.md`); Android is a separate APK.
