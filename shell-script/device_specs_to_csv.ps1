param(
    [Parameter(Mandatory=$true)]
    [string]$output
)

# Helper: safe getter
function Get-Safe($script) {
    try {
        $value = Invoke-Expression $script
        if ($null -eq $value -or $value -eq "") { return "" }
        return $value.ToString().Trim()
    } catch {
        return ""
    }
}

# --- Auto-detected fields ---

# Brand & Model
$brand  = Get-Safe "(Get-CimInstance Win32_ComputerSystem).Manufacturer"
$model  = Get-Safe "(Get-CimInstance Win32_ComputerSystem).Model"
$brandModel = "$brand $model".Trim()

# CPU
$cpu = Get-Safe "(Get-CimInstance Win32_Processor).Name"
$cpuSpeed = Get-Safe "(Get-CimInstance Win32_Processor).MaxClockSpeed"
if ($cpuSpeed -ne "") { $cpuSpeed = [math]::Round($cpuSpeed/1000,2).ToString() + ' GHz' }

# Codename (not exposed on Windows)
$codename = ""

# x86-64 level (not exposed)
$x86 = ""

# GPU
$gpu = Get-Safe "(Get-CimInstance Win32_VideoController | Select-Object -First 1).Name"

# AI/NPU (Windows does not expose this)
$ai = ""

# RAM
$ramGB = Get-Safe "([math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB,2)).ToString() + ' GB'"

# Storage
$storage = (Get-CimInstance Win32_DiskDrive | ForEach-Object {
    $sizeGB = [math]::Round($_.Size / 1GB,2)
    "$($_.Model) $sizeGB GB"
}) -join "; "

# Connectivity
$wifi = Get-Safe "(Get-NetAdapter | Where-Object {$_.Status -eq 'Up'} | Select-Object -First 1).Name"
$connectivity = $wifi

# Audio ports
$audio = Get-Safe "(Get-CimInstance Win32_SoundDevice | Select-Object -First 1).Name"

# Battery info (Windows)
$battery = Get-CimInstance Win32_Battery -ErrorAction SilentlyContinue
if ($battery) {
    $batFull   = ""   # Windows does not expose full capacity directly
    $batDesign = ""   # Windows does not expose design capacity directly
    $batHealth = ""   # Cannot compute without design capacity
    $batCycles = ""   # Windows does not expose cycle count
    $batteryRaw = "Battery Present"
} else {
    $batteryRaw = ""
    $batFull = ""
    $batDesign = ""
    $batHealth = ""
    $batCycles = ""
}

# Power supply
$power = Get-Safe "(Get-CimInstance Win32_Battery).BatteryStatus"

# Qi wireless charging (not detectable)
$qi = ""

# Form factor
$form = Get-Safe "(Get-CimInstance Win32_SystemEnclosure).ChassisTypes"
if ($form -is [array]) { $form = $form[0] }

# Dimensions & Weight (not exposed)
$dimensions = ""

# Display
$display = Get-Safe "(Get-CimInstance Win32_VideoController | Select-Object -First 1).VideoModeDescription"

# Build & Durability
$build = $brand

# Cameras
$cameras = (Get-CimInstance Win32_PnPEntity | Where-Object {$_.Name -like '*Camera*'} | Select-Object -ExpandProperty Name) -join "; "

# Biometrics
$biometrics = (Get-CimInstance Win32_PnPEntity | Where-Object {$_.Name -like '*Fingerprint*'} | Select-Object -ExpandProperty Name) -join "; "

# Regional
$regional = (Get-WinSystemLocale).Name

# Software & Updates
$os = (Get-CimInstance Win32_OperatingSystem).Caption
$kernel = (Get-CimInstance Win32_OperatingSystem).Version
$software = "$os ($kernel)"

# Color (not detectable)
$color = ""

# Upgrade options (not detectable)
$upgrade = ""

# Ecosystem lock-in (not detectable)
$ecosystem = ""

# Wear detection (not applicable)
$wear = ""

# Touch control (detect touch screen)
$touch = Get-Safe "(Get-CimInstance Win32_PnPEntity | Where-Object {$_.Name -like '*Touch*'} | Select-Object -First 1).Name"

# Storage case (not applicable)
$storageCase = ""

# Special features
$special = ""

# Official site
$official = ""

# Info links
$links = ""

# BIOS key (not detectable)
$biosKey = ""

# --- Compose CSV row in EXACT schema order ---
$row = @(
    $brandModel,
    "", # Launch Date
    "", # Price
    $cpu,
    $codename,
    $cpuSpeed,
    $x86,
    $gpu,
    $ai,
    "$ramGB / $storage",
    $connectivity,
    $audio,
    "", # NFC & Wallet
    $batteryRaw,
    $power,
    $qi,
    $form,
    $dimensions,
    $display,
    $build,
    $cameras,
    $biometrics,
    $regional,
    $software,
    $color,
    $upgrade,
    $ecosystem,
    $wear,
    $touch,
    $storageCase,
    $special,
    $official,
    $links,
    $biosKey
) -join ","

# Write output
Set-Content -Path $output -Value $row

Write-Host "Device specs written to $output"