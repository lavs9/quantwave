# QuantWave: Project Instructions

QuantWave is a high-performance, Polars-native technical analysis library for Rust. It extends `talib-rs-core` with modern indicators, Ehlers DSP suites, and ML feature engineering tools, ensuring bit-identical results between batch processing (Polars) and real-time streaming.

## 🏗 Architecture Overview

The project is structured as a Rust workspace to maximize modularity and performance:

- **`quantwave-core`**: The engine containing core traits (`Next<T>`), state machines, and streaming implementations.
- **`quantwave-polars`**: High-level Polars integration providing the `.ta()` namespace on LazyFrames and Series.
- **`quantwave-backtest`**: Vectorized/streaming backtest engine + performance metrics.
- **`quantwave-py`**: The single PyO3 (abi3) extension. One cdylib exposes the indicator classes/batch fns (`quantwave._quantwave`), the backtest bindings (`quantwave._backtest`), and the native `#[polars_expr]` plugins behind the `pl.col().ta` namespace. Built with one `maturin build` → one `cp39-abi3` wheel (no wheel-merge step).

## 🛠 Building and Running

### Prerequisites
- Rust (2024 edition)
- `cargo-expand` (recommended for macro debugging)

### Key Commands
- **Build All**: `cargo build`
- **Test All**: `cargo nextest run` (MANDATORY: Use nextest only)
- **Install pre-push gate**: `./scripts/install-git-hooks.sh` — runs verify before each push (CI runs fast doc/metadata sanity only)
- **Verify (full gate)**: `./scripts/quantwave_verify.sh` — metadata drift + nextest + pytest smoke (cached via `scripts/verify_cache.py`; `VERIFY_NO_CACHE=1` to force) (cached via `scripts/verify_cache.py`; `VERIFY_NO_CACHE=1` to force)
- **Run Benchmarks**: `cargo bench`
- **Check Linting**: `cargo clippy`
- **Check Formatting**: `./scripts/rustfmt_check.sh` — ⚠️ **never `cargo fmt --all`**. `quantwave-core/src/options_india/iv_solver.rs` holds Horner-form polynomial approximations (~12 levels of nested parens on 500-char lines) that make rustfmt spin at 100% CPU indefinitely. rustfmt follows `mod` declarations, so `src/lib.rs` and `options_india/mod.rs` inherit the hang and the whole crate becomes unformattable via cargo. This script batches and skips those paths — full workspace in ~1.2s. Interrupted `cargo fmt` runs leave **orphaned rustfmt children** burning CPU; clean up with `pkill -f "bin/rustfmt"; pkill -f cargo-fmt`.

## 🧪 Development Conventions

### 1. The "Universal Indicator" Pattern
Every indicator must implement the `Next<Input>` trait. This single source of mathematical truth powers both the streaming structs and the Polars plugins.

### 2. Parity & Validation
- **Streaming-Batch Parity**: Every indicator must have a `proptest` that asserts `Batch(data) == Streaming.collect(data)` using `approx` tolerances.
- **Gold Standard**: Reference data is stored in `quantwave-core/tests/gold_standard/*.json`. All implementations must match these industry-standard vectors.
- **Tests Location**: ALL integration tests and gold standard files MUST reside in `quantwave-core/tests/`. Root-level `tests/` folders are prohibited.
- SOURCE of calculation for all indicators must be recorded. IF you do not have a source do not assume, validate with the human before assuming the source. Research and give options for source.

### Indicator Formula References
When implementing indicators, refer to the following authoritative sources for calculation logic and edge-case handling:
- **TradingView (Pine Script):** De facto standard for retail algorithmic trading.
- **Devexperts:** https://devexperts.com/dxcharts/kb/docs/indicators
- **Sierra Chart:** https://www.sierrachart.com/index.php?page=doc/TechnicalStudiesReference.php
- **QuantConnect:** https://www.quantconnect.com/docs/v2/writing-algorithms/indicators/supported-indicators/wave-trend-oscillator
- **MQL5:** https://www.mql5.com/en/articles/indicators
- **StockCharts:** https://chartschool.stockcharts.com/table-of-contents/overview

### 3. Depth over Breadth
Prioritize generic, extensible components. For example, moving averages should support swappable smoothing algorithms (SMA, EMA, HMA) via the `SmoothingAlgorithm` trait.

### 4. Performance
- Use **Polars Expression Plugins** for all custom vectorized logic.
- Avoid memory copies; operate directly on Arrow buffers.
- Leverage `talib-rs-core`'s SIMD-optimized foundations for classic indicators.

## 🗺 Roadmap (Phase 1)
- [ ] Initialize workspace and foundational traits.
- [ ] Implement `SuperTrend` as the "Steel Thread" indicator.
- [ ] Establish the `gold_standard` testing infrastructure.


This project uses br (beads_rust) for issue tracking. br replaced bd
(beads/Dolt) on 2026-07-26 after the bd Dolt store's journal became
corrupted; issues were recovered from a pre-corruption JSONL export and
migrated into br's SQLite + JSONL store. bd's original data is preserved
(not deleted) in `.beads.bak-20260726-092341/` and
`.beads.bd-legacy-20260726/` at the repo root.

- Run `br robot-docs guide` for workflow context and command guidance.
- Use `br ready`, `br show <id>`, `br update <id> --claim`, and `br close <id>`.
- br has no `bd remember` equivalent; use `br create --type task` (or a comment via `br comments add <id>`) to persist project memory instead of MEMORY.md files.
- Do not use markdown TODO lists for task tracking.
- br is **non-invasive**: it never commits, pushes, pulls, or installs git hooks on its own. After mutating commands (which auto-flush `.beads/issues.jsonl` by default), remember to `git add .beads/ && git commit` yourself when you want the JSONL export captured in git history.

<!-- BEGIN BR INTEGRATION (migrated from bd 2026-07-26) -->
## Issue Tracking with br (beads_rust)

**IMPORTANT**: This project uses **br (beads_rust)** for ALL issue tracking. Do NOT use markdown TODOs, task lists, or other tracking methods.

### Why br?

- Dependency-aware: Track blockers and relationships between issues
- Git-friendly: SQLite (fast local queries) + JSONL export (clean git diffs/merges)
- Agent-optimized: JSON output (`--json` everywhere), ready work detection, discovered-from links, `br robot-docs`
- Prevents duplicate tracking systems and confusion

### Quick Start

**Check for ready work:**

```bash
br ready --json
```

**Create new issues:**

```bash
br create "Issue title" --description="Detailed context" -t bug|feature|task -p 0-4 --json
br create "Issue title" --description="What this issue is about" -p 1 --deps discovered-from:quantwave-123 --json
```

**Claim and update:**

```bash
br update <id> --status in_progress --json
br update quantwave-42 --priority 1 --json
```

**Complete work:**

```bash
br close quantwave-42 --reason "Completed" --json
```

### Issue Types

- `bug` - Something broken
- `feature` - New functionality
- `task` - Work item (tests, docs, refactoring)
- `epic` - Large feature with subtasks
- `chore` - Maintenance (dependencies, tooling)

### Priorities

- `0` - Critical (security, data loss, broken builds)
- `1` - High (major features, important bugs)
- `2` - Medium (default, nice-to-have)
- `3` - Low (polish, optimization)
- `4` - Backlog (future ideas)

### Workflow for AI Agents

1. **Check ready work**: `br ready` shows unblocked issues
2. **Claim your task atomically**: `br update <id> --claim`
3. **Work on it**: Implement, test, document
4. **Reference Management**: When a task is completed, move the source paper/documentation from the original folder to the `implemented/` subfolder (e.g., `references/Ehlers Papers/implemented/`).
5. **Discover new work?** Create linked issue:
   - `br create "Found bug" --description="Details about what was found" -p 1 --deps discovered-from:<parent-id>`
6. **Complete**: `br close <id> --reason "Done"`

### Sync (Git-Explicit, Not Automatic)

Unlike bd (which auto-committed to a Dolt server), **br never touches git on its own**:

- Mutating commands (`create`, `update`, `close`, `dep add`, ...) auto-flush `.beads/issues.jsonl` locally by default.
- Nothing is committed or pushed until you run `git add .beads/ && git commit` yourself.
- Run `br sync --flush-only` as an idempotent final export check before staging `.beads/` (useful after `--no-auto-flush` runs or before landing the plane).
- There is no `bd dolt push`/`bd dolt pull` equivalent — plain `git push`/`git pull` on `.beads/issues.jsonl` is the sync mechanism now.

### Important Rules

- ✅ Use br for ALL task tracking
- ✅ Always use `--json` flag for programmatic use
- ✅ Link discovered work with `discovered-from` dependencies
- ✅ Check `br ready` before asking "what should I work on?"
- ✅ `git add .beads/ && git commit` after issue-tracker mutations you want preserved — br will not do this for you
- ❌ Do NOT create markdown TODO lists
- ❌ Do NOT use external issue trackers
- ❌ Do NOT duplicate tracking systems

For more details, see README.md and docs/QUICKSTART.md.

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update IndicatorMetadata for Rust indicators** — If any new indicator is added or an existing one is significantly changed (in `quantwave-core/src/indicators/` or related), you **MUST** create or update its `XXX_METADATA` constant (of type `IndicatorMetadata`) in the same session. This metadata is the single source of truth consumed by documentation (mkdocs) and the Python package. See task `quantwave-i9dn`.
4. **Update issue status** - Close finished work, update in-progress items
5. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   br sync --flush-only
   git add .beads/ && git commit -m "sync beads" || true  # no-op if nothing changed
   git push
   git status  # MUST show "up to date with origin"
   ```
6. **Clean up** - Clear stashes, prune remote branches
7. **Verify** - All changes committed AND pushed
8. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds

- **IndicatorMetadata rule (quantwave-i9dn)**: Adding or modifying any Rust indicator without also creating/updating its `IndicatorMetadata` (the `XXX_METADATA` constant) is not permitted. This metadata is the source of truth for both documentation and Python DX. It must be done as part of landing the plane, before `git push`. See task `quantwave-i9dn`.

<!-- END BR INTEGRATION -->
