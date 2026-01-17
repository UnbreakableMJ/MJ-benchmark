# MJ Benchmark Stack Installer (Windows)
# Installs: Chocolatey, PHP, Python, Git, Playwright, Google API libs, PTS

$ErrorActionPreference = "Stop"

Write-Host "=== MJ Benchmark Stack Installer (Windows) ==="

# --- 1) Install Chocolatey if missing ---
if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Chocolatey..."
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12
    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
} else {
    Write-Host "Chocolatey already installed."
}

# --- 2) Install core dependencies ---
Write-Host "Installing PHP, Python, Git..."
choco install -y php python git

# Ensure PATH is updated
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine")

# --- 3) Install Google API Python libraries ---
Write-Host "Installing Google API Python libraries..."
pip install google-api-python-client google-auth-httplib2 google-auth-oauthlib

# --- 4) Install Playwright + browsers ---
Write-Host "Installing Playwright..."
pip install playwright
python -m playwright install

# --- 5) Install Phoronix Test Suite ---
Write-Host "Downloading Phoronix Test Suite..."
$ptsZip = "$env:TEMP\pts.zip"
Invoke-WebRequest -Uri "https://phoronix-test-suite.com/releases/phoronix-test-suite-10.8.4.zip" -OutFile $ptsZip

$ptsDir = "C:\pts"
if (-not (Test-Path $ptsDir)) { New-Item -ItemType Directory -Path $ptsDir | Out-Null }

Expand-Archive $ptsZip -DestinationPath $ptsDir -Force
Set-Location $ptsDir
.\install.bat

# --- 6) Create directory for Google OAuth credentials ---
$credDir = "$env:USERPROFILE\MJ_bench_sync"
if (-not (Test-Path $credDir)) {
    New-Item -ItemType Directory -Path $credDir | Out-Null
}

Write-Host ""
Write-Host "=== INSTALLATION COMPLETE ==="
Write-Host ""
Write-Host "Next steps:"
Write-Host "1. Download your Google OAuth credentials.json"
Write-Host "2. Place it here: $credDir\credentials.json"
Write-Host ""
Write-Host "You can now run:"
Write-Host "  run_bench.ps1"
Write-Host ""
Write-Host "Everything is installed and ready."