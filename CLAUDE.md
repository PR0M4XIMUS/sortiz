# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build                 # Debug build
cargo build --release       # Optimized (opt-level=2; see note below)
cargo run -- [OPTIONS]      # Run with CLI options
cargo test                  # Run all 3 algorithm correctness tests
cargo clippy                # Lint
cargo fmt                   # Format
```

**CLI options:**
```
--algorithm <ALGO>    bubble | insertion | selection | merge | quick | heap | shell
-n, --array-size <N> Number of elements (default: 50, clamped 5–500)
-s, --speed <MS>     Milliseconds per step (default: 50, clamped 5–5000)
-c, --config <PATH>  Custom config file path
-l, --loop           Cycle through algorithms automatically
```

**Build profile note:** `Cargo.toml` has `[profile.release]` set to `opt-level = 2` and `codegen-units = 1`. This is a workaround for Arch Linux's rustc 1.94.1 which crashes at opt-level=3 during trait monomorphization. On other systems, opt-level=3 is safe but not necessary for this small TUI.

## Tests

Three correctness tests in `src/algorithms/mod.rs`:
- `all_sort_correctly` — verifies all 7 algorithms sort correctly across 9 input cases (empty, single, reversed, duplicates, already-sorted, etc.)
- `steps_never_empty` — guards against algorithms producing no animation frames
- `each_step_has_correct_length` — ensures array length never changes mid-sort

Run with: `cargo test`

## Architecture

**Data flow:** CLI args → `App::new()` pre-computes all animation steps → main loop polls events & advances `step_idx` → `ui::render()` draws the current frame.

**Key design decision:** Algorithms generate the full `Vec<SortStep>` upfront rather than yielding steps lazily. Each `SortStep` captures the full array state plus sets of indices being compared/swapped/marked-sorted, so the renderer only needs to read `step_idx` — no algorithm logic runs at render time. This bounds memory use to 100,000 steps per algorithm.

**Adding a new algorithm:**
1. Create `src/algorithms/<name>.rs` implementing `pub fn steps(data: &[usize]) -> Vec<SortStep>`
2. Register it in `src/algorithms/mod.rs` → `all_algorithms()` vec
3. Add test cases to `algorithms::tests::all_sort_correctly` if testing specific edge cases

**Module responsibilities:**

| File | Responsibility |
|------|----------------|
| `main.rs` | Terminal setup (crossterm raw mode + alternate screen), event loop, keyboard + resize event handling |
| `app.rs` | `App` struct: step index, speed, algorithm selection, pause state, auto-transition (1.5 s delay between algorithms in loop mode), step capping (max 100,000) |
| `ui.rs` | ratatui rendering: dynamic layout (title/bars/status rows conditionally included based on config), centered bar group, fractional bar heights in block mode, seamless resize support |
| `config.rs` | Loads `~/.config/sortiz/config.toml`; `ParsedDisplay` holds `gap`, `show_title`, `show_progress`; colors parsed from hex or named strings; falls back to built-in defaults |
| `algorithms/mod.rs` | `Algorithm` struct, `all_algorithms()` registry, integration tests |

## Recent Features (v0.1.0)

- **Live terminal resize** (`Event::Resize`) — tiling WM splits redraw cleanly without artifacts. Bar width recalculated every frame from `frame.area()`.
- **Centered bars** — remaining horizontal space split equally left/right; no more empty right margin on narrow terminals.
- **Hideable UI elements** — `show_title` and `show_progress` in config remove the algorithm label and/or step counter entirely, reclaiming the space for bars.
- **Seamless responsive layout** — gap auto-drops if it doesn't fit; all 3 UI rows conditionally included in the layout instead of just hidden.
- **Full test coverage** — all 7 algorithms verified on 9 input cases per test run.

## Configuration

Lives at `~/.config/sortiz/config.toml`; see `config.example.toml` for the full schema. All fields are optional with sensible built-in defaults.

Three sections:
- `[colors]` — 6 color slots (bar, comparing, swapping, sorted, background, text); accepts `#rrggbb` hex, named colors, or `"default"` (transparent)
- `[display]` — `default_style` (`"block"`/`"ascii"`), `gap` (0–5 columns), `show_title` (bool), `show_progress` (bool)
- `[chars]` — 9 character overrides for block/ASCII bar drawing

## AUR Package

The package `sortiz-git` is published to the Arch User Repository (AUR) and builds from HEAD. 

**Maintenance workflow:**
- Tag releases with `git tag vX.Y.Z`
- The PKGBUILD's `pkgver()` function automatically generates version strings from `git rev-list --count HEAD` and current commit SHA
- `.SRCINFO` must be regenerated and committed whenever PKGBUILD metadata changes: `makepkg --printsrcinfo > .SRCINFO`
- Push both `PKGBUILD` and `.SRCINFO` to `ssh://aur@aur.archlinux.org/sortiz-git.git` on the `master` branch

## Cargo.lock

Committed to the repository for reproducible builds. Do not remove it.

## Benchmarking & Profiling

None implemented yet. Algorithm performance varies wildly (bubble sort: O(n²), quick sort: O(n log n)). The 100,000-step cap prevents excessive memory use on large arrays; consider profiling if adding features that generate more steps.
