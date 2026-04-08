use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

// ── Bar style ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BarStyle {
    Block,
    Ascii,
}

// ── Colors ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorsConfig {
    #[serde(default = "default_bar")]
    pub bar: String,
    #[serde(default = "default_comparing")]
    pub comparing: String,
    #[serde(default = "default_swapping")]
    pub swapping: String,
    #[serde(default = "default_sorted")]
    pub sorted: String,
    #[serde(default = "default_background")]
    pub background: String,
    #[serde(default = "default_text")]
    pub text: String,
}

fn default_bar() -> String { "#89b4fa".to_string() }
fn default_comparing() -> String { "#fab387".to_string() }
fn default_swapping() -> String { "#f38ba8".to_string() }
fn default_sorted() -> String { "#a6e3a1".to_string() }
fn default_background() -> String { "default".to_string() }
fn default_text() -> String { "#cdd6f4".to_string() }

impl Default for ColorsConfig {
    fn default() -> Self {
        Self {
            bar: default_bar(),
            comparing: default_comparing(),
            swapping: default_swapping(),
            sorted: default_sorted(),
            background: default_background(),
            text: default_text(),
        }
    }
}

/// Resolved ratatui `Color` values ready for rendering.
#[derive(Debug, Clone)]
pub struct ParsedColors {
    pub bar: Color,
    pub comparing: Color,
    pub swapping: Color,
    pub sorted: Color,
    pub background: Color,
    pub text: Color,
}

// ── Display ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplayConfig {
    /// "block" | "ascii" — if set, skips the startup menu entirely.
    pub default_style: Option<String>,
    /// Columns of space between bars. Clamped to 0–5 at parse time.
    pub gap: Option<u8>,
}

/// Resolved display settings.
#[derive(Debug, Clone)]
pub struct ParsedDisplay {
    /// `None` = show the startup menu; `Some(style)` = go straight to that style.
    pub default_style: Option<BarStyle>,
    pub gap: usize,
}

// ── Chars ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharsConfig {
    // Block mode
    pub block_fill: Option<String>,

    // ASCII mode — box-drawing characters for the outlined bars
    pub ascii_top_left:   Option<String>,
    pub ascii_top_mid:    Option<String>,
    pub ascii_top_right:  Option<String>,
    pub ascii_body_left:  Option<String>,
    pub ascii_body_fill:  Option<String>,
    pub ascii_body_right: Option<String>,
    /// Used when bar width is exactly 1 column.
    pub ascii_single_top:  Option<String>,
    pub ascii_single_body: Option<String>,
}

/// Resolved single characters for bar rendering.
#[derive(Debug, Clone)]
pub struct ParsedChars {
    pub block_fill:        char,
    pub ascii_top_left:    char,
    pub ascii_top_mid:     char,
    pub ascii_top_right:   char,
    pub ascii_body_left:   char,
    pub ascii_body_fill:   char,
    pub ascii_body_right:  char,
    pub ascii_single_top:  char,
    pub ascii_single_body: char,
}

// ── Top-level config ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub colors: ColorsConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub chars: CharsConfig,
}

impl Config {
    /// Load from the default XDG config path (~/.config/sortiz/config.toml),
    /// falling back to built-in defaults if the file doesn't exist or is malformed.
    pub fn load() -> Self {
        if let Some(path) = config_path() {
            if path.exists() {
                return Self::load_from(&path);
            }
        }
        Config::default()
    }

    pub fn load_from(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(content) => toml::from_str::<Config>(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    }

    pub fn colors(&self) -> ParsedColors {
        ParsedColors {
            bar: parse_color(&self.colors.bar),
            comparing: parse_color(&self.colors.comparing),
            swapping: parse_color(&self.colors.swapping),
            sorted: parse_color(&self.colors.sorted),
            background: parse_color(&self.colors.background),
            text: parse_color(&self.colors.text),
        }
    }

    pub fn display(&self) -> ParsedDisplay {
        let default_style = self.display.default_style.as_deref().and_then(|s| {
            match s.to_lowercase().as_str() {
                "block" => Some(BarStyle::Block),
                "ascii" => Some(BarStyle::Ascii),
                _ => None,
            }
        });
        let gap = self.display.gap
            .map(|g| (g as usize).clamp(0, 5))
            .unwrap_or(1);
        ParsedDisplay { default_style, gap }
    }

    pub fn chars(&self) -> ParsedChars {
        let c = &self.chars;
        ParsedChars {
            block_fill:        first_char(&c.block_fill,        '█'),
            ascii_top_left:    first_char(&c.ascii_top_left,    '╔'),
            ascii_top_mid:     first_char(&c.ascii_top_mid,     '═'),
            ascii_top_right:   first_char(&c.ascii_top_right,   '╗'),
            ascii_body_left:   first_char(&c.ascii_body_left,   '║'),
            ascii_body_fill:   first_char(&c.ascii_body_fill,   '█'),
            ascii_body_right:  first_char(&c.ascii_body_right,  '║'),
            ascii_single_top:  first_char(&c.ascii_single_top,  '╤'),
            ascii_single_body: first_char(&c.ascii_single_body, '│'),
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("sortiz").join("config.toml"))
}

/// Returns the first `char` of the string option, or `default` if absent/empty.
fn first_char(opt: &Option<String>, default: char) -> char {
    opt.as_deref().and_then(|s| s.chars().next()).unwrap_or(default)
}

pub fn parse_color(s: &str) -> Color {
    let bytes = s.as_bytes();
    if bytes.first() == Some(&b'#') && bytes.len() == 7 && bytes[1..].iter().all(|b| b.is_ascii_hexdigit()) {
        let parse = |hi: u8, lo: u8| u8::from_str_radix(std::str::from_utf8(&[hi, lo]).unwrap_or("ff"), 16).unwrap_or(255);
        let r = parse(bytes[1], bytes[2]);
        let g = parse(bytes[3], bytes[4]);
        let b = parse(bytes[5], bytes[6]);
        return Color::Rgb(r, g, b);
    }
    match s.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        _ => Color::Reset,
    }
}
