#!/usr/bin/env bash
# quantwave verify — one-shot quality gate (quantwave-072m)
# Installed as pre-push hook via ./scripts/install-git-hooks.sh
# Usage: ./scripts/quantwave_verify.sh [--skip-rust] [--skip-python] [--skip-metadata]
# Cache: scripts/verify_cache.py (disable with VERIFY_NO_CACHE=1)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

SKIP_RUST=0
SKIP_PYTHON=0
SKIP_METADATA=0
SKIP_WHEEL=0
for arg in "$@"; do
  case "$arg" in
    --skip-rust) SKIP_RUST=1 ;;
    --skip-python) SKIP_PYTHON=1 ;;
    --skip-metadata) SKIP_METADATA=1 ;;
    --skip-wheel) SKIP_WHEEL=1 ;;
    -h|--help)
      echo "Usage: $0 [--skip-rust] [--skip-python] [--skip-metadata] [--skip-wheel]"
      echo "Env: VERIFY_NO_CACHE=1 to force full rebuild"
      exit 0
      ;;
  esac
done

run_cached() {
  local step="$1"
  shift
  python3 scripts/verify_cache.py ensure "$step" -- "$@"
}

echo "== quantwave verify =="

if [[ "$SKIP_METADATA" -eq 0 ]]; then
  run_cached metadata bash -c '
    echo "-- metadata codegen drift check"
    python3 scripts/check_metadata_drift.py
    echo "-- API registry + .pyi stub drift"
    python3 scripts/check_api_stubs_drift.py
    echo "-- indicator nav + slug redirect sync"
    python3 scripts/sync_indicator_docs.py
    echo "-- documentation drift check"
    python3 scripts/check_doc_drift.py
  echo "-- public metadata + AEO (llms.txt, indicator count)"
  python3 scripts/check_public_metadata.py

  echo "-- AEO governance (llms URLs, rust guides nav)"
  python3 scripts/check_aeo.py
  echo "-- indicator count sync + drift"
  python3 scripts/sync_indicator_count.py
  echo "-- benchmark claim drift (no unmeasured perf numbers)"
  python3 scripts/check_benchmark_claims.py
  echo "-- benchmark harness dry-run"
  python3 benchmarks/harness.py --dry-run
  python3 scripts/check_repo_hygiene.py
  python3 scripts/check_indicator_parity_coverage.py
  python3 scripts/check_core_safety.py
  python3 scripts/collect_validation_stats.py
  python3 scripts/render_validation_docs.py
  python3 scripts/check_validation_docs.py
  python3 scripts/check_release_invariants.py
    echo "-- documentation standards lint"
    python3 docs/upgrade_to_standards.py --lint
    echo "-- documentation depth lint"
    python3 docs/upgrade_to_standards.py --depth-lint
  '
fi

if [[ "$SKIP_RUST" -eq 0 ]]; then
  run_cached rust bash -c '
    echo "-- cargo fmt --check"
    cargo fmt --all -- --check
    echo "-- cargo clippy (-D warnings)"
    cargo clippy -p quantwave-core -p quantwave-polars -p quantwave-backtest --all-targets -- -D warnings
    echo "-- cargo nextest (core)"
    cargo nextest run -p quantwave-core
    echo "-- cargo nextest (polars)"
    cargo nextest run -p quantwave-polars
    echo "-- cargo nextest (backtest)"
    cargo nextest run -p quantwave-backtest
  '
fi

if [[ "$SKIP_WHEEL" -eq 0 ]]; then
  run_cached wheel bash -c '
    echo "-- unified PyPI wheel build + smoke test"
    python3 -m pip install -q wheel maturin "uniffi-bindgen==0.31.0" 2>/dev/null || true
    python3 scripts/build_unified_wheel.py --out dist
    WHEEL="$(ls -t dist/quantwave-*.whl 2>/dev/null | head -1)"
    test -n "$WHEEL" || { echo "no quantwave wheel in dist/"; exit 1; }
    python3 scripts/pypi_smoke_test.py "$WHEEL"
  '
fi

if [[ "$SKIP_PYTHON" -eq 0 ]]; then
  run_cached pytest bash -c '
    echo "-- pytest (Python DX + backtest smoke)"
    python3 -m pytest \
      tests/python/test_gold_parity.py \
      quantwave-python/tests/test_python_dx.py \
      quantwave-python/tests/test_warmup_options.py \
      quantwave-python/tests/test_metadata_codegen.py \
      quantwave-python/tests/test_streaming_readiness.py \
      quantwave-python/tests/test_backtest.py \
      quantwave-python/tests/test_tier2.py \
      quantwave-python/tests/test_frac_diff.py \
      quantwave-python/tests/test_tearsheet.py \
      quantwave-python/tests/test_portfolio_backtest.py \
      -q
  '
fi

echo "== quantwave verify: OK =="