# sortiz

A terminal UI sorting algorithm visualizer with smooth animations, multiple algorithms, and full theme support via a simple TOML config file.

---

## Installation

**Build from source** (requires [Rust](https://rustup.rs)):

```bash
git clone https://github.com/PR0M4XIMUS/sortiz.git
cd sortiz
cargo build --release
# Copy the binary somewhere on your PATH:
cp target/release/sortiz ~/.local/bin/
```

---

## Usage

```
sortiz [OPTIONS]
```

| Flag | Description | Default |
|------|-------------|---------|
| `-a, --algorithm <NAME>` | Algorithm to visualize | random |
| `-n, --array-size <N>` | Number of elements (5–500) | `50` |
| `-s, --speed <MS>` | Milliseconds per step (5–5000) | `50` |
| `-c, --config <PATH>` | Path to a custom config file | `~/.config/sortiz/config.toml` |
| `-l, --loop-mode` | Cycle through algorithms automatically | off |

**Available algorithms:** `bubble` · `insertion` · `selection` · `merge` · `quick` · `heap` · `shell`

**Examples:**

```bash
sortiz                                  # random algorithm, default settings
sortiz --algorithm quick -n 100 -s 25  # quick sort, 100 bars, fast
sortiz --loop                           # cycle through all algorithms
sortiz -c ~/my-theme.toml              # use a custom config file
```

---

## Keyboard Controls

| Key | Action |
|-----|--------|
| `Space` | Pause / Resume |
| `↑` | Speed up |
| `↓` | Slow down |
| `r` | Restart current sort with a new array |
| `n` | Skip to next algorithm |
| `q` / `Esc` | Quit |

---

## Configuration

The config file lives at `~/.config/sortiz/config.toml`. Create the directory if it doesn't exist:

```bash
mkdir -p ~/.config/sortiz
cp config.example.toml ~/.config/sortiz/config.toml
```

All fields are optional — remove any line to fall back to the built-in default. The full schema:

```toml
# Colors accept: "#rrggbb", named colors (black, red, …), or "default" (transparent)
[colors]
bar        = "#89b4fa"   # idle bar color
comparing  = "#fab387"   # bars being compared
swapping   = "#f38ba8"   # bars being swapped
sorted     = "#a6e3a1"   # bars confirmed in place
background = "default"   # background ("default" = terminal transparent)
text       = "#cdd6f4"   # title and status text

[display]
# "block" or "ascii" — skips the startup menu if set
# default_style = "block"

# Gap between bars in columns (0–5, default 1)
# gap = 1

[chars]
# Only the first character of each value is used.

# Block mode fill character
# block_fill = "█"

# ASCII mode — box-drawing characters for outlined bars
# ascii_top_left   = "╔"
# ascii_top_mid    = "═"
# ascii_top_right  = "╗"
# ascii_body_left  = "║"
# ascii_body_fill  = "█"
# ascii_body_right = "║"
# ascii_single_top  = "╤"   # single-column bar top
# ascii_single_body = "│"   # single-column bar body
```

---

## Themes

Copy a `[colors]` block into your config file:

<details>
<summary><strong>Catppuccin Mocha</strong> (default)</summary>

```toml
[colors]
bar        = "#89b4fa"
comparing  = "#fab387"
swapping   = "#f38ba8"
sorted     = "#a6e3a1"
background = "default"
text       = "#cdd6f4"
```
</details>

<details>
<summary><strong>Dracula</strong></summary>

```toml
[colors]
bar        = "#ff79c6"
comparing  = "#ffb86c"
swapping   = "#ff5555"
sorted     = "#50fa7b"
background = "#282a36"
text       = "#f8f8f2"
```
</details>

<details>
<summary><strong>Nord</strong></summary>

```toml
[colors]
bar        = "#81a1c1"
comparing  = "#ebcb8b"
swapping   = "#bf616a"
sorted     = "#a3be8c"
background = "#2e3440"
text       = "#d8dee9"
```
</details>

<details>
<summary><strong>Gruvbox</strong></summary>

```toml
[colors]
bar        = "#83a598"
comparing  = "#fabd2f"
swapping   = "#fb4934"
sorted     = "#b8bb26"
background = "#282828"
text       = "#ebdbb2"
```
</details>

---

## Custom Bar Characters

You can completely redesign how bars look by overriding the characters in `[chars]`.

**Minimal ASCII** (pure `+/-/|` style):

```toml
[display]
default_style = "ascii"

[chars]
ascii_top_left   = "+"
ascii_top_mid    = "-"
ascii_top_right  = "+"
ascii_body_left  = "|"
ascii_body_fill  = "#"
ascii_body_right = "|"
ascii_single_top  = "+"
ascii_single_body = "|"
```

**Hash bars** (dense `#` fill in block mode):

```toml
[display]
default_style = "block"

[chars]
block_fill = "#"
```

Any single character works — letters, symbols, Unicode, whatever your terminal supports.

---

## Adding a New Algorithm

1. Create `src/algorithms/<name>.rs` implementing:
   ```rust
   pub fn steps(data: &[usize]) -> Vec<SortStep>
   ```
2. Register it in `src/algorithms/mod.rs` inside `all_algorithms()`:
   ```rust
   Algorithm { name: "My Sort", key: "mysort", generate_steps: my_sort::steps }
   ```

That's it — the CLI flag, loop mode, and rendering all pick it up automatically.

---

## Contributing

Pull requests are welcome. Run `cargo clippy` and `cargo fmt` before submitting. For new features or bug reports, open an issue first so we can discuss the approach.
