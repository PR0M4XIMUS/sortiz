mod algorithms;
mod app;
mod audio;
mod config;
mod ui;

use app::{App, Distribution, RaceApp};
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use config::{BarStyle, Config};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::{Duration, Instant}};

/// sortiz — sorting algorithm visualizer for the terminal
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Algorithm to visualize
    #[arg(short, long)]
    algorithm: Option<String>,

    /// Number of elements in the array (5–500)
    #[arg(short = 'n', long = "array-size", default_value_t = 50)]
    array_size: usize,

    /// Milliseconds per animation step (5–5000)
    #[arg(short, long, default_value_t = 50)]
    speed: u64,

    /// Path to a custom config file
    #[arg(short, long)]
    config: Option<std::path::PathBuf>,

    /// Cycle through algorithms automatically
    #[arg(short, long)]
    loop_mode: bool,

    /// Reproducible shuffle seed
    #[arg(long)]
    seed: Option<u64>,

    /// Initial array distribution: uniform | reversed | nearly-sorted | few-unique | sawtooth | sorted | worst-case
    #[arg(long, default_value = "uniform")]
    distribution: String,

    /// Start muted (no audio)
    #[arg(long)]
    mute: bool,

    /// Print all algorithm keys (one per line) and exit
    #[arg(long)]
    list: bool,

    /// Run headless benchmark: print step/comparison/swap counts for all algorithms and exit
    #[arg(long)]
    benchmark: bool,

    /// Race mode: show all algorithms competing simultaneously
    #[arg(long)]
    race: bool,

    /// Auto-demo mode: loop through all algorithms without interaction
    #[arg(long)]
    demo: bool,

    /// Generate shell completions and exit (bash | zsh | fish | powershell | elvish)
    #[arg(long, value_name = "SHELL")]
    generate_completions: Option<String>,

    /// Generate man page to stdout and exit
    #[arg(long, hide = true)]
    generate_man: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // ── One-shot output modes (no TUI) ────────────────────────────────────────

    if cli.generate_man {
        let cmd = Cli::command();
        let man = clap_mangen::Man::new(cmd);
        let mut buf = Vec::new();
        man.render(&mut buf)?;
        print!("{}", String::from_utf8_lossy(&buf));
        return Ok(());
    }

    if let Some(ref shell_str) = cli.generate_completions {
        let mut cmd = Cli::command();
        let shell: Shell = shell_str.parse().unwrap_or(Shell::Bash);
        clap_complete::generate(shell, &mut cmd, "sortiz", &mut io::stdout());
        return Ok(());
    }

    if cli.list {
        for algo in algorithms::all_algorithms() {
            println!("{}", algo.key);
        }
        return Ok(());
    }

    let config = match &cli.config {
        Some(path) => Config::load_from(path),
        None => Config::load(),
    };
    let colors  = config.colors();
    let display = config.display();
    let chars   = config.chars();
    let mut audio_cfg = config.audio();
    if cli.mute { audio_cfg.enabled = false; }

    let array_size = cli.array_size.clamp(5, 500);
    let speed      = cli.speed.clamp(5, 5000);
    let seed       = cli.seed;
    let dist = Distribution::from_str(&cli.distribution).unwrap_or_else(|| {
        eprintln!("Unknown distribution '{}', defaulting to uniform.", cli.distribution);
        Distribution::Uniform
    });

    if cli.benchmark {
        return run_benchmark(array_size, seed, dist);
    }

    // ── TUI setup ─────────────────────────────────────────────────────────────

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let bar_style_result: anyhow::Result<Option<BarStyle>> = match display.default_style {
        Some(style) => Ok(Some(style)),
        None => select_bar_style(&mut terminal, &colors),
    };

    let result = match bar_style_result {
        Ok(Some(style)) => {
            if cli.race {
                let effective_seed = seed.unwrap_or_else(|| rand::random());
                let mut race = RaceApp::new(array_size.min(30), speed, effective_seed, dist);
                run_race(&mut terminal, &mut race, &colors, style, &chars)
            } else {
                let loop_mode = cli.loop_mode || cli.demo;
                let mut app = App::new(
                    array_size, speed,
                    cli.algorithm.as_deref(),
                    loop_mode, seed, dist, &audio_cfg,
                );
                if cli.mute { app.toggle_mute(); }
                run(&mut terminal, &mut app, &colors, style, &chars, &display)
            }
        }
        Ok(None) => Ok(()),
        Err(e)   => Err(e),
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

// ── Startup menu ──────────────────────────────────────────────────────────────

fn select_bar_style(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    colors: &config::ParsedColors,
) -> anyhow::Result<Option<BarStyle>> {
    let mut selected = 0usize;
    loop {
        terminal.draw(|f| ui::render_menu(f, selected, colors))?;
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Resize(_, _) => { terminal.clear()?; }
                Event::Key(key) => match key.code {
                    KeyCode::Up   => { selected = selected.saturating_sub(1); }
                    KeyCode::Down => { if selected < 1 { selected += 1; } }
                    KeyCode::Char('1') => return Ok(Some(BarStyle::Block)),
                    KeyCode::Char('2') => return Ok(Some(BarStyle::Ascii)),
                    KeyCode::Enter => return Ok(Some(if selected == 0 { BarStyle::Block } else { BarStyle::Ascii })),
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(None),
                    _ => {}
                }
                _ => {}
            }
        }
    }
}

// ── Main visualizer loop ──────────────────────────────────────────────────────

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    colors: &config::ParsedColors,
    bar_style: BarStyle,
    chars: &config::ParsedChars,
    display: &config::ParsedDisplay,
) -> anyhow::Result<()> {
    // Local mutable copy of display so runtime toggles work
    let mut display = display.clone();
    let mut last_step = Instant::now();

    loop {
        terminal.draw(|f| ui::render(f, app, colors, bar_style, chars, &display))?;

        if event::poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Resize(_, _) => { terminal.clear()?; }
                Event::Key(key) => {
                    // Dismiss overlays first
                    if app.show_help {
                        app.show_help = false;
                        continue;
                    }
                    if app.show_summary {
                        app.show_summary = false;
                        continue;
                    }

                    match (key.code, key.modifiers) {
                        (KeyCode::Char('q'), _)
                        | (KeyCode::Char('c'), KeyModifiers::CONTROL)
                        | (KeyCode::Esc, _) => break,

                        (KeyCode::Char('?'), _) | (KeyCode::Char('h'), _) => {
                            app.show_help = !app.show_help;
                        }

                        (KeyCode::Char(' '), _) => {
                            app.paused = !app.paused;
                        }

                        // Step scrubbing (when paused)
                        (KeyCode::Left, KeyModifiers::SHIFT) => {
                            if app.paused {
                                for _ in 0..10 { app.step_back(); }
                            }
                        }
                        (KeyCode::Right, KeyModifiers::SHIFT) => {
                            if app.paused {
                                for _ in 0..10 { app.step_forward(); }
                            }
                        }
                        (KeyCode::Left, _) => {
                            if app.paused { app.step_back(); }
                        }
                        (KeyCode::Right, _) => {
                            if app.paused { app.step_forward(); }
                        }

                        (KeyCode::Up, _)   => app.speed_up(),
                        (KeyCode::Down, _) => app.speed_down(),
                        (KeyCode::Char('+'), _) => app.speed_inc(),
                        (KeyCode::Char('-'), _) => app.speed_dec(),

                        (KeyCode::Char('r'), KeyModifiers::SHIFT) => {
                            app.restart_new_seed();
                            last_step = Instant::now();
                        }
                        (KeyCode::Char('r'), _) => {
                            app.restart();
                            last_step = Instant::now();
                        }
                        (KeyCode::Char('n'), _) => {
                            app.next_algorithm();
                            last_step = Instant::now();
                        }
                        (KeyCode::Char('['), _) => {
                            app.prev_algorithm_sequential();
                            last_step = Instant::now();
                        }
                        (KeyCode::Char(']'), _) => {
                            app.next_algorithm_sequential();
                            last_step = Instant::now();
                        }

                        (KeyCode::Char('m'), _) => { app.toggle_mute(); }
                        (KeyCode::Char('b'), _) => { display.rainbow = !display.rainbow; }
                        (KeyCode::Char('s'), _) => { display.show_stats = !display.show_stats; }
                        (KeyCode::Char('t'), _) => { display.show_title = !display.show_title; }

                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if !app.paused && last_step.elapsed() >= Duration::from_millis(app.speed_ms) {
            app.advance();
            last_step = Instant::now();

            // Show summary overlay when done (only once per run); skip in loop/demo mode
            if app.is_done() && app.sort_elapsed_ms.is_some() && !app.summary_shown {
                app.summary_shown = true;
                if !app.loop_mode {
                    app.show_summary = true;
                }
            }
        }
    }

    Ok(())
}

// ── Race mode loop ────────────────────────────────────────────────────────────

fn run_race(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    race: &mut RaceApp,
    colors: &config::ParsedColors,
    bar_style: BarStyle,
    chars: &config::ParsedChars,
) -> anyhow::Result<()> {
    let mut last_step = Instant::now();

    loop {
        terminal.draw(|f| ui::render_race(f, race, colors, bar_style, chars))?;

        if event::poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Resize(_, _) => { terminal.clear()?; }
                Event::Key(key) => match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), _)
                    | (KeyCode::Char('c'), KeyModifiers::CONTROL)
                    | (KeyCode::Esc, _) => break,
                    (KeyCode::Char(' '), _) => { race.paused = !race.paused; }
                    (KeyCode::Up, _)   => race.speed_up(),
                    (KeyCode::Down, _) => race.speed_down(),
                    _ => {}
                }
                _ => {}
            }
        }

        if !race.paused && last_step.elapsed() >= Duration::from_millis(race.speed_ms) {
            race.advance();
            last_step = Instant::now();
        }

        if race.all_done() {
            // Stay visible until user quits
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(_) = event::read()? { break; }
            }
        }
    }

    Ok(())
}

// ── Headless benchmark ────────────────────────────────────────────────────────

fn run_benchmark(size: usize, seed: Option<u64>, dist: Distribution) -> anyhow::Result<()> {
    let effective_seed = seed.unwrap_or_else(rand::random);
    println!("Benchmark: n={size}  seed={effective_seed}  dist={}", dist.to_str());
    println!("{:<20} {:>8} {:>12} {:>8}", "Algorithm", "Steps", "Comparisons", "Swaps");
    println!("{}", "-".repeat(52));

    let data = app::build_array(size, effective_seed, dist, "uniform");
    for algo in algorithms::all_algorithms() {
        let steps = (algo.generate_steps)(&data);
        match steps.last() {
            Some(last) => println!(
                "{:<20} {:>8} {:>12} {:>8}",
                algo.name,
                steps.len(),
                last.comparisons,
                last.swaps,
            ),
            None => eprintln!("{:<20} produced no steps (skipped)", algo.name),
        }
    }
    Ok(())
}
