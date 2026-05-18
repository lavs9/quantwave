# Changelog

All notable changes to this project will be documented in this file.

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
