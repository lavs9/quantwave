// Node smoke + parity test for the quantwave-wasm build (quantwave-stb8).
//   ./scripts/build_wasm_playground.sh --node
//   cargo run -q -p quantwave-wasm --example parity_fixture > target/wasm-node/fixture.json
//   node quantwave-wasm/tests-node/parity.mjs target/wasm-node
// Asserts WASM output is bit-identical to native quantwave-core output.
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

// The playground's own DOM-free pivot-to-chart mapping (docs/assets/wasm/harmonic_map.js).
const here = path.dirname(fileURLToPath(import.meta.url));
const { harmonicPoints } = await import(path.join(here, "../../docs/assets/wasm/harmonic_map.js"));

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
const sma = new qw.Sma(20), wma = new qw.Wma(20), ss = new qw.SuperSmoother(10);
const atr = new qw.Atr(14), adx = new qw.Adx(14), adxr = new qw.Adxr(14);
const macd = new qw.Macd(12, 26, 9), bbands = new qw.Bbands(20, 2.0, 2.0);
const stoch = new qw.Stoch(14, 3, 3), ichimoku = new qw.Ichimoku(9, 26, 52);
const rf = new qw.RoofingFilter(48, 10), kf = new qw.KalmanFilter(0.01, 0.1);
const ttm = new qw.TtmSqueeze(20, 2.0, 1.5), ms = new qw.MarketStructure(3);
const cc = new qw.CyberCycle(14), renko = new qw.Renko(1.5), kagi = new qw.Kagi(2.0);
const harm = new qw.HarmonicScanner(2);
const stride = qw.HarmonicScanner.stride();
const out = {
  supertrend: [], rsi: [], ema: [], sma: [], wma: [], super_smoother: [],
  atr: [], adx: [], adxr: [], macd: [], bbands: [], stoch: [], ichimoku: [],
  roofing_filter: [], kalman_filter: [], ttm_squeeze: [], market_structure: [],
  cyber_cycle: [], renko: [], kagi: [], harmonic_d_bar_price_score: []
};
let harmonicCount = 0;
const kinds = new Set();
for (const [h, l, c] of bars) {
  out.supertrend.push(...st.next(h, l, c));
  out.rsi.push(rsi.next(c));
  out.ema.push(ema.next(c));
  out.sma.push(sma.next(c));
  out.wma.push(wma.next(c));
  out.super_smoother.push(ss.next(c));
  out.atr.push(atr.next(h, l, c));
  out.adx.push(adx.next(h, l, c));
  out.adxr.push(adxr.next(h, l, c));
  out.macd.push(...macd.next(c));
  out.bbands.push(...bbands.next(c));
  out.stoch.push(...stoch.next(h, l, c));
  out.ichimoku.push(...ichimoku.next(h, l));
  out.roofing_filter.push(rf.next(c));
  out.kalman_filter.push(kf.next(c));
  out.ttm_squeeze.push(...ttm.next(h, l, c));
  out.market_structure.push(...ms.next(h, l));
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
let xPinned = 0;
let mappedPoints = 0;
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
    // The playground mapping: every mapped point (X when present, then A..D)
    // must sit on the high or low of the chart bar it is drawn at.
    const pts = harmonicPoints(g, 0);
    const wantPts = Number.isNaN(g[2]) ? 4 : 5;
    let mapOk = pts.length === wantPts;
    for (const [bar, price] of pts) {
      if (!(Number.isInteger(bar) && (hl[2 * bar] === price || hl[2 * bar + 1] === price))) mapOk = false;
    }
    if (!Number.isNaN(g[2])) xPinned++;
    mappedPoints += pts.length;
    if (kind !== w.kind || !fieldsOk || !bullOk || !xOk || !baseOk || !mapOk) {
      ok = false;
      console.log("MISMATCH", s.name, k, { kind, want: w.kind, fieldsOk, bullOk, xOk, baseOk, mapOk });
    }
    seen.add(`${w.kind}/${w.bull ? "bull" : "bear"}`);
  }
  console.log(`${ok ? "PASS" : "FAIL"} harmonic ${s.name}: core=${s.patterns.length} wasm=${got.length}`);
  if (!ok) failures++;
}
const mapCoverOk = xPinned > 0 && mappedPoints > 0;
console.log(`${mapCoverOk ? "PASS" : "FAIL"} playground harmonicPoints mapping: ${mappedPoints} points on bar high/low (${xPinned} patterns with X)`);
if (!mapCoverOk) failures++;
const missing = KINDS.flatMap((k) => ["bull", "bear"].map((d) => `${k}/${d}`)).filter((k) => !seen.has(k));
console.log(`${missing.length ? "FAIL" : "PASS"} harmonic kind/direction coverage: ${seen.size}/16${missing.length ? " missing " + missing.join(",") : ""}`);
if (missing.length) failures++;
const unknownOk = qw.harmonicKindName(NaN) === "unknown" && qw.harmonicKindName(-1) === "unknown" && qw.harmonicKindName(1.5) === "unknown";
console.log(`${unknownOk ? "PASS" : "FAIL"} harmonicKindName(NaN/-1/1.5) === "unknown"`);
if (!unknownOk) failures++;

// ---- Gold-standard JSON fixtures (extending parity to quantwave-core/tests/gold_standard) ----
const goldDir = path.join(here, "../../quantwave-core/tests/gold_standard");
const approxEq = (a, b, tol = 1e-9) => (a === null || Number.isNaN(a)) ? Number.isNaN(b) : Math.abs(a - b) <= tol;

{
  const g = JSON.parse(readFileSync(path.join(goldDir, "sma_5.json"), "utf8"));
  const s = new qw.Sma(3);
  let ok = true;
  for (let i = 0; i < g.input.length; i++) {
    if (!approxEq(g.expected[i], s.next(g.input[i]))) ok = false;
  }
  s.free();
  console.log(`${ok ? "PASS" : "FAIL"} gold_standard/sma_5.json`);
  if (!ok) failures++;
}

{
  const g = JSON.parse(readFileSync(path.join(goldDir, "supertrend_10_3.json"), "utf8"));
  const s = new qw.SuperTrend(10, 3.0);
  let ok = true;
  for (let i = 0; i < g.close.length; i++) {
    const [val, dir] = s.next(g.high[i], g.low[i], g.close[i]);
    if (!approxEq(g.expected_st[i], val) || dir !== g.expected_dir[i]) ok = false;
  }
  s.free();
  console.log(`${ok ? "PASS" : "FAIL"} gold_standard/supertrend_10_3.json`);
  if (!ok) failures++;
}

{
  const g = JSON.parse(readFileSync(path.join(goldDir, "ichimoku.json"), "utf8"));
  const s = new qw.Ichimoku(9, 26, 52);
  let ok = true;
  for (let i = 0; i < g.high.length; i++) {
    const [t, k, sa, sb] = s.next(g.high[i], g.low[i]);
    if (!approxEq(g.expected_tenkan[i], t) || !approxEq(g.expected_kijun[i], k) || !approxEq(g.expected_senkou_a[i], sa) || !approxEq(g.expected_senkou_b[i], sb)) ok = false;
  }
  s.free();
  console.log(`${ok ? "PASS" : "FAIL"} gold_standard/ichimoku.json`);
  if (!ok) failures++;
}

{
  const g = JSON.parse(readFileSync(path.join(goldDir, "ttm_squeeze_20_2_15.json"), "utf8"));
  const s = new qw.TtmSqueeze(20, 2.0, 1.5);
  let ok = true;
  for (let i = 0; i < g.close.length; i++) {
    const [hist, sq] = s.next(g.high[i], g.low[i], g.close[i]);
    if (!approxEq(g.expected_histogram[i], hist) || (sq === 1) !== g.expected_squeezed[i]) ok = false;
  }
  s.free();
  console.log(`${ok ? "PASS" : "FAIL"} gold_standard/ttm_squeeze_20_2_15.json`);
  if (!ok) failures++;
}

if (failures) {
  console.error(`${failures} parity failure(s)`);
  process.exit(1);
}
console.log("wasm == core: all curated indicators bit-identical");
