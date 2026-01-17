# MJ Benchmark Suite Auto-Logger (Windows)
$CSVFile = "$env:USERPROFILE\MJ_benchmarks.csv"
$RunName = $args[0]
$ResultDir = "$env:USERPROFILE\.phoronix-test-suite\test-results\$RunName"
$ResultJson = "$ResultDir\results.json"

# Ensure headers exist
if (!(Test-Path $CSVFile)) {
  "Date,Computer,CPU,GPU,NPU,RAM,Storage,Compilation Flags,Distro,Shell,DE,Repo Level,7-Zip MIPS,OpenSSL MB/s,RAMspeed MB/s,fio Seq Read MB/s,fio Seq Write MB/s,fio Rand Read IOPS,fio Rand Write IOPS,glmark2 Score,Kernel Build Time (s),Speedometer 2.1 Score,JetStream 2.2 Score,MotionMark 1.3 Score,Notes" | Out-File $CSVFile
}

# Metadata
$Date = Get-Date -Format "yyyy/MM/dd HH:mm"
$Computer = $env:COMPUTERNAME
$CPU = (Get-CimInstance Win32_Processor).Name
$GPU = (Get-CimInstance Win32_VideoController).Name
$RAM = "{0} GB" -f ([math]::Round((Get-CimInstance Win32_PhysicalMemory | Measure-Object -Property Capacity -Sum).Sum / 1GB))
$Storage = (Get-CimInstance Win32_DiskDrive | Select-Object -First 1).Model
$Distro = "Windows $(Get-ComputerInfo | Select-Object -ExpandProperty WindowsVersion)"
$Shell = "PowerShell"
$DE = "Explorer"

# Manual fields
$NPU = "None"
$CompFlags = "(fill manually)"
$RepoLevel = "(fill manually)"

# Parse PTS JSON (requires jq for Windows or PowerShell JSON parsing)
$Results = Get-Content $ResultJson | ConvertFrom-Json
$SevenZip = ($Results.Results | Where-Object {$_.Identifier -eq "pts/compress-7zip"}).Result
$OpenSSL = ($Results.Results | Where-Object {$_.Identifier -eq "pts/openssl"}).Result
$RAMSpeed = ($Results.Results | Where-Object {$_.Identifier -eq "pts/ramspeed"}).Result
$FioSeqRead = ($Results.Results | Where-Object {$_.Identifier -eq "pts/fio"}).Result
$Glmark2 = ($Results.Results | Where-Object {$_.Identifier -eq "pts/glmark2"}).Result
$KernelBuild = ($Results.Results | Where-Object {$_.Identifier -eq "pts/build-linux-kernel"}).Result

# Browser tests (manual entry for now)
$Speedometer = "(fill)"
$JetStream = "(fill)"
$MotionMark = "(fill)"
$Notes = "(fill)"

# Append row
"$Date,$Computer,$CPU,$GPU,$NPU,$RAM,$Storage,$CompFlags,$Distro,$Shell,$DE,$RepoLevel,$SevenZip,$OpenSSL,$RAMSpeed,$FioSeqRead,,,,$Glmark2,$KernelBuild,$Speedometer,$JetStream,$MotionMark,$Notes" | Add-Content $CSVFile

Write-Output "✅ Benchmark metadata + PTS results appended to $CSVFile"
