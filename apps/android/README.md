# srcthread (Android)

Kotlin + Jetpack Compose UI that loads the Rust engine via
[UniFFI](https://mozilla.github.io/uniffi-rs/) (`crates/srcthread-uniffi`).

Hello-world talks **both ways**:

- Kotlin → Rust: `hello`, `listThreads` (query), `createThread` (mutation)
- Rust → Kotlin: `HostCallbacks.onLog` / `onThreadsChanged`

## What’s here

- Compose `MainActivity` (`dev.kbroom.srcthread`)
- `applicationId` / `namespace`: `dev.kbroom.srcthread`
- UniFFI package: `dev.kbroom.srcthread.uniffi`
- detekt + ktlint-wrapper (`config/detekt.yaml`; also via `mise run detekt` / hk)
- `minSdk` 26, `compileSdk` 35

## mise tasks (android CLI — not sdkmanager)

```sh
mise run android-emulator   # android emulator start <avd>
mise run android-uniffi     # generate Kotlin + NDK .so into jniLibs
mise run android-start      # android run --activity .MainActivity
```

Override the AVD with `SRCTHREAD_AVD=medium_phone`.

## Manual build

```sh
cd apps/android
./gradlew bootstrapUniffi cargoNdkUniffi
./gradlew :app:assembleDebug
./gradlew detekt
```

Needs the `android` CLI on `PATH`, NDK (`android sdk install 'ndk;…'`),
`cargo-ndk`, and Rust targets `aarch64-linux-android` /
`x86_64-linux-android`.
