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

pub fn collect_bsd_specs() -> DeviceSpecs {
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
        ecosystem_lock_in: "".into(),
        wear_detection: "".into(),
        touch_control: "".into(),
        storage_case: "".into(),
        special_features: "".into(),
        official_site: "".into(),
        info_links: "".into(),
        bios_boot_key: "F2/Del".into(),
    }
}

fn detect_brand_model() -> String {
    if let Some(out) = run("sysctl", &["-n", "hw.vendor"]) {
        let vendor = out.trim();
        let product = run("sysctl", &["-n", "hw.product"])
            .unwrap_or_else(|| "Unknown".into());
        return format!("{} {}", vendor, product.trim());
    }
    "Unknown BSD Machine".into()
}

fn detect_cpu() -> String {
    run("sysctl", &["-n", "hw.model"])
        .unwrap_or_else(|| "Unknown CPU".into())
        .trim()
        .into()
}

fn detect_cpu_speed() -> String {
    if let Some(out) = run("sysctl", &["-n", "hw.clockrate"]) {
        if let Ok(mhz) = out.trim().parse::<f64>() {
            return format!("{:.2} GHz", mhz / 1000.0);
        }
    }
    "".into()
}

fn detect_arch() -> String {
    run("uname", &["-m"])
        .unwrap_or_else(|| "".into())
        .trim()
        .into()
}

fn detect_ram() -> String {
    if let Some(out) = run("sysctl", &["-n", "hw.physmem"]) {
        if let Ok(bytes) = out.trim().parse::<f64>() {
            return format!("{:.1} GB RAM", bytes / 1024.0 / 1024.0 / 1024.0);
        }
    }
    "Unknown RAM".into()
}

fn detect_storage() -> String {
    if let Some(out) = run("camcontrol", &["devlist"]) {
        return out;
    }
    if let Some(out) = run("geom", &["disk", "list"]) {
        return out;
    }
    "Unknown Storage".into()
}

fn detect_gpu() -> String {
    if let Some(out) = run("pciconf", &["-lv"]) {
        let gpus: Vec<_> = out
            .lines()
            .filter(|l| l.contains("display") || l.contains("VGA"))
            .collect();
        if !gpus.is_empty() {
            return gpus.join("\n");
        }
    }
    "Unknown GPU".into()
}

fn detect_connectivity() -> String {
    if let Some(out) = run("ifconfig", &[]) {
        return out;
    }
    "Unknown Connectivity".into()
}

fn detect_audio() -> String {
    if let Some(out) = run("cat", &["/dev/sndstat"]) {
        return out;
    }
    "Unknown Audio".into()
}

fn detect_display() -> String {
    // BSD doesn't have xrandr by default; fallback to dmesg
    if let Some(out) = run("dmesg", &[]) {
        let lines: Vec<_> = out
            .lines()
            .filter(|l| l.contains("drm") || l.contains("i915") || l.contains("radeon"))
            .collect();
        if !lines.is_empty() {
            return lines.join("\n");
        }
    }
    "Unknown Display".into()
}

fn detect_battery() -> String {
    if let Some(out) = run("acpiconf", &["-i", "0"]) {
        if out.contains("Battery") {
            return "Battery Present".into();
        }
    }
    "".into()
}

fn detect_power() -> String {
    if let Some(out) = run("acpiconf", &["-i", "0"]) {
        if out.contains("State:") {
            return out;
        }
    }
    "".into()
}

fn detect_cameras() -> String {
    if let Some(out) = run("usbconfig", &[]) {
        let cams: Vec<_> = out.lines().filter(|l| l.contains("Camera")).collect();
        return cams.join("\n");
    }
    "".into()
}

fn detect_biometrics() -> String {
    if let Some(out) = run("usbconfig", &[]) {
        let fps: Vec<_> = out.lines().filter(|l| l.contains("Fingerprint")).collect();
        return fps.join("\n");
    }
    "".into()
}

fn detect_locale() -> String {
    std::env::var("LANG").unwrap_or_else(|_| "unknown".into())
}

fn detect_os_version() -> String {
    run("uname", &["-a"]).unwrap_or_else(|| "".into())
}

fn detect_form_factor() -> String {
    // BSD doesn't expose chassis type easily
    "Unknown".into()
}