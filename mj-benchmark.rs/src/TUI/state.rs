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
    pub current_step: PipelineStep,

    // Logs
    pub logs: Vec<String>,
    pub log_scroll: usize,

    // Panels
    pub active_panel: ActivePanel,

    // Search
    pub in_search_mode: bool,
    pub search_query: String,
    pub search_results: Vec<usize>,
    pub search_index: usize,

    // Data
    pub specs: Option<DeviceSpecs>,
    pub bench: Option<BenchResults>,
}

impl TuiState {
    pub fn new() -> Self {
        Self {
            current_step: PipelineStep::Specs,
            logs: Vec::new(),
            log_scroll: 0,
            active_panel: ActivePanel::Progress,
            in_search_mode: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_index: 0,
            specs: None,
            bench: None,
        }
    }

    // ───────────── Logging ─────────────

    pub fn log<S: Into<String>>(&mut self, msg: S) {
        self.logs.push(msg.into());
        self.scroll_to_bottom();
    }

    // ───────────── Pipeline ─────────────

    pub fn set_step(&mut self, step: PipelineStep) {
        self.current_step = step;
    }

    // ───────────── Scrolling ─────────────

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

    // ───────────── Panels ─────────────

    pub fn toggle_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::Progress => ActivePanel::Logs,
            ActivePanel::Logs => ActivePanel::Progress,
        };
    }

    // ───────────── Search ─────────────

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
        let query = self.search_query.to_lowercase();
        self.search_results = self.logs
            .iter()
            .enumerate()
            .filter(|(_, l)| l.to_lowercase().contains(&query))
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