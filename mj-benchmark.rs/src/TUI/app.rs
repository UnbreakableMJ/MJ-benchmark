use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::event::KeyCode;

use crate::{
    platform::Platform,
    collect_specs,
    pts,
    browser_bench,
    csv_row,
    google_auth,
    google_sheets,
    google_drive,
};
use crate::model::{DeviceSpecs, BenchResults};

use super::{state::{TuiState, PipelineStep}, ui::draw};
use super::events::TuiEvent;

pub async fn run_full_pipeline_with_tui(
    sheet_id: &str,
    drive_folder_id: &str,
    csv_path: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<(), Box<dyn Error>> {
    // Setup terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let res = run_tui_loop(
        &mut terminal,
        sheet_id,
        drive_folder_id,
        csv_path,
        client_id,
        client_secret,
    )
    .await;

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

async fn run_tui_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    sheet_id: &str,
    drive_folder_id: &str,
    csv_path: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<(), Box<dyn Error>> {
    let mut state = TuiState::new();
    let platform = crate::platform::detect_platform();

    state.log(format!("Platform: {}", platform));

    // Pipeline runs in this function sequentially, UI refreshes every tick
    let mut pipeline_done = false;
    let tick_rate = Duration::from_millis(200);

    // Pre-run: get Google token early so we can fail fast
    state.log("Authenticating with Google…");
    let token = google_auth::get_token(client_id, client_secret).await?;
    state.log("Google auth OK.");

    loop {
        terminal.draw(|f| draw(f, &state))?;

        if !pipeline_done {
            // Run pipeline step-by-step in this loop
            match state.current_step {
                PipelineStep::Specs => {
                    state.log("Collecting device specs…");
                    let specs: DeviceSpecs = collect_specs(platform);
                    state.specs = Some(specs);
                    state.set_step(PipelineStep::Pts);
                }
                PipelineStep::Pts => {
                    state.log("Running PTS benchmarks…");
                    pts::ensure_pts_installed()?;
                    pts::ensure_suite_exists()?;
                    let bench = pts::run_suite()?;
                    state.bench = Some(bench);
                    state.set_step(PipelineStep::Browser);
                }
                PipelineStep::Browser => {
                    state.log("Running browser benchmarks (WebDriver must be running)…");
                    let browser = browser_bench::run_browser_benchmarks().await;
                    if let Some(ref mut bench) = state.bench {
                        bench.speedometer_score = browser.speedometer;
                        bench.jetstream_score = browser.jetstream;
                        bench.motionmark_score = browser.motionmark;
                    }
                    state.set_step(PipelineStep::Csv);
                }
                PipelineStep::Csv => {
                    state.log("Building CSV row…");
                    if let (Some(specs), Some(bench)) = (&state.specs, &state.bench) {
                        let row = csv_row::build_csv_row(specs, bench);
                        csv_row::append_to_csv(csv_path, &row)?;
                        state.log(format!("CSV written to {}", csv_path));

                        state.log("Uploading row to Google Sheets…");
                        google_sheets::append_row(sheet_id, &row, &token).await?;
                        state.log("Row appended to Google Sheets.");

                        state.set_step(PipelineStep::Sheets);
                    } else {
                        state.log("ERROR: specs or bench missing before CSV step.");
                        state.set_step(PipelineStep::Done);
                    }
                }
                PipelineStep::Sheets => {
                    state.log("Uploading CSV to Google Drive…");
                    google_drive::upload_csv(drive_folder_id, csv_path, &token).await?;
                    state.log("CSV uploaded to Google Drive.");
                    state.set_step(PipelineStep::Drive);
                }
                PipelineStep::Drive => {
                    state.log("Pipeline complete.");
                    state.set_step(PipelineStep::Done);
                }
                PipelineStep::Done => {
                    pipeline_done = true;
                }
            }
        }

        // Handle input / ticks
        if let Some(ev) = super::events::poll_event(tick_rate) {
            match ev {
                TuiEvent::Tick => {
                    // could animate progress, etc.
                }
                TuiEvent::Key(key) => match key {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Up => state.scroll_up(),
                    KeyCode::Down => state.scroll_down(),
                    KeyCode::Tab => state.toggle_panel(),
                    _ => {}
                },
            }
        }
    }
}