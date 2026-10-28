# Plan: desktop + Android

## Desktop (rune library)

- `apps/desktop` — GPU UI on Wayland/Arch first; **library**, not a binary.
- Launch: `srcthread desktop` (CLI feature `desktop`, on by default locally).
- In-process or socket client to the same engine; prefer socket so it shares
  the warm daemon with CLI/nvim.
- First screens: hello → thread list → thread detail → open anchor in editor.

## Android (Kotlin Compose + UniFFI)

- `apps/android` — Gradle toolchain (not KMP); **Jetpack Compose** UI (not
  Views).
- Namespace / `applicationId`: `dev.kbroom.srcthread`.
- **UniFFI** binds the Rust `srcthread` lib for on-device store open/create/read
  without requiring the PC daemon.
- Capture notes/refs/excerpts on the go; sync later via git or peer transfer.
- CI does **not** require Android SDK in the default matrix.

## Shared rules

- One Rust engine; UIs do not reimplement store logic.
- File-canonical store format identical across PC and phone.
- PC distributable is still just `srcthread` (see
  `research/08-herdr-single-binary.md`); Android is a separate APK.
