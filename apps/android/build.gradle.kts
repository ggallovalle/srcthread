plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
    id("io.gitlab.arturbosch.detekt")
}

android {
    namespace = "dev.kbroom.srcthread"
    compileSdk = 35

    defaultConfig {
        applicationId = "dev.kbroom.srcthread"
        minSdk = 26
        targetSdk = 35
        versionCode = 200
        versionName = "0.2.0"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            // Personal releases: sign with the debug keystore until a
            // release keystore is wired via env / CI secrets.
            signingConfig = signingConfigs.getByName("debug")
        }
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
        buildConfig = true
    }
}

detekt {
    buildUponDefaultConfig = true
    allRules = false
    config.setFrom(files("${rootProject.projectDir}/detekt.yaml"))
    source.setFrom(files("src/main/java/dev/kbroom/srcthread/MainActivity.kt"))
}

dependencies {
    implementation(project(":ktpackages:srcthread"))

    val composeBom = platform("androidx.compose:compose-bom:2024.10.01")
    implementation(composeBom)
    implementation("androidx.activity:activity-compose:1.9.3")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.ui:ui-tooling-preview")
    debugImplementation("androidx.compose.ui:ui-tooling")

    detektPlugins("io.gitlab.arturbosch.detekt:detekt-formatting:1.23.8")
}
