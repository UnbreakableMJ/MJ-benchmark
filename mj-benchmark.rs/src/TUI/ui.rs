use ratatui::{
    backend::Backend,
    Frame,
    layout::{Layout, Constraint, Direction, Rect},
    widgets::{Block, Borders, Paragraph, List, ListItem, Wrap},
    style::{Modifier},
    text::{Span, Spans},
};

use crate::tui::state::{TuiState, PipelineStep, ActivePanel};
use crate::tui::theme::*;

pub fn draw<B: Backend>(f: &mut Frame<B>, state: &TuiState) {
    let size = f.size();

    // Sidebar-left layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(size);

    draw_sidebar(f, state, chunks[0]);
    draw_main(f, state, chunks[1]);
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
            format!("Model: {}", specs.brand_model),
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
        lines.push(Spans::from(Span::styled("Specs: collecting…", text_style())));
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

fn draw_main<B: Backend>(f: &mut Frame<B>, state: &TuiState, area: Rect) {
    // Main area: progress, logs, and optional bottom search bar
    let constraints = if state.in_search_mode {
        vec![Constraint::Length(8), Constraint::Min(5), Constraint::Length(3)]
    } else {
        vec![Constraint::Length(8), Constraint::Min(5)]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    draw_progress(f, state, chunks[0]);
    draw_logs(f, state, chunks[1]);

    if state.in_search_mode {
        draw_search_bar(f, state, chunks[2]);
    }
}

fn draw_progress<B: Backend>(f: &mut Frame<B>, state: &TuiState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Progress", border_style()))
        .border_style(border_style());

    let mut items: Vec<ListItem> = Vec::new();

    for (label, step) in [
        ("Specs", PipelineStep::Specs),
        ("PTS Benchmarks", PipelineStep::Pts),
        ("Browser Benchmarks", PipelineStep::Browser),
        ("CSV Build", PipelineStep::Csv),
        ("Google Sheets Sync", PipelineStep::Sheets),
        ("Google Drive Upload", PipelineStep::Drive),
    ] {
        let is_active = step == state.current_step;
        let is_done = step_completed(step, state.current_step);

        let (marker, marker_style) = if is_active {
            (state.spinner(), state.pulse_style())
        } else if state.is_flashing_success(step) {
            ("✔", state.pulse_style())
        } else if is_done {
            ("✔", success_style())
        } else {
            (" ", text_style())
        };

        // Time: show current step’s live timer; completed steps show their locked time if available
        let time_str = if is_active {
            state.formatted_elapsed_for_current()
        } else if is_done {
            state.formatted_elapsed_for_step(step)
        } else {
            String::new()
        };

        let label_style = if is_active {
            state.pulse_style()
        } else if state.is_flashing_success(step) {
            state.pulse_style()
        } else {
            text_style()
        };

        items.push(ListItem::new(Spans::from(vec![
            Span::styled(format!("[{}] ", marker), marker_style),
            Span::styled(format!("{:<22}", label), label_style),
            Span::styled(time_str, label_style),
        ])));

        // Animated progress bar under active step
        if is_active {
            let bar = render_progress_bar(state, 20);
            items.push(ListItem::new(Spans::from(vec![
                Span::styled("    ", text_style()),
                Span::styled(bar, state.pulse_style()),
            ])));
        }
    }

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

fn render_progress_bar(state: &TuiState, width: u16) -> String {
    let mut s = String::with_capacity(width as usize);
    for i in 0..width {
        if i == state.progress_pos {
            s.push('█');
        } else {
            s.push('░');
        }
    }
    s
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
    let border = if matches!(state.active_panel, ActivePanel::Logs) {
        border_style().add_modifier(Modifier::BOLD)
    } else {
        border_style()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            "Logs (↑↓/jk scroll, Tab/hl switch, / search, n/N next/prev, q quit)",
            border,
        ))
        .border_style(border);

    let inner = block.inner(area);
    f.render_widget(block, area);

    // We keep rendering newest first (reverse), matching your previous UI.
    // Scroll is treated as a simple offset; this is intentionally lightweight.
    let start = state.log_scroll.min(state.logs.len());
    let visible_iter = state.logs.iter().rev().skip(start);

    let items: Vec<ListItem> = visible_iter
        .take(inner.height as usize)
        .map(|l| ListItem::new(Spans::from(Span::styled(l.clone(), text_style()))))
        .collect();

    let list = List::new(items);
    f.render_widget(list, inner);
}

fn draw_search_bar<B: Backend>(f: &mut Frame<B>, state: &TuiState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Search", border_style()))
        .border_style(border_style());

    let inner = block.inner(area);
    f.render_widget(block, area);

    let matches = state.search_results.len();
    let hint = if matches == 0 {
        " (no matches)"
    } else {
        ""
    };

    let text = vec![
        Spans::from(vec![
            Span::styled("/", state.pulse_style()),
            Span::styled(state.search_query.clone(), text_style()),
            Span::styled("▏", state.pulse_style()), // cursor-ish
        ]),
        Spans::from(Span::styled(
            format!("Enter: select | Esc: cancel | matches: {}{}", matches, hint),
            text_style().add_modifier(Modifier::DIM),
        )),
    ];

    let para = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .style(text_style());

    f.render_widget(para, inner);
}