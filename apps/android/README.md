# srcthread (Android)

Kotlin + Jetpack Compose UI. The app depends only on
[`ktpackages/srcthread`](../../ktpackages/srcthread), which owns UniFFI and
exposes a Compose-aware [`SrcThread`](../../ktpackages/srcthread/src/main/java/dev/kbroom/srcthread/SrcThread.kt)
state holder.

Hello-world:

- Reads `greeting` / `threads` as Compose `State`
- Mutates via `createThread` (Rust → Kotlin push updates state inside the package)

Gradle lives at the **repo root** (next to `Cargo.toml`). UniFFI bindgen + NDK
run automatically on `:ktpackages:srcthread` `preBuild`.

## What’s here

- Compose `MainActivity` (`dev.kbroom.srcthread`)
- `namespace`: `dev.kbroom.srcthread`
- Debug/`android-start`: package `dev.kbroom.srcthread.dev`, label
  `srcthread (dev)`, Linux-matching ship-badge launcher (side-by-side with
  a non-dev install)
- Release: package `dev.kbroom.srcthread`, label `srcthread`, unbadged icon
- detekt + ktlint-wrapper (repo-root `detekt.yaml`; also
  `mise run detekt` / hk)
- `minSdk` 26, `compileSdk` 35

## mise tasks (android CLI — not sdkmanager)

```sh
mise run android-emulator              # android emulator start <avd>
mise run android-start                 # assembleDebug → install → launch on every reachable target
mise run android-start --emulator      # emulators only (fail if none)
mise run android-start --device         # physical devices only (fail if none)
mise run android-start --emulator --device  # both kinds required
```

With no flags, any online emulator and/or handset is used (at least one
required). Installs the debug/dev package (`dev.kbroom.srcthread.dev`) so it
sits beside a non-dev install. Override the AVD with
`SRCTHREAD_AVD=medium_phone`. Pin a handset with
`SRCTHREAD_ANDROID_SERIAL=<serial>` (or `ANDROID_SERIAL`).

Default AVD `Pixel_9_API_37.2` needs host keyboard support so the
emulator sidebar Home / Back / Recents buttons inject keys:

```ini
# ~/.android/avd/Pixel_9_API_37.2.avd/config.ini
hw.keyboard=yes
hw.mainKeys=no
```

Cold-boot once after changing those (`android emulator start` after a full
quit, or delete the AVD `*.lock` files if a previous run left them stale).

## Manual build

```sh
# from repo root
./gradlew :apps:android:assembleDebug
./gradlew :apps:android:detekt :ktpackages:srcthread:detekt
```

Needs the `android` CLI on `PATH`, NDK (`android sdk install 'ndk;…'`),
`cargo-ndk`, and Rust targets `aarch64-linux-android` /
`x86_64-linux-android`.
