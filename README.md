# sortiz

A terminal UI sorting algorithm visualizer with smooth animations, 20 algorithms, audio feedback, race mode, and full theme support.

---

## Installation

### Arch Linux (AUR)

```bash
paru -S sortiz-git
# or
yay -S sortiz-git
```

### Build from source

Requires [Rust](https://rustup.rs) (stable) and `alsa-lib` (for audio):

```bash
git clone https://github.com/PR0M4XIMUS/sortiz.git
cd sortiz
cargo build --release
cp target/release/sortiz ~/.local/bin/
```

To build without audio (no ALSA dependency):

```bash
cargo build --release --no-default-features
```

---

## Usage

```
sortiz [OPTIONS]
```

| Flag | Description | Default |
|------|-------------|---------|
| `-a, --algorithm <KEY>` | Algorithm to visualize (see list below) | random |
| `-n, --array-size <N>` | Number of elements (5–500) | `50` |
| `-s, --speed <MS>` | Milliseconds per step (5–5000) | `50` |
| `-c, --config <PATH>` | Path to a custom config file | `~/.config/sortiz/config.toml` |
| `-l, --loop-mode` | Cycle through algorithms automatically | off |
| `--seed <U64>` | Reproducible shuffle seed | random |
| `--distribution <KIND>` | Initial array shape (see below) | `uniform` |
| `--mute` | Start with audio muted | off |
| `--list` | Print all algorithm keys and exit | — |
| `--benchmark` | Headless step/comparison/swap counts for all algorithms | — |
| `--race` | Race mode: all algorithms compete side-by-side | off |
| `--demo` | Auto-demo: loop through all algorithms without interaction | off |
| `--generate-completions <SHELL>` | Print shell completions (`bash`/`zsh`/`fish`/…) and exit | — |

**Distributions:** `uniform` · `reversed` · `nearly-sorted` · `few-unique` · `sawtooth` · `sorted` · `worst-case`

**Examples:**

```bash
sortiz                                        # random algorithm, default settings
sortiz --algorithm quick -n 100 -s 25        # quick sort, 100 bars, fast
sortiz --loop-mode                            # cycle through all algorithms
sortiz --seed 42 --distribution reversed     # reproducible reversed input
sortiz --race -n 30                          # race mode, 30 elements
sortiz --benchmark -n 200                    # headless comparison counts
sortiz -c ~/my-theme.toml                    # use a custom config file
```

---

## Algorithms

| Name | Key | Complexity | Description |
|------|-----|------------|-------------|
| Bubble Sort | `bubble` | O(n²) | Repeatedly swaps adjacent out-of-order pairs until sorted. |
| Insertion Sort | `insertion` | O(n²) | Builds sorted list by inserting each element leftward into place. |
| Selection Sort | `selection` | O(n²) | Finds the minimum of the unsorted portion and swaps it into place. |
| Merge Sort | `merge` | O(n log n) | Divides array in half, sorts each half, merges them in order. |
| Quick Sort | `quick` | O(n log n) | Picks a pivot and partitions elements smaller/larger to each side. |
| Heap Sort | `heap` | O(n log n) | Builds a max-heap, then extracts the maximum repeatedly to the end. |
| Shell Sort | `shell` | O(n log² n) | Insertion sort with shrinking gap — sorts far-apart elements first. |
| Roulette Sort | `roulette` | O(n²) | Spins the unsorted reel like a slot machine — rigged to always win. |
| Cocktail Sort | `cocktail` | O(n²) | Bidirectional bubble sort — sweeps left→right then right→left. |
| Comb Sort | `comb` | O(n log n) | Bubble sort with a shrinking gap to eliminate turtles early. |
| Pancake Sort | `pancake` | O(n²) | Sorts by flipping prefixes to bring the max to the front, then drop. |
| Gnome Sort | `gnome` | O(n²) | A gnome steps forward, swaps if out of order, then steps back. |
| Stalin Sort | `stalin` | O(n) | Exiles non-conforming elements to the tail, then re-integrates them. |
| Radix Sort | `radix` | O(nk) | Sorts digit by digit from least to most significant (LSD base-10). |
| Cycle Sort | `cycle` | O(n²) | Minimizes writes by cycling each element to its final position. |
| Bitonic Sort | `bitonic` | O(n log² n) | Comparator network — builds a bitonic sequence then merges it. |
| Tim Sort | `tim` | O(n log n) | Hybrid of insertion sort on small runs + merge. Used in Python/Java. |
| Intro Sort | `intro` | O(n log n) | Starts quicksort, switches to heapsort when depth limit exceeded. |
| Sleep Sort | `sleep` | O(n + max) | Each element "sleeps" proportional to its value before joining output. |
| Bogo Sort | `bogo` | O((n+1)!) | Randomly shuffles until sorted. Pray for a short run. |

---

## Keyboard Controls

| Key | Action |
|-----|--------|
| `Space` | Pause / Resume |
| `Left` / `Right` | Step back / forward (when paused) |
| `Shift+Left` / `Shift+Right` | Jump back / forward 10 steps (when paused) |
| `↑` / `↓` | Speed up / slow down (halve or double delay) |
| `+` / `-` | Fine speed adjust (±10 ms) |
| `r` | Restart with the same seed |
| `R` | Restart with a new random shuffle |
| `n` | Jump to a random different algorithm |
| `[` / `]` | Cycle algorithms backward / forward |
| `m` | Toggle mute |
| `b` | Toggle rainbow bar coloring |
| `s` | Toggle stats row (comparisons / swaps) |
| `t` | Toggle title / complexity row |
| `?` / `h` | Help overlay |
| `q` / `Esc` / `Ctrl+C` | Quit |

**Startup menu:** `↑↓` navigate, `Enter` confirm, `1`/`2` select Block/ASCII directly, `q`/`Esc` quit.

---

## Audio

sortiz plays tones as it sorts — pitch maps to bar value, so higher elements sound higher.

- On sort completion a higher chime plays.
- Press `m` to toggle mute at any time, or start muted with `--mute`.
- Requires ALSA (`alsa-lib`) on Linux. Falls back to a terminal bell (`\x07`) if audio init fails.
- Disable at compile time with `--no-default-features` (no ALSA dependency).

Configure in `~/.config/sortiz/config.toml`:

```toml
[audio]
enabled = true      # false to disable completely
volume  = 0.5       # 0.0–1.0
backend = "auto"    # "auto" | "rodio" | "bel" | "silent"
```

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

# Show the algorithm name and complexity above the bars (default: true)
# show_title = true

# Show comparison/swap stats row (default: true)
# show_stats = true

# Show the step progress counter below the bars (default: true)
# show_progress = true

# Show complexity in title row (default: true)
# show_complexity = true

# Show the random seed value in the title row (default: false)
# show_seed = false

# Rainbow coloring: bar color maps to value (default: false)
# rainbow = false

[audio]
# enabled = true      # false to silence completely
# volume  = 0.5       # 0.0–1.0
# backend = "auto"    # "auto" | "rodio" | "bel" | "silent"

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
# ascii_single_top  = "╤"
# ascii_single_body = "│"
```

---

## Themes

Ready-made theme files live in the [`themes/`](themes/) directory. To apply one:

```bash
mkdir -p ~/.config/sortiz
cp themes/catppuccin-mocha.toml ~/.config/sortiz/config.toml
```

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

---

## Shell Completions

Generate and install completions for your shell:

```bash
# bash
sortiz --generate-completions bash >> ~/.bash_completion

# zsh
sortiz --generate-completions zsh > ~/.zfunc/_sortiz

# fish
sortiz --generate-completions fish > ~/.config/fish/completions/sortiz.fish
```

---

## Adding a New Algorithm

1. Create `src/algorithms/<name>.rs` implementing:
   ```rust
   pub fn steps(data: &[usize]) -> Vec<SortStep>
   ```
2. Add `pub mod <name>;` in `src/algorithms/mod.rs`
3. Add an `Algorithm { name, key, complexity, description, generate_steps }` entry to `all_algorithms()`
4. Add `("<key>", <name>::steps)` to the `ALGOS` const in the test module

The CLI flag, loop mode, race mode, and rendering all pick it up automatically.

---

## Recent Changes

- **20 algorithms** — added Radix, Cycle, Bitonic, Tim, Intro, Sleep, and Bogo sort
- **Audio** — real-time tone synthesis via rodio; pitch maps to bar value; BEL fallback; `m` to mute
- **Race mode** (`--race`) — all algorithms compete in a grid, same starting array
- **Step scrubbing** — pause and step frame-by-frame with `Left`/`Right`; jump 10 with `Shift`
- **Stats row** — live comparisons and swap counts per step
- **Help overlay** (`?`) — in-app keybinding reference
- **Post-sort summary** — comparisons, swaps, steps, and wall time on completion
- **Sequential algorithm cycling** (`[`/`]`) — browse algorithms in order
- **Fine speed control** (`+`/`-`) — ±10 ms adjustments alongside the halve/double `↑`/`↓`
- **Rainbow bars** (`b`) — HSL coloring by value
- **Reproducible seeds** (`--seed`) — same input every run
- **Distributions** (`--distribution`) — reversed, nearly-sorted, few-unique, sawtooth, worst-case
- **Benchmark mode** (`--benchmark`) — headless step/comparison/swap stats for all algorithms
- **Shell completions** (`--generate-completions`) — bash, zsh, fish, powershell, elvish
- **Theme-aware startup menu** — menu respects your `[colors]` config

---

## Contributing

Pull requests are welcome. Run `cargo clippy` and `cargo fmt` before submitting. For new features or bug reports, open an issue first so we can discuss the approach.
