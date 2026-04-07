mod algorithms;
mod app;
mod config;
mod ui;

use app::App;
use clap::Parser;
use config::Config;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};

/// sortiz — sorting algorithm visualizer for the terminal
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Algorithm to visualize: bubble, insertion, selection, merge, quick, heap, shell
    #[arg(short, long)]
    algorithm: Option<String>,

    /// Number of elements in the array
    #[arg(short = 'n', long = "array-size", default_value_t = 50)]
    array_size: usize,

    /// Milliseconds per animation step (lower = faster)
    #[arg(short, long, default_value_t = 50)]
    speed: u64,

    /// Path to a custom config file (default: ~/.config/sortiz/config.toml)
    #[arg(short, long)]
    config: Option<std::path::PathBuf>,

    /// Loop continuously through random algorithms (default when no --algorithm given)
    #[arg(short, long)]
    loop_mode: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = match &cli.config {
        Some(path) => Config::load_from(path),
        None => Config::load(),
    };
    let colors = config.colors();

    let array_size = cli.array_size.clamp(5, 500);
    let speed = cli.speed.clamp(5, 5000);

    let mut app = App::new(array_size, speed, cli.algorithm.as_deref(), cli.loop_mode);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app, &colors);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    colors: &config::ParsedColors,
) -> anyhow::Result<()> {
    let mut last_step = Instant::now();

    loop {
        terminal.draw(|f| ui::render(f, app, colors))?;

        // Non-blocking input poll (1 ms)
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), _)
                    | (KeyCode::Char('c'), KeyModifiers::CONTROL)
                    | (KeyCode::Esc, _) => break,

                    (KeyCode::Char(' '), _) => {
                        app.paused = !app.paused;
                    }
                    (KeyCode::Up, _) => app.speed_up(),
                    (KeyCode::Down, _) => app.speed_down(),
                    (KeyCode::Char('r'), _) => {
                        app.restart();
                        last_step = Instant::now();
                    }
                    (KeyCode::Char('n'), _) => {
                        app.next_algorithm();
                        last_step = Instant::now();
                    }
                    _ => {}
                }
            }
        }

        if !app.paused && last_step.elapsed() >= Duration::from_millis(app.speed_ms) {
            app.advance();
            last_step = Instant::now();
        }
    }

    Ok(())
}
