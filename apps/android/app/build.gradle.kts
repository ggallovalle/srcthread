import java.io.File

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("io.gitlab.arturbosch.detekt")
}

android {
    namespace = "dev.kbroom.srcthread"
    compileSdk = 35

    defaultConfig {
        applicationId = "dev.kbroom.srcthread"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.0.1"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }
}

detekt {
    buildUponDefaultConfig = true
    allRules = false
    config.setFrom(files("${rootProject.projectDir}/config/detekt.yaml"))
    source.setFrom(
        files(
            "src/main/java",
            "src/main/kotlin",
        ),
    )
}

dependencies {
    implementation("androidx.appcompat:appcompat:1.7.0")
    // UniFFI Kotlin runtime (JNA loads libsrcthread_uniffi.so from jniLibs).
    implementation("net.java.dev.jna:jna:5.17.0@aar")
    detektPlugins("io.gitlab.arturbosch.detekt:detekt-rules-ktlint-wrapper:1.23.8")
}

val repoRoot = rootProject.projectDir.parentFile!!.parentFile!!
val cargoTargetDir = File(repoRoot, "target")
val uniffiPackageDir =
    File(projectDir, "src/main/java/dev/kbroom/srcthread/uniffi")
val jniLibsDir = File(projectDir, "src/main/jniLibs")

tasks.register<Exec>("cargoBuildUniffiHost") {
    group = "uniffi"
    description = "Build srcthread-uniffi for the host (binding generation)"
    workingDir = repoRoot
    commandLine("cargo", "build", "-p", "srcthread-uniffi")
}

tasks.register<Exec>("generateUniffiKotlin") {
    group = "uniffi"
    description = "Generate Kotlin bindings from libsrcthread_uniffi"
    dependsOn("cargoBuildUniffiHost")
    workingDir = repoRoot

    val libName =
        when {
            org.gradle.internal.os.OperatingSystem.current().isMacOsX ->
                "libsrcthread_uniffi.dylib"
            org.gradle.internal.os.OperatingSystem.current().isWindows ->
                "srcthread_uniffi.dll"
            else -> "libsrcthread_uniffi.so"
        }
    val library = File(cargoTargetDir, "debug/$libName")

    doFirst {
        uniffiPackageDir.mkdirs()
        require(library.isFile) {
            "missing $library — run cargoBuildUniffiHost first"
        }
    }

    commandLine(
        "cargo",
        "run",
        "-p",
        "srcthread-uniffi",
        "--bin",
        "uniffi-bindgen",
        "--",
        "generate",
        "--library",
        library.absolutePath,
        "--language",
        "kotlin",
        "--out-dir",
        uniffiPackageDir.absolutePath,
    )
}

tasks.register<Exec>("cargoNdkUniffi") {
    group = "uniffi"
    description =
        "Cross-compile libsrcthread_uniffi.so into app/src/main/jniLibs " +
            "(requires cargo-ndk + Android NDK)"
    workingDir = repoRoot
    commandLine(
        "cargo",
        "ndk",
        "-o",
        jniLibsDir.absolutePath,
        "-t",
        "arm64-v8a",
        "-t",
        "x86_64",
        "build",
        "-p",
        "srcthread-uniffi",
        "--release",
    )
}

tasks.register("bootstrapUniffi") {
    group = "uniffi"
    description = "Host build + Kotlin bindings (NDK optional via cargoNdkUniffi)"
    dependsOn("generateUniffiKotlin")
}

tasks.named("preBuild").configure {
    // Prefer committed/generated bindings; regenerate when explicitly asked.
    // Local full bootstrap: ./gradlew bootstrapUniffi cargoNdkUniffi
}
