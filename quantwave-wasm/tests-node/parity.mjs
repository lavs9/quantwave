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
// ---- Harmonic scanner: every field, every kind & direction, 0-based bars ----
const KINDS = ["abcd", "alternate_abcd", "5-0", "gartley", "bat", "butterfly", "crab", "alternate_bat"];
const seen = new Set();
for (const s of fx.harmonic_series) {
  const hl = s.bars.map(fromHex);
  const sc = new qw.HarmonicScanner(s.ss);
  const got = [];
  for (let i = 0; i < hl.length; i += 2) {
    const f = sc.next(hl[i], hl[i + 1]);
    for (let j = 0; j < f.length; j += stride) got.push(Array.from(f.slice(j, j + stride)));
  }
  sc.free();
  let ok = got.length === s.patterns.length;
  if (s.name.startsWith("no_match")) ok = ok && got.length === 0;
  for (let k = 0; ok && k < got.length; k++) {
    const g = got[k], w = s.patterns[k];
    const kind = qw.harmonicKindName(g[0]);
    const fieldsOk = g.slice(2).map(toHex).join() === w.f.join();
    const bullOk = (g[1] === 1) === w.bull && (g[1] === 0 || g[1] === 1);
    // AB=CD family has no X point.
    const xOk = !kind.includes("abcd") || (Number.isNaN(g[2]) && Number.isNaN(g[3]));
    // Pin 0-based bar indices: each pivot price is high or low of bars[bar].
    let baseOk = true;
    for (let j = 4; j <= 10; j += 2) {
      const bar = g[j], price = g[j + 1];
      if (!(hl[2 * bar] === price || hl[2 * bar + 1] === price)) baseOk = false;
    }
    if (kind !== w.kind || !fieldsOk || !bullOk || !xOk || !baseOk) {
      ok = false;
      console.log("MISMATCH", s.name, k, { kind, want: w.kind, fieldsOk, bullOk, xOk, baseOk });
    }
    seen.add(`${w.kind}/${w.bull ? "bull" : "bear"}`);
  }
  console.log(`${ok ? "PASS" : "FAIL"} harmonic ${s.name}: core=${s.patterns.length} wasm=${got.length}`);
  if (!ok) failures++;
}
const missing = KINDS.flatMap((k) => ["bull", "bear"].map((d) => `${k}/${d}`)).filter((k) => !seen.has(k));
console.log(`${missing.length ? "FAIL" : "PASS"} harmonic kind/direction coverage: ${seen.size}/16${missing.length ? " missing " + missing.join(",") : ""}`);
if (missing.length) failures++;
const unknownOk = qw.harmonicKindName(NaN) === "unknown" && qw.harmonicKindName(-1) === "unknown" && qw.harmonicKindName(1.5) === "unknown";
console.log(`${unknownOk ? "PASS" : "FAIL"} harmonicKindName(NaN/-1/1.5) === "unknown"`);
if (!unknownOk) failures++;

if (failures) {
  console.error(`${failures} parity failure(s)`);
  process.exit(1);
}
console.log("wasm == core: all curated indicators bit-identical");
