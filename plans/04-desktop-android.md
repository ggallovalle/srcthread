# Plan: desktop + Android

## Desktop (rune)

- `apps/desktop` — GPU UI on Wayland/Arch first.
- In-process or socket client to the same engine; prefer socket so it shares
  the warm daemon with CLI/nvim.
- First screens: thread list → thread detail → open anchor in editor.

## Android (Kotlin + Gradle + UniFFI)

- `apps/android` — Gradle toolchain (not KMP).
- UI in Kotlin; **UniFFI** binds the Rust `srcthread` lib for on-device store
  open/create/read without requiring `srcthreadd`.
- Capture notes/refs/excerpts on the go; sync later via git or peer transfer.
- CI does **not** require Android SDK in the default matrix.

## Shared rules

- One Rust engine; UIs do not reimplement store logic.
- File-canonical store format identical across PC and phone.
