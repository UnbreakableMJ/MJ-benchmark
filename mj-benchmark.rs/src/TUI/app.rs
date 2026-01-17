use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    event::{KeyCode, KeyModifiers},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::{
    collect_specs,
    pts,
    browser_bench,
    csv_row,
    google_auth,
    google_sheets,
    google_drive,
};

use super::{
    state::{TuiState, PipelineStep},
    ui::draw,
    events::{poll_event, TuiEvent},
};

pub async fn run_full_pipeline_with_tui(
    sheet_id: &str,
    drive_folder_id: &str,
    csv_path: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let res = run_loop(
        &mut terminal,
        sheet_id,
        drive_folder_id,
        csv_path,
        client_id,
        client_secret,
    )
    .await;

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

async fn run_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    sheet_id: &str,
    drive_folder_id: &str,
    csv_path: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<(), Box<dyn Error>> {
    let mut state = TuiState::new();
    let platform = crate::platform::detect_platform();
    let tick_rate = Duration::from_millis(200);
    let mut pipeline_done = false;

    state.log(format!("Platform: {}", platform));

    // Authenticate early
    state.log("Authenticating with Google…");
    let token = google_auth::get_token(client_id, client_secret).await?;
    state.log("Google authentication successful.");

    // Start first step timer
    state.start_step_timer();

    loop {
        terminal.draw(|f| draw(f, &state))?;

        if !pipeline_done {
            match state.current_step {
                PipelineStep::Specs => {
                    state.log("Collecting device specs…");
                    state.specs = Some(collect_specs(platform));

                    state.stop_step_timer();
                    state.trigger_success(PipelineStep::Specs);

                    state.set_step(PipelineStep::Pts);
                    state.start_step_timer();
                    state.reset_progress_bar();
                }

                PipelineStep::Pts => {
                    state.log("Running PTS benchmarks…");
                    pts::ensure_pts_installed()?;
                    pts::ensure_suite_exists()?;
                    state.bench = Some(pts::run_suite()?);

                    state.stop_step_timer();
                    state.trigger_success(PipelineStep::Pts);

                    state.set_step(PipelineStep::Browser);
                    state.start_step_timer();
                    state.reset_progress_bar();
                }

                PipelineStep::Browser => {
                    state.log("Running browser benchmarks…");
                    let browser = browser_bench::run_browser_benchmarks().await;

                    if let Some(b) = &mut state.bench {
                        b.speedometer_score = browser.speedometer;
                        b.jetstream_score = browser.jetstream;
                        b.motionmark_score = browser.motionmark;
                    }

                    state.stop_step_timer();
                    state.trigger_success(PipelineStep::Browser);

                    state.set_step(PipelineStep::Csv);
                    state.start_step_timer();
                    state.reset_progress_bar();
                }

                PipelineStep::Csv => {
                    state.log("Building CSV and syncing Google Sheets…");

                    let row = csv_row::build_csv_row(
                        state.specs.as_ref().unwrap(),
                        state.bench.as_ref().unwrap(),
                    );

                    csv_row::append_to_csv(csv_path, &row)?;
                    google_sheets::append_row(sheet_id, &row, &token).await?;

                    state.stop_step_timer();
                    state.trigger_success(PipelineStep::Csv);

                    state.set_step(PipelineStep::Sheets);
                    state.start_step_timer();
                    state.reset_progress_bar();
                }

                PipelineStep::Sheets => {
                    state.log("Uploading CSV to Google Drive…");
                    google_drive::upload_csv(drive_folder_id, csv_path, &token).await?;

                    state.stop_step_timer();
                    state.trigger_success(PipelineStep::Sheets);

                    state.set_step(PipelineStep::Drive);
                    state.start_step_timer();
                    state.reset_progress_bar();
                }

                PipelineStep::Drive => {
                    state.log("Pipeline complete.");

                    state.stop_step_timer();
                    state.trigger_success(PipelineStep::Drive);

                    state.set_step(PipelineStep::Done);
                }

                PipelineStep::Done => {
                    pipeline_done = true;
                }
            }
        }

        if let Some(ev) = poll_event(tick_rate) {
            match ev {
                TuiEvent::Tick => {
                    state.tick_spinner();
                    state.tick_pulse();
                    state.tick_progress_bar(20);
                    state.tick_step_timer();
                    state.tick_success_flash();
                }

                TuiEvent::Key(key) => {
                    // Search mode input
                    if state.in_search_mode {
                        match key {
                            KeyCode::Esc => state.cancel_search(),
                            KeyCode::Enter => state.finalize_search(),
                            KeyCode::Backspace => state.pop_search_char(),
                            KeyCode::Char(c) => state.push_search_char(c),
                            _ => {}
                        }
                        continue;
                    }

                    match key {
                        // Quit
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),

                        // Vim :q
                        KeyCode::Char(':') => {
                            if let Some(TuiEvent::Key(KeyCode::Char('q'))) =
                                poll_event(Duration::from_millis(50))
                            {
                                return Ok(());
                            }
                        }

                        // Vim ZZ
                        KeyCode::Char('Z') => {
                            if let Some(TuiEvent::Key(KeyCode::Char('Z'))) =
                                poll_event(Duration::from_millis(50))
                            {
                                return Ok(());
                            }
                        }

                        // Scroll
                        KeyCode::Up | KeyCode::Char('k') => state.scroll_up(),
                        KeyCode::Down | KeyCode::Char('j') => state.scroll_down(),
                        KeyCode::PageUp => state.scroll_page_up(),
                        KeyCode::PageDown => state.scroll_page_down(),

                        // Half-page scroll
                        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) =>
                            state.scroll_half_page_up(),
                        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) =>
                            state.scroll_half_page_down(),

                        // Jump
                        KeyCode::Char('g') => {
                            if let Some(TuiEvent::Key(KeyCode::Char('g'))) =
                                poll_event(Duration::from_millis(50))
                            {
                                state.scroll_to_top();
                            }
                        }
                        KeyCode::Char('G') => state.scroll_to_bottom(),

                        // Panel switch
                        KeyCode::Tab | KeyCode::Char('h') | KeyCode::Char('l') =>
                            state.toggle_panel(),

                        // Search
                        KeyCode::Char('/') => state.start_search(),
                        KeyCode::Char('n') => state.search_next(),
                        KeyCode::Char('N') => state.search_prev(),

                        _ => {}
                    }
                }
            }
        }
    }
}