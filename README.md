# sortiz

A terminal UI sorting algorithm visualizer with smooth animations, multiple algorithms, and full theme support via a simple TOML config file.

---

## Installation

### Arch Linux (AUR)

```bash
paru -S sortiz-git
# or
yay -S sortiz-git
```

### Build from source

Requires [Rust](https://rustup.rs) (stable):

```bash
git clone https://github.com/PR0M4XIMUS/sortiz.git
cd sortiz
cargo build --release
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

**Available algorithms:** `bubble` · `insertion` · `selection` · `merge` · `quick` · `heap` · `shell` · `roulette`

**Examples:**

```bash
sortiz                                  # random algorithm, default settings
sortiz --algorithm quick -n 100 -s 25  # quick sort, 100 bars, fast
sortiz --loop                           # cycle through all algorithms
sortiz -c ~/my-theme.toml              # use a custom config file
```

---

## Algorithms

| Name | Key | How it works |
|------|-----|--------------|
| Bubble Sort | `bubble` | Repeatedly steps through the list comparing adjacent pairs and swapping them if out of order. Each pass bubbles the largest unsorted element to the end. |
| Insertion Sort | `insertion` | Builds the sorted list one element at a time by taking each element and shifting it left until it's in the correct position. |
| Selection Sort | `selection` | Finds the minimum element from the unsorted portion and swaps it into the next sorted position. |
| Merge Sort | `merge` | Divides the array in half recursively, sorts each half, then merges them back together in order. |
| Quick Sort | `quick` | Picks a pivot, partitions elements smaller/larger than the pivot to either side, then recurses on each partition. |
| Heap Sort | `heap` | Builds a max-heap from the array, then repeatedly extracts the maximum to the end, shrinking the heap each time. |
| Shell Sort | `shell` | A generalization of insertion sort that starts by sorting elements far apart, then progressively reduces the gap until it becomes a standard insertion sort. |
| Roulette Sort | `roulette` | Goes index by index and spins (shuffles) the remaining unsorted elements randomly. After each spin it checks whether the right value landed in place — some spins fail for drama, but the wheel is secretly rigged so it always sorts in the end. |
| Cocktail Shaker Sort | `cocktail` | Bidirectional bubble sort — sweeps left→right then right→left each pass, closing the sorted region from both ends simultaneously. |
| Comb Sort | `comb` | Like bubble sort but starts comparing elements far apart (gap = n/1.3) and shrinks the gap each pass until it becomes a standard bubble-sort polish. |
| Pancake Sort | `pancake` | Sorts by flipping prefixes: finds the largest unsorted element, flips the prefix to bring it to the front, then flips again to drop it into place — each flip animated bar by bar. |
| Gnome Sort | `gnome` | A garden gnome moves forward until it finds an out-of-order pair, swaps them, then steps back one position to recheck — wandering erratically until everything is in order. |
| Stalin Sort | `stalin` | Scans left to right and eliminates any element that isn't in order. Eliminated elements are quietly re-integrated at the end once the economy collapses. |

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

# Show the algorithm name above the bars (default: true)
# show_title = true

# Show the step progress counter below the bars (default: true)
# show_progress = true

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

Ready-made theme files live in the [`themes/`](themes/) directory. To apply one:

```bash
mkdir -p ~/.config/sortiz
cp themes/catppuccin-mocha.toml ~/.config/sortiz/config.toml
```

Or paste a `[colors]` block directly into your existing config.

<details>
<summary><strong>Catppuccin Mocha</strong></summary>

```toml
[colors]
bar        = "#cdd6f4"
comparing  = "#89b4fa"
swapping   = "#f38ba8"
sorted     = "#a6e3a1"
background = "#1e1e2e"
text       = "#cdd6f4"
```
</details>

<details>
<summary><strong>Gruvbox Dark</strong></summary>

```toml
[colors]
bar        = "#ebdbb2"
comparing  = "#83a598"
swapping   = "#fb4934"
sorted     = "#b8bb26"
background = "#282828"
text       = "#ebdbb2"
```
</details>

<details>
<summary><strong>Nord</strong></summary>

```toml
[colors]
bar        = "#d8dee9"
comparing  = "#81a1c1"
swapping   = "#bf616a"
sorted     = "#a3be8c"
background = "#2e3440"
text       = "#eceff4"
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

## Recent Changes

- **Roulette Sort** — a new novelty algorithm that spins (shuffles) the remaining unsorted elements at each position. The wheel is secretly rigged: it fails dramatically a few times before guaranteeing a win, so it always finishes but keeps you guessing.
- **Theme files** — ready-made Catppuccin Mocha, Gruvbox Dark, and Nord configs in the `themes/` directory, ready to copy into `~/.config/sortiz/`.
- **Live resize** — the visualizer now reacts to terminal resize events instantly, making it fully compatible with tiling window managers (Hyprland, i3, sway, etc.).
- **Centered bars** — leftover horizontal space is distributed equally on both sides.
- **Hide UI elements** — `show_title` and `show_progress` config options let you strip the algorithm label and/or step counter.

---

## Contributing

Pull requests are welcome. Run `cargo clippy` and `cargo fmt` before submitting. For new features or bug reports, open an issue first so we can discuss the approach.
