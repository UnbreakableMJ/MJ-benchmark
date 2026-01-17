# Install MJ's benchmark stack on Windows (Phoronix Test Suite + PHP + Python basics)

$ErrorActionPreference = "Stop"

# 1) Install Chocolatey if missing
if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Chocolatey..."
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12
    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
}

# 2) Install PHP, Python, Git (for general tooling)
choco install -y php python git

# 3) Install Phoronix Test Suite
Write-Host "Downloading Phoronix Test Suite..."
$ptsZip = "$env:TEMP\pts.zip"
Invoke-WebRequest -Uri "https://phoronix-test-suite.com/releases/phoronix-test-suite-10.8.4.zip" -OutFile $ptsZip

$ptsDir = "C:\pts"
if (-not (Test-Path $ptsDir)) { New-Item -ItemType Directory -Path $ptsDir | Out-Null }

Expand-Archive $ptsZip -DestinationPath $ptsDir -Force
Set-Location $ptsDir
.\install.bat

# 4) Python Playwright for browser benchmarks
pip install playwright
python -m playwright install

Write-Host "✅ MJ benchmark stack installation complete on Windows."