#!/usr/bin/env bash
# WASM == quantwave-core parity smoke test for the playground indicators (quantwave-stb8).
# Builds the node wasm target, dumps a native core fixture, and compares bit-for-bit in node.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
./scripts/build_wasm_playground.sh --node
cargo run -q -p quantwave-wasm --example parity_fixture > "${CARGO_TARGET_DIR:-target}"/wasm-node/fixture.json
node quantwave-wasm/tests-node/parity.mjs "${CARGO_TARGET_DIR:-target}"/wasm-node
