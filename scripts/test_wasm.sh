#!/usr/bin/env bash
# WASM full engineering verification (quantwave-hwxa):
#   1. Build npm (bundler) and Node.js WASM packages via wasm-pack
#   2. Enforce gzipped .wasm bundle size budget (<= 512 KiB, well under 1.5 MiB)
#   3. Run wasm-pack test --node
#   4. Run bit-identical f64 Node parity + gold-standard fixture checks
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

TARGET_DIR="${CARGO_TARGET_DIR:-target}"
NODE_OUT="$TARGET_DIR/wasm-node"
PKG_OUT="$TARGET_DIR/wasm-pkg"
MAX_GZIP_BYTES=$((512 * 1024)) # 512 KiB strict budget (limit: 1.5 MiB)

echo "== quantwave-wasm: build bundler (npm) + nodejs packages =="
wasm-pack build --target bundler --out-dir "../$PKG_OUT" quantwave-wasm --release
wasm-pack build --target nodejs --out-dir "../$NODE_OUT" quantwave-wasm --release

WASM_FILE="$PKG_OUT/quantwave_wasm_bg.wasm"
RAW_BYTES=$(wc -c < "$WASM_FILE" | tr -d ' ')
GZIP_BYTES=$(gzip -c "$WASM_FILE" | wc -c | tr -d ' ')
echo "WASM size: raw=${RAW_BYTES} B, gzip=${GZIP_BYTES} B (budget=${MAX_GZIP_BYTES} B)"
if [[ "$GZIP_BYTES" -gt "$MAX_GZIP_BYTES" ]]; then
  echo "FAIL: gzipped wasm bundle ${GZIP_BYTES} B exceeds budget ${MAX_GZIP_BYTES} B"
  exit 1
fi
echo "PASS: WASM gzipped size budget OK"

echo "== quantwave-wasm: wasm-pack test --node =="
wasm-pack test --node quantwave-wasm

echo "== quantwave-wasm: Node bit-identical parity + gold fixtures =="
cargo run -q -p quantwave-wasm --example parity_fixture > "$NODE_OUT/fixture.json"
node quantwave-wasm/tests-node/parity.mjs "$NODE_OUT"
