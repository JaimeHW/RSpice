<#
.SYNOPSIS
Build or run a symbol-bearing Windows x64 library test without changing release flags.
#>
[CmdletBinding()]
param(
    [string]$Package = 'rspice-app',
    [string]$TestFilter = 'state::pdk_config::technology_callback',
    [string]$TargetDirectory = 'target/diagnostic-windows',
    [switch]$NoRun
)

$ErrorActionPreference = 'Stop'
$workspaceRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
if ($env:OS -ne 'Windows_NT') {
    throw 'This procedure targets Windows x64/MSVC.'
}
if ($env:RUSTFLAGS -or $env:CARGO_ENCODED_RUSTFLAGS) {
    throw 'Clear RUSTFLAGS and CARGO_ENCODED_RUSTFLAGS before this diagnostic build; the procedure supplies both symbol and code-generation flags.'
}

$previousLocation = Get-Location
try {
    Set-Location -LiteralPath $workspaceRoot
    $compiler = & rustc --version --verbose
    if ($LASTEXITCODE -ne 0 -or $compiler -notcontains 'host: x86_64-pc-windows-msvc') {
        throw 'The active compiler must be x86_64-pc-windows-msvc.'
    }
    $targetPath = if ([System.IO.Path]::IsPathRooted($TargetDirectory)) {
        $TargetDirectory
    } else {
        Join-Path $workspaceRoot $TargetDirectory
    }
    $resolvedTarget = [System.IO.Path]::GetFullPath($targetPath)
    New-Item -ItemType Directory -Path $resolvedTarget -Force | Out-Null
    # RUSTFLAGS replaces .cargo/config.toml's flags. Preserve the FastISel
    # workaround while replacing /DEBUG:NONE with /DEBUG:FULL.
    $env:RUSTFLAGS = '-C llvm-args=-fast-isel=false -C link-arg=/DEBUG:FULL'
    $arguments = @('test', '--locked', '-j1', '--profile', 'diagnostic',
        '--target-dir', $resolvedTarget, '-p', $Package, '--lib')
    if ($TestFilter) { $arguments += $TestFilter }
    if ($NoRun) { $arguments += '--no-run' }
    else { $arguments += @('--', '--test-threads=1', '--nocapture') }
    [ordered]@{
        compiler = $compiler
        rustflags = $env:RUSTFLAGS
        command = @('cargo') + $arguments
        lock_sha256 = (Get-FileHash -LiteralPath Cargo.lock -Algorithm SHA256).Hash
        started_utc = [DateTime]::UtcNow.ToString('o')
    } | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $resolvedTarget 'build-context.json') -Encoding utf8
    & cargo @arguments
    $buildExitCode = $LASTEXITCODE
    if ($buildExitCode -ne 0) {
        throw "Diagnostic build/test exited with $buildExitCode; retain this target directory and the build context."
    }
}
finally {
    Remove-Item Env:RUSTFLAGS -ErrorAction SilentlyContinue
    Set-Location -LiteralPath $previousLocation.Path
}
