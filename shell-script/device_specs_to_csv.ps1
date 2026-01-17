# MJ Benchmark Suite Logger
$CSVFile = "$env:USERPROFILE\MJ_benchmarks.csv"

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

# Benchmark results (fill manually or parse from PTS export)
$SevenZip = "(fill)"
$OpenSSL = "(fill)"
$RAMSpeed = "(fill)"
$FioSeqRead = "(fill)"
$FioSeqWrite = "(fill)"
$FioRandRead = "(fill)"
$FioRandWrite = "(fill)"
$Glmark2 = "(fill)"
$KernelBuild = "(fill)"
$Speedometer = "(fill)"
$JetStream = "(fill)"
$MotionMark = "(fill)"
$Notes = "(fill)"

# Append row
"$Date,$Computer,$CPU,$GPU,$NPU,$RAM,$Storage,$CompFlags,$Distro,$Shell,$DE,$RepoLevel,$SevenZip,$OpenSSL,$RAMSpeed,$FioSeqRead,$FioSeqWrite,$FioRandRead,$FioRandWrite,$Glmark2,$KernelBuild,$Speedometer,$JetStream,$MotionMark,$Notes" | Add-Content $CSVFile

Write-Output "✅ Benchmark metadata appended to $CSVFile"
