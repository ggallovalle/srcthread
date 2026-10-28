import java.io.File

plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
    id("io.gitlab.arturbosch.detekt")
}

android {
    // Distinct from the app module namespace; Kotlin public API stays
    // package `dev.kbroom.srcthread`.
    namespace = "dev.kbroom.srcthread.lib"
    compileSdk = 35

    defaultConfig {
        minSdk = 26
        consumerProguardFiles("consumer-rules.pro")
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    buildFeatures {
        compose = true
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
            java.srcDir(layout.buildDirectory.dir("generated/source/uniffi"))
        }
    }
}

detekt {
    buildUponDefaultConfig = true
    allRules = false
    config.setFrom(files("${rootProject.projectDir}/detekt.yaml"))
    source.setFrom(
        files(
            "src/main/java/dev/kbroom/srcthread/SrcThread.kt",
            "src/main/java/dev/kbroom/srcthread/SrcThreadFfi.kt",
        ),
    )
}

dependencies {
    implementation(platform("androidx.compose:compose-bom:2024.10.01"))
    implementation("androidx.compose.runtime:runtime")
    implementation("net.java.dev.jna:jna:5.17.0@aar")
    detektPlugins("io.gitlab.arturbosch.detekt:detekt-formatting:1.23.8")
}

val repoRoot = rootProject.projectDir
val cargoTargetDir = File(repoRoot, "target")
val uniffiOutDir = layout.buildDirectory.dir("generated/source/uniffi")
val jniLibsDir = File(projectDir, "src/main/jniLibs")
val rustSources = fileTree(File(repoRoot, "crates/srcthread-uniffi")) {
    include("**/*.rs")
    include("**/*.toml")
}
val engineSources = fileTree(File(repoRoot, "crates/srcthread")) {
    include("**/*.rs")
    include("**/*.toml")
}
val uniffiToml = File(repoRoot, "crates/srcthread-uniffi/uniffi.toml")

tasks.register<Exec>("cargoBuildUniffiHost") {
    group = "uniffi"
    description = "Build srcthread-uniffi for the host (binding generation)"
    workingDir = repoRoot
    commandLine("cargo", "build", "-p", "srcthread-uniffi")
    inputs.files(rustSources, engineSources)
    outputs.dir(File(cargoTargetDir, "debug"))
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
    val outDir = uniffiOutDir.get().asFile

    inputs.files(rustSources, engineSources, uniffiToml)
    outputs.dir(outDir)

    doFirst {
        outDir.mkdirs()
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
        outDir.absolutePath,
        "--config",
        uniffiToml.absolutePath,
        "--no-format",
    )
}

tasks.register<Exec>("cargoNdkUniffi") {
    group = "uniffi"
    description =
        "Cross-compile libsrcthread_uniffi.so into jniLibs " +
            "(requires cargo-ndk + Android NDK)"
    workingDir = repoRoot
    inputs.files(rustSources, engineSources)
    outputs.dir(jniLibsDir)
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

tasks.named("preBuild").configure {
    dependsOn("generateUniffiKotlin", "cargoNdkUniffi")
}
