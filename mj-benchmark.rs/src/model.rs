// SPDX-License-Identifier: GPL-3.0-or-later
//
// MJ Benchmark
// Copyright (c) 2024-2026
// Mohamed Hammad
//
// Trademarks:
//   Steelbore, S3cure, S3cure me, S3cure us, MJ Benchmark
//   These names are trademarks of Mohamed Hammad and may not be used
//   to endorse or promote derivative products without prior permission.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSpecs {
    pub brand_model: String,
    pub launch_date: String,
    pub price: String,
    pub cpu: String,
    pub codename: String,
    pub cpu_speed: String,
    pub x86_level: String,
    pub gpu: String,
    pub ai_npu: String,
    pub ram_storage: String,
    pub connectivity: String,
    pub audio_ports: String,
    pub nfc_wallet: String,
    pub battery: String,
    pub power_charging: String,
    pub qi_charging: String,
    pub form_factor: String,
    pub dimensions_weight: String,
    pub display: String,
    pub build_durability: String,
    pub cameras: String,
    pub biometrics_health: String,
    pub regional: String,
    pub software_updates: String,
    pub color: String,
    pub upgrade_options: String,
    pub ecosystem_lock_in: String,
    pub wear_detection: String,
    pub touch_control: String,
    pub storage_case: String,
    pub special_features: String,
    pub official_site: String,
    pub info_links: String,
    pub bios_boot_key: String,
}

impl DeviceSpecs {
    pub fn dummy() -> Self {
        Self {
            brand_model: "Example Brand Example Model".into(),
            launch_date: "".into(),
            price: "".into(),
            cpu: "Example CPU".into(),
            codename: "".into(),
            cpu_speed: "2.30 GHz".into(),
            x86_level: "".into(),
            gpu: "Example GPU".into(),
            ai_npu: "".into(),
            ram_storage: "8 GB / 256 GB SSD".into(),
            connectivity: "Wi-Fi, Ethernet".into(),
            audio_ports: "3.5mm jack".into(),
            nfc_wallet: "".into(),
            battery: "Battery Present".into(),
            power_charging: "USB-C PD".into(),
            qi_charging: "".into(),
            form_factor: "Laptop".into(),
            dimensions_weight: "".into(),
            display: "1920x1080".into(),
            build_durability: "Plastic/Metal".into(),
            cameras: "Integrated webcam".into(),
            biometrics_health: "Fingerprint reader".into(),
            regional: "en_US".into(),
            software_updates: "Linux / Rolling".into(),
            color: "Black".into(),
            upgrade_options: "RAM/SSD upgradable".into(),
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchResults {
    pub seven_zip_mips: Option<f64>,
    pub openssl_mb_s: Option<f64>,
    pub ramspeed_mb_s: Option<f64>,
    pub fio_seq_read_mb_s: Option<f64>,
    pub fio_seq_write_mb_s: Option<f64>,
    pub fio_rand_read_iops: Option<f64>,
    pub fio_rand_write_iops: Option<f64>,
    pub glmark2_score: Option<f64>,
    pub kernel_build_time_s: Option<f64>,
    pub speedometer_score: Option<f64>,
    pub jetstream_score: Option<f64>,
    pub motionmark_score: Option<f64>,
    pub battery_full_wh: Option<f64>,
    pub battery_design_wh: Option<f64>,
    pub battery_health_percent: Option<f64>,
    pub battery_cycle_count: Option<u32>,
    pub notes: String,
}

impl BenchResults {
    pub fn dummy() -> Self {
        Self {
            seven_zip_mips: Some(25000.0),
            openssl_mb_s: Some(500.0),
            ramspeed_mb_s: Some(10000.0),
            fio_seq_read_mb_s: Some(800.0),
            fio_seq_write_mb_s: Some(700.0),
            fio_rand_read_iops: Some(50000.0),
            fio_rand_write_iops: Some(45000.0),
            glmark2_score: Some(1500.0),
            kernel_build_time_s: Some(900.0),
            speedometer_score: Some(120.0),
            jetstream_score: Some(200.0),
            motionmark_score: Some(300.0),
            battery_full_wh: Some(48.0),
            battery_design_wh: Some(50.0),
            battery_health_percent: Some(96.0),
            battery_cycle_count: Some(150),
            notes: "Dummy benchmark run (Rust prototype)".into(),
        }
    }
}