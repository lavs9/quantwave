// Node smoke + parity test for the quantwave-wasm build (quantwave-stb8).
//   ./scripts/build_wasm_playground.sh --node
//   cargo run -q -p quantwave-wasm --example parity_fixture > target/wasm-node/fixture.json
//   node quantwave-wasm/tests-node/parity.mjs target/wasm-node
// Asserts WASM output is bit-identical to native quantwave-core output.
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import path from "node:path";

const dir = path.resolve(process.argv[2] || "target/wasm-node");
const require = createRequire(import.meta.url);
const qw = require(path.join(dir, "quantwave_wasm.js"));
const fx = JSON.parse(readFileSync(path.join(dir, "fixture.json"), "utf8"));

const buf = new DataView(new ArrayBuffer(8));
const fromHex = (h) => (buf.setBigUint64(0, BigInt("0x" + h)), buf.getFloat64(0));
const toHex = (x) => (buf.setFloat64(0, x), buf.getBigUint64(0).toString(16).padStart(16, "0"));
const flat = fx.bars.map(fromHex);
const bars = [];
for (let i = 0; i < flat.length; i += 3) bars.push([flat[i], flat[i + 1], flat[i + 2]]);

const st = new qw.SuperTrend(10, 3.0), rsi = new qw.Rsi(14), ema = new qw.Ema(20);
const cc = new qw.CyberCycle(14), renko = new qw.Renko(1.5), kagi = new qw.Kagi(2.0);
const harm = new qw.HarmonicScanner(2);
const stride = qw.HarmonicScanner.stride();
const out = { supertrend: [], rsi: [], ema: [], cyber_cycle: [], renko: [], kagi: [], harmonic_d_bar_price_score: [] };
let harmonicCount = 0;
const kinds = new Set();
for (const [h, l, c] of bars) {
  out.supertrend.push(...st.next(h, l, c));
  out.rsi.push(rsi.next(c));
  out.ema.push(ema.next(c));
  out.cyber_cycle.push(...cc.next(c));
  out.renko.push(...renko.next(c));
  out.kagi.push(...kagi.next(c));
  const p = harm.next(h, l);
  for (let i = 0; i < p.length; i += stride) {
    harmonicCount++;
    kinds.add(qw.harmonicKindName(p[i]));
    out.harmonic_d_bar_price_score.push(p[i + 10], p[i + 11], p[i + 12]);
  }
}

let failures = 0;
for (const [name, got] of Object.entries(out)) {
  const want = fx[name];
  const g = got.map(toHex);
  let ok = g.length === want.length && g.every((v, i) => v === want[i]);
  const finite = got.filter(Number.isFinite).length;
  console.log(`${ok ? "PASS" : "FAIL"} ${name}: ${g.length} values (${finite} finite), bit-identical=${ok}`);
  if (!ok) failures++;
}
const countOk = harmonicCount === fx.harmonic_count && harmonicCount > 0;
console.log(`${countOk ? "PASS" : "FAIL"} harmonic patterns: ${harmonicCount} (core ${fx.harmonic_count}) kinds=${[...kinds].join(",")}`);
if (!countOk) failures++;
if (failures) {
  console.error(`${failures} parity failure(s)`);
  process.exit(1);
}
console.log("wasm == core: all curated indicators bit-identical");
