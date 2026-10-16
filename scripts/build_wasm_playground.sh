#!/usr/bin/env bash
# Build quantwave-wasm for the docs playground (quantwave-stb8).
#
#   ./scripts/build_wasm_playground.sh          # web build -> docs/assets/wasm/
#   ./scripts/build_wasm_playground.sh --node   # node build -> target/wasm-node/ (smoke tests)
#
# Needs: rustup target wasm32-unknown-unknown, and wasm-bindgen-cli matching the
# wasm-bindgen version in Cargo.lock (cargo install wasm-bindgen-cli --version <v> --locked).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
TARGET=web
OUT=docs/assets/wasm
if [[ "${1:-}" == "--node" ]]; then
  TARGET=nodejs
  OUT=target/wasm-node
fi
cargo build -p quantwave-wasm --target wasm32-unknown-unknown --release
wasm-bindgen --target "$TARGET" --no-typescript --out-dir "$OUT" \
  target/wasm32-unknown-unknown/release/quantwave_wasm.wasm
ls -l "$OUT"/quantwave_wasm_bg.wasm
