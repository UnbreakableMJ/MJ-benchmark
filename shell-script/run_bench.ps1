# MJ Unified Benchmark Runner (Windows PowerShell)
# Collects device specs + benchmarks → merges → appends → syncs to Google Drive + Google Sheets

$ErrorActionPreference = "Stop"

# --- Paths ---
$HomeDir = $env:USERPROFILE
$CsvFile = "$HomeDir\MJ_benchmarks.csv"
$SpecsTmp = "$HomeDir\.device_specs_tmp.csv"
$BenchTmp = "$HomeDir\.bench_results_tmp.csv"
$SyncScript = "$HomeDir\sync_to_google.py"

# --- Google IDs (fill these in) ---
$GoogleSheetID = "PUT_YOUR_SHEET_ID_HERE"
$GoogleDriveFolderID = "PUT_YOUR_DRIVE_FOLDER_ID_HERE"

Write-Host "Collecting device specs..."
powershell -ExecutionPolicy Bypass -File "$HomeDir\device_specs_to_csv.ps1" --output $SpecsTmp

Write-Host "Running benchmark suite..."
python "$HomeDir\MJ_pipeline.py" --output $BenchTmp

# --- Create master CSV if missing ---
if (-not (Test-Path $CsvFile)) {
    Write-Host "Creating master CSV..."
    $header = "Brand & Model,Launch Date,Price,CPU & Performance,Codename,CPU Speed,x86-64 Level,GPU,AI & NPU,RAM & Storage,Connectivity,Audio Ports,NFC & Wallet,Battery,Power & Charging,Qi Wireless Charging,Form Factor,Dimensions & Weight,Display,Build & Durability,Cameras,Biometrics & Health,Regional,Software & Updates,Color,Upgrade Options,Ecosystem Lock-in,Wear Detection,Touch Control,Storage Case,Special Features,Official Site,Info Links,BIOS/Boot Key,7-Zip MIPS,OpenSSL MB/s,RAMspeed MB/s,fio Seq Read MB/s,fio Seq Write MB/s,fio Rand Read IOPS,fio Rand Write IOPS,glmark2 Score,Kernel Build Time (s),Speedometer 2.1 Score,JetStream 2.2 Score,MotionMark 1.3 Score,Battery Full Capacity (Wh),Battery Design Capacity (Wh),Battery Health (%),Battery Cycle Count,Notes"
    Set-Content -Path $CsvFile -Value $header
}

# --- Merge rows ---
Write-Host "Merging results..."
$specs = Get-Content $SpecsTmp
$bench = Get-Content $BenchTmp
$row = "$specs,$bench"

Add-Content -Path $CsvFile -Value $row

# --- Cleanup temp files ---
Remove-Item $SpecsTmp -Force
Remove-Item $BenchTmp -Force

# --- Sync to Google Drive + Sheets ---
Write-Host "Syncing to Google Drive + Google Sheets..."
python $SyncScript `
    --csv $CsvFile `
    --sheet $GoogleSheetID `
    --drive-folder $GoogleDriveFolderID

Write-Host "✅ Benchmark synced to cloud."