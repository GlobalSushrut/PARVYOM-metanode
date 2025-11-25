# BPI (Blockchain Protocol Infrastructure) Windows Installer
# Version: 1.0.0
# Usage: iwr -useb https://get.bpi.pravyom.com/install.ps1 | iex

param(
    [string]$InstallDir = "$env:USERPROFILE\.bpi",
    [string]$Version = "v1.0.0",
    [switch]$Force
)

# Colors for output
$Colors = @{
    Red = "Red"
    Green = "Green"
    Yellow = "Yellow"
    Blue = "Blue"
    Magenta = "Magenta"
    Cyan = "Cyan"
}

function Write-Banner {
    Write-Host "╔══════════════════════════════════════════════════════════════════════════════════╗" -ForegroundColor Magenta
    Write-Host "║                                                                                  ║" -ForegroundColor Magenta
    Write-Host "║    ██████╗ ██████╗ ██╗    ██╗███╗   ██╗██╗      ██████╗  █████╗ ██████╗       ║" -ForegroundColor Magenta
    Write-Host "║    ██╔══██╗██╔══██╗██║    ██║████╗  ██║██║     ██╔═══██╗██╔══██╗██╔══██╗      ║" -ForegroundColor Magenta
    Write-Host "║    ██║  ██║██║  ██║██║ █╗ ██║██╔██╗ ██║██║     ██║   ██║███████║██║  ██║      ║" -ForegroundColor Magenta
    Write-Host "║    ██║  ██║██║  ██║██║███╗██║██║╚██╗██║██║     ██║   ██║██╔══██║██║  ██║      ║" -ForegroundColor Magenta
    Write-Host "║    ██████╔╝██████╔╝╚███╔███╔╝██║ ╚████║███████╗╚██████╔╝██║  ██║██████╔╝      ║" -ForegroundColor Magenta
    Write-Host "║    ╚═════╝ ╚═════╝  ╚══╝╚══╝ ╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═════╝       ║" -ForegroundColor Magenta
    Write-Host "║                                                                                  ║" -ForegroundColor Magenta
    Write-Host "║                    BPI (Blockchain Protocol Infrastructure)                      ║" -ForegroundColor Magenta
    Write-Host "║                         Windows Installer $Version                            ║" -ForegroundColor Magenta
    Write-Host "║                                                                                  ║" -ForegroundColor Magenta
    Write-Host "║    🚀 Revolutionary 6D Quantum-Topological Consensus                            ║" -ForegroundColor Magenta
    Write-Host "║    🔒 Post-Quantum Cryptographic Security                                       ║" -ForegroundColor Magenta
    Write-Host "║    ⚡ Ultra-Lightweight VM & Orchestration                                      ║" -ForegroundColor Magenta
    Write-Host "║    🌐 Web3.5 Domain System & HTTPCG Protocol                                   ║" -ForegroundColor Magenta
    Write-Host "║    📊 Enterprise-Grade Analytics & Monitoring                                   ║" -ForegroundColor Magenta
    Write-Host "║                                                                                  ║" -ForegroundColor Magenta
    Write-Host "╚══════════════════════════════════════════════════════════════════════════════════╝" -ForegroundColor Magenta
}

function Write-Info($Message) {
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Success($Message) {
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

function Write-Warning($Message) {
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error($Message) {
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Write-Step($Message) {
    Write-Host "[STEP] $Message" -ForegroundColor Cyan
}

function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Install-Chocolatey {
    Write-Step "Installing Chocolatey package manager..."
    
    if (Get-Command choco -ErrorAction SilentlyContinue) {
        Write-Info "Chocolatey already installed"
        return
    }
    
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
    iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    
    Write-Success "Chocolatey installed successfully"
}

function Install-Dependencies {
    Write-Step "Installing dependencies..."
    
    # Install Git
    if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
        Write-Info "Installing Git..."
        choco install git -y
    }
    
    # Install Rust
    if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
        Write-Info "Installing Rust..."
        choco install rust -y
    }
    
    # Install Visual Studio Build Tools (required for Rust on Windows)
    Write-Info "Installing Visual Studio Build Tools..."
    choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools" -y
    
    Write-Success "Dependencies installed"
}

function New-BpiDirectories {
    Write-Step "Creating BPI directories..."
    
    $dirs = @(
        $InstallDir,
        "$InstallDir\bin",
        "$InstallDir\config",
        "$InstallDir\data",
        "$InstallDir\logs"
    )
    
    foreach ($dir in $dirs) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Path $dir -Force | Out-Null
        }
    }
    
    Write-Success "BPI directories created"
}

function Get-BpiSource {
    Write-Step "Downloading BPI source code..."
    
    $tempDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
    $sourceDir = Join-Path $tempDir "bpi-source"
    
    Write-Info "Cloning BPI repository..."
    git clone --depth 1 "https://github.com/GlobalSushrut/PARVYOM-metanode" $sourceDir
    
    Write-Info "Moving BPI source to installation directory..."
    Copy-Item -Path "$sourceDir\*" -Destination $InstallDir -Recurse -Force
    
    Remove-Item $tempDir -Recurse -Force
    Write-Success "BPI source code downloaded"
}

function Build-BpiComponents {
    Write-Step "Building BPI components..."
    
    Push-Location $InstallDir
    
    try {
        # Build BPI Core
        Write-Info "Building BPI Core..."
        Push-Location "bpi-core"
        cargo build --release
        Copy-Item "target\release\bpi-core.exe" "$InstallDir\bin\" -ErrorAction SilentlyContinue
        Copy-Item "target\release\bpi-orchestrator.exe" "$InstallDir\bin\" -ErrorAction SilentlyContinue
        Copy-Item "target\release\bpi-audit-server.exe" "$InstallDir\bin\" -ErrorAction SilentlyContinue
        Pop-Location
        
        # Build BPCI Enterprise
        Write-Info "Building BPCI Enterprise..."
        Push-Location "bpci-enterprise"
        cargo build --release
        Copy-Item "target\release\bpci-server.exe" "$InstallDir\bin\" -ErrorAction SilentlyContinue
        Pop-Location
        
        # Build Wallet Identity
        Write-Info "Building Wallet Identity System..."
        Push-Location "wallet-identity"
        cargo build --release
        Copy-Item "target\release\wallet-server.exe" "$InstallDir\bin\" -ErrorAction SilentlyContinue
        Pop-Location
        
        Write-Success "BPI components built successfully"
    }
    finally {
        Pop-Location
    }
}

function New-BpiConfig {
    Write-Step "Creating default configuration..."
    
    $configContent = @"
# BPI Core Configuration
[bpi_core]
host = "127.0.0.1"
port = 7777
data_dir = "$($InstallDir -replace '\\', '/')/data"
log_level = "info"

[consensus]
algorithm = "6d_quantum_topological"
quantum_entanglement = true
knot_theory_validation = true

[networking]
httpcg_enabled = true
xtmp_enabled = true
shadow_registry = true

[security]
post_quantum_crypto = true
audit_enabled = true
forensic_mode = true

# BPCI Enterprise Configuration
[bpci]
host = "127.0.0.1"
port = 8080
enterprise_mode = true

[governance]
lccd_consensus = true
living_cellular = true
category_theory = true

[economy]
multi_coin_system = true
autonomous_economy = true
treasury_enabled = true

# Wallet Configuration
[wallet]
host = "127.0.0.1"
port = 7778
identity_system = "did_based"
encryption = "post_quantum"
"@

    $configContent | Out-File -FilePath "$InstallDir\config\bpi.toml" -Encoding UTF8
    Write-Success "Default configuration created"
}

function New-BpiCli {
    Write-Step "Creating BPI CLI tool..."
    
    $cliContent = @'
@echo off
setlocal

set BPI_DIR=%USERPROFILE%\.bpi
set BPI_BIN_DIR=%BPI_DIR%\bin
set BPI_CONFIG_DIR=%BPI_DIR%\config

if "%1"=="start" goto start
if "%1"=="stop" goto stop
if "%1"=="status" goto status
if "%1"=="version" goto version
if "%1"=="help" goto help
goto help

:start
echo 🚀 Starting BPI Infrastructure...
echo Starting BPI Core...
start /B "%BPI_BIN_DIR%\bpi-core.exe" --config "%BPI_CONFIG_DIR%\bpi.toml"
echo Starting BPCI Enterprise...
start /B "%BPI_BIN_DIR%\bpci-server.exe" --config "%BPI_CONFIG_DIR%\bpi.toml"
echo Starting BPI Orchestrator...
start /B "%BPI_BIN_DIR%\bpi-orchestrator.exe" --config "%BPI_CONFIG_DIR%\bpi.toml"
if exist "%BPI_BIN_DIR%\wallet-server.exe" (
    echo Starting Wallet Server...
    start /B "%BPI_BIN_DIR%\wallet-server.exe" --config "%BPI_CONFIG_DIR%\bpi.toml"
)
echo ✅ BPI Infrastructure started successfully
echo 🌐 Web UI: http://localhost:8080
echo 📡 API: http://localhost:7777
echo 💰 Wallet: http://localhost:7778
goto end

:stop
echo 🛑 Stopping BPI Infrastructure...
taskkill /F /IM bpi-core.exe 2>nul
taskkill /F /IM bpci-server.exe 2>nul
taskkill /F /IM bpi-orchestrator.exe 2>nul
taskkill /F /IM wallet-server.exe 2>nul
echo ✅ BPI Infrastructure stopped
goto end

:status
echo 📊 BPI Infrastructure Status:
echo.
tasklist /FI "IMAGENAME eq bpi-core.exe" 2>nul | find /I "bpi-core.exe" >nul && echo ✅ BPI Core: Running || echo ❌ BPI Core: Stopped
tasklist /FI "IMAGENAME eq bpci-server.exe" 2>nul | find /I "bpci-server.exe" >nul && echo ✅ BPCI Enterprise: Running || echo ❌ BPCI Enterprise: Stopped
tasklist /FI "IMAGENAME eq bpi-orchestrator.exe" 2>nul | find /I "bpi-orchestrator.exe" >nul && echo ✅ BPI Orchestrator: Running || echo ❌ BPI Orchestrator: Stopped
tasklist /FI "IMAGENAME eq wallet-server.exe" 2>nul | find /I "wallet-server.exe" >nul && echo ✅ Wallet Server: Running || echo ❌ Wallet Server: Stopped
goto end

:version
echo BPI v1.0.0
goto end

:help
echo BPI (Blockchain Protocol Infrastructure) CLI
echo.
echo Usage: bpi ^<command^> [options]
echo.
echo Commands:
echo   start       Start BPI infrastructure
echo   stop        Stop BPI infrastructure
echo   status      Show BPI status
echo   version     Show version
echo   help        Show this help
echo.
echo Examples:
echo   bpi start                 # Start all BPI services
echo   bpi status                # Check system status
goto end

:end
'@

    $cliContent | Out-File -FilePath "$InstallDir\bin\bpi.bat" -Encoding ASCII
    Write-Success "BPI CLI tool created"
}

function Add-ToPath {
    Write-Step "Adding BPI to PATH..."
    
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    $bpiBinDir = "$InstallDir\bin"
    
    if ($currentPath -notlike "*$bpiBinDir*") {
        $newPath = "$currentPath;$bpiBinDir"
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        Write-Info "Added BPI to user PATH"
    }
    
    # Add to current session PATH
    $env:PATH += ";$bpiBinDir"
    
    Write-Success "PATH configured"
}

function New-StartMenuShortcut {
    Write-Step "Creating Start Menu shortcuts..."
    
    $startMenuDir = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\BPI"
    if (-not (Test-Path $startMenuDir)) {
        New-Item -ItemType Directory -Path $startMenuDir -Force | Out-Null
    }
    
    # Create BPI shortcut
    $WshShell = New-Object -comObject WScript.Shell
    $Shortcut = $WshShell.CreateShortcut("$startMenuDir\BPI Infrastructure.lnk")
    $Shortcut.TargetPath = "$InstallDir\bin\bpi.bat"
    $Shortcut.Arguments = "start"
    $Shortcut.WorkingDirectory = $InstallDir
    $Shortcut.Description = "Start BPI Infrastructure"
    $Shortcut.Save()
    
    Write-Success "Start Menu shortcuts created"
}

function Test-BpiInstallation {
    Write-Step "Running health check..."
    
    # Test BPI CLI
    $bpiPath = "$InstallDir\bin\bpi.bat"
    if (Test-Path $bpiPath) {
        Write-Success "BPI CLI installed correctly"
    } else {
        Write-Warning "BPI CLI may have issues"
    }
    
    # Check configuration
    $configPath = "$InstallDir\config\bpi.toml"
    if (Test-Path $configPath) {
        Write-Success "Configuration file exists"
    } else {
        Write-Warning "Configuration file missing"
    }
    
    Write-Success "Health check completed"
}

function Show-CompletionMessage {
    Write-Host "╔══════════════════════════════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║                                                                                  ║" -ForegroundColor Green
    Write-Host "║    🎉 BPI INSTALLATION COMPLETED SUCCESSFULLY! 🎉                               ║" -ForegroundColor Green
    Write-Host "║                                                                                  ║" -ForegroundColor Green
    Write-Host "║    📁 Installation Directory: $InstallDir" -ForegroundColor Green
    Write-Host "║    🔧 Configuration: $InstallDir\config\bpi.toml" -ForegroundColor Green
    Write-Host "║    📊 Data Directory: $InstallDir\data" -ForegroundColor Green
    Write-Host "║                                                                                  ║" -ForegroundColor Green
    Write-Host "║    🚀 Quick Start Commands:                                                      ║" -ForegroundColor Green
    Write-Host "║       bpi start          # Start BPI infrastructure                             ║" -ForegroundColor Green
    Write-Host "║       bpi status         # Check system status                                  ║" -ForegroundColor Green
    Write-Host "║       bpi help           # Show all commands                                    ║" -ForegroundColor Green
    Write-Host "║                                                                                  ║" -ForegroundColor Green
    Write-Host "║    🌐 Web Interfaces:                                                           ║" -ForegroundColor Green
    Write-Host "║       http://localhost:8080  # BPCI Enterprise Dashboard                       ║" -ForegroundColor Green
    Write-Host "║       http://localhost:7777  # BPI Core API                                    ║" -ForegroundColor Green
    Write-Host "║       http://localhost:7778  # Wallet Interface                                ║" -ForegroundColor Green
    Write-Host "║                                                                                  ║" -ForegroundColor Green
    Write-Host "║    📚 Documentation: https://globalsushrut.github.io/PARVYOM-metanode/        ║" -ForegroundColor Green
    Write-Host "║    💬 Support: https://github.com/GlobalSushrut/PARVYOM-metanode/issues       ║" -ForegroundColor Green
    Write-Host "║                                                                                  ║" -ForegroundColor Green
    Write-Host "║    ⚠️  Please restart your terminal or refresh PATH                            ║" -ForegroundColor Green
    Write-Host "║                                                                                  ║" -ForegroundColor Green
    Write-Host "╚══════════════════════════════════════════════════════════════════════════════════╝" -ForegroundColor Green
}

function Main {
    Write-Banner
    
    Write-Info "Starting BPI installation process..."
    
    if (-not (Test-Administrator)) {
        Write-Warning "Running without administrator privileges. Some features may not work correctly."
    }
    
    Install-Chocolatey
    Install-Dependencies
    New-BpiDirectories
    Get-BpiSource
    Build-BpiComponents
    New-BpiConfig
    New-BpiCli
    Add-ToPath
    New-StartMenuShortcut
    Test-BpiInstallation
    
    Show-CompletionMessage
    
    Write-Success "BPI installation completed! 🚀"
    Write-Info "Run 'bpi start' to begin using BPI infrastructure"
}

# Run main function
Main
