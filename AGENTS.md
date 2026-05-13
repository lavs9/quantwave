# QuantWave: Project Instructions

QuantWave is a high-performance, Polars-native technical analysis library for Rust. It extends `talib-rs-core` with modern indicators, Ehlers DSP suites, and ML feature engineering tools, ensuring bit-identical results between batch processing (Polars) and real-time streaming.

## 🏗 Architecture Overview

The project is structured as a Rust workspace to maximize modularity and performance:

- **`quantwave-core`**: The engine containing core traits (`Next<T>`), state machines, and streaming implementations.
- **`quantwave-polars`**: High-level Polars integration providing the `.ta()` namespace on LazyFrames and Series.
- **`quantwave-plugins`**: Native Polars Expression Plugins (UDFs) for zero-copy, high-performance vectorized calculations.

## 🛠 Building and Running

### Prerequisites
- Rust (2024 edition)
- `cargo-expand` (recommended for macro debugging)

### Key Commands
- **Build All**: `cargo build`
- **Test All**: `cargo nextest run` (MANDATORY: Use nextest only)
- **Run Benchmarks**: `cargo bench`
- **Check Linting**: `cargo clippy`

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


This project uses bd (beads) for issue tracking.

- Run `bd prime` for workflow context and command guidance.
- Use `bd ready`, `bd show <id>`, `bd update <id> --claim`, and `bd close <id>`.
- Use `bd remember "insight"` for persistent project memory; do not create MEMORY.md files.
- Do not use markdown TODO lists for task tracking.

<!-- BEGIN BEADS INTEGRATION v:1 profile:full hash:d4f96305 -->
## Issue Tracking with bd (beads)

**IMPORTANT**: This project uses **bd (beads)** for ALL issue tracking. Do NOT use markdown TODOs, task lists, or other tracking methods.

### Why bd?

- Dependency-aware: Track blockers and relationships between issues
- Git-friendly: Dolt-powered version control with native sync
- Agent-optimized: JSON output, ready work detection, discovered-from links
- Prevents duplicate tracking systems and confusion

### Quick Start

**Check for ready work:**

```bash
bd ready --json
```

**Create new issues:**

```bash
bd create "Issue title" --description="Detailed context" -t bug|feature|task -p 0-4 --json
bd create "Issue title" --description="What this issue is about" -p 1 --deps discovered-from:bd-123 --json
```

**Claim and update:**

```bash
bd update <id> --claim --json
bd update bd-42 --priority 1 --json
```

**Complete work:**

```bash
bd close bd-42 --reason "Completed" --json
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

1. **Check ready work**: `bd ready` shows unblocked issues
2. **Claim your task atomically**: `bd update <id> --claim`
3. **Work on it**: Implement, test, document
4. **Reference Management**: When a task is completed, move the source paper/documentation from the original folder to the `implemented/` subfolder (e.g., `references/Ehlers Papers/implemented/`).
5. **Discover new work?** Create linked issue:
   - `bd create "Found bug" --description="Details about what was found" -p 1 --deps discovered-from:<parent-id>`
6. **Complete**: `bd close <id> --reason "Done"`

### Auto-Sync

bd automatically syncs via Dolt:

- Each write auto-commits to Dolt history
- Use `bd dolt push`/`bd dolt pull` for remote sync
- No manual export/import needed!

### Important Rules

- ✅ Use bd for ALL task tracking
- ✅ Always use `--json` flag for programmatic use
- ✅ Link discovered work with `discovered-from` dependencies
- ✅ Check `bd ready` before asking "what should I work on?"
- ❌ Do NOT create markdown TODO lists
- ❌ Do NOT use external issue trackers
- ❌ Do NOT duplicate tracking systems

For more details, see README.md and docs/QUICKSTART.md.

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd dolt push
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds

<!-- END BEADS INTEGRATION -->
