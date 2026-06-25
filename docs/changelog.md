# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased / Docs] - 2026-06-25 IST

### Added
- **Documentation standards bulk rollout** (quantwave-6br5 / epic p1k6 Phase 1 complete): `docs/upgrade_to_standards.py` upgrades all native indicator stubs to `DOCUMENTATION_STANDARDS.md` v1.0 (202 pages rewritten; 19 hand-upgraded exemplars preserved). Every page now has mandatory sections, 3-surface code, edge cases, authoritative core-path sources, and visuals or precise placeholders. `python docs/upgrade_to_standards.py --lint` enforces the checklist. Gallery and native index updated to reflect full conformance.

## [Unreleased / Docs] - 2026-06-01 IST

### Fixed
- **Empty Python API Reference page** (quantwave-rbz4): https://lavs9.github.io/quantwave/api/ was rendering completely empty on the public site. Rewrote `docs/gen_python_api.py` to produce a professional landing page that clearly explains the three Python surfaces and directs users to the high-quality manual documentation in the Guides. The auto-generated reference is now intentionally lightweight and useful as a package surface reference rather than a duplicate of the indicator docs. Updated workflow comment, mkdocs nav label, roadmap, and decision record in the same change set.
- **Broken imports for quantwave >=0.4.1 on fresh installs without polars** (downstream report): `import quantwave` now succeeds cleanly even if `polars` is not present in the environment (previously the unconditional `from . import polars` + top-level `import polars as pl` in the submodule would raise). The Polars layer (`quantwave.polars`) is now optional and guarded like `results`/`options`/`talib`. Added `polars` and `all` extras in `quantwave-python/pyproject.toml`. Downstream projects pinning `quantwave>=0.4.0` will no longer be forced to the latest broken-on-import 0.5.x or have to manually pin 0.4.0. Updated install docs in READMEs and getting-started. Core DX (metadata, discovery, streaming, parity, talib, options) remains fully usable without polars.

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

