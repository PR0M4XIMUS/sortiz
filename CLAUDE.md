# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build          # Debug build
cargo build --release  # Optimized binary (opt-level=2, codegen-units=1 in Cargo.toml)
cargo run -- [OPTIONS]
cargo test           # Runs algorithm correctness tests (3 tests covering all 7 algorithms)
cargo clippy
cargo fmt
```

**CLI options:**
```
--algorithm <ALGO>    bubble | insertion | selection | merge | quick | heap | shell
-n, --array-size <N> Number of elements (default: 50, clamped 5–500)
-s, --speed <MS>     Milliseconds per step (default: 50, clamped 5–5000)
-c, --config <PATH>  Custom config file path
-l, --loop           Cycle through algorithms automatically
```

## Architecture

**Data flow:** CLI args → `App::new()` pre-computes all animation steps → main loop polls events & advances `step_idx` → `ui::render()` draws the current frame.

**Key design decision:** Algorithms generate the full `Vec<SortStep>` upfront rather than yielding steps lazily. Each `SortStep` captures the full array state plus a set of indices being compared/swapped/marked-sorted, so the renderer only needs to read `step_idx` — no algorithm logic runs at render time.

**Adding a new algorithm:**
1. Create `src/algorithms/<name>.rs` implementing `pub fn steps(data: &[usize]) -> Vec<SortStep>`
2. Register it in `src/algorithms/mod.rs` → `all_algorithms()` vec

**Module responsibilities:**

| File | Responsibility |
|------|----------------|
| `main.rs` | Terminal setup (crossterm raw mode + alternate screen), event loop, keyboard + resize handling |
| `app.rs` | `App` struct: step index, speed, algorithm selection, pause state, auto-transition (1.5 s between algorithms in loop mode) |
| `ui.rs` | ratatui rendering: dynamic layout (title/bars/status rows conditionally included), centered bar group, fractional bar heights in block mode |
| `config.rs` | Loads `~/.config/sortiz/config.toml`; `ParsedDisplay` holds `gap`, `show_title`, `show_progress`; colors parsed from hex or named strings |
| `algorithms/mod.rs` | `Algorithm` struct and `all_algorithms()` registry |

**ui.rs bar layout** — bar width and gap are computed each frame from the current terminal width so resizing is seamless. When the configured gap can't fit all bars, the gap is dropped and bars fill the full width. Remaining horizontal space is split equally left/right to keep bars centered. Title and progress rows are omitted from the ratatui `Layout` entirely when `show_title`/`show_progress` are false (not just hidden — the space is reclaimed for bars).

**Config** lives at `~/.config/sortiz/config.toml`; see `config.example.toml` for the full schema. All fields are optional and fall back to built-in defaults. The three config sections are `[colors]` (6 slots), `[display]` (`default_style`, `gap`, `show_title`, `show_progress`), and `[chars]` (9 character overrides for block/ASCII bar drawing).
