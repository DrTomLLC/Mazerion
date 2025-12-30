# Build Mazerion for Android using cargo-ndk

Write-Host '=== Building Mazerion for Android ===' -ForegroundColor Green

$targets = @(
    @{ndk='arm64-v8a'; triple='aarch64-linux-android'; abi='arm64-v8a'},
    @{ndk='armeabi-v7a'; triple='armv7-linux-androideabi'; abi='armeabi-v7a'},
    @{ndk='x86_64'; triple='x86_64-linux-android'; abi='x86_64'},
    @{ndk='x86'; triple='i686-linux-android'; abi='x86'}
)

foreach ($target in $targets) {
    Write-Host ''
    Write-Host "Building for $($target.ndk)..." -ForegroundColor Cyan

    cargo ndk --target $($target.ndk) --platform 24 build --release -p mazerion-ffi

    if ($LASTEXITCODE -ne 0) {
        Write-Host "Build failed for $($target.ndk)" -ForegroundColor Red
        exit 1
    }

    $outDir = "android\app\src\main\jniLibs\$($target.abi)"
    New-Item -ItemType Directory -Force -Path $outDir | Out-Null

    $sourceFile = "target\$($target.triple)\release\libmazerion_ffi.so"

    if (Test-Path $sourceFile) {
        Copy-Item $sourceFile -Destination "$outDir\libmazerion_ffi.so" -Force

        $size = (Get-Item "$outDir\libmazerion_ffi.so").Length / 1MB
        $sizeMB = [math]::Round($size, 2)
        Write-Host "Copied to $outDir ($sizeMB MB)" -ForegroundColor Green
    } else {
        Write-Host "ERROR: Could not find $sourceFile" -ForegroundColor Red
        exit 1
    }
}

Write-Host ''
Write-Host 'All Android libraries built successfully!' -ForegroundColor Green
Write-Host ''
Write-Host 'Library files:' -ForegroundColor Yellow
Get-ChildItem -Path "android\app\src\main\jniLibs" -Recurse -Filter "*.so" | ForEach-Object {
    $sizeMB = [math]::Round($_.Length / 1MB, 2)
    Write-Host "  $($_.FullName.Replace((Get-Location).Path + '\', '')) - $sizeMB MB" -ForegroundColor Gray
}
Write-Host ''
Write-Host 'Next: Open Android Studio and click Run' -ForegroundColor Yellow