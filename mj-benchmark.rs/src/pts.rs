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

use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;

use crate::model::BenchResults;

#[derive(Debug)]
pub enum PtsError {
    MissingPts,
    CommandFailed(String),
    NoResultsFound,
    ParseError(String),
}

fn run(cmd: &str, args: &[&str]) -> Result<String, PtsError> {
    let out = Command::new(cmd).args(args).output()
        .map_err(|e| PtsError::CommandFailed(format!("Failed to run {}: {}", cmd, e)))?;

    if !out.status.success() {
        return Err(PtsError::CommandFailed(format!(
            "{} {:?} exited with {}",
            cmd, args, out.status
        )));
    }

    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

pub fn ensure_pts_installed() -> Result<(), PtsError> {
    let status = Command::new("phoronix-test-suite")
        .arg("version")
        .output();

    match status {
        Ok(o) if o.status.success() => Ok(()),
        _ => Err(PtsError::MissingPts),
    }
}

pub fn ensure_suite_exists() -> Result<(), PtsError> {
    println!("Ensuring PTS suite 'mohamed-core' exists...");

    // Build suite with your chosen tests
    run("phoronix-test-suite", &[
        "build-suite",
        "mohamed-core",
        "pts/compress-7zip",
        "pts/openssl",
        "pts/ramspeed",
        "pts/fio",
        "pts/glmark2",
        "pts/build-linux-kernel",
    ])?;

    Ok(())
}

pub fn run_suite() -> Result<BenchResults, PtsError> {
    println!("Running PTS suite 'mohamed-core'...");

    run("phoronix-test-suite", &[
        "batch-benchmark",
        "mohamed-core",
    ])?;

    let result_dir = find_latest_result_dir()?;
    println!("Latest PTS result directory: {:?}", result_dir);

    let xml_path = convert_to_xml(&result_dir)?;
    println!("Converted XML: {:?}", xml_path);

    parse_xml_results(&xml_path)
}

fn find_latest_result_dir() -> Result<PathBuf, PtsError> {
    let home = std::env::var("HOME").unwrap_or("".into());
    let pts_dir = Path::new(&home).join(".phoronix-test-suite/test-results");

    let mut entries: Vec<_> = fs::read_dir(&pts_dir)
        .map_err(|_| PtsError::NoResultsFound)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    entries.sort_by_key(|e| e.metadata().unwrap().modified().unwrap());

    let latest = entries.last().ok_or(PtsError::NoResultsFound)?;
    Ok(latest.path())
}

fn convert_to_xml(result_dir: &Path) -> Result<PathBuf, PtsError> {
    let name = result_dir.file_name().unwrap().to_string_lossy();
    let xml = run("phoronix-test-suite", &[
        "result-file-to-xml",
        &name,
    ])?;

    let xml_path = result_dir.join("composite.xml");
    if xml_path.exists() {
        Ok(xml_path)
    } else {
        Err(PtsError::CommandFailed("XML not generated".into()))
    }
}

fn parse_xml_results(xml_path: &Path) -> Result<BenchResults, PtsError> {
    let xml = fs::read_to_string(xml_path)
        .map_err(|e| PtsError::ParseError(format!("Failed to read XML: {}", e)))?;

    // VERY SIMPLE extraction — you can refine later
    let seven_zip = extract_metric(&xml, "7-Zip Compression");
    let openssl = extract_metric(&xml, "OpenSSL");
    let ramspeed = extract_metric(&xml, "RAMspeed");
    let fio_seq_read = extract_metric(&xml, "FIO Sequential Read");
    let fio_seq_write = extract_metric(&xml, "FIO Sequential Write");
    let glmark2 = extract_metric(&xml, "GLMark2");
    let kernel_build = extract_metric(&xml, "Timed Linux Kernel Compilation");

    Ok(BenchResults {
        seven_zip_mips: seven_zip,
        openssl_mb_s: openssl,
        ramspeed_mb_s: ramspeed,
        fio_seq_read_mb_s: fio_seq_read,
        fio_seq_write_mb_s: fio_seq_write,
        fio_rand_read_iops: None,
        fio_rand_write_iops: None,
        glmark2_score: glmark2,
        kernel_build_time_s: kernel_build,
        speedometer_score: None,
        jetstream_score: None,
        motionmark_score: None,
        battery_full_wh: None,
        battery_design_wh: None,
        battery_health_percent: None,
        battery_cycle_count: None,
        notes: "PTS results parsed from XML".into(),
    })
}

fn extract_metric(xml: &str, test_name: &str) -> Option<f64> {
    let start = xml.find(test_name)?;
    let slice = &xml[start..];

    // Look for <value>...</value>
    let val_start = slice.find("<value>")? + "<value>".len();
    let val_end = slice[val_start..].find("</value>")? + val_start;

    slice[val_start..val_end].trim().parse().ok()
}