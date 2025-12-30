# Build Mazerion for Android - All ABIs

Write-Host "Building Mazerion for Android..." -ForegroundColor Green

# Check if Android NDK is installed
if (-not $env:ANDROID_NDK_HOME) {
    Write-Host "ERROR: ANDROID_NDK_HOME not set!" -ForegroundColor Red
    Write-Host "Set it to your Android NDK path, e.g.:" -ForegroundColor Yellow
    Write-Host '$env:ANDROID_NDK_HOME = "C:\Users\YourName\AppData\Local\Android\Sdk\ndk\26.1.10909125"'
    exit 1
}

$targets = @(
    "aarch64-linux-android",
    "armv7-linux-androideabi",
    "x86_64-linux-android",
    "i686-linux-android"
)

$abiMap = @{
    "aarch64-linux-android" = "arm64-v8a"
    "armv7-linux-androideabi" = "armeabi-v7a"
    "x86_64-linux-android" = "x86_64"
    "i686-linux-android" = "x86"
}

foreach ($target in $targets) {
    Write-Host "`nBuilding for $target..." -ForegroundColor Cyan

    cargo build --release --target $target -p mazerion-ffi

    if ($LASTEXITCODE -ne 0) {
        Write-Host "Build failed for $target" -ForegroundColor Red
        exit 1
    }

    $abi = $abiMap[$target]
    $outDir = "android\app\src\main\jniLibs\$abi"
    New-Item -ItemType Directory -Force -Path $outDir | Out-Null

    Copy-Item "target\$target\release\libmazerion_ffi.so" -Destination "$outDir\libmazerion_ffi.so" -Force
    Write-Host "✓ Copied to $outDir" -ForegroundColor Green
}

Write-Host "`n✓ All Android libraries built successfully!" -ForegroundColor Green
Write-Host "Libraries are in: android\app\src\main\jniLibs\" -ForegroundColor Yellow