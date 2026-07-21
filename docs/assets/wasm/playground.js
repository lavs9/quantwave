// quantwave-wasm live playground (bead quantwave-stb8).
//
// Runs quantwave-core's streaming SuperTrend + RSI *in the browser* over sample
// OHLC data, recomputing live as you drag the sliders. Same Rust engine, same
// numbers as the native/Python paths — no server, no Python.
//
// Loaded site-wide via mkdocs `extra_javascript` (type: module); it no-ops on
// every page that has no #qw-playground, and lazy-loads the wasm only here.

// Populated on first use by a dynamic import of the wasm-bindgen glue.
let wasm = null;
let resizeBound = false;

// ---- Deterministic sample OHLC (seeded random walk) ------------------------
function sampleBars(n) {
  let seed = 20260721;
  const rnd = () => {
    // Mulberry32 — small, deterministic PRNG.
    seed |= 0;
    seed = (seed + 0x6d2b79f5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
  const bars = [];
  let price = 100;
  let drift = 0.15;
  for (let i = 0; i < n; i++) {
    if (i % 22 === 0) drift = (rnd() - 0.5) * 1.2; // regime shifts
    const open = price;
    const move = drift + (rnd() - 0.5) * 2.4;
    const close = Math.max(5, open + move);
    const high = Math.max(open, close) + rnd() * 1.3;
    const low = Math.min(open, close) - rnd() * 1.3;
    bars.push({ open, high, low, close });
    price = close;
  }
  return bars;
}

const BARS = sampleBars(130);

// ---- Theme-aware colors ----------------------------------------------------
function colors() {
  const css = getComputedStyle(document.body);
  const text = css.color || "#333";
  return {
    text,
    grid: "rgba(128,128,128,0.18)",
    up: "#26a69a",
    down: "#ef5350",
    stUp: "#2e7d32",
    stDown: "#c62828",
    rsi: css.getPropertyValue("--md-accent-fg-color").trim() || "#5c6bc0",
    band: "rgba(128,128,128,0.35)",
  };
}

// ---- Compute indicators via wasm -------------------------------------------
function computeSuperTrend(period, mult) {
  const st = new wasm.SuperTrend(period, mult);
  const out = [];
  for (const b of BARS) {
    const [value, dir] = st.next(b.high, b.low, b.close);
    out.push(Number.isFinite(value) ? { value, dir } : null);
  }
  st.free();
  return out;
}

function computeRsi(period) {
  const r = new wasm.Rsi(period);
  const out = BARS.map((b) => {
    const v = r.next(b.close);
    return Number.isFinite(v) ? v : null;
  });
  r.free();
  return out;
}

// ---- Canvas helpers --------------------------------------------------------
function fitCanvas(cv, cssHeight) {
  const dpr = window.devicePixelRatio || 1;
  const w = cv.clientWidth || 700;
  cv.height = cssHeight * dpr;
  cv.width = w * dpr;
  cv.style.height = cssHeight + "px";
  const ctx = cv.getContext("2d");
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  return { ctx, w, h: cssHeight };
}

function priceScale(pad, h) {
  let lo = Infinity;
  let hi = -Infinity;
  for (const b of BARS) {
    lo = Math.min(lo, b.low);
    hi = Math.max(hi, b.high);
  }
  const span = hi - lo || 1;
  lo -= span * 0.05;
  hi += span * 0.05;
  return (p) => pad + (h - 2 * pad) * (1 - (p - lo) / (hi - lo));
}

function drawMain(cv, st) {
  const c = colors();
  const { ctx, w, h } = fitCanvas(cv, 360);
  ctx.clearRect(0, 0, w, h);
  const pad = 14;
  const y = priceScale(pad, h);
  const n = BARS.length;
  const step = (w - 2 * pad) / n;
  const x = (i) => pad + step * (i + 0.5);

  // gridlines
  ctx.strokeStyle = c.grid;
  ctx.lineWidth = 1;
  for (let g = 0; g <= 4; g++) {
    const yy = pad + ((h - 2 * pad) * g) / 4;
    ctx.beginPath();
    ctx.moveTo(pad, yy);
    ctx.lineTo(w - pad, yy);
    ctx.stroke();
  }

  // candles
  const bw = Math.max(1.5, step * 0.6);
  for (let i = 0; i < n; i++) {
    const b = BARS[i];
    const up = b.close >= b.open;
    ctx.strokeStyle = up ? c.up : c.down;
    ctx.fillStyle = up ? c.up : c.down;
    ctx.beginPath();
    ctx.moveTo(x(i), y(b.high));
    ctx.lineTo(x(i), y(b.low));
    ctx.stroke();
    const top = y(Math.max(b.open, b.close));
    const bot = y(Math.min(b.open, b.close));
    ctx.fillRect(x(i) - bw / 2, top, bw, Math.max(1, bot - top));
  }

  // SuperTrend line (colored by direction, broken across flips)
  ctx.lineWidth = 2;
  let prev = null;
  for (let i = 0; i < n; i++) {
    const s = st[i];
    if (!s) {
      prev = null;
      continue;
    }
    if (prev && prev.dir === s.dir) {
      ctx.strokeStyle = s.dir > 0 ? c.stUp : c.stDown;
      ctx.beginPath();
      ctx.moveTo(x(i - 1), y(prev.value));
      ctx.lineTo(x(i), y(s.value));
      ctx.stroke();
    }
    prev = s;
  }
}

function drawRsi(cv, rsi) {
  const c = colors();
  const { ctx, w, h } = fitCanvas(cv, 140);
  ctx.clearRect(0, 0, w, h);
  const pad = 12;
  const n = rsi.length;
  const step = (w - 2 * pad) / n;
  const x = (i) => pad + step * (i + 0.5);
  const y = (v) => pad + (h - 2 * pad) * (1 - v / 100);

  // 30 / 70 bands
  ctx.strokeStyle = c.band;
  ctx.setLineDash([4, 4]);
  for (const lvl of [30, 70]) {
    ctx.beginPath();
    ctx.moveTo(pad, y(lvl));
    ctx.lineTo(w - pad, y(lvl));
    ctx.stroke();
  }
  ctx.setLineDash([]);
  ctx.fillStyle = c.text;
  ctx.font = "10px sans-serif";
  ctx.globalAlpha = 0.6;
  ctx.fillText("70", w - pad - 16, y(70) - 3);
  ctx.fillText("30", w - pad - 16, y(30) - 3);
  ctx.globalAlpha = 1;

  ctx.strokeStyle = c.rsi;
  ctx.lineWidth = 1.6;
  ctx.beginPath();
  let started = false;
  for (let i = 0; i < n; i++) {
    const v = rsi[i];
    if (v == null) continue;
    if (!started) {
      ctx.moveTo(x(i), y(v));
      started = true;
    } else {
      ctx.lineTo(x(i), y(v));
    }
  }
  ctx.stroke();
}

// ---- Wire up ---------------------------------------------------------------
function el(id) {
  return document.getElementById(id);
}

function render() {
  const stPeriod = +el("qw-st-period").value;
  const stMult = +el("qw-st-mult").value;
  const rsiPeriod = +el("qw-rsi-period").value;
  el("qw-st-period-val").textContent = stPeriod;
  el("qw-st-mult-val").textContent = stMult.toFixed(1);
  el("qw-rsi-period-val").textContent = rsiPeriod;
  drawMain(el("qw-main"), computeSuperTrend(stPeriod, stMult));
  drawRsi(el("qw-rsi"), computeRsi(rsiPeriod));
}

async function main() {
  const root = document.getElementById("qw-playground");
  if (!root) return; // not the playground page — no-op
  const status = el("qw-status");
  try {
    if (!wasm) {
      const mod = await import("./quantwave_wasm.js");
      await mod.default(); // init() — fetches quantwave_wasm_bg.wasm
      wasm = mod;
    }
  } catch (e) {
    status.textContent = "⚠️ Failed to load the WebAssembly module: " + e;
    return;
  }
  status.textContent =
    "✅ quantwave-core running in your browser via WebAssembly — drag the sliders.";
  for (const id of ["qw-st-period", "qw-st-mult", "qw-rsi-period"]) {
    el(id).addEventListener("input", render);
  }
  if (!resizeBound) {
    window.addEventListener("resize", () => {
      if (wasm && document.getElementById("qw-playground")) render();
    });
    resizeBound = true;
  }
  render();
}

// mkdocs-material swaps page content via instant navigation; `document$` fires
// on every page load/navigation. Fall back to a plain call without it.
if (typeof document$ !== "undefined") {
  document$.subscribe(() => main());
} else if (document.readyState !== "loading") {
  main();
} else {
  document.addEventListener("DOMContentLoaded", main);
}
