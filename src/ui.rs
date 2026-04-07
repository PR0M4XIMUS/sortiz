use crate::algorithms::SortStep;
use crate::app::App;
use crate::config::ParsedColors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Bar, BarChart, BarGroup, Block, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App, colors: &ParsedColors) {
    let area = frame.area();

    // Optional solid background (skipped if "default" to let terminal show through)
    if colors.background != Color::Reset {
        frame.render_widget(
            Block::default().style(Style::default().bg(colors.background)),
            area,
        );
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // algorithm name
            Constraint::Min(5),    // bar chart
            Constraint::Length(1), // status/hints
        ])
        .split(area);

    // ── Title ────────────────────────────────────────────────────────────────
    let title = Paragraph::new(app.algorithm_name.as_str())
        .style(Style::default().fg(colors.text))
        .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // ── Bar chart ─────────────────────────────────────────────────────────────
    let step = app.current_step();
    let n = step.data.len();
    let max_val = app.array_size as u64;
    let available_width = chunks[1].width as usize;

    let bar_width = if n > 0 {
        ((available_width / n) as u16).max(1)
    } else {
        1
    };

    let bars: Vec<Bar> = step
        .data
        .iter()
        .enumerate()
        .map(|(i, &val)| {
            Bar::default()
                .value(val as u64)
                .style(Style::default().fg(bar_color(i, step, colors)))
        })
        .collect();

    let chart = BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .max(max_val)
        .bar_width(bar_width)
        .bar_gap(0);

    frame.render_widget(chart, chunks[1]);

    // ── Status bar ────────────────────────────────────────────────────────────
    let (current, total) = app.progress();
    let state_label = if app.paused {
        "PAUSED"
    } else if app.is_done() {
        "DONE  "
    } else {
        "      "
    };
    let status = format!(
        " {}  q:quit  space:pause/resume  ↑↓:speed  r:restart  n:next  │  {}/{}  {}ms/step",
        state_label, current, total, app.speed_ms,
    );
    frame.render_widget(
        Paragraph::new(status).style(Style::default().fg(colors.text)),
        chunks[2],
    );
}

fn bar_color(idx: usize, step: &SortStep, colors: &ParsedColors) -> Color {
    if step.swapping.contains(&idx) {
        colors.swapping
    } else if step.comparing.contains(&idx) {
        colors.comparing
    } else if step.sorted.contains(&idx) {
        colors.sorted
    } else {
        colors.bar
    }
}
