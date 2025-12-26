# qa.ps1 - Robust Rust QA runner (Windows PowerShell 5.1 + PowerShell 7)
# Produces: qa_reports\qa_YYYYMMDD_HHMMSS\{summary.md,summary.json,logs\*.{out,err}.txt}

param(
    [string]$Toolchain = "",          # "+stable", "+beta", "+nightly" or ""
    [string]$ReportRoot = "qa_reports",
    [int]$TailLines = 80
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function New-Dir([string]$Path) {
    [void][System.IO.Directory]::CreateDirectory($Path)
}

function Timestamp() {
    (Get-Date).ToString("yyyyMMdd_HHmmss")
}

function Has-CargoSubcommand([string]$subcmd) {
    $exe = "cargo-$subcmd"
    return $null -ne (Get-Command $exe -ErrorAction SilentlyContinue)
}

function Tail-Lines([string]$path, [int]$n) {
    if (-not (Test-Path -LiteralPath $path)) { return "" }
    # Force array even if file has 0/1 lines; avoids strict-mode `.Count` problems on strings.
    $lines = @(Get-Content -LiteralPath $path -ErrorAction SilentlyContinue)
    if ($lines.Count -eq 0) { return "" }
    if ($lines.Count -le $n) { return ($lines -join "`n") }
    (($lines | Select-Object -Last $n) -join "`n")
}

function Quote-Arg([string]$s) {
    if ($null -eq $s) { return "" }
    if ($s -match '[\s"]') { return '"' + ($s -replace '"','\"') + '"' }
    return $s
}

function CargoArgs([string[]]$More) {
    # Always returns a non-null string[] with no empty elements.
    $list = New-Object 'System.Collections.Generic.List[string]'

    if (-not [string]::IsNullOrWhiteSpace($Toolchain)) {
        $list.Add($Toolchain.Trim())
    }

    foreach ($a in @($More)) {
        if ($null -ne $a -and -not [string]::IsNullOrWhiteSpace($a)) {
            $list.Add([string]$a)
        }
    }

    return $list.ToArray()
}

function Invoke-Step {
    param(
        [Parameter(Mandatory=$true)][string]$Name,
        [Parameter(Mandatory=$true)][string]$Exe,
        [Parameter()][AllowNull()][string[]]$Args = @(),
        [Parameter(Mandatory=$true)][string]$Id
    )

    if ($null -eq $Args) { $Args = @() }

    $stdoutPath = Join-Path $script:LogDir "$Id.out.txt"
    $stderrPath = Join-Path $script:LogDir "$Id.err.txt"

    Write-Host ""
    Write-Host "=== $Name ==="

    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    $exit = 999

    try {
        $psi = New-Object System.Diagnostics.ProcessStartInfo
        $psi.FileName = $Exe
        $psi.Arguments = (($Args | ForEach-Object { Quote-Arg $_ }) -join " ")
        $psi.RedirectStandardOutput = $true
        $psi.RedirectStandardError  = $true
        $psi.UseShellExecute = $false
        $psi.CreateNoWindow = $true
        $psi.WorkingDirectory = (Get-Location).Path

        $p = New-Object System.Diagnostics.Process
        $p.StartInfo = $psi

        $null = $p.Start()
        $stdout = $p.StandardOutput.ReadToEnd()
        $stderr = $p.StandardError.ReadToEnd()
        $p.WaitForExit()
        $exit = $p.ExitCode

        Set-Content -LiteralPath $stdoutPath -Value $stdout -Encoding utf8
        Set-Content -LiteralPath $stderrPath -Value $stderr -Encoding utf8
    }
    catch {
        # Record exception as stderr for this step
        $msg = ($_ | Out-String)
        Set-Content -LiteralPath $stdoutPath -Value "" -Encoding utf8
        Set-Content -LiteralPath $stderrPath -Value $msg -Encoding utf8
        $exit = 999
    }

    $sw.Stop()

    if ($exit -eq 0) {
        Write-Host $Exe
        Write-Host "=> PASS ($([math]::Round($sw.Elapsed.TotalSeconds,2))s)"
    } else {
        Write-Host $Exe
        Write-Host "=> FAIL exit=$exit ($([math]::Round($sw.Elapsed.TotalSeconds,2))s)"

        $stderrTail = Tail-Lines $stderrPath $TailLines
        $stdoutTail = Tail-Lines $stdoutPath $TailLines

        if ($stderrTail.Trim()) {
            Write-Host "---- stderr tail ($TailLines) ----"
            Write-Host $stderrTail
            Write-Host "--------------------------"
        }
        if ($stdoutTail.Trim()) {
            Write-Host "---- stdout tail ($TailLines) ----"
            Write-Host $stdoutTail
            Write-Host "--------------------------"
        }
    }

    return [pscustomobject]@{
        name    = $Name
        id      = $Id
        exe     = $Exe
        args    = $Args
        exit    = $exit
        seconds = [math]::Round($sw.Elapsed.TotalSeconds, 2)
        stdout  = $stdoutPath
        stderr  = $stderrPath
        skipped = $false
    }
}

function Skip-Step([string]$Name, [string]$Id, [string]$Reason) {
    Write-Host ""
    Write-Host "=== $Name ==="
    Write-Host "=> SKIP ($Reason)"
    return [pscustomobject]@{
        name    = $Name
        id      = $Id
        exe     = ""
        args    = @()
        exit    = $null
        seconds = 0
        stdout  = ""
        stderr  = ""
        skipped = $true
        reason  = $Reason
    }
}

# --- Setup report dirs ---
$repoRoot = (Get-Location).Path
$stamp = Timestamp
$script:ReportDir = Join-Path $repoRoot (Join-Path $ReportRoot ("qa_$stamp"))
$script:LogDir    = Join-Path $script:ReportDir "logs"
New-Dir $script:LogDir

$results = New-Object System.Collections.Generic.List[object]

# --- Basics ---
$results.Add((Invoke-Step "cargo --version" "cargo" (CargoArgs @("--version")) "cargo_version")) | Out-Null
$results.Add((Invoke-Step "rustc --version" "rustc" @("--version") "rustc_version")) | Out-Null
$results.Add((Invoke-Step "rustup show" "rustup" @("show") "rustup_show")) | Out-Null

# --- Core workflow ---
$results.Add((Invoke-Step "cargo metadata" "cargo" (CargoArgs @("metadata","--format-version","1","--no-deps")) "cargo_metadata")) | Out-Null
$results.Add((Invoke-Step "cargo fmt --check" "cargo" (CargoArgs @("fmt","--check")) "fmt")) | Out-Null
$results.Add((Invoke-Step "cargo check" "cargo" (CargoArgs @("check")) "check")) | Out-Null
$results.Add((Invoke-Step "cargo clippy -- -D warnings" "cargo" (CargoArgs @("clippy","--","-D","warnings")) "clippy")) | Out-Null
$results.Add((Invoke-Step "cargo build (debug)" "cargo" (CargoArgs @("build")) "build_debug")) | Out-Null
$results.Add((Invoke-Step "cargo test (debug, --no-fail-fast)" "cargo" (CargoArgs @("test","--no-fail-fast")) "test_debug")) | Out-Null
$results.Add((Invoke-Step "cargo test (release, --no-fail-fast)" "cargo" (CargoArgs @("test","--release","--no-fail-fast")) "test_release")) | Out-Null

# --- Optional cargo-* tools ---
if (Has-CargoSubcommand "nextest") {
    $results.Add((Invoke-Step "cargo nextest run" "cargo" (CargoArgs @("nextest","run")) "nextest")) | Out-Null
} else {
    $results.Add((Skip-Step "cargo nextest run" "nextest" "cargo-nextest not installed")) | Out-Null
}

if (Has-CargoSubcommand "llvm-cov") {
    $lcovPath = Join-Path $script:ReportDir "lcov.info"
    $results.Add((Invoke-Step "cargo llvm-cov (lcov)" "cargo" (CargoArgs @("llvm-cov","--lcov","--output-path",$lcovPath)) "llvm_cov_lcov")) | Out-Null
} else {
    $results.Add((Skip-Step "cargo llvm-cov (lcov)" "llvm_cov_lcov" "cargo-llvm-cov not installed")) | Out-Null
}

if (Has-CargoSubcommand "audit") {
    $results.Add((Invoke-Step "cargo audit" "cargo" (CargoArgs @("audit")) "audit")) | Out-Null
} else {
    $results.Add((Skip-Step "cargo audit" "audit" "cargo-audit not installed")) | Out-Null
}

if (Has-CargoSubcommand "deny") {
    $results.Add((Invoke-Step "cargo deny check" "cargo" (CargoArgs @("deny","check")) "deny")) | Out-Null
} else {
    $results.Add((Skip-Step "cargo deny check" "deny" "cargo-deny not installed")) | Out-Null
}

if (Has-CargoSubcommand "machete") {
    $results.Add((Invoke-Step "cargo machete" "cargo" (CargoArgs @("machete")) "machete")) | Out-Null
} else {
    $results.Add((Skip-Step "cargo machete" "machete" "cargo-machete not installed")) | Out-Null
}

if (Has-CargoSubcommand "geiger") {
    $results.Add((Invoke-Step "cargo geiger" "cargo" (CargoArgs @("geiger")) "geiger")) | Out-Null
} else {
    $results.Add((Skip-Step "cargo geiger" "geiger" "cargo-geiger not installed")) | Out-Null
}

# --- Summaries ---
$failures = @($results | Where-Object { -not $_.skipped -and $_.exit -ne 0 })
$passes   = @($results | Where-Object { -not $_.skipped -and $_.exit -eq 0 })
$skips    = @($results | Where-Object { $_.skipped })

$summaryMd = @()
$summaryMd += "DONE. Report folder:"
$summaryMd += $script:ReportDir
$summaryMd += ""
$summaryMd += "Summary:"
$summaryMd += "  $([System.IO.Path]::Combine($script:ReportDir,'summary.md'))"
$summaryMd += "  $([System.IO.Path]::Combine($script:ReportDir,'summary.json'))"
$summaryMd += ""
$summaryMd += "Counts:"
$summaryMd += "  PASS: $($passes.Count)"
$summaryMd += "  FAIL: $($failures.Count)"
$summaryMd += "  SKIP: $($skips.Count)"
$summaryMd += ""

if ($failures.Count -gt 0) {
    $summaryMd += "FAILURES:"
    foreach ($f in $failures) {
        $summaryMd += "- $($f.name)  (stderr: $($f.stderr))"
    }
} else {
    $summaryMd += "ALL CHECKS PASSED."
}

Set-Content -LiteralPath (Join-Path $script:ReportDir "summary.md") -Value ($summaryMd -join "`n") -Encoding utf8

$summaryObj = [pscustomobject]@{
    report_dir   = $script:ReportDir
    repo_root    = $repoRoot
    toolchain    = $Toolchain.Trim()
    generated_at = (Get-Date).ToString("o")
    counts       = [pscustomobject]@{
        pass = $passes.Count
        fail = $failures.Count
        skip = $skips.Count
    }
    results      = $results
}

($summaryObj | ConvertTo-Json -Depth 10) | Set-Content -LiteralPath (Join-Path $script:ReportDir "summary.json") -Encoding utf8

Write-Host ""
Write-Host "DONE. Report folder:"
Write-Host $script:ReportDir
Write-Host "Summary:"
Write-Host "  $(Join-Path $script:ReportDir 'summary.md')"
Write-Host "  $(Join-Path $script:ReportDir 'summary.json')"
Write-Host ""

if ($failures.Count -gt 0) {
    Write-Host "FAILURES:"
    foreach ($f in $failures) {
        Write-Host "- $($f.name)  (stderr: $($f.stderr))"
    }
    exit 1
} else {
    exit 0
}
