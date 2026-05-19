# Changelog

All notable changes to this project will be documented in this file.

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

## [Unreleased]


### Added
- High-performance Rust core with `Next<T>` trait.
- Polars integration via Expressions and Series.
- Initial set of indicators (SuperTrend, SMA, EMA).
- Documentation framework with MkDocs, Marimo, and social previews.
- Gold standard testing infrastructure for bit-identical results.

### Fixed
- Improved parity between batch and streaming implementations.

### Changed
- Refactored indicator traits for better extensibility.
