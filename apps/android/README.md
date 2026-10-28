# srcthread (Android)

Kotlin + Gradle UI that loads the Rust engine via
[UniFFI](https://mozilla.github.io/uniffi-rs/) (`crates/srcthread-uniffi`).

## What’s here

- Single-Activity app (`dev.kbroom.srcthread.MainActivity`)
- `applicationId` / `namespace`: `dev.kbroom.srcthread`
- UniFFI Kotlin package target: `dev.kbroom.srcthread.uniffi`
- detekt + ktlint-wrapper config under `config/detekt.yaml`
- `minSdk` 26, `compileSdk` 35

CI does **not** assume the Android SDK. Build locally when you have
SDK + JDK installed.

## Build (local)

1. Install Android Studio or command-line SDK + JDK 17+.
2. From this directory, generate the Gradle wrapper if needed:

   ```sh
   gradle wrapper --gradle-version 8.10.2
   ```

3. (Optional) Generate UniFFI Kotlin + host lib:

   ```sh
   ./gradlew bootstrapUniffi
   ```

4. (Optional) Cross-compile `.so` into `app/src/main/jniLibs`
   (needs `cargo-ndk` + Android NDK):

   ```sh
   ./gradlew cargoNdkUniffi
   ```

5. Build:

   ```sh
   ./gradlew :app:assembleDebug
   ```

Without step 4 the Activity shows a static **srcthread hello** label.
With the native lib present it calls UniFFI `hello()`.

## Lint

```sh
./gradlew detekt
```
