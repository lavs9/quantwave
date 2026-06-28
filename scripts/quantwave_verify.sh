#!/usr/bin/env bash
# quantwave verify — one-shot quality gate (quantwave-072m)
# Usage: ./scripts/quantwave_verify.sh [--skip-rust] [--skip-python] [--skip-metadata]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

SKIP_RUST=0
SKIP_PYTHON=0
SKIP_METADATA=0
for arg in "$@"; do
  case "$arg" in
    --skip-rust) SKIP_RUST=1 ;;
    --skip-python) SKIP_PYTHON=1 ;;
    --skip-metadata) SKIP_METADATA=1 ;;
    -h|--help)
      echo "Usage: $0 [--skip-rust] [--skip-python] [--skip-metadata]"
      exit 0
      ;;
  esac
done

echo "== quantwave verify =="

if [[ "$SKIP_METADATA" -eq 0 ]]; then
  echo "-- metadata codegen drift check"
  python3 scripts/check_metadata_drift.py
  
  echo "-- documentation drift check"
  python3 scripts/check_doc_drift.py
  
  echo "-- documentation standards lint"
  python3 docs/upgrade_to_standards.py --lint
  
  echo "-- documentation depth lint"
  python3 docs/upgrade_to_standards.py --depth-lint
fi

if [[ "$SKIP_RUST" -eq 0 ]]; then
  echo "-- cargo nextest (core)"
  cargo nextest run -p quantwave-core
  echo "-- cargo nextest (polars)"
  cargo nextest run -p quantwave-polars
  echo "-- cargo nextest (backtest)"
  cargo nextest run -p quantwave-backtest
fi

if [[ "$SKIP_PYTHON" -eq 0 ]]; then
  echo "-- pytest (Python DX + backtest smoke)"
  python3 -m pytest \
    quantwave-python/tests/test_python_dx.py \
    quantwave-python/tests/test_warmup_options.py \
    quantwave-python/tests/test_metadata_codegen.py \
    quantwave-python/tests/test_streaming_readiness.py \
    quantwave-python/tests/test_backtest.py \
    quantwave-python/tests/test_tier2.py \
    quantwave-python/tests/test_frac_diff.py \
    quantwave-python/tests/test_tearsheet.py \
    -q
fi

echo "== quantwave verify: OK =="