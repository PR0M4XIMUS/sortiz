use crate::algorithms::SortStep;
use crate::app::App;
use crate::config::{BarStyle, ParsedChars, ParsedColors, ParsedDisplay};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

// ── Startup menu ──────────────────────────────────────────────────────────────

pub fn render_menu(frame: &mut Frame, selected: usize) {
    let area = frame.area();

    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(30, 30, 46))),
        area,
    );

    let menu_w = 38u16;
    let menu_h = 11u16;
    let x = area.width.saturating_sub(menu_w) / 2;
    let y = area.height.saturating_sub(menu_h) / 2;
    let menu_area = Rect::new(x, y, menu_w.min(area.width), menu_h.min(area.height));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(137, 180, 250)))
        .title(Span::styled(
            "  sortiz  ",
            Style::default().fg(Color::Rgb(205, 214, 244)),
        ))
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::Rgb(30, 30, 46)));

    let inner = block.inner(menu_area);
    frame.render_widget(block, menu_area);

    let options = ["Block Bars", "ASCII Bars"];
    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Select visualization mode",
            Style::default().fg(Color::Rgb(166, 173, 200)),
        )),
        Line::from(""),
    ];

    for (i, &name) in options.iter().enumerate() {
        let (arrow, style) = if i == selected {
            ("▶  ", Style::default().fg(Color::Rgb(250, 179, 135)))
        } else {
            ("   ", Style::default().fg(Color::Rgb(205, 214, 244)))
        };
        lines.push(Line::from(vec![Span::styled(
            format!("{}{}", arrow, name),
            style,
        )]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑↓  navigate     Enter  select     q  quit",
        Style::default().fg(Color::Rgb(108, 112, 134)),
    )));

    frame.render_widget(
        Paragraph::new(lines).alignment(Alignment::Center),
        inner,
    );
}

// ── Main visualizer ───────────────────────────────────────────────────────────

pub fn render(
    frame: &mut Frame,
    app: &App,
    colors: &ParsedColors,
    bar_style: BarStyle,
    chars: &ParsedChars,
    display: &ParsedDisplay,
) {
    let area = frame.area();

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
            Constraint::Min(5),    // bars
            Constraint::Length(1), // minimal state
        ])
        .split(area);

    // ── Title ─────────────────────────────────────────────────────────────────
    frame.render_widget(
        Paragraph::new(app.algorithm_name.as_str())
            .style(Style::default().fg(colors.text))
            .alignment(Alignment::Center),
        chunks[0],
    );

    // ── Bars ──────────────────────────────────────────────────────────────────
    frame.render_widget(
        SortBars {
            step: app.current_step(),
            max: app.array_size,
            colors,
            bar_style,
            chars,
            gap: display.gap,
        },
        chunks[1],
    );

    // ── Minimal status (no keyboard hints) ────────────────────────────────────
    let (current, total) = app.progress();
    let status = if app.paused {
        format!("PAUSED  ·  {} / {}", current, total)
    } else if app.is_done() {
        "DONE".to_string()
    } else {
        format!("{} / {}", current, total)
    };
    frame.render_widget(
        Paragraph::new(status)
            .style(Style::default().fg(colors.text))
            .alignment(Alignment::Center),
        chunks[2],
    );
}

// ── Custom bar renderer ───────────────────────────────────────────────────────

struct SortBars<'a> {
    step: &'a SortStep,
    max: usize,
    colors: &'a ParsedColors,
    bar_style: BarStyle,
    chars: &'a ParsedChars,
    gap: usize,
}

impl Widget for SortBars<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let n = self.step.data.len();
        if n == 0 || area.width == 0 || area.height == 0 {
            return;
        }

        let w = area.width as usize;
        let h = area.height as usize;
        let config_gap = self.gap;

        // Fit n bars with config_gap columns between them.
        // total = n * bar_width + (n-1) * gap  →  solve for bar_width.
        // Fall back to gap=0 if the configured gap makes bars impossibly narrow.
        let (bar_width, gap) = if n >= w {
            (1usize, 0usize)
        } else {
            let try_bw = w.saturating_sub((n - 1) * config_gap) / n;
            if try_bw == 0 {
                (1usize, 0usize)
            } else {
                (try_bw.max(1), config_gap)
            }
        };

        // ASCII outlined bars need at least 3 columns to render ║…║.
        let bar_width = match self.bar_style {
            BarStyle::Ascii => bar_width.max(3),
            BarStyle::Block => bar_width,
        };

        let max = self.max.max(1) as f64;

        for (i, &val) in self.step.data.iter().enumerate() {
            let x_off = i * (bar_width + gap);
            if x_off >= w {
                break;
            }
            let bw = bar_width.min(w - x_off);
            let color = bar_color(i, self.step, self.colors);

            // Exact bar height in cells + fractional remainder.
            let bar_h_f = val as f64 * h as f64 / max;
            let bar_h = bar_h_f as usize;
            let frac = bar_h_f - bar_h as f64;

            for row in 0..h {
                // row=0 → terminal bottom of the area; row=h-1 → terminal top.
                let y = area.y + (h - 1 - row) as u16;
                let x_base = area.x + x_off as u16;

                if row < bar_h {
                    // ── Filled cell ───────────────────────────────────────────
                    let is_top = row == bar_h - 1;
                    for col in 0..bw {
                        let x = x_base + col as u16;
                        let ch = match self.bar_style {
                            BarStyle::Block => self.chars.block_fill,
                            BarStyle::Ascii => ascii_char(col, bw, is_top, self.chars),
                        };
                        if let Some(cell) = buf.cell_mut((x, y)) {
                            cell.set_char(ch).set_fg(color);
                        }
                    }
                } else if row == bar_h {
                    // ── Fractional / sub-cell top (Block mode only) ───────────
                    match self.bar_style {
                        BarStyle::Block => {
                            let frac_ch = match (frac * 8.0) as usize {
                                0 => continue,
                                1 => '▁',
                                2 => '▂',
                                3 => '▃',
                                4 => '▄',
                                5 => '▅',
                                6 => '▆',
                                7 => '▇',
                                _ => '█',
                            };
                            for col in 0..bw {
                                let x = x_base + col as u16;
                                if let Some(cell) = buf.cell_mut((x, y)) {
                                    cell.set_char(frac_ch).set_fg(color);
                                }
                            }
                        }
                        BarStyle::Ascii => {} // integer heights only in ASCII mode
                    }
                }
            }
        }
    }
}

/// Returns the character for a given column within an ASCII-mode bar.
/// `is_top` selects the cap row (╔═╗ / user-defined) vs the body (║█║ / user-defined).
#[inline]
fn ascii_char(col: usize, bw: usize, is_top: bool, chars: &ParsedChars) -> char {
    if bw == 1 {
        return if is_top { chars.ascii_single_top } else { chars.ascii_single_body };
    }
    let left = col == 0;
    let right = col == bw - 1;
    if is_top {
        if left { chars.ascii_top_left } else if right { chars.ascii_top_right } else { chars.ascii_top_mid }
    } else {
        if left { chars.ascii_body_left } else if right { chars.ascii_body_right } else { chars.ascii_body_fill }
    }
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
