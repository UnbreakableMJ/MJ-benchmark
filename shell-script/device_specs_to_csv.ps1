# Collect system metadata for Mohamed Benchmark Suite

$Date = Get-Date -Format "yyyy/MM/dd HH:mm"
$Computer = $env:COMPUTERNAME
$CPU = (Get-CimInstance Win32_Processor).Name
$GPU = (Get-CimInstance Win32_VideoController).Name
$RAM = "{0} GB" -f ([math]::Round((Get-CimInstance Win32_PhysicalMemory | Measure-Object -Property Capacity -Sum).Sum / 1GB))
$Storage = (Get-CimInstance Win32_DiskDrive | Select-Object -First 1).Model
$Distro = "Windows $(Get-ComputerInfo | Select-Object -ExpandProperty WindowsVersion)"
$Shell = "PowerShell"
$DE = "Explorer"
$NPU = "None"
$CompFlags = "(fill manually)"
$RepoLevel = "(fill manually)"

Write-Output "Date: $Date"
Write-Output "Computer: $Computer"
Write-Output "CPU: $CPU"
Write-Output "GPU: $GPU"
Write-Output "NPU: $NPU"
Write-Output "RAM: $RAM"
Write-Output "Storage: $Storage"
Write-Output "Compilation Flags: $CompFlags"
Write-Output "Distro: $Distro"
Write-Output "Shell: $Shell"
Write-Output "DE: $DE"
Write-Output "Repo Level: $RepoLevel"
