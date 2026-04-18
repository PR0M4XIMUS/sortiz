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

fn default_bar()        -> String { "#89b4fa".to_string() }
fn default_comparing()  -> String { "#fab387".to_string() }
fn default_swapping()   -> String { "#f38ba8".to_string() }
fn default_sorted()     -> String { "#a6e3a1".to_string() }
fn default_background() -> String { "default".to_string() }
fn default_text()       -> String { "#cdd6f4".to_string() }

impl Default for ColorsConfig {
    fn default() -> Self {
        Self {
            bar:        default_bar(),
            comparing:  default_comparing(),
            swapping:   default_swapping(),
            sorted:     default_sorted(),
            background: default_background(),
            text:       default_text(),
        }
    }
}

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
    /// "block" | "ascii" — skips startup menu if set
    pub default_style: Option<String>,
    /// Columns between bars (0–5)
    pub gap: Option<u8>,
    pub show_title:      Option<bool>,
    pub show_progress:   Option<bool>,
    pub show_stats:      Option<bool>,
    pub show_complexity: Option<bool>,
    pub show_seed:       Option<bool>,
    /// HSL rainbow coloring: bar color maps to value
    pub rainbow:         Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ParsedDisplay {
    pub default_style:   Option<BarStyle>,
    pub gap:             usize,
    pub show_title:      bool,
    pub show_progress:   bool,
    pub show_stats:      bool,
    pub show_complexity: bool,
    pub show_seed:       bool,
    pub rainbow:         bool,
}

// ── Chars ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharsConfig {
    pub block_fill:       Option<String>,
    pub ascii_top_left:   Option<String>,
    pub ascii_top_mid:    Option<String>,
    pub ascii_top_right:  Option<String>,
    pub ascii_body_left:  Option<String>,
    pub ascii_body_fill:  Option<String>,
    pub ascii_body_right: Option<String>,
    pub ascii_single_top:  Option<String>,
    pub ascii_single_body: Option<String>,
}

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

// ── Audio ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioConfig {
    /// Enable audio (default: true)
    pub enabled: Option<bool>,
    /// Volume 0.0–1.0 (default: 0.5)
    pub volume: Option<f32>,
    /// "auto" | "rodio" | "bel" | "silent"
    pub backend: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedAudio {
    pub enabled: bool,
    pub volume:  f32,
    pub backend: AudioBackend,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AudioBackend {
    Auto,
    Rodio,
    Bel,
    Silent,
}

// ── Top-level config ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub colors:  ColorsConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub chars:   CharsConfig,
    #[serde(default)]
    pub audio:   AudioConfig,
}

impl Config {
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
            bar:        parse_color(&self.colors.bar),
            comparing:  parse_color(&self.colors.comparing),
            swapping:   parse_color(&self.colors.swapping),
            sorted:     parse_color(&self.colors.sorted),
            background: parse_color(&self.colors.background),
            text:       parse_color(&self.colors.text),
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
        ParsedDisplay {
            default_style,
            gap:             self.display.gap.map(|g| (g as usize).clamp(0, 5)).unwrap_or(1),
            show_title:      self.display.show_title.unwrap_or(true),
            show_progress:   self.display.show_progress.unwrap_or(true),
            show_stats:      self.display.show_stats.unwrap_or(true),
            show_complexity: self.display.show_complexity.unwrap_or(true),
            show_seed:       self.display.show_seed.unwrap_or(false),
            rainbow:         self.display.rainbow.unwrap_or(false),
        }
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

    pub fn audio(&self) -> ParsedAudio {
        let backend = match self.audio.backend.as_deref().unwrap_or("auto").to_lowercase().as_str() {
            "rodio"  => AudioBackend::Rodio,
            "bel"    => AudioBackend::Bel,
            "silent" => AudioBackend::Silent,
            _        => AudioBackend::Auto,
        };
        ParsedAudio {
            enabled: self.audio.enabled.unwrap_or(true),
            volume:  self.audio.volume.unwrap_or(0.5).clamp(0.0, 1.0),
            backend,
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

pub fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("sortiz").join("config.toml"))
}

fn first_char(opt: &Option<String>, default: char) -> char {
    opt.as_deref().and_then(|s| s.chars().next()).unwrap_or(default)
}

pub fn parse_color(s: &str) -> Color {
    let bytes = s.as_bytes();
    if bytes.first() == Some(&b'#') && bytes.len() == 7
        && bytes[1..].iter().all(|b| b.is_ascii_hexdigit())
    {
        let parse = |hi: u8, lo: u8| {
            u8::from_str_radix(std::str::from_utf8(&[hi, lo]).unwrap_or("ff"), 16).unwrap_or(255)
        };
        return Color::Rgb(
            parse(bytes[1], bytes[2]),
            parse(bytes[3], bytes[4]),
            parse(bytes[5], bytes[6]),
        );
    }
    match s.to_lowercase().as_str() {
        "black"        => Color::Black,
        "red"          => Color::Red,
        "green"        => Color::Green,
        "yellow"       => Color::Yellow,
        "blue"         => Color::Blue,
        "magenta"      => Color::Magenta,
        "cyan"         => Color::Cyan,
        "white"        => Color::White,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightred"     => Color::LightRed,
        "lightgreen"   => Color::LightGreen,
        "lightyellow"  => Color::LightYellow,
        "lightblue"    => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan"    => Color::LightCyan,
        _              => Color::Reset,
    }
}

/// Convert HSL (h: 0–360, s: 0–1, l: 0–1) to ratatui Color::Rgb.
pub fn hsl_to_color(h: f64, s: f64, l: f64) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r1, g1, b1) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::Rgb(
        ((r1 + m) * 255.0) as u8,
        ((g1 + m) * 255.0) as u8,
        ((b1 + m) * 255.0) as u8,
    )
}
