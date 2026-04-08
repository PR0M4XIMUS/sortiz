mod algorithms;
mod app;
mod config;
mod ui;

use app::App;
use clap::Parser;
use config::{BarStyle, Config};
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
    let display = config.display();
    let chars = config.chars();

    let array_size = cli.array_size.clamp(5, 500);
    let speed = cli.speed.clamp(5, 5000);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // If [display] default_style is set in config, skip the startup menu.
    let bar_style_result: anyhow::Result<Option<BarStyle>> = match display.default_style {
        Some(style) => Ok(Some(style)),
        None => select_bar_style(&mut terminal),
    };

    let result = match bar_style_result {
        Ok(Some(style)) => {
            let mut app = App::new(array_size, speed, cli.algorithm.as_deref(), cli.loop_mode);
            run(&mut terminal, &mut app, &colors, style, &chars, &display)
        }
        Ok(None) => Ok(()), // user quit from menu
        Err(e) => Err(e),
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn select_bar_style(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> anyhow::Result<Option<BarStyle>> {
    let mut selected = 0usize;
    loop {
        terminal.draw(|f| ui::render_menu(f, selected))?;
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
            Event::Resize(_, _) => { terminal.clear()?; }
            Event::Key(key) => match key.code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Char('1') => return Ok(Some(BarStyle::Block)),
                    KeyCode::Char('2') => return Ok(Some(BarStyle::Ascii)),
                    KeyCode::Enter => {
                        return Ok(Some(if selected == 0 {
                            BarStyle::Block
                        } else {
                            BarStyle::Ascii
                        }));
                    }
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                        return Ok(None);
                    }
                    _ => {}
                }
            _ => {}
            }
        }
    }
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    colors: &config::ParsedColors,
    bar_style: BarStyle,
    chars: &config::ParsedChars,
    display: &config::ParsedDisplay,
) -> anyhow::Result<()> {
    let mut last_step = Instant::now();

    loop {
        terminal.draw(|f| ui::render(f, app, colors, bar_style, chars, display))?;

        if event::poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Resize(_, _) => { terminal.clear()?; }
                Event::Key(key) => match (key.code, key.modifiers) {
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
                _ => {}
            }
        }

        if !app.paused && last_step.elapsed() >= Duration::from_millis(app.speed_ms) {
            app.advance();
            last_step = Instant::now();
        }
    }

    Ok(())
}
