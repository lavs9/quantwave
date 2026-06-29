# ADR: Shared-Capital Portfolio Simulation (quantwave-qzpi.6 / quantwave-8v4s)

**Status:** Accepted  
**Date:** 2026-06-29

## Context

`quantwave-backtest` supports multi-symbol long-format data via `BacktestConfig.symbol_col`, but today each symbol runs an **independent book** with its own `initial_cash`. Portfolio equity is the **sum** of per-symbol curves (`aggregate_portfolio_equity`). That matches research on isolated sleeves but not a single funded account trading multiple symbols.

Epic `quantwave-8v4s` / `quantwave-qzpi` requires portfolio-wide simulation with one cash pool while preserving the existing independent mode for regression.

**Sources (recorded per AGENTS.md):**
- **polars-backtest** (Yvictor): long-format multi-symbol portfolio UX — inspiration only, clean-room.
- **vectorbt**: vectorized signal→position→PnL; portfolio value = cash + Σ positionᵢ × priceᵢ.
- **QF-Lib / RaptorBT**: position sizing as fraction of equity; we reuse `InitialRiskPositionSizer` on portfolio equity when configured.

## Decision

### 1. Portfolio modes (`PortfolioMode`)

| Mode | Semantics | Default |
|------|-----------|---------|
| `IndependentBooks` | Per-symbol `initial_cash`; portfolio equity = sum of books | **Yes** (backward compatible) |
| `SharedCapital` | Single `initial_cash` pool; bar-by-bar simulation across symbols at each timestamp | Opt-in |

Activate shared capital with `BacktestConfig.portfolio_mode = PortfolioMode::SharedCapital` (and `symbol_col` set).

### 2. Cash pool

- One `cash` balance shared across all symbols.
- Per-symbol position state: signed units, entry price, stops, trade ids.
- Mark-to-market equity at bar *t*: `cash + Σᵢ positionᵢ × closeᵢ` (symbols with open positions; flat symbols contribute 0).
- `stats["initial_cash"]` = single pool (not × `num_symbols`).

### 3. Phase-1 allocators (`PortfolioAllocator`)

When **opening** a new position (flat → non-flat, or flip close+open same bar):

| Allocator | Budget for symbol *s* with raw signal *gₛ* |
|-----------|-----------------------------------------------|
| `EqualWeight` (default) | `equity / N` where *N* = count of symbols with non-zero entry intent this bar |
| `SignalWeighted` | `equity × (|gₛ| / Σⱼ |gⱼ|)` over symbols with non-zero entry intent |

Converted units: `budget / price`, capped by `|gₛ|` when signal is in unit semantics (|g| ≤ 1 treated as max units; |g| > 1 as explicit unit cap). Position sizer, if set, runs on **portfolio equity** before allocation.

Exits and stop-outs release cash back to the pool immediately (same execution model as single-symbol `run_simulation`).

### 4. API surface

**Rust**
- `BacktestConfig.portfolio_mode: PortfolioMode`
- `BacktestConfig.portfolio_allocator: PortfolioAllocator`
- `BacktestEngine::run` routes to `run_shared_capital_multi_symbol` when mode is `SharedCapital` and `symbol_col` is set.
- `PortfolioBar { ts, symbol, close }` + `run_shared_capital_streaming_simulation` for batch↔streaming parity.

**Python**
- `lf.bt.portfolio_backtest(..., portfolio_mode="shared_capital", portfolio_allocator="equal_weight")`
- PyO3: `portfolio_mode` / `portfolio_allocator` kwargs on `BacktestConfig`.

### 5. Parity

- **Independent mode**: existing `multi_symbol.rs` tests unchanged.
- **Shared-capital mode**: mandatory batch↔streaming parity (`portfolio_streaming_parity.rs`) within documented tolerances (equity 1e-8, trades exact).
- **Proptest**: single-symbol batch↔streaming parity (`proptest_parity.rs`) — shared-capital proptest deferred to grunt bead `qzpi.11`.

### 6. Phase-1 scope (MVP)

- 2-symbol long-format grid (full timestamp × symbol rows).
- `EqualWeight` + `SignalWeighted` allocators.
- Zero-cost and simple `CostModel` execution paths.
- No cross-sectional rank engine changes (still independent per symbol until follow-up).

**Out of scope (follow-up beads):** 3+ stress tests (`qzpi.11`), wide-format panels, portfolio optimization, partial fills.

## Consequences

- Researchers can model a single account across symbols without manual cash accounting.
- Independent mode remains default; no breaking change to existing `.bt.backtest()` calls.
- Metrics (`PerformanceMetrics::from_raw`) use pooled equity and single `initial_cash` in shared mode.
- WFO / sweep inherit mode from `BacktestConfig` when passed through.

## References

- `quantwave-backtest/src/portfolio.rs` (shared-capital engine)
- `quantwave-backtest/tests/portfolio_shared_capital.rs`
- `planning/BACKTEST_ENGINE_RESEARCH.md` § multi-symbol
- polars-backtest: https://github.com/Yvictor/polars-backtest (UX reference)