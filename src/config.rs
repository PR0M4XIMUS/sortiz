use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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

fn default_bar() -> String {
    "#4a9eff".to_string()
}
fn default_comparing() -> String {
    "#ffcc00".to_string()
}
fn default_swapping() -> String {
    "#ff4444".to_string()
}
fn default_sorted() -> String {
    "#44ff88".to_string()
}
fn default_background() -> String {
    "default".to_string()
}
fn default_text() -> String {
    "#cccccc".to_string()
}

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub colors: ColorsConfig,
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
}

fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("sortiz").join("config.toml"))
}

pub fn parse_color(s: &str) -> Color {
    if s.starts_with('#') && s.len() == 7 {
        let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(255);
        let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(255);
        let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(255);
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
