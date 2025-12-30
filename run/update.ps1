# Zaxiom Quick Update Script
# Builds and copies to install location - that's it!

$ErrorActionPreference = "Stop"

$AppName = "Zaxiom"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot = Split-Path -Parent $ScriptDir
$BuildExe = Join-Path $RepoRoot "target\release\zaxiom.exe"
$InstallDir = Join-Path $env:LOCALAPPDATA $AppName
$InstalledExe = Join-Path $InstallDir "zaxiom.exe"

Write-Host ""
Write-Host "  Updating $AppName..." -ForegroundColor Cyan
Write-Host ""

# Build
Write-Host "  [1/2] Building..." -ForegroundColor Yellow
Push-Location $RepoRoot
& cargo build --release
$exitCode = $LASTEXITCODE
Pop-Location

if ($exitCode -ne 0) {
    Write-Host "  Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "  Build OK" -ForegroundColor Green

# Copy
Write-Host "  [2/2] Copying to install..." -ForegroundColor Yellow
if (-not (Test-Path $InstallDir)) {
    Write-Host "  Install dir not found. Run install.ps1 first!" -ForegroundColor Red
    exit 1
}

# Kill running instance if any
Get-Process -Name "zaxiom" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Milliseconds 500

Copy-Item $BuildExe $InstalledExe -Force
Write-Host "  Copied OK" -ForegroundColor Green

Write-Host ""
Write-Host "  Updated! Launch from Start Menu or Desktop." -ForegroundColor Green
Write-Host ""
