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
mod tui;

use platform::Platform;
use model::{DeviceSpecs, BenchResults};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Install {
        #[arg(long)]
        execute: bool,
    },

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

        /// tui (default) or cli
        #[arg(long, default_value = "tui")]
        mode: String,
    },

    Detect,
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
            mode,
        } => {
            let want_tui = mode == "tui";
            let use_tui = want_tui && is_tty_stdout() && !is_ci_env();

            if want_tui && !use_tui {
                eprintln!("No usable TTY detected — falling back to CLI mode.");
            }

            if use_tui {
                tui::app::run_full_pipeline_with_tui(
                    &sheet_id,
                    &drive_folder_id,
                    &csv_path,
                    &client_id,
                    &client_secret,
                )
                .await?;
            } else {
                run_full_pipeline_cli(
                    &sheet_id,
                    &drive_folder_id,
                    &csv_path,
                    &client_id,
                    &client_secret,
                )
                .await?;
            }
        }
    }

    Ok(())
}

fn is_tty_stdout() -> bool {
    use std::io::IsTerminal;
    std::io::stdout().is_terminal()
}

fn is_ci_env() -> bool {
    std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("GITLAB_CI").is_ok()
        || std::env::var("BUILD_BUILDID").is_ok()
}

async fn run_full_pipeline_cli(
    sheet_id: &str,
    drive_folder_id: &str,
    csv_path: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<(), Box<dyn Error>> {
    let platform = platform::detect_platform();
    println!("Platform: {}", platform);

    println!("Collecting specs…");
    let specs = collect_specs(platform);

    println!("Running PTS benchmarks…");
    pts::ensure_pts_installed()?;
    pts::ensure_suite_exists()?;
    let mut bench = pts::run_suite()?;

    println!("Running browser benchmarks…");
    let browser = browser_bench::run_browser_benchmarks().await;
    bench.speedometer_score = browser.speedometer;
    bench.jetstream_score = browser.jetstream;
    bench.motionmark_score = browser.motionmark;

    let row = csv_row::build_csv_row(&specs, &bench);
    csv_row::append_to_csv(csv_path, &row)?;

    let token = google_auth::get_token(client_id, client_secret).await?;
    google_sheets::append_row(sheet_id, &row, &token).await?;
    google_drive::upload_csv(drive_folder_id, csv_path, &token).await?;

    println!("Pipeline complete.");
    Ok(())
}

fn collect_specs(platform: Platform) -> DeviceSpecs {
    match platform {
        Platform::DebianLike
        | Platform::FedoraLike
        | Platform::ArchLike
        | Platform::Nix => specs_linux::collect_linux_specs(),

        Platform::MacOs => specs_macos::collect_macos_specs(),

        Platform::FreeBsd | Platform::NetBsd | Platform::OpenBsd => {
            specs_bsd::collect_bsd_specs()
        }

        Platform::Windows => specs_windows::collect_windows_specs(),

        Platform::Unknown => DeviceSpecs::dummy(),
    }
}