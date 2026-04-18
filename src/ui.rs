use crate::algorithms::SortStep;
use crate::app::{App, RaceApp};
use crate::config::{hsl_to_color, BarStyle, ParsedChars, ParsedColors, ParsedDisplay};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
    Frame,
};

// ── Startup menu ──────────────────────────────────────────────────────────────

pub fn render_menu(frame: &mut Frame, selected: usize, colors: &ParsedColors) {
    let area = frame.area();
    let bg = if colors.background != Color::Reset { colors.background } else { Color::Rgb(30, 30, 46) };
    let border = colors.comparing;
    let title_col = colors.text;
    let option_col = colors.text;
    let hint_col = Color::DarkGray;
    let sel_col = colors.swapping;

    frame.render_widget(Block::default().style(Style::default().bg(bg)), area);

    let menu_w = 38u16;
    let menu_h = 11u16;
    let x = area.width.saturating_sub(menu_w) / 2;
    let y = area.height.saturating_sub(menu_h) / 2;
    let menu_area = Rect::new(x, y, menu_w.min(area.width), menu_h.min(area.height));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border))
        .title(Span::styled("  sortiz  ", Style::default().fg(title_col)))
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(bg));

    let inner = block.inner(menu_area);
    frame.render_widget(block, menu_area);

    let options = ["Block Bars", "ASCII Bars"];
    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled("Select visualization mode", Style::default().fg(Color::DarkGray))),
        Line::from(""),
    ];

    for (i, &name) in options.iter().enumerate() {
        let (arrow, col) = if i == selected {
            ("▶  ", sel_col)
        } else {
            ("   ", option_col)
        };
        lines.push(Line::from(Span::styled(format!("{}{}", arrow, name), Style::default().fg(col))));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑↓  navigate     Enter  select     q  quit",
        Style::default().fg(hint_col),
    )));

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), inner);
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
        frame.render_widget(Block::default().style(Style::default().bg(colors.background)), area);
    }

    let step = app.current_step();
    let mut constraints: Vec<Constraint> = Vec::new();
    if display.show_title      { constraints.push(Constraint::Length(1)); }
    constraints.push(Constraint::Min(5));
    if display.show_stats      { constraints.push(Constraint::Length(1)); }
    if display.show_progress   { constraints.push(Constraint::Length(1)); }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let mut idx = 0usize;

    // ── Title + complexity ────────────────────────────────────────────────────
    if display.show_title {
        let title = if display.show_complexity {
            format!("{}  {}", app.algorithm_name, app.complexity)
        } else {
            app.algorithm_name.clone()
        };
        let mut spans = vec![Span::styled(title, Style::default().fg(colors.text))];
        if display.show_seed {
            spans.push(Span::styled(
                format!("  [seed: {}]", app.seed),
                Style::default().fg(Color::DarkGray),
            ));
        }
        if app.is_muted() {
            spans.push(Span::styled("  [muted]", Style::default().fg(Color::DarkGray)));
        }
        frame.render_widget(
            Paragraph::new(Line::from(spans)).alignment(Alignment::Center),
            chunks[idx],
        );
        idx += 1;
    }

    // ── Bars ──────────────────────────────────────────────────────────────────
    frame.render_widget(
        SortBars { step, max: app.array_size, colors, bar_style, chars, gap: display.gap, rainbow: display.rainbow },
        chunks[idx],
    );
    idx += 1;

    // ── Stats row ─────────────────────────────────────────────────────────────
    if display.show_stats {
        let stats = format!(
            "cmp: {}   swp: {}",
            step.comparisons, step.swaps
        );
        frame.render_widget(
            Paragraph::new(stats)
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center),
            chunks[idx],
        );
        idx += 1;
    }

    // ── Status / progress ─────────────────────────────────────────────────────
    if display.show_progress {
        let (current, total) = app.progress();
        let status = if app.paused {
            format!("PAUSED  ·  {} / {}", current, total)
        } else if app.is_done() {
            format!("DONE  ·  {} steps", total)
        } else {
            format!("{} / {}", current, total)
        };
        frame.render_widget(
            Paragraph::new(status)
                .style(Style::default().fg(colors.text))
                .alignment(Alignment::Center),
            chunks[idx],
        );
    }

    // ── Help overlay ──────────────────────────────────────────────────────────
    if app.show_help {
        render_help_overlay(frame, area, colors);
    }

    // ── Post-sort summary overlay ─────────────────────────────────────────────
    if app.show_summary && app.is_done() {
        render_summary(frame, area, app, colors);
    }
}

// ── Help overlay ──────────────────────────────────────────────────────────────

fn render_help_overlay(frame: &mut Frame, area: Rect, colors: &ParsedColors) {
    let w = 46u16;
    let h = 22u16;
    let x = area.width.saturating_sub(w) / 2;
    let y = area.height.saturating_sub(h) / 2;
    let popup = Rect::new(x, y, w.min(area.width), h.min(area.height));

    frame.render_widget(Clear, popup);

    let bg = if colors.background != Color::Reset { colors.background } else { Color::Rgb(30, 30, 46) };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.comparing))
        .title(Span::styled("  Keyboard Controls  ", Style::default().fg(colors.text)))
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(bg));

    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let key = |k: &str| Span::styled(format!("{:>12}  ", k), Style::default().fg(colors.swapping));
    let desc = |d: &str| Span::styled(d.to_string(), Style::default().fg(colors.text));
    let sep = || Line::from(Span::styled("", Style::default()));

    let lines = vec![
        sep(),
        Line::from(vec![key("Space"),       desc("Pause / Resume")]),
        Line::from(vec![key("Left / Right"), desc("Step back / forward (paused)")]),
        Line::from(vec![key("↑ / ↓"),        desc("Speed up / slow down (×2)")]),
        Line::from(vec![key("+ / -"),         desc("Fine speed adjust (±10ms)")]),
        sep(),
        Line::from(vec![key("r"),            desc("Restart (same seed)")]),
        Line::from(vec![key("R"),            desc("Restart (new shuffle)")]),
        Line::from(vec![key("n"),            desc("Jump to random algorithm")]),
        Line::from(vec![key("[ / ]"),        desc("Cycle algorithms ←/→")]),
        sep(),
        Line::from(vec![key("m"),            desc("Toggle mute")]),
        Line::from(vec![key("b"),            desc("Toggle rainbow bars")]),
        Line::from(vec![key("s"),            desc("Toggle stats row")]),
        Line::from(vec![key("t"),            desc("Toggle title/complexity")]),
        sep(),
        Line::from(vec![key("? / h"),        desc("This help screen")]),
        Line::from(vec![key("q / Esc"),      desc("Quit")]),
        sep(),
        Line::from(Span::styled(
            "Press any key to close",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Left), inner);
}

// ── Post-sort summary overlay ─────────────────────────────────────────────────

fn render_summary(frame: &mut Frame, area: Rect, app: &App, colors: &ParsedColors) {
    let w = 42u16;
    let h = 12u16;
    let x = area.width.saturating_sub(w) / 2;
    let y = area.height.saturating_sub(h) / 2;
    let popup = Rect::new(x, y, w.min(area.width), h.min(area.height));

    frame.render_widget(Clear, popup);

    let bg = if colors.background != Color::Reset { colors.background } else { Color::Rgb(30, 30, 46) };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.sorted))
        .title(Span::styled("  Sort Complete  ", Style::default().fg(colors.sorted)))
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(bg));

    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let step = app.current_step();
    let elapsed_str = app.sort_elapsed_ms
        .map(|ms| format!("{}.{:03}s", ms / 1000, ms % 1000))
        .unwrap_or_else(|| "—".to_string());

    let val_col = colors.comparing;
    let lbl_col = colors.text;
    let dim_col = Color::DarkGray;

    let lbl = |l: &str| Span::styled(format!("  {:>16}  ", l), Style::default().fg(lbl_col));
    let val = |v: String| Span::styled(v, Style::default().fg(val_col));

    let lines = vec![
        Line::from(""),
        Line::from(vec![lbl("Algorithm"), val(app.algorithm_name.clone())]),
        Line::from(vec![lbl("Complexity"), val(app.complexity.to_string())]),
        Line::from(vec![lbl("Array size"), val(format!("{}", app.array_size))]),
        Line::from(vec![lbl("Distribution"), val(app.distribution.to_str().to_string())]),
        Line::from(vec![lbl("Comparisons"), val(format!("{}", step.comparisons))]),
        Line::from(vec![lbl("Swaps"), val(format!("{}", step.swaps))]),
        Line::from(vec![lbl("Steps"), val(format!("{}", app.steps.len()))]),
        Line::from(vec![lbl("Wall time"), val(elapsed_str)]),
        Line::from(""),
        Line::from(Span::styled("  Press any key to continue", Style::default().fg(dim_col))),
    ];

    frame.render_widget(Paragraph::new(lines), inner);
}

// ── Race mode renderer ────────────────────────────────────────────────────────

pub fn render_race(
    frame: &mut Frame,
    race: &RaceApp,
    colors: &ParsedColors,
    bar_style: BarStyle,
    chars: &ParsedChars,
) {
    let area = frame.area();

    if colors.background != Color::Reset {
        frame.render_widget(Block::default().style(Style::default().bg(colors.background)), area);
    }

    let n_algos = race.racers.len();
    if n_algos == 0 { return; }

    // Fit as many panels as possible given terminal width; minimum 12 cols each
    let min_panel_w = 12u16;
    let panels_across = ((area.width / min_panel_w) as usize).min(n_algos).max(1);
    let rows = (n_algos + panels_across - 1) / panels_across;

    let row_h = area.height / rows as u16;
    let col_w = area.width / panels_across as u16;

    for (i, racer) in race.racers.iter().enumerate() {
        let col = i % panels_across;
        let row = i / panels_across;
        let x = area.x + col as u16 * col_w;
        let y = area.y + row as u16 * row_h;
        let w = if col == panels_across - 1 { area.width - col as u16 * col_w } else { col_w };
        let h = if row == rows - 1 { area.height - row as u16 * row_h } else { row_h };
        if w == 0 || h == 0 { continue; }

        let panel = Rect::new(x, y, w, h);

        // Title row (1 line) + bars
        let title_h = 1u16.min(h);
        let bar_h = h.saturating_sub(title_h);

        let done_marker = if racer.is_done() { " DONE" } else { "" };
        let title = format!("{}{}", racer.name, done_marker);
        let title_col = if racer.is_done() { colors.sorted } else { colors.text };

        frame.render_widget(
            Paragraph::new(title)
                .style(Style::default().fg(title_col))
                .alignment(Alignment::Center),
            Rect::new(x, y, w, title_h),
        );

        if bar_h > 0 {
            frame.render_widget(
                SortBars {
                    step: racer.current_step(),
                    max: race.array_size,
                    colors,
                    bar_style,
                    chars,
                    gap: 0,
                    rainbow: false,
                },
                Rect::new(x, y + title_h, w, bar_h),
            );
        }
    }

    // Global speed hint at the bottom-right
    let hint = format!("Speed: {}ms  Space=pause  q=quit", race.speed_ms);
    if area.height > 1 {
        frame.render_widget(
            Paragraph::new(hint)
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Right),
            Rect::new(area.x, area.y + area.height - 1, area.width, 1),
        );
    }
}

// ── Custom bar renderer ───────────────────────────────────────────────────────

struct SortBars<'a> {
    step:      &'a SortStep,
    max:       usize,
    colors:    &'a ParsedColors,
    bar_style: BarStyle,
    chars:     &'a ParsedChars,
    gap:       usize,
    rainbow:   bool,
}

impl Widget for SortBars<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let n = self.step.data.len();
        if n == 0 || area.width == 0 || area.height == 0 { return; }

        let w = area.width as usize;
        let h = area.height as usize;
        let config_gap = self.gap;

        let (bar_width, gap) = if n >= w {
            (1usize, 0usize)
        } else {
            let try_bw = w.saturating_sub((n - 1) * config_gap) / n;
            if try_bw >= 1 { (try_bw, config_gap) } else { (w / n, 0usize) }
        };

        let bar_width = match self.bar_style {
            BarStyle::Ascii => bar_width.max(3),
            BarStyle::Block => bar_width,
        };

        let max = self.max.max(1) as f64;
        let used = n * bar_width + n.saturating_sub(1) * gap;
        let x_pad = w.saturating_sub(used) / 2;

        let color_map = self.build_color_map(n);

        for (i, &val) in self.step.data.iter().enumerate() {
            let x_off = x_pad + i * (bar_width + gap);
            if x_off >= w { break; }
            let bw = bar_width.min(w - x_off);
            let color = color_map[i];

            let bar_h_f = val as f64 * h as f64 / max;
            let bar_h = bar_h_f as usize;
            let frac = bar_h_f - bar_h as f64;

            for row in 0..h {
                let y = area.y + (h - 1 - row) as u16;
                let x_base = area.x + x_off as u16;

                if row < bar_h {
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
                    if let BarStyle::Block = self.bar_style {
                        let frac_ch = match (frac * 8.0) as usize {
                            0 => continue,
                            1 => '▁', 2 => '▂', 3 => '▃', 4 => '▄',
                            5 => '▅', 6 => '▆', 7 => '▇', _ => '█',
                        };
                        for col in 0..bw {
                            let x = x_base + col as u16;
                            if let Some(cell) = buf.cell_mut((x, y)) {
                                cell.set_char(frac_ch).set_fg(color);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<'a> SortBars<'a> {
    fn build_color_map(&self, n: usize) -> Vec<Color> {
        let step = self.step;
        let max = self.max.max(1);

        let mut map: Vec<Color> = if self.rainbow {
            // Rainbow: color by value using HSL hue
            step.data.iter().map(|&v| {
                let hue = (v as f64 / max as f64) * 300.0; // 0–300 deg, skip red wrap
                hsl_to_color(hue, 0.9, 0.55)
            }).collect()
        } else {
            vec![self.colors.bar; n]
        };

        // Overlay state colors (sorted → comparing → swapping, each overrides previous)
        for &i in &step.sorted    { if i < n { map[i] = self.colors.sorted;    } }
        for &i in &step.comparing { if i < n { map[i] = self.colors.comparing; } }
        for &i in &step.swapping  { if i < n { map[i] = self.colors.swapping;  } }
        map
    }
}

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
