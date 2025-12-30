# Android/Kotlin Multiplatform Integration Guide

## Prerequisites

- Rust toolchain with Android targets
- Android NDK
- Kotlin 1.9+
- Gradle 8.0+

## Step 1: Install Android Targets
```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

## Step 2: Build for Android

### Configure Cargo for Android

Create `.cargo/config.toml` in project root:
```toml
[target.aarch64-linux-android]
linker = "<NDK_PATH>/toolchains/llvm/prebuilt/<HOST>/bin/aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
linker = "<NDK_PATH>/toolchains/llvm/prebuilt/<HOST>/bin/armv7a-linux-androideabi21-clang"

[target.i686-linux-android]
linker = "<NDK_PATH>/toolchains/llvm/prebuilt/<HOST>/bin/i686-linux-android21-clang"

[target.x86_64-linux-android]
linker = "<NDK_PATH>/toolchains/llvm/prebuilt/<HOST>/bin/x86_64-linux-android21-clang"
```

Replace `<NDK_PATH>` with your Android NDK path and `<HOST>` with your platform (e.g., `linux-x86_64`, `darwin-x86_64`, `windows-x86_64`).

### Build Script

**File: `build-android.sh`**
```bash
#!/bin/bash
set -e

echo "Building Mazerion for Android..."

targets=("aarch64-linux-android" "armv7-linux-androideabi" "x86_64-linux-android" "i686-linux-android")

for target in "${targets[@]}"; do
    echo "Building for $target..."
    cargo build --release --target $target -p mazerion-ffi
done

echo "Copying libraries to jniLibs..."
mkdir -p android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86_64,x86}

cp target/aarch64-linux-android/release/libmazerion_ffi.so android/app/src/main/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/libmazerion_ffi.so android/app/src/main/jniLibs/armeabi-v7a/
cp target/x86_64-linux-android/release/libmazerion_ffi.so android/app/src/main/jniLibs/x86_64/
cp target/i686-linux-android/release/libmazerion_ffi.so android/app/src/main/jniLibs/x86/

echo "✓ Android libraries ready in android/app/src/main/jniLibs/"
```

## Step 3: Generate Kotlin Bindings
```bash
# Windows
.\crates\ffi\generate-bindings.ps1

# Linux/Mac
./crates/ffi/generate-bindings.sh
```

## Step 4: Add to Kotlin Project

### Project Structure
```
your-kotlin-project/
├── app/src/main/
│   ├── kotlin/
│   │   └── com/yourcompany/mazerion/
│   │       └── [Generated .kt files from bindings/kotlin/]
│   └── jniLibs/
│       ├── arm64-v8a/
│       │   └── libmazerion_ffi.so
│       ├── armeabi-v7a/
│       │   └── libmazerion_ffi.so
│       ├── x86_64/
│       │   └── libmazerion_ffi.so
│       └── x86/
│           └── libmazerion_ffi.so
└── build.gradle.kts
```

### Gradle Configuration

**File: `app/build.gradle.kts`**
```kotlin
plugins {
    id("com.android.application")
    kotlin("android")
}

android {
    namespace = "com.yourcompany.mazerion"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.yourcompany.mazerion"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "0.30.0"

        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64", "x86")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("net.java.dev.jna:jna:5.14.0@aar")
}
```

## Step 5: Usage Example

**File: `MainActivity.kt`**
```kotlin
package com.yourcompany.mazerion

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import android.util.Log
import mazerion.*

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        try {
            // Get version
            val version = version()
            Log.i("Mazerion", "Version: $version")

            // List calculators
            val calculators = listCalculators()
            Log.i("Mazerion", "Found ${calculators.size} calculators")

            // Calculate ABV
            val params = listOf(
                CalcParam("og", "1.080"),
                CalcParam("fg", "1.010")
            )
            
            val result = executeCalculator("abv", params)
            Log.i("Mazerion", "ABV: ${result.value} ${result.unit}")
            
            // Batch calculations
            val batchRequests = listOf(
                BatchCalculatorRequest("abv", params),
                BatchCalculatorRequest("abv", listOf(
                    CalcParam("og", "1.050"),
                    CalcParam("fg", "1.000")
                ))
            )
            
            val batchResults = executeBatch(batchRequests)
            batchResults.forEach { batch ->
                batch.result?.let {
                    Log.i("Mazerion", "${batch.calculatorId}: ${it.value} ${it.unit}")
                }
                batch.error?.let {
                    Log.e("Mazerion", "${batch.calculatorId} error: $it")
                }
            }

        } catch (e: MazerionException) {
            Log.e("Mazerion", "Error: ${e.message}", e)
        }
    }
}
```

## Step 6: Proguard Rules (if using minification)

**File: `app/proguard-rules.pro`**
```proguard
# Keep all Mazerion classes
-keep class mazerion.** { *; }

# Keep JNA classes
-keep class com.sun.jna.** { *; }
-keep class * implements com.sun.jna.** { *; }

# Don't warn about JNA
-dontwarn com.sun.jna.**
```

## Wear OS Support

For Wear OS, use the same libraries but with Wear-specific UI:

**File: `wear/build.gradle.kts`**
```kotlin
dependencies {
    implementation("com.google.android.gms:play-services-wearable:18.1.0")
    implementation("androidx.wear:wear:1.3.0")
    implementation("net.java.dev.jna:jna:5.14.0@aar")
}
```

## Troubleshooting

### Library Not Found
Ensure libraries are in correct `jniLibs` directories with correct ABI names.

### UnsatisfiedLinkError
Check that target API level matches compiled library (min API 24).

### Crashes on Older Devices
Verify all required ABIs are included and properly built.

### Performance Issues
Use batch operations (`executeBatch`) instead of individual calls for better battery life.