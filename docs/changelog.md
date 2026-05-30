# Changelog

All notable changes to this project will be documented in this file.

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
- **Backtest Engine v0.2** (major milestone under epic `quantwave-n1yc`):
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

