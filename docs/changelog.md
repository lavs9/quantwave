# Changelog

All notable changes to this project will be documented in this file.

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

