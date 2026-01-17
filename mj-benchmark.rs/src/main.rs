use std::process::Command;
use std::fs;

use crate::model::DeviceSpecs;

fn run(cmd: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(cmd).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

pub fn collect_macos_specs() -> DeviceSpecs {
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
        ai_npu: "".into(), // Apple Neural Engine could be detected later
        ram_storage,
        connectivity,
        audio_ports,
        nfc_wallet: "Apple Pay capable (assumed)".into(),
        battery,
        power_charging,
        qi_charging: "".into(),
        form_factor: detect_form_factor(),
        dimensions_weight: "".into(),
        display,
        build_durability: "Aluminium chassis (likely)".into(),
        cameras,
        biometrics_health,
        regional,
        software_updates,
        color: "".into(),
        upgrade_options: "Most Apple devices have limited upgrade options".into(),
        ecosystem_lock_in: "Apple ecosystem".into(),
        wear_detection: "".into(),
        touch_control: "".into(),
        storage_case: "".into(),
        special_features: "".into(),
        official_site: "https://www.apple.com/mac/".into(),
        info_links: "".into(),
        bios_boot_key: "Option (⌥) key at boot".into(),
    }
}

fn detect_brand_model() -> String {
    if let Some(sp) = run("system_profiler", &["SPHardwareDataType"]) {
        let mut model = "Unknown Mac".to_string();
        let mut identifier = String::new();

        for line in sp.lines() {
            let l = line.trim();
            if l.starts_with("Model Name:") {
                model = l.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if l.starts_with("Model Identifier:") {
                identifier = l.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }

        if identifier.is_empty() {
            model
        } else {
            format!("{} ({})", model, identifier)
        }
    } else {
        "Unknown Mac".into()
    }
}

fn detect_cpu() -> String {
    // Try sysctl first
    if let Some(out) = run("sysctl", &["-n", "machdep.cpu.brand_string"]) {
        return out.trim().to_string();
    }
    // Fallback to system_profiler
    if let Some(sp) = run("system_profiler", &["SPHardwareDataType"]) {
        for line in sp.lines() {
            let l = line.trim();
            if l.starts_with("Processor Name:") || l.starts_with("Chip:") {
                return l.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
    }
    "Unknown CPU".into()
}

fn detect_cpu_speed() -> String {
    if let Some(sp) = run("system_profiler", &["SPHardwareDataType"]) {
        for line in sp.lines() {
            let l = line.trim();
            if l.starts_with("Processor Speed:") {
                return l.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
    }
    "".into()
}

fn detect_arch() -> String {
    if let Some(out) = run("uname", &["-m"]) {
        out.trim().to_string()
    } else {
        "".into()
    }
}

fn detect_ram() -> String {
    if let Some(sp) = run("system_profiler", &["SPHardwareDataType"]) {
        for line in sp.lines() {
            let l = line.trim();
            if l.starts_with("Memory:") {
                return l.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
    }
    "Unknown RAM".into()
}

fn detect_storage() -> String {
    // Basic: use diskutil list
    if let Some(out) = run("diskutil", &["list"]) {
        return out;
    }
    "Unknown Storage".into()
}

fn detect_gpu() -> String {
    if let Some(sp) = run("system_profiler", &["SPDisplaysDataType"]) {
        let mut lines: Vec<String> = Vec::new();
        for line in sp.lines() {
            let l = line.trim();
            if l.starts_with("Chipset Model:") || l.starts_with("Graphics:") {
                lines.push(l.split(':').nth(1).unwrap_or("").trim().to_string());
            }
        }
        if !lines.is_empty() {
            return lines.join("; ");
        }
    }
    "Unknown GPU".into()
}

fn detect_connectivity() -> String {
    // networksetup -listallhardwareports
    if let Some(out) = run("networksetup", &["-listallhardwareports"]) {
        return out;
    }
    "Unknown Connectivity".into()
}

fn detect_audio() -> String {
    // system_profiler SPAudioDataType
    if let Some(sp) = run("system_profiler", &["SPAudioDataType"]) {
        return sp;
    }
    "Unknown Audio Devices".into()
}

fn detect_display() -> String {
    if let Some(sp) = run("system_profiler", &["SPDisplaysDataType"]) {
        return sp;
    }
    "Unknown Display".into()
}

fn detect_battery() -> String {
    // ioreg -rc AppleSmartBattery
    if let Some(out) = run("ioreg", &["-rc", "AppleSmartBattery"]) {
        if out.contains("AppleSmartBattery") {
            return "Battery Present".into();
        }
    }
    "".into()
}

fn detect_power() -> String {
    if let Some(out) = run("pmset", &["-g", "batt"]) {
        if out.to_lowercase().contains("ac attached") {
            return "AC Power".into();
        }
        return "Battery Power".into();
    }
    "".into()
}

fn detect_cameras() -> String {
    if let Some(sp) = run("system_profiler", &["SPCameraDataType"]) {
        return sp;
    }
    "".into()
}

fn detect_biometrics() -> String {
    // Touch ID presence is tricky; we keep this simple
    if let Ok(data) = fs::read_to_string("/usr/libexec/PlistBuddy") {
        let _ = data; // placeholder; realistically we'd parse a plist or use more specific tools
    }
    "Touch ID (if present)".into()
}

fn detect_locale() -> String {
    std::env::var("LANG").unwrap_or_else(|_| "unknown".into())
}

fn detect_os_version() -> String {
    if let Some(out) = run("sw_vers", &[]) {
        return out;
    }
    "".into()
}

fn detect_form_factor() -> String {
    // crude heuristic: MacBook vs Desktop
    if let Some(sp) = run("system_profiler", &["SPHardwareDataType"]) {
        if sp.contains("MacBook") {
            return "Laptop".into();
        }
    }
    "Desktop/Unknown".into()
}