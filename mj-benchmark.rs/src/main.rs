use clap::{Parser, Subcommand};
use std::error::Error;

mod platform;
mod install;
mod specs_linux;
mod specs_macos;
mod specs_bsd;
mod specs_windows;
mod pts;
mod browser_bench;
mod model;
mod csv_row;
mod google_auth;
mod google_sheets;
mod google_drive;

use platform::Platform;
use model::{DeviceSpecs, BenchResults};

/// MJ-benchmark: Mohamed's Benchmarking Ecosystem orchestrator
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Detect OS/distro and install dependencies
    Install {
        #[arg(long)]
        execute: bool,
    },

    /// Run full pipeline: specs + benchmarks + CSV + sync
    Run {
        #[arg(long)]
        sheet_id: String,
        #[arg(long)]
        drive_folder_id: String,
        #[arg(long, default_value = "mj_benchmarks.csv")]
        csv_path: String,
        #[arg(long)]
        client_id: String,
        #[arg(long)]
        client_secret: String,
    },

    /// Only detect OS/distro
    Detect,

    /// Show planned install commands
    PlanInstall,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Detect => {
            println!("Detected platform: {}", platform::detect_platform());
        }

        Commands::PlanInstall => {
            let p = platform::detect_platform();
            println!("Detected platform: {}", p);
            install::print_install_plan(p);
        }

        Commands::Install { execute } => {
            let p = platform::detect_platform();
            println!("Detected platform: {}", p);
            if execute {
                install::run_install(p)?;
            } else {
                install::print_install_plan(p);
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
    let specs = match platform {
        Platform::DebianLike
        | Platform::FedoraLike
        | Platform::ArchLike
        | Platform::Nix => specs_linux::collect_linux_specs(),

        Platform::MacOs => specs_macos::collect_macos_specs(),

        Platform::FreeBsd | Platform::NetBsd | Platform::OpenBsd => {
            specs_bsd::collect_bsd_specs()
        }

        Platform::Windows => specs_windows::collect_windows_specs(),

        Platform::Unknown => {
            println!("Unknown platform; using dummy specs");
            DeviceSpecs::dummy()
        }
    };
    println!("Specs collected.");

    // 2. Run PTS benchmarks
    println!("== Running PTS benchmarks ==");
    pts::ensure_pts_installed()?;
    pts::ensure_suite_exists()?;
    let mut bench = pts::run_suite()?;
    println!("PTS results collected.");

    // 3. Browser benchmarks
    println!("== Running browser benchmarks ==");
    let browser = browser_bench::run_browser_benchmarks().await;
    println!("Browser results: {:?}", browser);

    bench.speedometer_score = browser.speedometer;
    bench.jetstream_score = browser.jetstream;
    bench.motionmark_score = browser.motionmark;

    // 4. Build CSV row
    println!("== Building CSV row ==");
    let row = csv_row::build_csv_row(&specs, &bench);

    // 5. Save CSV
    println!("== Writing CSV to {} ==", csv_path);
    csv_row::append_to_csv(csv_path, &row)?;

    // 6. Google OAuth
    println!("== Authenticating with Google ==");
    let token = google_auth::get_token(client_id, client_secret).await?;

    // 7. Append to Google Sheets
    println!("== Uploading row to Google Sheets ==");
    google_sheets::append_row(sheet_id, &row, &token).await?;

    // 8. Upload CSV to Google Drive
    println!("== Uploading CSV to Google Drive ==");
    google_drive::upload_csv(drive_folder_id, csv_path, &token).await?;

    println!("== Pipeline complete ==");
    Ok(())
}