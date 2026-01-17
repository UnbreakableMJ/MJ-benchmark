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

    let para = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .style(text_style());

    f.render_widget(para, inner);
}

fn draw_main<B: Backend>(f: &mut Frame<B>, state: &TuiState, area: Rect) {
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

        let (marker, style) = if state.failed_step == Some(step) {
            ("✖", state.failure_style())
        } else if is_active {
            (state.spinner(), state.pulse_style())
        } else if state.is_flashing_success(step) {
            ("✔", state.pulse_style())
        } else if is_done {
            ("✔", success_style())
        } else {
            (" ", text_style())
        };

        let time = if is_active {
            state.formatted_elapsed_for_current()
        } else if is_done {
            state.formatted_elapsed_for_step(step)
        } else {
            String::new()
        };

        items.push(ListItem::new(Spans::from(vec![
            Span::styled(format!("[{}] ", marker), style),
            Span::styled(format!("{:<22}", label), style),
            Span::styled(time, style),
        ])));

        if is_active {
            let bar = render_progress_bar(state, 20);
            items.push(ListItem::new(Spans::from(vec![
                Span::styled("    ", text_style()),
                Span::styled(bar, state.pulse_style()),
            ])));
        }
    }

    f.render_widget(List::new(items).block(block), area);
}

fn draw_logs<B: Backend>(f: &mut Frame<B>, state: &TuiState, area: Rect) {
    let border = if matches!(state.active_panel, ActivePanel::Logs) {
        border_style().add_modifier(Modifier::BOLD)
    } else {
        border_style()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Logs", border))
        .border_style(border);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut y = inner.y;

    if let Some(err) = &state.error_message {
        let err_para = Paragraph::new(Spans::from(Span::styled(
            format!("ERROR: {}", err),
            warning_style().add_modifier(Modifier::BOLD),
        )));
        f.render_widget(err_para, Rect { y, height: 1, ..inner });
        y += 1;
    }

    let visible = state.logs.iter().rev().skip(state.log_scroll);
    let items: Vec<ListItem> = visible
        .take((inner.height - (y - inner.y)) as usize)
        .map(|l| ListItem::new(Spans::from(Span::styled(l.clone(), text_style()))))
        .collect();

    f.render_widget(List::new(items), Rect { y, ..inner });
}

fn draw_search_bar<B: Backend>(f: &mut Frame<B>, state: &TuiState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Search", border_style()))
        .border_style(border_style());

    let inner = block.inner(area);
    f.render_widget(block, area);

    let para = Paragraph::new(vec![
        Spans::from(vec![
            Span::styled("/", state.pulse_style()),
            Span::styled(state.search_query.clone(), text_style()),
            Span::styled("▏", state.pulse_style()),
        ]),
        Spans::from(Span::styled(
            format!("matches: {}", state.search_results.len()),
            text_style().add_modifier(Modifier::DIM),
        )),
    ]);

    f.render_widget(para, inner);
}

fn render_progress_bar(state: &TuiState, width: u16) -> String {
    (0..width)
        .map(|i| if i == state.progress_pos { '█' } else { '░' })
        .collect()
}

fn step_completed(step: PipelineStep, current: PipelineStep) -> bool {
    use PipelineStep::*;
    matches!(
        (step, current),
        (Specs, Pts | Browser | Csv | Sheets | Drive | Done)
            | (Pts, Browser | Csv | Sheets | Drive | Done)
            | (Browser, Csv | Sheets | Drive | Done)
            | (Csv, Sheets | Drive | Done)
            | (Sheets, Drive | Done)
            | (Drive, Done)
    )
}