use std::fs;
use std::process::Command;

pub fn collect_linux_specs() -> crate::model::DeviceSpecs {
    let brand_model = read_dmi("sys_vendor")
        .unwrap_or_else(|| "Unknown Vendor".into())
        + " "
        + &read_dmi("product_name").unwrap_or_else(|| "Unknown Model".into());

    let cpu = read_cpu_model();
    let cpu_speed = read_cpu_speed();
    let x86_level = read_x86_level();
    let gpu = detect_gpu();
    let ram_storage = format!("{} / {}", read_ram(), detect_storage());
    let connectivity = detect_connectivity();
    let audio_ports = detect_audio();
    let display = detect_display();
    let regional = detect_locale();
    let software_updates = detect_os_version();
    let bios_boot_key = "F2/Del".into(); // cannot reliably detect

    crate::model::DeviceSpecs {
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
        battery: detect_battery(),
        power_charging: detect_power(),
        qi_charging: "".into(),
        form_factor: detect_chassis(),
        dimensions_weight: "".into(),
        display,
        build_durability: "".into(),
        cameras: detect_cameras(),
        biometrics_health: detect_biometrics(),
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
        bios_boot_key,
    }
}

fn read_dmi(field: &str) -> Option<String> {
    let path = format!("/sys/class/dmi/id/{}", field);
    fs::read_to_string(path).ok().map(|s| s.trim().into())
}

fn read_cpu_model() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|data| {
            data.lines()
                .find(|l| l.starts_with("model name"))
                .map(|l| l.split(':').nth(1).unwrap_or("").trim().into())
        })
        .unwrap_or_else(|| "Unknown CPU".into())
}

fn read_cpu_speed() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|data| {
            data.lines()
                .find(|l| l.starts_with("cpu MHz"))
                .and_then(|l| l.split(':').nth(1))
                .map(|mhz| {
                    let mhz: f64 = mhz.trim().parse().unwrap_or(0.0);
                    format!("{:.2} GHz", mhz / 1000.0)
                })
        })
        .unwrap_or_else(|| "".into())
}

fn read_x86_level() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|data| {
            data.lines()
                .find(|l| l.starts_with("flags"))
                .map(|l| l.to_string())
        })
        .unwrap_or_default()
}

fn read_ram() -> String {
    fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|data| {
            data.lines()
                .find(|l| l.starts_with("MemTotal"))
                .and_then(|l| l.split_whitespace().nth(1))
                .map(|kb| {
                    let mb: f64 = kb.parse::<f64>().unwrap_or(0.0) / 1024.0;
                    format!("{:.1} GB RAM", mb / 1024.0)
                })
        })
        .unwrap_or_else(|| "Unknown RAM".into())
}

fn detect_storage() -> String {
    let out = Command::new("lsblk")
        .arg("-o")
        .arg("NAME,SIZE,TYPE")
        .output();

    match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => "Unknown Storage".into(),
    }
}

fn detect_gpu() -> String {
    let out = Command::new("lspci").output();
    if let Ok(o) = out {
        let text = String::from_utf8_lossy(&o.stdout);
        if let Some(line) = text.lines().find(|l| l.contains("VGA") || l.contains("3D")) {
            return line.to_string();
        }
    }
    "Unknown GPU".into()
}

fn detect_connectivity() -> String {
    let out = Command::new("nmcli").arg("device").output();
    if let Ok(o) = out {
        return String::from_utf8_lossy(&o.stdout).to_string();
    }
    "Unknown Connectivity".into()
}

fn detect_audio() -> String {
    let out = Command::new("aplay").arg("-l").output();
    if let Ok(o) = out {
        return String::from_utf8_lossy(&o.stdout).to_string();
    }
    "Unknown Audio".into()
}

fn detect_display() -> String {
    let out = Command::new("xrandr").output();
    if let Ok(o) = out {
        return String::from_utf8_lossy(&o.stdout).to_string();
    }
    "Unknown Display".into()
}

fn detect_battery() -> String {
    if fs::metadata("/sys/class/power_supply/BAT0").is_ok() {
        "Battery Present".into()
    } else {
        "".into()
    }
}

fn detect_power() -> String {
    if let Ok(status) = fs::read_to_string("/sys/class/power_supply/AC/online") {
        if status.trim() == "1" {
            return "AC Power".into();
        }
    }
    "Battery Power".into()
}

fn detect_chassis() -> String {
    read_dmi("chassis_type").unwrap_or_default()
}

fn detect_cameras() -> String {
    let out = Command::new("lsusb").output();
    if let Ok(o) = out {
        let text = String::from_utf8_lossy(&o.stdout);
        let cams: Vec<_> = text.lines().filter(|l| l.contains("Camera")).collect();
        return cams.join("; ");
    }
    "".into()
}

fn detect_biometrics() -> String {
    let out = Command::new("lsusb").output();
    if let Ok(o) = out {
        let text = String::from_utf8_lossy(&o.stdout);
        let fps: Vec<_> = text.lines().filter(|l| l.contains("Fingerprint")).collect();
        return fps.join("; ");
    }
    "".into()
}

fn detect_locale() -> String {
    std::env::var("LANG").unwrap_or_else(|_| "unknown".into())
}

fn detect_os_version() -> String {
    let out = Command::new("hostnamectl").output();
    if let Ok(o) = out {
        return String::from_utf8_lossy(&o.stdout).to_string();
    }
    "".into()
}