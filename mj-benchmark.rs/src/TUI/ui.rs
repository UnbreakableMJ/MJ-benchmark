use ratatui::{
    backend::Backend,
    Frame,
    layout::{Layout, Constraint, Direction, Rect},
    widgets::{Block, Borders, Paragraph, List, ListItem, Wrap},
    style::{Style, Modifier},
    text::{Span, Spans},
};

use crate::tui::state::{TuiState, PipelineStep, ActivePanel};
use crate::tui::theme::*;

pub fn draw<B: Backend>(f: &mut Frame<B>, state: &TuiState) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(size);

    draw_sidebar(f, state, chunks[0]);
    draw_main_area(f, state, chunks[1]);
}

fn draw_sidebar<B: Backend>(f: &mut Frame<B>, state: &TuiState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            "Steelbore / MJ-Benchmark",
            text_style().add_modifier(Modifier::BOLD),
        ))
        .border_style(border_style());

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Spans> = Vec::new();

    if let Some(specs) = &state.specs {
        lines.push(Spans::from(Span::styled(
            format!("Platform: {}", specs.brand_model),
            text_style(),
        )));
        lines.push(Spans::from(Span::styled(
            format!("CPU: {}", specs.cpu),
            text_style(),
        )));
        lines.push(Spans::from(Span::styled(
            format!("RAM / Storage: {}", specs.ram_storage),
            text_style(),
        )));
        lines.push(Spans::from(Span::styled(
            format!("GPU: {}", specs.gpu),
            text_style(),
        )));
        lines.push(Spans::from(Span::styled(
            format!("Battery: {}", specs.battery),
            text_style(),
        )));
    } else {
        lines.push(Spans::from(Span::styled(
            "Specs: collecting…",
            text_style(),
        )));
    }

    if let Some(bench) = &state.bench {
        lines.push(Spans::from(Span::raw("")));
        lines.push(Spans::from(Span::styled(
            "Browser:",
            text_style().add_modifier(Modifier::BOLD),
        )));
        if let Some(s) = bench.speedometer_score {
            lines.push(Spans::from(Span::styled(
                format!("Speedometer: {:.1}", s),
                text_style(),
            )));
        }
        if let Some(j) = bench.jetstream_score {
            lines.push(Spans::from(Span::styled(
                format!("JetStream: {:.1}", j),
                text_style(),
            )));
        }
        if let Some(m) = bench.motionmark_score {
            lines.push(Spans::from(Span::styled(
                format!("MotionMark: {:.1}", m),
                text_style(),
            )));
        }
    }

    let para = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .style(text_style());

    f.render_widget(para, inner);
}

fn draw_main_area<B: Backend>(f: &mut Frame<B>, state: &TuiState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(5)])
        .split(area);

    draw_progress(f, state, chunks[0]);
    draw_logs(f, state, chunks[1]);
}

fn draw_progress<B: Backend>(f: &mut Frame<B>, state: &TuiState, area: Rect) {
    let title = match state.current_step {
        PipelineStep::Done => "Progress (Done)",
        _ => "Progress",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(title, border_style()))
        .border_style(border_style());

    let items = vec![
        ("Specs", PipelineStep::Specs),
        ("PTS Benchmarks", PipelineStep::Pts),
        ("Browser Benchmarks", PipelineStep::Browser),
        ("CSV Build", PipelineStep::Csv),
        ("Google Sheets Sync", PipelineStep::Sheets),
        ("Google Drive Upload", PipelineStep::Drive),
    ]
    .into_iter()
    .map(|(label, step)| {
        let (marker, style) = if step == state.current_step {
            ("⟳", success_style().add_modifier(Modifier::BOLD))
        } else if step_completed(step, state.current_step) {
            ("✔", success_style())
        } else {
            (" ", text_style())
        };

        ListItem::new(Spans::from(vec![
            Span::styled(format!("[{}] ", marker), style),
            Span::styled(label, text_style()),
        ]))
    })
    .collect::<Vec<_>>();

    let list = List::new(items).block(block);

    f.render_widget(list, area);
}

fn step_completed(step: PipelineStep, current: PipelineStep) -> bool {
    use PipelineStep::*;
    match (step, current) {
        (Specs, Pts | Browser | Csv | Sheets | Drive | Done) => true,
        (Pts, Browser | Csv | Sheets | Drive | Done) => true,
        (Browser, Csv | Sheets | Drive | Done) => true,
        (Csv, Sheets | Drive | Done) => true,
        (Sheets, Drive | Done) => true,
        (Drive, Done) => true,
        _ => false,
    }
}

fn draw_logs<B: Backend>(f: &mut Frame<B>, state: &TuiState, area: Rect) {
    let border_style = if matches!(state.active_panel, ActivePanel::Logs) {
        border_style().add_modifier(Modifier::BOLD)
    } else {
        border_style()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Logs (↑↓ scroll, Tab switch, q quit)", border_style))
        .border_style(border_style);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let visible_logs: Vec<ListItem> = state
        .logs
        .iter()
        .rev()
        .map(|l| ListItem::new(Spans::from(Span::styled(l.clone(), text_style()))))
        .collect();

    let list = List::new(visible_logs);

    f.render_widget(list, inner);
}