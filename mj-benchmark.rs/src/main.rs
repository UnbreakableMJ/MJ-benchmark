use clap::{Parser, Subcommand};
use std::error::Error;

mod platform;
mod install;
mod specs_linux;
mod specs_macos;
mod specs_bsd;
mod specs_windows;
mod pts;
mod model;
mod csv_row;
mod google_auth;
mod google_sheets;
mod google_drive;

use platform::Platform;
use model::{DeviceSpecs, BenchResults};

/// MJ-benchmark: Mohamed's Benchmarking Ecosystem orchestrator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
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
        sheet_id: String,
        /// Google Drive folder ID
        #[arg(long)]
        drive_folder_id: String,
        /// Path to output CSV file
        #[arg(long, default_value = "mj_benchmarks.csv")]
        csv_path: String,
        /// Google OAuth client ID
        #[arg(long)]
        client_id: String,
        /// Google OAuth client secret
        #[arg(long)]
        client_secret: String,
    },

    /// Only detect OS/distro and print it
    Detect,

    /// Only show what would be installed
    PlanInstall,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
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
            if execute {
                println!("Executing install commands...");
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
            client_id,
            client_secret,
        } => {
            run_full_pipeline(
                &sheet_id,
                &drive_folder_id,
                &csv_path,
                &client_id,
                &client_secret,
            )
            .await?;
        }
    }

    Ok(())
}

async fn run_full_pipeline(
    sheet_id: &str,
    drive_folder_id: &str,
    csv_path: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<(), Box<dyn Error>> {
    let platform = platform::detect_platform();
    println!("== Platform: {} ==", platform);

    // 1. Collect device specs
    println!("== Collecting device specs ==");
    let specs: DeviceSpecs = match platform {
        Platform::DebianLike
        | Platform::FedoraLike
        | Platform::ArchLike
        | Platform::Nix => {
            println!("Using Linux specs collector");
            specs_linux::collect_linux_specs()
        }

        Platform::MacOs => {
            println!("Using macOS specs collector");
            specs_macos::collect_macos_specs()
        }

        Platform::FreeBsd | Platform::NetBsd | Platform::OpenBsd => {
            println!("Using BSD specs collector");
            specs_bsd::collect_bsd_specs()
        }

        Platform::Windows => {
            println!("Using Windows specs collector");
            specs_windows::collect_windows_specs()
        }

        Platform::Unknown => {
            println!("Unknown platform; falling back to dummy specs");
            DeviceSpecs::dummy()
        }
    };
    println!("Specs collected: {:?}", specs);

    // 2. Run PTS benchmarks
    println!("== Running PTS benchmarks ==");
    pts::ensure_pts_installed()?;
    pts::ensure_suite_exists()?;
    let bench: BenchResults = pts::run_suite()?;
    println!("Bench results: {:?}", bench);

    // 3. Build CSV row
    println!("== Building CSV row ==");
    let row = csv_row::build_csv_row(&specs, &bench);
    println!("CSV row:\n{}", row);

    // 4. Save to CSV
    println!("== Writing CSV to {} ==", csv_path);
    csv_row::append_to_csv(csv_path, &row)?;

    // 5. Google OAuth
    println!("== Authenticating with Google ==");
    let token = google_auth::get_token(client_id, client_secret).await?;

    // 6. Append to Google Sheets
    println!("== Uploading row to Google Sheets ({}) ==", sheet_id);
    google_sheets::append_row(sheet_id, &row, &token).await?;

    // 7. Upload CSV to Google Drive
    println!(
        "== Uploading CSV ({}) to Google Drive folder ({}) ==",
        csv_path, drive_folder_id
    );
    google_drive::upload_csv(drive_folder_id, csv_path, &token).await?;

    println!("== Pipeline complete ==");
    Ok(())
}