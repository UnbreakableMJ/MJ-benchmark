use crate::model::{BenchResults, DeviceSpecs};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub fn build_csv_row(specs: &DeviceSpecs, bench: &BenchResults) -> String {
    // IMPORTANT: order must match your CSV header
    let vals = [
        &specs.brand_model,
        &specs.launch_date,
        &specs.price,
        &specs.cpu,
        &specs.codename,
        &specs.cpu_speed,
        &specs.x86_level,
        &specs.gpu,
        &specs.ai_npu,
        &specs.ram_storage,
        &specs.connectivity,
        &specs.audio_ports,
        &specs.nfc_wallet,
        &specs.battery,
        &specs.power_charging,
        &specs.qi_charging,
        &specs.form_factor,
        &specs.dimensions_weight,
        &specs.display,
        &specs.build_durability,
        &specs.cameras,
        &specs.biometrics_health,
        &specs.regional,
        &specs.software_updates,
        &specs.color,
        &specs.upgrade_options,
        &specs.ecosystem_lock_in,
        &specs.wear_detection,
        &specs.touch_control,
        &specs.storage_case,
        &specs.special_features,
        &specs.official_site,
        &specs.info_links,
        &specs.bios_boot_key,
        &fmt_opt(bench.seven_zip_mips.as_ref()),
        &fmt_opt(bench.openssl_mb_s.as_ref()),
        &fmt_opt(bench.ramspeed_mb_s.as_ref()),
        &fmt_opt(bench.fio_seq_read_mb_s.as_ref()),
        &fmt_opt(bench.fio_seq_write_mb_s.as_ref()),
        &fmt_opt(bench.fio_rand_read_iops.as_ref()),
        &fmt_opt(bench.fio_rand_write_iops.as_ref()),
        &fmt_opt(bench.glmark2_score.as_ref()),
        &fmt_opt(bench.kernel_build_time_s.as_ref()),
        &fmt_opt(bench.speedometer_score.as_ref()),
        &fmt_opt(bench.jetstream_score.as_ref()),
        &fmt_opt(bench.motionmark_score.as_ref()),
        &fmt_opt(bench.battery_full_wh.as_ref()),
        &fmt_opt(bench.battery_design_wh.as_ref()),
        &fmt_opt(bench.battery_health_percent.as_ref()),
        &bench
            .battery_cycle_count
            .map(|v| v.to_string())
            .unwrap_or_default(),
        &bench.notes,
    ];

    // Use csv crate for escaping
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(vec![]);
    wtr.write_record(vals.iter()) // iter over &str
        .expect("write_record failed");
    let data = wtr.into_inner().expect("into_inner failed");
    String::from_utf8_lossy(&data).trim_end().to_string()
}

fn fmt_opt(v: Option<&f64>) -> String {
    v.map(|x| format!("{}", x)).unwrap_or_default()
}

pub fn append_to_csv(path: &str, row: &str) -> Result<(), std::io::Error> {
    let needs_header = match File::open(path) {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            let mut first_line = String::new();
            reader.read_line(&mut first_line)?;
            first_line.trim().is_empty()
        }
        Err(_) => true,
    };

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    if needs_header {
        writeln!(
            file,
            "Brand & Model,Launch Date,Price,CPU & Performance,Codename,CPU Speed,x86-64 Level,GPU,AI & NPU,RAM & Storage,Connectivity,Audio Ports,NFC & Wallet,Battery,Power & Charging,Qi Wireless Charging,Form Factor,Dimensions & Weight,Display,Build & Durability,Cameras,Biometrics & Health,Regional,Software & Updates,Color,Upgrade Options,Ecosystem Lock-in,Wear Detection,Touch Control,Storage Case,Special Features,Official Site,Info Links,BIOS/Boot Key,7-Zip MIPS,OpenSSL MB/s,RAMspeed MB/s,fio Seq Read MB/s,fio Seq Write MB/s,fio Rand Read IOPS,fio Rand Write IOPS,glmark2 Score,Kernel Build Time (s),Speedometer 2.1 Score,JetStream 2.2 Score,MotionMark 1.3 Score,Battery Full Capacity (Wh),Battery Design Capacity (Wh),Battery Health (%),Battery Cycle Count,Notes"
        )?;
    }

    writeln!(file, "{}", row)?;
    Ok(())
}