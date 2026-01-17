use std::time::{Instant, Duration};

use crate::model::{DeviceSpecs, BenchResults};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineStep {
    Specs,
    Pts,
    Browser,
    Csv,
    Sheets,
    Drive,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Progress,
    Logs,
}

#[derive(Debug, Clone)]
pub struct TuiState {
    // Pipeline
    pub current_step: PipelineStep,

    // Logs
    pub logs: Vec<String>,
    pub log_scroll: usize,

    // Panels
    pub active_panel: ActivePanel,

    // Spinner + pulse
    pub spinner_index: usize,
    pub pulse_phase: u8,

    // Progress bar animation
    pub progress_pos: u16,
    pub progress_dir: i8,

    // Timing
    pub step_start: Option<Instant>,
    pub step_elapsed: Duration,
    pub step_times: Vec<(PipelineStep, Duration)>,

    // Success animation
    pub success_flash_ticks: u8,
    pub last_completed_step: Option<PipelineStep>,

    // Search
    pub in_search_mode: bool,
    pub search_query: String,
    pub search_results: Vec<usize>,
    pub search_index: usize,

    // Data
    pub specs: Option<DeviceSpecs>,
    pub bench: Option<BenchResults>,
}

const SPINNER_FRAMES: [&str; 10] = [
    "⠋", "⠙", "⠹", "⠸", "⠼",
    "⠴", "⠦", "⠧", "⠇", "⠏",
];

impl TuiState {
    pub fn new() -> Self {
        Self {
            current_step: PipelineStep::Specs,

            logs: Vec::new(),
            log_scroll: 0,

            active_panel: ActivePanel::Progress,

            spinner_index: 0,
            pulse_phase: 0,

            progress_pos: 0,
            progress_dir: 1,

            step_start: None,
            step_elapsed: Duration::ZERO,
            step_times: Vec::new(),

            success_flash_ticks: 0,
            last_completed_step: None,

            in_search_mode: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_index: 0,

            specs: None,
            bench: None,
        }
    }

    /* ───────────── Logging ───────────── */

    pub fn log<S: Into<String>>(&mut self, msg: S) {
        self.logs.push(msg.into());
        self.scroll_to_bottom();
    }

    /* ───────────── Pipeline ───────────── */

    pub fn set_step(&mut self, step: PipelineStep) {
        self.current_step = step;
    }

    /* ───────────── Spinner & Pulse ───────────── */

    pub fn tick_spinner(&mut self) {
        self.spinner_index = (self.spinner_index + 1) % SPINNER_FRAMES.len();
    }

    pub fn spinner(&self) -> &'static str {
        SPINNER_FRAMES[self.spinner_index]
    }

    pub fn tick_pulse(&mut self) {
        self.pulse_phase = (self.pulse_phase + 1) % 6;
    }

    pub fn pulse_style(&self) -> ratatui::style::Style {
        use ratatui::style::{Modifier, Style};
        use crate::tui::theme::COLOR_SUCCESS;

        match self.pulse_phase {
            0 | 3 => Style::default().fg(COLOR_SUCCESS),
            1 | 4 => Style::default().fg(COLOR_SUCCESS).add_modifier(Modifier::DIM),
            2 | 5 => Style::default().fg(COLOR_SUCCESS).add_modifier(Modifier::BOLD),
            _ => Style::default().fg(COLOR_SUCCESS),
        }
    }

    /* ───────────── Progress Bar ───────────── */

    pub fn tick_progress_bar(&mut self, width: u16) {
        if width == 0 {
            return;
        }

        if self.progress_pos == 0 {
            self.progress_dir = 1;
        } else if self.progress_pos >= width - 1 {
            self.progress_dir = -1;
        }

        self.progress_pos = (self.progress_pos as i16 + self.progress_dir as i16)
            .clamp(0, (width - 1) as i16) as u16;
    }

    pub fn reset_progress_bar(&mut self) {
        self.progress_pos = 0;
        self.progress_dir = 1;
    }

    /* ───────────── Timing ───────────── */

    pub fn start_step_timer(&mut self) {
        self.step_start = Some(Instant::now());
        self.step_elapsed = Duration::ZERO;
    }

    pub fn tick_step_timer(&mut self) {
        if let Some(start) = self.step_start {
            self.step_elapsed = start.elapsed();
        }
    }

    pub fn stop_step_timer(&mut self) {
        if let Some(start) = self.step_start {
            let elapsed = start.elapsed();
            self.step_elapsed = elapsed;
            self.step_times.push((self.current_step, elapsed));
        }
        self.step_start = None;
    }

    pub fn formatted_elapsed_for_current(&self) -> String {
        format_duration(self.step_elapsed)
    }

    pub fn formatted_elapsed_for_step(&self, step: PipelineStep) -> String {
        self.step_times
            .iter()
            .find(|(s, _)| *s == step)
            .map(|(_, d)| format_duration(*d))
            .unwrap_or_default()
    }

    /* ───────────── Success Animation ───────────── */

    pub fn trigger_success(&mut self, step: PipelineStep) {
        self.last_completed_step = Some(step);
        self.success_flash_ticks = 3;
    }

    pub fn tick_success_flash(&mut self) {
        if self.success_flash_ticks > 0 {
            self.success_flash_ticks -= 1;
        }
    }

    pub fn is_flashing_success(&self, step: PipelineStep) -> bool {
        self.last_completed_step == Some(step) && self.success_flash_ticks > 0
    }

    /* ───────────── Scrolling ───────────── */

    pub fn scroll_up(&mut self) {
        self.log_scroll = self.log_scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.log_scroll = self.log_scroll.saturating_add(1);
    }

    pub fn scroll_page_up(&mut self) {
        self.log_scroll = self.log_scroll.saturating_sub(10);
    }

    pub fn scroll_page_down(&mut self) {
        self.log_scroll = self.log_scroll.saturating_add(10);
    }

    pub fn scroll_half_page_up(&mut self) {
        self.log_scroll = self.log_scroll.saturating_sub(5);
    }

    pub fn scroll_half_page_down(&mut self) {
        self.log_scroll = self.log_scroll.saturating_add(5);
    }

    pub fn scroll_to_top(&mut self) {
        self.log_scroll = 0;
    }

    pub fn scroll_to_bottom(&mut self) {
        self.log_scroll = self.logs.len().saturating_sub(1);
    }

    /* ───────────── Panels ───────────── */

    pub fn toggle_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::Progress => ActivePanel::Logs,
            ActivePanel::Logs => ActivePanel::Progress,
        };
    }

    /* ───────────── Search ───────────── */

    pub fn start_search(&mut self) {
        self.in_search_mode = true;
        self.search_query.clear();
        self.search_results.clear();
        self.search_index = 0;
    }

    pub fn cancel_search(&mut self) {
        self.in_search_mode = false;
        self.search_query.clear();
        self.search_results.clear();
        self.search_index = 0;
    }

    pub fn push_search_char(&mut self, c: char) {
        self.search_query.push(c);
    }

    pub fn pop_search_char(&mut self) {
        self.search_query.pop();
    }

    pub fn finalize_search(&mut self) {
        let q = self.search_query.to_lowercase();
        self.search_results = self.logs
            .iter()
            .enumerate()
            .filter(|(_, l)| l.to_lowercase().contains(&q))
            .map(|(i, _)| i)
            .collect();

        self.search_index = 0;
        self.in_search_mode = false;

        if let Some(&idx) = self.search_results.first() {
            self.log_scroll = idx;
        }
    }

    pub fn search_next(&mut self) {
        if self.search_results.is_empty() {
            return;
        }
        self.search_index = (self.search_index + 1) % self.search_results.len();
        self.log_scroll = self.search_results[self.search_index];
    }

    pub fn search_prev(&mut self) {
        if self.search_results.is_empty() {
            return;
        }
        if self.search_index == 0 {
            self.search_index = self.search_results.len() - 1;
        } else {
            self.search_index -= 1;
        }
        self.log_scroll = self.search_results[self.search_index];
    }
}

/* ───────────── Helpers ───────────── */

fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    format!("{:02}:{:02}", secs / 60, secs % 60)
}