# Zaxiom Installation Script
# Builds, installs to AppData, and creates shortcuts

param(
    [switch]$Silent,
    [switch]$NoDesktop,
    [switch]$NoStartMenu,
    [switch]$NoBuild,
    [switch]$Uninstall
)

$ErrorActionPreference = "Stop"

# Configuration
$AppName = "Zaxiom"
$ExeName = "zaxiom.exe"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$BuildExe = Join-Path $ScriptDir "target\release\$ExeName"
$InstallDir = Join-Path $env:LOCALAPPDATA $AppName
$InstalledExe = Join-Path $InstallDir $ExeName
$IconSource = Join-Path $ScriptDir "assets\icon.ico"
$InstalledIcon = Join-Path $InstallDir "icon.ico"

function Write-Success { param($Message) Write-Host $Message -ForegroundColor Green }
function Write-Info { param($Message) Write-Host $Message -ForegroundColor Cyan }
function Write-Warn { param($Message) Write-Host $Message -ForegroundColor Yellow }
function Write-Err { param($Message) Write-Host $Message -ForegroundColor Red }

function Show-Banner {
    Write-Host ""
    Write-Host "  =======================================" -ForegroundColor Magenta
    Write-Host "         Zaxiom Installer" -ForegroundColor Magenta
    Write-Host "       Linux terminal for Windows" -ForegroundColor Magenta
    Write-Host "  =======================================" -ForegroundColor Magenta
    Write-Host ""
}

function Start-Uninstall {
    Show-Banner
    Write-Info "Uninstalling $AppName..."

    $StartMenuPath = Join-Path ([Environment]::GetFolderPath('Programs')) "$AppName.lnk"
    $DesktopPath = Join-Path ([Environment]::GetFolderPath('Desktop')) "$AppName.lnk"

    if (Test-Path $StartMenuPath) {
        Remove-Item $StartMenuPath -Force
        Write-Success "Removed Start Menu shortcut"
    }
    if (Test-Path $DesktopPath) {
        Remove-Item $DesktopPath -Force
        Write-Success "Removed Desktop shortcut"
    }
    if (Test-Path $InstallDir) {
        Remove-Item $InstallDir -Recurse -Force
        Write-Success "Removed installation directory"
    }

    Write-Host ""
    Write-Success "$AppName has been uninstalled."
}

function Test-Cargo {
    try {
        $null = Get-Command cargo -ErrorAction Stop
        return $true
    } catch {
        return $false
    }
}

function Build-Project {
    Write-Info "Building $AppName release..."
    Write-Host "  This may take a minute on first build..." -ForegroundColor Gray

    Push-Location $ScriptDir
    try {
        & cargo build --release
        if ($LASTEXITCODE -ne 0) {
            Write-Err "Build failed!"
            return $false
        }
        Write-Success "Build successful!"
        return $true
    } catch {
        Write-Err "Build error: $_"
        return $false
    } finally {
        Pop-Location
    }
}

function Install-Files {
    Write-Info "Installing to $InstallDir..."

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    Copy-Item $BuildExe $InstalledExe -Force
    Write-Success "Copied $ExeName"

    if (Test-Path $IconSource) {
        Copy-Item $IconSource $InstalledIcon -Force
        Write-Success "Copied icon.ico"
    }

    return $true
}

function New-Shortcut {
    param(
        [string]$ShortcutPath,
        [string]$TargetPath,
        [string]$IconPath,
        [string]$Description
    )

    try {
        $WshShell = New-Object -ComObject WScript.Shell
        $Shortcut = $WshShell.CreateShortcut($ShortcutPath)
        $Shortcut.TargetPath = $TargetPath
        $Shortcut.Description = $Description
        $Shortcut.WorkingDirectory = Split-Path -Parent $TargetPath

        if ($IconPath -and (Test-Path $IconPath)) {
            $Shortcut.IconLocation = "$IconPath,0"
        } else {
            $Shortcut.IconLocation = "$TargetPath,0"
        }

        $Shortcut.Save()
        [System.Runtime.Interopservices.Marshal]::ReleaseComObject($WshShell) | Out-Null
        return $true
    } catch {
        Write-Err "Failed to create shortcut: $_"
        return $false
    }
}

function Install-StartMenuShortcut {
    $StartMenuPath = [Environment]::GetFolderPath('Programs')
    $ShortcutPath = Join-Path $StartMenuPath "$AppName.lnk"

    Write-Info "Creating Start Menu shortcut..."
    if (New-Shortcut -ShortcutPath $ShortcutPath -TargetPath $InstalledExe -IconPath $InstalledIcon -Description "Launch $AppName") {
        Write-Success "Start Menu shortcut created"
        return $true
    }
    return $false
}

function Install-DesktopShortcut {
    $DesktopPath = [Environment]::GetFolderPath('Desktop')
    $ShortcutPath = Join-Path $DesktopPath "$AppName.lnk"

    Write-Info "Creating Desktop shortcut..."
    if (New-Shortcut -ShortcutPath $ShortcutPath -TargetPath $InstalledExe -IconPath $InstalledIcon -Description "Launch $AppName") {
        Write-Success "Desktop shortcut created"
        return $true
    }
    return $false
}

function Get-UserConfirmation {
    param(
        [string]$Prompt,
        [bool]$Default = $true
    )

    if ($Default) {
        $defaultText = "[Y/n]"
    } else {
        $defaultText = "[y/N]"
    }
    Write-Host "$Prompt $defaultText " -NoNewline -ForegroundColor Yellow
    $response = Read-Host

    if ([string]::IsNullOrWhiteSpace($response)) {
        return $Default
    }

    return $response -match '^[Yy]'
}

function Start-Installation {
    Show-Banner

    if ($Uninstall) {
        Start-Uninstall
        return
    }

    if (-not $NoBuild) {
        if (-not (Test-Cargo)) {
            Write-Err "Cargo not found! Please install Rust first:"
            Write-Host "  https://rustup.rs" -ForegroundColor Cyan
            Write-Host ""
            Write-Host "Or run with -NoBuild if you have a pre-built executable." -ForegroundColor Gray
            return
        }

        if (-not (Build-Project)) {
            Write-Err "Build failed. Cannot continue."
            return
        }
    } else {
        if (-not (Test-Path $BuildExe)) {
            Write-Err "Executable not found at: $BuildExe"
            Write-Warn "Run without -NoBuild to build first"
            return
        }
        Write-Info "Using existing build: $BuildExe"
    }

    if (-not (Install-Files)) {
        Write-Err "Installation failed."
        return
    }

    Write-Host ""

    $createStartMenu = $true
    $createDesktop = $true

    if (-not $Silent) {
        Write-Info "Shortcut Options:"
        Write-Host "-----------------" -ForegroundColor Gray
        if (-not $NoStartMenu) {
            $createStartMenu = Get-UserConfirmation "Create Start Menu shortcut?" -Default $true
        } else {
            $createStartMenu = $false
        }
        if (-not $NoDesktop) {
            $createDesktop = Get-UserConfirmation "Create Desktop shortcut?" -Default $true
        } else {
            $createDesktop = $false
        }
    } else {
        $createStartMenu = -not $NoStartMenu
        $createDesktop = -not $NoDesktop
    }

    Write-Host ""

    if ($createStartMenu) {
        Install-StartMenuShortcut | Out-Null
    }
    if ($createDesktop) {
        Install-DesktopShortcut | Out-Null
    }

    Write-Host ""
    Write-Host "=======================================" -ForegroundColor Green
    Write-Success "  Installation complete!"
    Write-Host "=======================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "  Installed to: $InstallDir" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  Launch from:" -ForegroundColor White
    if ($createStartMenu) {
        Write-Host "    - Start Menu (search Zaxiom)" -ForegroundColor Gray
    }
    if ($createDesktop) {
        Write-Host "    - Desktop shortcut" -ForegroundColor Gray
    }
    Write-Host ""
    Write-Host "  To uninstall: .\install.ps1 -Uninstall" -ForegroundColor Gray
    Write-Host ""

    if (-not $Silent) {
        if (Get-UserConfirmation "Launch $AppName now?" -Default $true) {
            Start-Process $InstalledExe
        }
    }
}

Start-Installation

if (-not $Silent) {
    Write-Host ""
    Write-Host "Press any key to exit..." -ForegroundColor Gray
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
}
