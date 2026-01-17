use clap::{Parser, Subcommand};
use std::fmt;
use std::process::Command;

mod platform;
mod model;
mod install;
mod csv_row;
mod sync;

use platform::Platform;
use model::{DeviceSpecs, BenchResults};
use csv_row::build_csv_row;

/// MJ-benchmark: Mohamed's Benchmarking Ecosystem orchestrator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Subcommand to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Detect OS/distro and install dependencies
    Install {
        /// Actually run commands (otherwise just print them)
        #[arg(long)]
        execute: bool,
    },

    /// Run full pipeline: specs + benchmarks + CSV + sync
    Run {
        /// Google Sheets ID
        #[arg(long)]
        sheet_id: Option<String>,
        /// Google Drive folder ID
        #[arg(long)]
        drive_folder_id: Option<String>,
        /// Path to output CSV file
        #[arg(long, default_value = "mj_benchmarks.csv")]
        csv_path: String,
    },

    /// Only detect OS/distro and print it
    Detect,

    /// Only show what would be installed
    PlanInstall,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Detect => {
            let platform = platform::detect_platform();
            println!("Detected platform: {}", platform);
        }

        Commands::PlanInstall => {
            let platform = platform::detect_platform();
            println!("Detected platform: {}", platform);
            println!("Planned install commands:");
            install::print_install_plan(platform);
        }

        Commands::Install { execute } => {
            let platform = platform::detect_platform();
            println!("Detected platform: {}", platform);
            if *execute {
                install::run_install(platform)?;
            } else {
                println!("(dry run) Planned install commands:");
                install::print_install_plan(platform);
            }
        }

        Commands::Run {
            sheet_id,
            drive_folder_id,
            csv_path,
        } => {
            let platform = platform::detect_platform();
            println!("Detected platform: {}", platform);

            // 1) Ensure dependencies (for now, just print plan)
            println!("== Install phase (dry-run) ==");
            install::print_install_plan(platform);

            // 2) Collect device specs (dummy for now)
            println!("== Collecting device specs ==");
            let specs = DeviceSpecs::dummy();
            println!("Specs: {:?}", specs);

            // 3) Run benchmarks (dummy for now)
            println!("== Running benchmarks (stub) ==");
            let bench = BenchResults::dummy();
            println!("Bench results: {:?}", bench);

            // 4) Build CSV row
            println!("== Building CSV row ==");
            let row = build_csv_row(&specs, &bench);
            println!("CSV row:\n{}", row);

            // 5) Save to CSV file (append, create header if missing)
            println!("== Writing to CSV file: {} ==", csv_path);
            csv_row::append_to_csv(csv_path, &row)?;

            // 6) Sync to Google (stub)
            if let Some(sheet) = sheet_id {
                println!("== [STUB] Would append row to Google Sheet: {} ==", sheet);
                sync::append_row_to_sheet_stub(sheet, &row)?;
            } else {
                println!("No --sheet-id provided; skipping Sheets sync.");
            }

            if let Some(folder) = drive_folder_id {
                println!("== [STUB] Would upload CSV to Google Drive folder: {} ==", folder);
                sync::upload_csv_to_drive_stub(folder, csv_path)?;
            } else {
                println!("No --drive-folder-id provided; skipping Drive upload.");
            }
        }
    }

    Ok(())
}