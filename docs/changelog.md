# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Changed
- **BREAKING (Polars plugin surface): `ta_atr`, `ta_natr` and `ta_trange` now take `close` in the receiver, matching their non-prefixed siblings** (`quantwave-sww3`).

    These were generated with opaque positional parameters — `ta_atr(self, in2, in3)` — and forwarded as `[self, in2, in3]` to a plugin that consumes `(high, low, close)`. The receiver therefore had to be **high**, while the sibling `.ta.atr` takes **close**. Writing the call the way its sibling reads permuted the inputs and returned a plausible wrong number with **no error**: on a 200-bar random walk, `1.252823` against a hand-computed Wilder RMA of `1.384819` — roughly 10% off, and data-dependent, so it never looked obviously broken. All three arguments are equal-length `f64` columns, so nothing could raise.

    This bit hardest exactly where it mattered least tolerably: you reach for a `ta_`-prefixed function specifically when you want TA-Lib-identical values, usually to reconcile against a chart.

    ```python
    # before — receiver had to be high
    pl.col("high").ta.ta_atr("low", "close", timeperiod=14)
    # now — same shape as .ta.atr
    pl.col("close").ta.ta_atr("high", "low", timeperiod=14)
    ```

    Parameters are now named `high` / `low` rather than `in2` / `in3`, so a mis-ordered call is visible at the call site and keyword arguments work. `ta_beta` and `ta_correl` already matched their siblings' shape and are unchanged apart from `in2` becoming `other`. The generator (`scripts/gen_pyo3_plugins_py.py`) was fixed, not just its output, so regenerating cannot reintroduce this. Regression cover lives in `tests/python/test_ta_prefixed_arg_order.py`, anchored to independently computed reference values rather than to current behaviour.

- **All 60 remaining candlestick patterns ported to native Rust — `talib-rs` has left the shipped dependency graph.** The `native_cdl!` macro pushed every bar onto a 32-bar window and re-ran the full `talib_rs::pattern::*` **batch** function per bar, discarding all but the last value. All 60 are now O(1) streaming `Next<T>` implementations under `quantwave_core::indicators::patterns`, joining `CDLDOJI` and the 14 already-native single-bar patterns for **61 native candlestick patterns**.

    `talib-rs` moved from `[dependencies]` to `[dev-dependencies]` in `quantwave-core`, and the unused declaration was dropped from `quantwave-py`. It is retained solely as the `#[cfg(test)]` parity oracle and the `benches/talib_comparison` baseline, both of which stay. **The "221 native indicators" claim is now literally true** — nothing in the shipped wheel or crate delegates to C TA-Lib or to a third-party TA crate. Struct names are unchanged (`CDLHAMMER`, `CDL2CROWS`, …), so this is not a breaking change for any caller.

    The dead `native_cdl!` macro and the eleven never-invoked `talib_*` macros were deleted from `indicators/talib_wrapper.rs`; the two live native macros (`native_pointwise_1!`, `native_binary_2!`) remain.

- **BREAKING (Rust API only): `talib_rs::MaType` replaced by a native `quantwave_core::MaType`** (`quantwave-wxii`). Every `matype` field and constructor argument — `APO`, `PPO`, `BBANDS`, `MACDEXT`, `MAVP`, `STOCH`, `STOCHF`, `STOCHRSI`, and the corresponding `quantwave-polars` `.ta()` methods — now takes `quantwave_core::MaType`. The discriminants are unchanged (`Sma = 0` … `T3 = 8`), so **Python users are unaffected**: `matype=` still takes the same integers and strings.

    `pub use talib_rs as talib;` has been removed from `quantwave-core`. talib-rs is a parity **oracle** used from `#[cfg(test)]` and `benches/`, and a dependency in that role must not appear in a public signature. Rust callers substitute:

    ```rust
    // before
    use quantwave_core::talib::MaType;
    // after
    use quantwave_core::MaType;
    ```

    `MaType` offers `to_i32()`, `TryFrom<i32>`, `FromStr`, `Display`, `MaType::ALL`, and `from_u8_or_sma()`. To cross to talib-rs in your own tests or benches, go through the integer code: `talib_rs::MaType::try_from(m.to_i32())`.

- **BREAKING: `execution_delay` now defaults to `"next_bar"` (T+1) instead of `"same_bar"` (T+0)** (`quantwave-zmjw`). Affects `.bt.backtest`, `.bt.backtest_with_report`, `.bt.backtest_metrics`, `.bt.portfolio_backtest` and every other `.bt` entry point, the Python `BacktestConfig`, the Rust `BacktestConfig::default()` / `ExecutionDelay::default()`, and `BtOptions::default()` in `quantwave-polars`.

    `"same_bar"` fills at the close of the very bar that produced the signal. Since signals are almost always derived from that same close (e.g. `(rsi < 30)` computed on bar `t`), the old default executed on information that only existed at the instant the bar ended — a look-ahead the live strategy never has, and one that flatters results systematically. On an identical signal frame over a rising series, `same_bar` entered at `100.5` where `next_bar` entered at `101.0`.

    **Your existing backtests will report different — generally worse — numbers after upgrading. That difference is the look-ahead being removed, not a regression.**

    To restore the previous behaviour, pass it explicitly:

    ```python
    lf.bt.backtest(signal="signal", execution_delay="same_bar")
    ```

    `"same_bar"` remains fully supported and is the correct choice when it genuinely describes your execution: you trade the closing auction, or your signal is built purely from data through bar `t-1` so bar `t`'s close is not an input. Otherwise, prefer the new default.

### Fixed
- **Four candlestick patterns diverged from the TA-Lib reference in ways the random-walk parity tests never reached.** Each substituted a different condition group for the reference's, and each was masked by a fixture that happened to satisfy both readings:
    - `CDLGAPSIDESIDEWHITE` — had no `EQUAL` accumulator at all, substituting `open[i] < close[i-1]` for the reference's near-equal-opens test, and derived the signal's sign from candle colour instead of gap direction. The reference returns `+100` for an upside gap and `-100` for a downside gap with **both** candles white; the old code required a black pair for the bearish case, so it never emitted `-100`.
    - `CDLCONCEALBABYSWALL` — tested the real-body gap between bars `i-2` and `i-3` instead of `i-1` and `i-2`, required the first two marubozu shadows to be *exactly* zero rather than shorter than the `SHADOW_VERY_SHORT` average, and imposed a long-lower-shadow test on the 3rd bar that the reference does not have.
    - `CDL3STARSINSOUTH` — the entire third-candle group was substituted: the reference's short-body, short-upper-shadow and short-lower-shadow tests were missing, replaced by body-containment within the 2nd candle and an exact-zero lower shadow, and the `low`/`high` containment was tested against the 2nd body rather than the 2nd bar's range.
    - `CDLBREAKAWAY` — tracked the three-candle descent with `close` where the reference uses `high`/`low`, added a long-body requirement on the closing candle that the reference does not impose, constrained the 4th candle's colour (the reference leaves it free), and compared the final close against the 2nd/1st candle bodies rather than `open[i-3]` / `close[i-4]`.

    The fixtures in `patterns/fixtures.rs` for all four were rebuilt so they **discriminate**: each now contains blocks that fire under the reference semantics but not the old reading (and, where applicable, the reverse), separated by re-priming bars so the rolling candle averages return to known values. Each was verified to fail against the pre-fix implementation before the fix was applied.
- **`MaStream` silently substituted SMA for every matype except EMA** (`quantwave-ii0g`). The enum carried only `Sma` and `Ema` variants and a catch-all arm mapped Wma/Dema/Tema/Trima/Kama/Mama/T3 to SMA, so `APO::new(12, 26, MaType::Wma)` returned an **SMA-based APO** with no error or warning. `PPO`, `MACDEXT`, `STOCH`, `STOCHF` and `STOCHRSI` were affected the same way. All nine families now dispatch to their native streaming implementation (`MaType::Mama` uses `fastlimit = 0.5` / `slowlimit = 0.05` and `MaType::T3` uses `vfactor = 0.7`, matching C TA-Lib's `ta_MA.c` when reached through a `matype` argument). **Values change for any non-SMA, non-EMA matype — the old numbers were the wrong algorithm.**
- **`BBANDS` with a non-SMA matype was O(n) per bar and leaked memory** (`quantwave-3nyh`). The non-SMA path appended every bar to an unbounded `history` and re-ran the *batch* `bbands` across the whole thing each tick — O(n²) over a series, in a struct whose SMA path is O(1). It is now genuinely incremental: the middle band comes from `MaStream` and the deviation is a two-pass sum of squared deviations over a rolling window of `timeperiod` values. Memory is bounded by `timeperiod`. Results are unchanged for `MaType::Sma`.
- `KAMA`'s streaming state retained the entire input history although only the last `timeperiod + 1` samples are ever read; it now keeps a bounded ring buffer. The arithmetic, including summation order, is unchanged.
- **Shared-capital portfolio streaming ignored `execution_delay`** (`quantwave-zmjw`). `run_shared_capital_streaming_simulation` passed the delay down to `simulate_shared_capital`, which discards it — the batch path pre-shifts signals per timestamp group, but the streaming path never did. Under `SameBar` (the old default) both paths agreed, so the bug was invisible; any caller who explicitly asked for `next_bar` silently got same-bar fills in streaming mode and broke batch↔streaming parity. The streaming path now applies the same per-timestamp-group shift as batch.
- **68 indicators silently resolved to a streaming class instead of their batch function** (`quantwave-84cu`). The generated TA registry introduced in 0.7.0 derived native batch symbol names with `pascal_to_snake()` (`SuperTrend` → `super_trend`), but the `export_*!` macros emit `pub fn [<$name:lower>]` (`SuperTrend` → `supertrend`). Every multi-word name missed; the 44 single-word ones (`rsi`, `sma`, `atr`) passed only because `pascal_to_snake("Rsi") == "rsi"`. `_resolve_ta_binding` treated the miss as a fallback and returned `native_streaming`, so `qw.supertrend` was a **class** while `qw.rsi` was a function — with no error or warning. `qw.supertrend(period=10, multiplier=3.0, high=…, low=…, close=…)` again returns `list[SuperTrendResult]` as it did in 0.6; callers need no changes.
- `_resolve_ta_binding` now raises `ImportError` when an entry declares a `native_batch` symbol the build does not export, rather than silently substituting the streaming class (whose calling convention differs). A `native_batch` of `None` still falls through to streaming/polars as before.
- Corrected stale hand-written aliases in `scripts/api_slug_aliases.json`: `fm_demodulator`, `fourier_series_model`, `my_rsi`, `precision_trend_analysis` named non-existent snake_cased symbols; `linreg`, `oc2`, `true_range` declared batch exports that do not exist and now fall through to their polars methods; `sr_monitor` declared `SrInteractionMonitor`, a class never exported to Python.

### Added
- Batch↔streaming parity proptests **parameterised over every `MaType`** for `MaStream`, `BBANDS`, `APO`, `PPO` and `MACDEXT`, plus SMA/EMA/WMA/TRIMA coverage for `STOCH` / `STOCHF`. The pre-existing suite only ever constructed `MaType::Sma`, which is precisely why both bugs above survived unnoticed.
- TA-Lib-parity streaming `TalibWma`, `TalibTema` and `TalibMama` (the last reusing the existing incremental Hilbert engine), each with its own parity proptest. The general-purpose `WMA`, `TEMA` and `MAMA` use different seeding and do not reproduce TA-Lib's values.
- **`qw.trim_warmup()` / `qw.warmup_rows()`** (`quantwave-4rsq`). Indicator warmup is emitted as `NaN`, never `null`, so `drop_nulls()` / `dropna()` is a **silent no-op** on it and warmup rows flow into backtests and feature matrices unnoticed. `qw.trim_warmup(frame, *specs, extra=0, strict=True)` slices off the **maximum** warmup across every named indicator, keeping columns with different warmups row-aligned (unlike `drop_nans()`, which trims per column set). Accepts `"rsi"`, `("rsi", {"period": 21})`, `{"rsi": {...}, "ema": {...}}`, or an explicit `int` bar count, and works on `DataFrame` / `LazyFrame` / `Series`, including `df.pipe(qw.trim_warmup, "rsi")`. Unknown indicator names raise by default rather than silently trimming nothing.
- **`.bt` warmup warning** (`quantwave-4rsq`). `backtest`, `backtest_with_report`, `backtest_metrics`, `portfolio_backtest`, `walk_forward`, `monte_carlo` and `order_backtest` now emit a `quantwave.WarmupWarning` when the `signal` or `close` column they receive starts with `NaN`/`null` rows. It is a warning, not an error — existing code keeps working — and is silenceable with `warnings.filterwarnings("ignore", category=qw.WarmupWarning)`.
- NaN-vs-null semantics documented prominently in the Python getting-started guide, the backtest quickstart, and the FAQ.
- `test_registry_native_symbols_resolve_against_build` — asserts every declared native symbol exists in the compiled module. The prior test only checked a name was *present*, never that it *resolved*, so `native_batch: "super_trend"` passed cleanly. Plus a regression test that multi-word slugs bind as batch functions, not classes.

## [0.7.0] - 2026-07-13

### Added
- **Complete classic TA-Lib surface** (`quantwave.talib`): **161** functions, up from 8 — RSI, MACD, SMA, EMA, ATR, ADX, BBANDS, STOCH, OBV, all 61 candlestick patterns, and the math/price transforms. The classic array-in/array-out API (`talib.RSI(close, timeperiod=14)`, multi-output tuples, OHLC/candlestick inputs) delegates to the Polars `.ta` plugins, so values are the talib-rs-parity-tested Rust results. (`quantwave-yp9a`)
- Top-level native symbol access restored: `qw.FracDiff`, `qw.fracdiff`, `qw.rsi`, `qw.SuperTrend`, … bind alongside the slug-based `qw.ta` namespace.

### Changed
- **Unified the Python FFI on PyO3 (abi3), retiring uniffi.** The indicator bindings, the Polars expression plugins, and the backtest engine are now a **single** PyO3 `abi3-py39` extension in one crate (`quantwave-py`) producing one cdylib — a single `maturin build` yields one `cp39-abi3` wheel with no wheel-merge step. (`quantwave-5ipk.10`, `quantwave-6dgg`)
- Collapsed the three PyO3 crates (`quantwave-python`, `quantwave-plugins`, `quantwave-backtest-py`) into `quantwave-py`; deleted `scripts/build_unified_wheel.py` and consolidated to one `pyproject.toml`.

### Fixed
- **Wheel tag / install correctness**: the published wheel is now `cp39-abi3` and installs correctly on **CPython 3.9–3.13**. 0.6.1 shipped a `py3-none` wheel bundling CPython-3.12-only extensions, which broke `pip install` on 3.9/3.10/3.11/3.13. (`quantwave-9gek.1`)

## [0.6.0] - 2026-06-28

### Added
- **Fractional differencing (`FracDiff`)** (`quantwave-wnd9`): Prado-style stationary features; Rust `Next<f64>`, Polars `lf.ta.frac_diff()`, Python `fracdiff()`
- **HTML tear sheets** (`quantwave-0gi1`): `BacktestReport.to_html()` / `save_html()` with equity, drawdown, and trade tables
- **Research loop (Tier 2)**: `qw.build_feature_matrix()`, `lf.ta().features().recommended_matrix()`, `lf.bt.monte_carlo()`, Rust `.bt` WFO-optimize + MC bootstrap
- **Product guardrails (Tier 1)**: `scripts/quantwave_verify.sh`, metadata drift gate, streamlined CI (verify → plugins → deploy-docs)
- **55 custom Polars expression plugins** and **98 auto-generated pyo3-polars bindings** for standard indicators
- **PA foundation**: S/R Polars, confluence, geometric patterns with H&S neckline breakout
- **Streaming readiness** (`quantwave-h6xe`) and Rust metadata codegen (`quantwave-iqq7`)
- **Plugin vs `.ta` guide**, expanded regime user guide, comparison one-pager
- **Indicator doc SOA complete** (`quantwave-frq0`): 220+ native pages under `DOCUMENTATION_STANDARDS.md` with PNG previews, doc drift script in verify
- **Full visual depth layer** (p1k6): `docs/generate_all_previews.py` + standards lint rejecting placeholders

### Changed
- GitHub Actions consolidated from four workflows to **CI** + **Release** (`v*` → crates.io + PyPI)
- Platform planning docs split into `INDICATORS_SOA.md` and `BACKTEST_SOA.md`

### Fixed
- CI: `cargo-nextest`, maturin venv, `uniffi-bindgen==0.31.0` for verify job
- Doc lint for `fractional_differentiation.md` (preview PNG + description depth)
- Empty Python API Reference page (`quantwave-rbz4`)
- Broken imports for `quantwave >=0.4.1` on fresh installs without polars

## [0.5.2] - 2026-05-31

### Added (Python DX improvements)
- **Discovery API**: `quantwave.indicators()` and `quantwave.is_indicator(name)`.
- **Rich Metadata**: `quantwave.metadata(name)` returning `IndicatorMeta` with params, data inputs, outputs, warmup_bars, category, etc.
- **Streaming lookup**: `quantwave.streaming_class(name)`.
- **Parity testing**: `quantwave.assert_parity()` helper for verifying batch vs streaming bit-identical behavior.
- **`warmup_bars(name, params)`** helper.
- **Namespace improvements**: New `quantwave.results`, `quantwave.options`, and `quantwave.talib` submodules. Old top-level access now emits deprecation warnings.
- **Public exception base**: `quantwave.QuantwaveError`.
- **`__version__`** properly exposed.
- Linux arm64 (aarch64) wheels are now built and published.

### Changed
- Release workflow no longer hard-gates on docs build (docs issues can be fixed independently).

### Documentation
- **Official Standards Published**: Created `docs/DOCUMENTATION_STANDARDS.md` (v1.0, 2026-05-31 IST) under task quantwave-d2hk / epic p1k6. Defines mandatory enforceable template for all 223+ indicator pages: required sections (Visual Example, full batch+streaming+Polars Usage Examples, Edge Cases & Limitations, Sources), type-specific guidance (classic scalar / patterns / rich struct / Ehlers), good-vs-bad examples, tone/visual/cross-link rules, and 4-phase rollout. 
- Updated `contributing.md` (new indicator docs step) and appended full decision record + rationale (diagnosis of thin stubs vs. PA notebook quality) to `DOCUMENTATION_DECISIONS.md`.
- Minor alignments in `gallery.md`.
- This is the foundation for all future indicator documentation work and the planned xtask generator. See `DOCUMENTATION_STANDARDS.md` for the complete template and checklist.
- **Candle Standards Proof batch (p1k6 child, 2026-05-31 IST)**: 8 worst-duplication candlestick pages (doji.md + gravestone/dragonfly variants, harami.md + harami_cross, three_black_crows.md + three_white_soldiers.md, abandoned_baby.md) + engulfing.md enhancement fully rewritten to DOCUMENTATION_STANDARDS.md (mandatory visuals, 3-surface code, edges, authoritative TA-Lib+core sources, no Nison boilerplate). `docs/gen_candle_previews.py` extended (portable + 8+ generators); 11 professional PNGs produced in `assets/candlestick-previews/`. Cross-refs + full decision record in DOCUMENTATION_DECISIONS.md. Proves template + gens scale for Phase 1 rollout. See decisions file for files touched and bd tracking attempt details.
- **Ehlers DSP Phase 1 batch 2 (p1k6, 2026-05-31 IST)**: 5 high-value thin Ehlers DSP pages (ehlers_filter.md, reflex.md, ehlers_stochastic.md, ehlers_loops.md, ultimatesmoother.md) rewritten to full Ehlers/scalar STANDARDS conformance. Extended `gen_indicator_previews.py` (portable, pure-numpy core ports for the 5, CLI, professional DSP styling); 5 new PNG visuals generated with 2026-05-31 IST captions mapping directly to core .rs Next logic. 3-surface examples, Edge Cases, authoritative sources (exact core paths + Ehlers papers). Cross-refs + detailed decision record appended. Worktree clean for merge. See DOCUMENTATION_DECISIONS.md for complete list of files + checklist confirmation.

## [0.5.1] - 2026-05-31

### Fixed
- **Publishing completeness**: Fixed workspace dependency configuration so that `cargo publish` succeeds for all internal crates (`quantwave-core`, `quantwave-polars`, `quantwave-plugins`, `quantwave-backtest`, `quantwave`). Internal crates now correctly declare `version.workspace = true` in `[workspace.dependencies]`.
- **Release reliability**: Added required `build-docs` job (export + `mkdocs build --strict`) to the release workflow. Release publishing now hard-gates on successful docs build. Removed all `continue-on-error: true` from publish steps — any failure is now fatal.
- **Docs build**: Fixed filename collision (`*.py.md` landing pages conflicting with `*.py` notebooks) that was breaking main deploys. Renamed affected landing pages and cleaned up references + committed `__pycache__`.
- Modernized `cargo publish` steps to use `CARGO_REGISTRY_TOKEN` environment variable (no more deprecated `--token` flag).

### Changed
- 0.5.1 is the first complete, trustworthy release of the Backtest Engine v0.2 features (including `quantwave-backtest` crate on crates.io) plus the full Polars + Python package set.

## [0.5.0] - 2026-05-30

### Added
- **Backtest Engine v0.2** (major milestone):
  - **Rich-Metadata Position Sizing**: New `PositionSizer` trait + `InitialRiskPositionSizer` that directly consumes rich PA detector metadata (`fraction_at_risk`, `pole_height_atr`, strength, etc.) for dynamic, risk-aware sizing. Inspired by QF-Lib patterns. Includes `SizingAdapter` for seamless streaming `Next<T>` generators.
  - **Pluggable Realistic Execution Models**: Proper `CommissionModel` and `SlippageModel` traits with high-quality implementations, including `SquareRootMarketImpactSlippage` (volatility × √(volume/ADV)) and `max_volume_share_limit` support.
  - **High-Fidelity Execution Simulator Mode**: New execution path that applies the full sophisticated models while being driven by the *exact same* rich `StrategySignal` / PA struct stream as the fast vectorized path. Perfect for "pre-live" validation.
  - **Professional Tearsheet & Reporting Layer**: New `BacktestTearsheet` with `PerformanceSummary`, `RiskMetrics`, `EnrichedTrade` (carries full PA metadata for attribution), `AttributionReport`, `to_markdown()`, and Polars DataFrame export for Excel. Institutional-quality output.
- Full batch + streaming `Next<T>` parity maintained across all new features.
- Updated canonical examples and documentation demonstrating PA detector + rich metadata workflows.

### Changed
- Backtester is now production-grade ready for complex PA + ML strategies (Flags, H&S, Market Structure, etc.).

## [0.4.0] - 2026-05-19

### Added
- **Options India Analytics**: Comprehensive suite for NSE options including Black-Scholes Greeks (Price, Delta, Gamma, Theta, Vega, Rho), Implied Volatility, and Chain Analytics (Max Pain, PCR, GEX, OI Zones, ATM Straddle, Synthetic Futures).
- **Polars Integration for Options**: Full support for `options_india` as native Polars expressions with robust handling of column-or-value parameters.
- **NSE Utilities**: Added `nse_lot_size` and `moneyness` helpers for the Indian market.

### Fixed
- **Release Build**: Resolved a critical 'maturin' conflict where tracked `__init__.py` files were being overwritten during the wheel build process.
- **Code Hygiene**: Cleaned up all compiler warnings and unused imports across the entire workspace.

## [0.3.0] - 2026-05-18

### Added
- **Multi-Asset Regime Detection**: Enhanced `MultiAssetClusterer` with rolling correlation structures and dispersion analysis to identify joint market states.
- **Advanced Conditioned Risk Metrics**: Expanded `regimes_conditioned_metrics` in Polars to include Skewness, Kurtosis, and Sortino Ratio.
- **Polars Enhancements**: Enabled `moment` and `cum_agg` features for vectorized higher-order statistics.

### Fixed
- **Release Stability**: Fixed workspace dependency alignment issues that caused CI failures in previous releases.
- **Compilation**: Resolved method resolution errors for `skew` and `kurtosis` in Polars pipelines.

## [0.2.0] - 2026-05-18

### Added
- **Regime Detection Suite (`quantwave::regimes`)**:
    - Volatility Clustering (Prakash et al. 2021) with online K-Means.
    - Hidden Markov Models (Hamilton 1989) with Viterbi decoding.
    - Gaussian Mixture Models (Two Sigma 2021) foundations.
    - Changepoint Detection (PELT - Killick et al. 2012) for exact segmentation.
- Polars integration for all regime detection tools.
- Comprehensive documentation and guides for market state tools.

