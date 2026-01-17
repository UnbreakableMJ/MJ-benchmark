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

    state.log("Authenticating with Google…");
    let token = match google_auth::get_token(client_id, client_secret).await {
        Ok(t) => t,
        Err(e) => {
            state.log(format!("ERROR: {}", e));
            state.trigger_failure(PipelineStep::Specs, e.to_string());
            state.set_step(PipelineStep::Done);
            pipeline_done = true;
            // Continue to UI loop so user can read error
            loop {
                terminal.draw(|f| draw(f, &state))?;
                if let Some(TuiEvent::Key(KeyCode::Char('q') | KeyCode::Esc)) =
                    poll_event(tick_rate)
                {
                    return Ok(());
                }
            }
        }
    };

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

                    let result: Result<(), Box<dyn Error>> = (|| {
                        pts::ensure_pts_installed()?;
                        pts::ensure_suite_exists()?;
                        state.bench = Some(pts::run_suite()?);
                        Ok(())
                    })();

                    if let Err(e) = result {
                        state.log(format!("ERROR: {}", e));
                        state.stop_step_timer();
                        state.trigger_failure(PipelineStep::Pts, e.to_string());
                        state.set_step(PipelineStep::Done);
                        pipeline_done = true;
                        continue;
                    }

                    state.stop_step_timer();
                    state.trigger_success(PipelineStep::Pts);

                    state.set_step(PipelineStep::Browser);
                    state.start_step_timer();
                    state.reset_progress_bar();
                }

                PipelineStep::Browser => {
                    state.log("Running browser benchmarks…");

                    let result = browser_bench::run_browser_benchmarks().await;

                    if let Some(b) = &mut state.bench {
                        b.speedometer_score = result.speedometer;
                        b.jetstream_score = result.jetstream;
                        b.motionmark_score = result.motionmark;
                    } else {
                        let msg = "Missing benchmark state";
                        state.log(format!("ERROR: {}", msg));
                        state.stop_step_timer();
                        state.trigger_failure(PipelineStep::Browser, msg);
                        state.set_step(PipelineStep::Done);
                        pipeline_done = true;
                        continue;
                    }

                    state.stop_step_timer();
                    state.trigger_success(PipelineStep::Browser);

                    state.set_step(PipelineStep::Csv);
                    state.start_step_timer();
                    state.reset_progress_bar();
                }

                PipelineStep::Csv => {
                    state.log("Building CSV and syncing Google Sheets…");

                    let result: Result<(), Box<dyn Error>> = (|| {
                        let row = csv_row::build_csv_row(
                            state.specs.as_ref().unwrap(),
                            state.bench.as_ref().unwrap(),
                        );
                        csv_row::append_to_csv(csv_path, &row)?;
                        google_sheets::append_row(sheet_id, &row, &token).await?;
                        Ok(())
                    })();

                    if let Err(e) = result {
                        state.log(format!("ERROR: {}", e));
                        state.stop_step_timer();
                        state.trigger_failure(PipelineStep::Csv, e.to_string());
                        state.set_step(PipelineStep::Done);
                        pipeline_done = true;
                        continue;
                    }

                    state.stop_step_timer();
                    state.trigger_success(PipelineStep::Csv);

                    state.set_step(PipelineStep::Sheets);
                    state.start_step_timer();
                    state.reset_progress_bar();
                }

                PipelineStep::Sheets => {
                    state.log("Uploading CSV to Google Drive…");

                    if let Err(e) =
                        google_drive::upload_csv(drive_folder_id, csv_path, &token).await
                    {
                        state.log(format!("ERROR: {}", e));
                        state.stop_step_timer();
                        state.trigger_failure(PipelineStep::Sheets, e.to_string());
                        state.set_step(PipelineStep::Done);
                        pipeline_done = true;
                        continue;
                    }

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
                    state.tick_failure_flash();
                }

                TuiEvent::Key(key) => {
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
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),

                        KeyCode::Up | KeyCode::Char('k') => state.scroll_up(),
                        KeyCode::Down | KeyCode::Char('j') => state.scroll_down(),
                        KeyCode::PageUp => state.scroll_page_up(),
                        KeyCode::PageDown => state.scroll_page_down(),

                        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) =>
                            state.scroll_half_page_up(),
                        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) =>
                            state.scroll_half_page_down(),

                        KeyCode::Char('g') => {
                            if let Some(TuiEvent::Key(KeyCode::Char('g'))) =
                                poll_event(Duration::from_millis(50))
                            {
                                state.scroll_to_top();
                            }
                        }
                        KeyCode::Char('G') => state.scroll_to_bottom(),

                        KeyCode::Tab | KeyCode::Char('h') | KeyCode::Char('l') =>
                            state.toggle_panel(),

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