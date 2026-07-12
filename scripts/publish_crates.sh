#!/usr/bin/env bash
# Publish workspace crates to crates.io in dependency order.
# Idempotent: skips crates whose version is already on the registry (safe re-runs).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION="${1:-$(grep -m1 '^\s*version\s*=' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')}"
# crates.io API requires a User-Agent (returns 403 without one).
CRATES_IO_UA="${CRATES_IO_UA:-quantwave-release/${VERSION} (https://github.com/lavs9/quantwave)}"

crate_on_registry() {
  local crate="$1"
  local version="$2"
  curl -sf -A "${CRATES_IO_UA}" \
    "https://crates.io/api/v1/crates/${crate}/${version}" >/dev/null 2>&1
}

publish_crate() {
  local pkg="$1"
  echo "== Publishing ${pkg} =="
  if crate_on_registry "${pkg}" "${VERSION}"; then
    echo "Skip ${pkg}: ${VERSION} already on crates.io"
    return 0
  fi
  set +e
  local output
  output="$(cargo publish -p "${pkg}" 2>&1)"
  local status=$?
  set -e
  echo "${output}"
  if [[ ${status} -eq 0 ]]; then
    echo "Published ${pkg}"
    return 0
  fi
  # Race: published between our check and cargo publish.
  if echo "${output}" | grep -qiE 'already (been uploaded|exists)'; then
    echo "Skip ${pkg}: version already on crates.io"
    return 0
  fi
  echo "Failed to publish ${pkg}" >&2
  return 1
}

wait_for_crate() {
  local crate="$1"
  local version="$2"
  local attempts="${3:-36}"
  if crate_on_registry "${crate}" "${version}"; then
    echo "Indexed: ${crate} ${version}"
    return 0
  fi
  for ((i = 1; i <= attempts; i++)); do
    if crate_on_registry "${crate}" "${version}"; then
      echo "Indexed: ${crate} ${version}"
      return 0
    fi
    echo "Waiting for ${crate} ${version} on crates.io (${i}/${attempts})..."
    sleep 10
  done
  echo "Timed out waiting for ${crate} ${version}" >&2
  return 1
}

echo "Publishing QuantWave crates (version ${VERSION})"

# 1. Leaf crate
publish_crate quantwave-core
wait_for_crate quantwave-core "${VERSION}"

# 2. Crates that depend only on core
publish_crate quantwave-backtest
wait_for_crate quantwave-backtest "${VERSION}"

# 3. Polars integration (depends on core + backtest)
publish_crate quantwave-polars
wait_for_crate quantwave-polars "${VERSION}"

# 4. Umbrella crate
publish_crate quantwave

echo "All crates published."