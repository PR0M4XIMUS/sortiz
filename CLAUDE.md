# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build                           # Debug build
cargo build --release                 # Optimized (opt-level=2; see note below)
cargo build --no-default-features     # Silent build (no ALSA dep)
cargo run -- [OPTIONS]                # Run with CLI options
cargo test                            # Run all 3 algorithm correctness tests
cargo clippy                          # Lint
cargo fmt                             # Format
```

**Key CLI options:**
```
--algorithm <KEY>        bubble | insertion | selection | merge | quick | heap | shell |
                         roulette | cocktail | comb | pancake | gnome | stalin |
                         radix | cycle | bitonic | tim | intro | sleep | bogo
-n, --array-size <N>     Number of elements (default: 50, clamped 5–500)
-s, --speed <MS>         Milliseconds per step (default: 50, clamped 5–5000)
-c, --config <PATH>      Custom config file path
-l, --loop-mode          Cycle through algorithms automatically
--seed <U64>             Reproducible shuffle seed
--distribution <KIND>    uniform | reversed | nearly-sorted | few-unique | sawtooth | sorted | worst-case
--mute                   Start with audio muted
--list                   Print all algorithm keys and exit
--benchmark              Headless step/comparison/swap counts for all algorithms
--race                   Race mode: all algorithms compete side-by-side
--demo                   Auto-demo: loop through all algorithms without interaction
--generate-completions   Print shell completions (bash/zsh/fish/…) and exit
```

**Keybindings (visualizer):**

| Key | Action |
|-----|--------|
| `Space` | Toggle pause |
| `Left` / `Right` | Step back / forward (paused only) |
| `Shift+Left` / `Shift+Right` | Jump 10 steps back / forward (paused only) |
| `Up` / `Down` | Speed up / slow down (halve or double `speed_ms`) |
| `+` / `-` | Fine speed ±10 ms |
| `r` | Restart with same seed |
| `R` | Restart with new random shuffle |
| `n` | Jump to a random different algorithm |
| `[` / `]` | Cycle algorithms backward / forward |
| `m` | Toggle mute |
| `b` | Toggle rainbow bars |
| `s` | Toggle stats row |
| `t` | Toggle title / complexity row |
| `?` / `h` | Help overlay |
| `q` / `Esc` / `Ctrl+C` | Quit |

**Startup menu:** `↑↓` navigate, `Enter` confirm, `1`/`2` select Block/ASCII directly, `q`/`Esc` quit.

**Build profile note:** `Cargo.toml` has `[profile.release]` set to `opt-level = 2` and `codegen-units = 1`. This is a workaround for Arch Linux's rustc 1.94.1 which crashes at opt-level=3 during trait monomorphization. On other systems, opt-level=3 is safe but not necessary for this small TUI.

## Tests

Three correctness tests in `src/algorithms/mod.rs`:
- `all_sort_correctly` — verifies all 20 algorithms sort correctly across 9 input cases (empty, single, reversed, duplicates, already-sorted, etc.). Uses `result == expected` so algorithms must produce a fully sorted array, not a subsequence.
- `steps_never_empty` — guards against algorithms producing no animation frames
- `each_step_has_correct_length` — ensures array length never changes mid-sort

Run with: `cargo test`

**Adding a new algorithm automatically includes it in all three tests — but only if you add it to both `all_algorithms()` AND the `ALGOS` const in the test module.** These two registries are maintained manually and must be kept in sync.

## Architecture

**Data flow:** CLI args → `App::new()` pre-computes all animation steps → main loop polls events & advances `step_idx` → `ui::render()` draws the current frame.

**Key design decision:** Algorithms generate the full `Vec<SortStep>` upfront rather than yielding steps lazily. Each `SortStep` captures the full array state plus sets of indices being compared/swapped/marked-sorted, so the renderer only needs to read `step_idx` — no algorithm logic runs at render time. This bounds memory use to 100,000 steps per algorithm (`MAX_STEPS` in `algorithms/mod.rs`).

**`SortStep` fields** (defined in `src/algorithms/mod.rs`):
- `data: Vec<usize>` — full array state for this frame
- `comparing: Vec<usize>` — indices highlighted in the "comparing" color
- `swapping: Vec<usize>` — indices highlighted in the "swapping" color
- `sorted: Vec<usize>` — indices confirmed in their final sorted position
- `comparisons: u32` — cumulative comparisons up to this step
- `swaps: u32` — cumulative swaps/writes up to this step

Array length must never change across steps — the `each_step_has_correct_length` test enforces this. Algorithms that conceptually delete elements (e.g. Stalin Sort) must maintain length by swapping deleted elements to a holding zone.

**Adding a new algorithm:**
1. Create `src/algorithms/<name>.rs` implementing `pub fn steps(initial: &[usize]) -> Vec<SortStep>`
2. Add `pub mod <name>;` in `src/algorithms/mod.rs`
3. Add an `Algorithm { name, key, complexity, description, generate_steps }` entry to `all_algorithms()`
4. Add `("<key>", <name>::steps)` to the `ALGOS` const in the test module

**Module responsibilities:**

| File | Responsibility |
|------|----------------|
| `main.rs` | Terminal setup, startup menu, main event loop, all keybindings, one-shot CLI modes (benchmark, list, race, completions) |
| `app.rs` | `App` / `RaceApp` structs, step index, speed, algorithm selection, pause state, audio player, seed, distribution, `build_array()`, summary tracking |
| `ui.rs` | ratatui rendering: title/stats/progress rows, bar widget, help overlay, summary overlay, race grid layout, rainbow color mapping |
| `audio.rs` | `Player` struct: rodio backend (feature-gated), BEL fallback, silent mode, pitch-per-step synthesis, mute toggle |
| `config.rs` | Loads `~/.config/sortiz/config.toml`; parses colors, display, chars, audio; `hsl_to_color()` helper |
| `algorithms/mod.rs` | `SortStep` / `Algorithm` structs, `MAX_STEPS` const, `all_algorithms()` registry, 3 integration tests |

**Known rough edges:**
- `Config::load_from` calls `toml::from_str(...).unwrap_or_default()` — malformed config silently falls back to defaults with no warning.
- The run loop polls at `Duration::from_millis(1)` — tight poll; step advancement is driven by elapsed time against `speed_ms`, not by the poll interval.
- Bogo sort caps its working set at 8 elements for performance; for inputs > 8 elements, the tail beyond position 7 is pre-sorted and not shuffled. The final frame is always correctly sorted.
- Bitonic sort pads to next-power-of-two with sentinel values internally; sentinel positions that temporarily appear in the visible region are rendered as zero-height bars.

## Configuration

Lives at `~/.config/sortiz/config.toml`; see `config.example.toml` for the full schema. All fields are optional with sensible built-in defaults.

Four sections:
- `[colors]` — 6 color slots (bar, comparing, swapping, sorted, background, text); accepts `#rrggbb` hex, named colors, or `"default"` (transparent)
- `[display]` — `default_style`, `gap`, `show_title`, `show_stats`, `show_progress`, `show_complexity`, `show_seed`, `rainbow`
- `[chars]` — 9 character overrides for block/ASCII bar drawing
- `[audio]` — `enabled`, `volume`, `backend` (`auto`/`rodio`/`bel`/`silent`)

Ready-made theme files are in `themes/` (Catppuccin Mocha, Gruvbox Dark, Nord) — copy one to `~/.config/sortiz/config.toml` to apply.

Setting `default_style` skips the startup menu entirely.

## Audio Feature

Audio uses rodio 0.17 and is enabled by default (`features = ["audio"]`). Build with `--no-default-features` for a binary with no ALSA dependency (audio silently disabled).

The `audio` feature gates:
- `rodio` dependency in `Cargo.toml`
- `Backend::Rodio` variant and all rodio imports in `audio.rs` (`#[cfg(feature = "audio")]`)

## AUR Package

The package `sortiz-git` is published to the Arch User Repository (AUR) and builds from HEAD.

**Maintenance workflow:**
- The PKGBUILD's `pkgver()` generates version strings automatically from `git rev-list --count HEAD` and current commit SHA — no manual tagging needed
- `.SRCINFO` must be regenerated and committed whenever `PKGBUILD` metadata changes: `makepkg --printsrcinfo > .SRCINFO`
- Push both `PKGBUILD` and `.SRCINFO` to `ssh://aur@aur.archlinux.org/sortiz-git.git` on the `master` branch
- `sortiz-git` is a VCS package — AUR users get the latest commit automatically on `paru -Syu`; no AUR push needed for code-only changes

**PKGBUILD notes:**
- `depends=('gcc-libs' 'alsa-lib')` — ALSA required for audio feature
- `options=('!debug')` — prevents Arch's makepkg from injecting `CARGO_PROFILE_RELEASE_DEBUG=2` and `-C force-frame-pointers=yes` via `DEBUG_RUSTFLAGS`, which trigger a SIGSEGV in rustc 1.94.1's LLVM codegen
- `[profile.release]` in `Cargo.toml` uses `opt-level = 2` (not 3) for the same rustc bug

## Cargo.lock

Committed to the repository for reproducible builds. Do not remove it.
