use std::process::Command;

use crate::model::DeviceSpecs;

fn run(cmd: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(cmd).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

fn run_powershell(script: &str) -> Option<String> {
    let out = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(script)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

pub fn collect_windows_specs() -> DeviceSpecs {
    let brand_model = detect_brand_model();
    let cpu = detect_cpu();
    let cpu_speed = detect_cpu_speed();
    let x86_level = detect_arch();
    let ram_storage = format!("{} / {}", detect_ram(), detect_storage());
    let gpu = detect_gpu();
    let connectivity = detect_connectivity();
    let audio_ports = detect_audio();
    let display = detect_display();
    let battery = detect_battery();
    let power_charging = detect_power();
    let cameras = detect_cameras();
    let biometrics_health = detect_biometrics();
    let regional = detect_locale();
    let software_updates = detect_os_version();

    DeviceSpecs {
        brand_model,
        launch_date: "".into(),
        price: "".into(),
        cpu,
        codename: "".into(),
        cpu_speed,
        x86_level,
        gpu,
        ai_npu: "".into(),
        ram_storage,
        connectivity,
        audio_ports,
        nfc_wallet: "".into(),
        battery,
        power_charging,
        qi_charging: "".into(),
        form_factor: detect_form_factor(),
        dimensions_weight: "".into(),
        display,
        build_durability: "".into(),
        cameras,
        biometrics_health,
        regional,
        software_updates,
        color: "".into(),
        upgrade_options: "".into(),
        ecosystem_lock_in: "Windows ecosystem".into(),
        wear_detection: "".into(),
        touch_control: "".into(),
        storage_case: "".into(),
        special_features: "".into(),
        official_site: "".into(),
        info_links: "".into(),
        bios_boot_key: "F2/F10/Del (varies by vendor)".into(),
    }
}

fn detect_brand_model() -> String {
    if let Some(out) = run(
        "wmic",
        &["computersystem", "get", "manufacturer,model", "/format:list"],
    ) {
        let mut vendor = "Unknown".to_string();
        let mut model = "Unknown".to_string();
        for line in out.lines() {
            let l = line.trim();
            if l.starts_with("Manufacturer=") {
                vendor = l["Manufacturer=".len()..].trim().to_string();
            } else if l.starts_with("Model=") {
                model = l["Model=".len()..].trim().to_string();
            }
        }
        return format!("{} {}", vendor, model);
    }
    "Unknown Windows Machine".into()
}

fn detect_cpu() -> String {
    if let Some(out) = run("wmic", &["cpu", "get", "Name", "/format:list"]) {
        for line in out.lines() {
            let l = line.trim();
            if l.starts_with("Name=") {
                return l["Name=".len()..].trim().to_string();
            }
        }
    }
    "Unknown CPU".into()
}

fn detect_cpu_speed() -> String {
    if let Some(out) = run("wmic", &["cpu", "get", "MaxClockSpeed", "/format:list"]) {
        for line in out.lines() {
            let l = line.trim();
            if l.starts_with("MaxClockSpeed=") {
                if let Ok(mhz) = l["MaxClockSpeed=".len()..].trim().parse::<f64>() {
                    return format!("{:.2} GHz", mhz / 1000.0);
                }
            }
        }
    }
    "".into()
}

fn detect_arch() -> String {
    if let Some(out) = run("wmic", &["os", "get", "OSArchitecture", "/format:list"]) {
        for line in out.lines() {
            let l = line.trim();
            if l.starts_with("OSArchitecture=") {
                return l["OSArchitecture=".len()..].trim().to_string();
            }
        }
    }
    "".into()
}

fn detect_ram() -> String {
    if let Some(out) = run("wmic", &["ComputerSystem", "get", "TotalPhysicalMemory", "/format:list"])
    {
        for line in out.lines() {
            let l = line.trim();
            if l.starts_with("TotalPhysicalMemory=") {
                if let Ok(bytes) = l["TotalPhysicalMemory=".len()..].trim().parse::<f64>() {
                    return format!("{:.1} GB RAM", bytes / 1024.0 / 1024.0 / 1024.0);
                }
            }
        }
    }
    "Unknown RAM".into()
}

fn detect_storage() -> String {
    if let Some(out) = run(
        "wmic",
        &["diskdrive", "get", "Model,Size,MediaType", "/format:table"],
    ) {
        return out;
    }
    "Unknown Storage".into()
}

fn detect_gpu() -> String {
    if let Some(out) =
        run("wmic", &["path", "win32_VideoController", "get", "Name", "/format:list"])
    {
        let mut names = Vec::new();
        for line in out.lines() {
            let l = line.trim();
            if l.starts_with("Name=") {
                names.push(l["Name=".len()..].trim().to_string());
            }
        }
        if !names.is_empty() {
            return names.join("; ");
        }
    }
    "Unknown GPU".into()
}

fn detect_connectivity() -> String {
    if let Some(out) = run("wmic", &["nic", "get", "Name,NetEnabled", "/format:table"]) {
        return out;
    }
    "Unknown Connectivity".into()
}

fn detect_audio() -> String {
    if let Some(out) =
        run("wmic", &["path", "win32_SoundDevice", "get", "Name", "/format:table"])
    {
        return out;
    }
    "Unknown Audio".into()
}

fn detect_display() -> String {
    if let Some(out) = run_powershell(
        "Get-CimInstance -Namespace root\\wmi -ClassName WmiMonitorBasicDisplayParams | Format-List",
    ) {
        return out;
    }
    "Unknown Display".into()
}

fn detect_battery() -> String {
    if let Some(out) =
        run("wmic", &["path", "Win32_Battery", "get", "BatteryStatus,EstimatedChargeRemaining"])
    {
        if out.lines().count() > 1 {
            return "Battery Present".into();
        }
    }
    "".into()
}

fn detect_power() -> String {
    if let Some(out) = run_powershell("(Get-CimInstance -ClassName Win32_Battery).BatteryStatus") {
        let s = out.trim();
        if !s.is_empty() {
            return format!("BatteryStatus={}", s);
        }
    }
    "".into()
}

fn detect_cameras() -> String {
    if let Some(out) = run_powershell(
        "Get-CimInstance Win32_PnPEntity | Where-Object {$_.Name -like '*Camera*'} | Select-Object -ExpandProperty Name",
    ) {
        return out;
    }
    "".into()
}

fn detect_biometrics() -> String {
    if let Some(out) = run_powershell(
        "Get-CimInstance Win32_PnPEntity | Where-Object {$_.Name -like '*Fingerprint*'} | Select-Object -ExpandProperty Name",
    ) {
        return out;
    }
    "".into()
}

fn detect_locale() -> String {
    if let Some(out) = run_powershell(
        "[System.Globalization.CultureInfo]::CurrentCulture.Name",
    ) {
        return out.trim().to_string();
    }
    "unknown".into()
}

fn detect_os_version() -> String {
    if let Some(out) = run("systeminfo", &[]) {
        return out;
    }
    "".into()
}

fn detect_form_factor() -> String {
    // Simple heuristic: use chassis type if available
    if let Some(out) = run_powershell(
        "Get-CimInstance -ClassName Win32_SystemEnclosure | Select-Object -ExpandProperty ChassisTypes",
    ) {
        let s = out.trim();
        if s.contains("8") || s.contains("9") || s.contains("10") {
            return "Laptop".into();
        }
    }
    "Desktop/Unknown".into()
}