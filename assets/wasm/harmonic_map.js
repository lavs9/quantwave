// Pure, DOM-free mapping from HarmonicScanner output to chart points
// (quantwave-stb8). Shared by playground.js (browser) and
// quantwave-wasm/tests-node/parity.mjs (node), so the test exercises the exact
// mapping the playground draws.

// Map one pattern, starting at `offset` in the scanner's flattened output `f`,
// to chart points [[barIndex, price], ...] in X (if present), A, B, C, D order.
// Scanner bar indices are 0-based, the same as the chart (BARS) index.
export function harmonicPoints(f, offset = 0) {
  const pts = [];
  if (Number.isFinite(f[offset + 2])) pts.push([f[offset + 2], f[offset + 3]]);
  for (let j = 4; j <= 10; j += 2) pts.push([f[offset + j], f[offset + j + 1]]);
  return pts;
}
