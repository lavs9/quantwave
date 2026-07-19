# Validation methodology

QuantWave's primary credibility claim is **correctness**: one mathematical implementation (`Next<T>`) powers batch Polars expressions, streaming structs, and Python bindings. This page documents how that claim is tested — every count below is **machine-generated**, not hand-written.

[Compare vs TA-Lib](comparison.md) · [Benchmarks](benchmarks.md) (measured only) · [Contributing](contributing.md)

## Coverage snapshot

<!-- VALIDATION:STATS:START -->
**Last updated:** 2026-07-19T02:27:45Z (UTC)

| Metric | Count | Source |
|--------|------:|--------|
| Native indicators (`*_METADATA`) | **221** | `metadata_export.json` |
| Metadata entries with gold file reference | 217 | `metadata_export.json` |
| Gold-standard JSON fixtures (on disk) | **28** | `quantwave-core/tests/gold_standard/` |
| Python streaming gold parity cases | **26** | `tests/python/gold_parity_registry.py` |
| Python gold parity deferred (HMM) | 2 | regime fixtures — separate suite |
| Rust `#[test]` functions (core) | 563 | `rg '#[test]' quantwave-core` |
| Rust `#[test]` functions (polars) | 45 | `rg '#[test]' quantwave-polars` |
| Rust `#[test]` functions (backtest) | 92 | `rg '#[test]' quantwave-backtest` |
| Rust tests (total, 3 crates) | **700** | sum of above |
| `proptest!` blocks (core) | **158** | `rg 'proptest!\{' quantwave-core` |
| `check_batch_streaming_parity` call sites | 4 | indicator modules |
| TA-Lib parity test functions | 134 | `test_*_talib_parity.rs` |
| Indicators with proptest parity (CI-enforced) | **214** | `check_indicator_parity_coverage.py` |
| Reviewed parity exemptions | 7 | `parity_exemptions.toml` |

Regenerate: `python scripts/collect_validation_stats.py && python scripts/render_validation_docs.py`
<!-- VALIDATION:STATS:END -->

## Gold-standard vectors

Industry reference outputs are stored as JSON in `quantwave-core/tests/gold_standard/`. Rust indicator modules load these fixtures in unit tests; the Python package runs **streaming parity** against the same files via `tests/python/test_gold_parity.py`.

Provenance varies by indicator:

- **TA-Lib / talib-rs** reference runs for classic indicators
- **Ehlers papers** and Trader's Tips HTML references (see per-indicator `formula_source` in metadata)
- **Hand-verified** small vectors for edge-case warmup behavior

### On-disk fixtures

<!-- VALIDATION:GOLD:START -->
Each file below lives in `quantwave-core/tests/gold_standard/` and is consumed by Rust unit tests
and/or the Python gold parity registry.

| Fixture |
|---------|
| `alma_9_085_6.json` |
| `atr_ts_14_25.json` |
| `cycle_trend_analytics.json` |
| `donchian_5.json` |
| `ehlers_autocorrelation.json` |
| `ehlers_loops.json` |
| `ehlers_stochastic.json` |
| `frac_diff.json` |
| `fractals.json` |
| `griffiths_spectrum.json` |
| `heikin_ashi.json` |
| `hma_14.json` |
| `hmm_gaussian_2state.json` |
| `hmm_lambda_2state.json` |
| `ichimoku.json` |
| `keltner_20_20_15.json` |
| `mad.json` |
| `mesa_stochastic.json` |
| `oc_price_rsi.json` |
| `pivot_points.json` |
| `rsih.json` |
| `sma_5.json` |
| `supertrend_10_3.json` |
| `tema_14.json` |
| `ttm_squeeze_20_2_15.json` |
| `vortex_14.json` |
| `voss_predictor.json` |
| `wavetrend_10_21_4.json` |
<!-- VALIDATION:GOLD:END -->

## Streaming vs batch parity (Rust)

Every streaming indicator is encouraged to use `check_batch_streaming_parity` (see `quantwave-core/src/test_utils.rs`): for random input sequences, collecting `indicator.next(x)` must match the batch/plugin path within tolerance. This is the property that matters for **live trading** — your research DataFrame and your bar-by-bar feed must not diverge.

`proptest!` blocks across `quantwave-core` exercise this property with randomized inputs and warmup-aware comparisons.

## TA-Lib cross-check

Dedicated integration tests in `quantwave-core/tests/test_all_talib_parity.rs` and `test_missing_talib_parity.rs` compare QuantWave streaming output against `talib-rs` batch functions for the mapped subset. This is a separate layer from gold JSON fixtures and catches drift in classic TA paths.

## Python cross-language parity

The unified PyPI wheel bundles the PyO3 (abi3) core + backtest + Polars plugins extensions. Python gold parity tests assert that **streaming classes** match the same JSON vectors as Rust — catching FFI field swaps, warmup mishandling, or ABI mismatches before release.

CI runs `tests/python/test_gold_parity.py` on **Linux and macOS** after `maturin develop` (see `.github/workflows/ci.yml`).

## Release invariants

No wheel reaches PyPI without:

1. **Tag verification** — `scripts/verify_wheel_tags.py` on every built wheel (abi3 tag must match extension naming)
2. **Install smoke** — Python 3.9 / 3.11 / 3.12 / 3.13 import + RSI batch/stream parity
3. **OIDC publish** — `pypa/gh-action-pypi-publish` via trusted publisher (`release.yml`); no long-lived API token

Enforced by `scripts/check_release_invariants.py` in CI. See [GitHub Actions README](https://github.com/lavs9/quantwave/blob/main/.github/workflows/README.md).

## Known exemptions

| Area | Status | Reason |
|------|--------|--------|
| HMM gold fixtures (`hmm_*_2state.json`) | Deferred in Python parity | Multi-field regime outputs — separate test suite in `quantwave-core/tests/hmm_*_gold.rs` |
| Performance marketing numbers | Removed / harness-only | See [benchmarks](benchmarks.md); unmeasured claims are blocked by `check_benchmark_claims.py` |
| Plugin `.ta` batch path | Partial Python gold coverage | Streaming parity lands first; Polars expression parity expands per indicator |

## How to extend

1. Add or update `quantwave-core/tests/gold_standard/<name>.json`
2. Wire Rust `load_gold_standard*` test in the indicator module
3. Add a row to `tests/python/gold_parity_registry.py` if Python exposes the indicator
4. Run `./scripts/quantwave_verify.sh` locally; CI regenerates this page via `check_validation_docs.py`