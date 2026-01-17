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

#[derive(Debug, Clone)]
pub enum ActivePanel {
    Progress,
    Logs,
}

#[derive(Debug, Clone)]
pub struct TuiState {
    pub current_step: PipelineStep,
    pub logs: Vec<String>,
    pub log_scroll: u16,
    pub active_panel: ActivePanel,

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
            specs: None,
            bench: None,
        }
    }

    pub fn log<S: Into<String>>(&mut self, msg: S) {
        self.logs.push(msg.into());
    }

    pub fn set_step(&mut self, step: PipelineStep) {
        self.current_step = step;
    }

    pub fn scroll_up(&mut self) {
        if self.log_scroll > 0 {
            self.log_scroll -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        self.log_scroll = self.log_scroll.saturating_add(1);
    }

    pub fn toggle_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::Progress => ActivePanel::Logs,
            ActivePanel::Logs => ActivePanel::Progress,
        };
    }
}