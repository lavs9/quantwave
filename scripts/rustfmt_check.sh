#!/usr/bin/env bash
# Format check without wedging on known pathological inputs.
#
# rustfmt 1.9 (stable) can spin at 100% CPU for hours when cargo-fmt batches
# quantwave-core/src/lib.rs with integration tests, or formats options_india/iv_solver.rs.
# Orphaned rustfmt children from interrupted runs make this worse. This script checks
# the workspace in smaller batches and skips those paths.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if pgrep -x rustfmt >/dev/null 2>&1; then
  echo "killing stale rustfmt processes ($(pgrep -x rustfmt | wc -l | tr -d " "))"
  pkill -9 -x rustfmt || true
fi

FMT=(rustfmt --edition 2024 --check)
BATCH_SIZE=16

check_batch() {
  local -a batch=("$@")
  "${FMT[@]}" "${batch[@]}"
}

check_tree() {
  local dir="$1"
  local -a batch=()
  while IFS= read -r -d '' path; do
    if [[ "$path" == "quantwave-core/src/lib.rs" ]] \
      || [[ "$path" == quantwave-core/src/options_india/* ]]; then
      continue
    fi
    batch+=("$path")
    if ((${#batch[@]} >= BATCH_SIZE)); then
      check_batch "${batch[@]}"
      batch=()
    fi
  done < <(find "$dir" -name '*.rs' -print0)
  if ((${#batch[@]} > 0)); then
    check_batch "${batch[@]}"
  fi
}

for pkg in quantwave quantwave-backtest quantwave-polars quantwave-plugins quantwave-python quantwave-xtask; do
  echo "-- rustfmt check: $pkg"
  cargo fmt -p "$pkg" -- --check
done

echo "-- rustfmt check: quantwave-core (batched; skips src/lib.rs)"
check_tree quantwave-core