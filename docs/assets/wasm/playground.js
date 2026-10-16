// quantwave-wasm live playground (bead quantwave-stb8).
//
// Runs quantwave-core's streaming SuperTrend, EMA, RSI, Ehlers CyberCycle,
// Renko/Kagi bar builders and the harmonic pattern scanner *in the browser*
// over sample OHLC data, recomputing live as you drag the sliders. Same Rust engine, same
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
    ema: "#f9a825",
    cycle: "#00897b",
    trigger: "#8e24aa",
    harmBull: "rgba(38,166,154,0.9)",
    harmBear: "rgba(239,83,80,0.9)",
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

function computeEma(period) {
  const e = new wasm.Ema(period);
  const out = BARS.map((b) => {
    const v = e.next(b.close);
    return Number.isFinite(v) ? v : null;
  });
  e.free();
  return out;
}

function computeCyberCycle(length) {
  const cc = new wasm.CyberCycle(length);
  const out = BARS.map((b) => {
    const [cycle, trigger] = cc.next((b.high + b.low) / 2);
    return Number.isFinite(cycle) ? { cycle, trigger } : null;
  });
  cc.free();
  return out;
}

// Bar builders return completed bricks/lines; flatten to {open, close, dir}.
function computeRenko(box) {
  const r = new wasm.Renko(box);
  const out = [];
  for (const b of BARS) {
    const f = r.next(b.close);
    for (let i = 0; i < f.length; i += 3) out.push({ open: f[i], close: f[i + 1], dir: f[i + 2] });
  }
  r.free();
  return out;
}

function computeKagi(reversal) {
  const k = new wasm.Kagi(reversal);
  const out = [];
  for (const b of BARS) {
    const f = k.next(b.close);
    if (f.length) out.push({ open: f[0], close: f[1], dir: f[2], thick: f[3] });
  }
  k.free();
  return out;
}

function computeHarmonics(swing) {
  const h = new wasm.HarmonicScanner(swing);
  const stride = wasm.HarmonicScanner.stride();
  const out = [];
  for (const b of BARS) {
    const f = h.next(b.high, b.low);
    for (let i = 0; i < f.length; i += stride) {
      const pts = [];
      // Scanner bars are 1-based; chart indices are 0-based.
      if (Number.isFinite(f[i + 2])) pts.push([f[i + 2] - 1, f[i + 3]]);
      for (let j = 4; j <= 10; j += 2) pts.push([f[i + j] - 1, f[i + j + 1]]);
      out.push({ kind: wasm.harmonicKindName(f[i]), bull: f[i + 1] === 1, pts, score: f[i + 12] });
    }
  }
  h.free();
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

function drawMain(cv, st, ema, harmonics) {
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

  // EMA overlay
  drawLine(ctx, ema, x, y, c.ema, 1.4);

  // Harmonic patterns: connect the pivots and label the kind at D.
  ctx.font = "11px sans-serif";
  for (const p of harmonics) {
    const col = p.bull ? c.harmBull : c.harmBear;
    ctx.strokeStyle = col;
    ctx.fillStyle = col;
    ctx.lineWidth = 1.5;
    ctx.setLineDash([5, 3]);
    ctx.beginPath();
    p.pts.forEach(([bi, pr], k) => (k ? ctx.lineTo(x(bi), y(pr)) : ctx.moveTo(x(bi), y(pr))));
    ctx.stroke();
    ctx.setLineDash([]);
    for (const [bi, pr] of p.pts) {
      ctx.beginPath();
      ctx.arc(x(bi), y(pr), 2.5, 0, 2 * Math.PI);
      ctx.fill();
    }
    const [dbi, dpr] = p.pts[p.pts.length - 1];
    ctx.fillText(`${p.kind} ${p.bull ? "▲" : "▼"} ${(p.score * 100).toFixed(0)}%`, x(dbi) + 4, y(dpr) + (p.bull ? 14 : -6));
  }
}

function drawLine(ctx, values, x, y, color, width) {
  ctx.strokeStyle = color;
  ctx.lineWidth = width;
  ctx.beginPath();
  let started = false;
  for (let i = 0; i < values.length; i++) {
    const v = values[i];
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

function drawCycle(cv, cyc) {
  const c = colors();
  const { ctx, w, h } = fitCanvas(cv, 120);
  ctx.clearRect(0, 0, w, h);
  const pad = 12;
  const n = cyc.length;
  let m = 1e-9;
  // Skip the filter's warmup transient when scaling and drawing.
  const warm = Math.min(n, 30);
  cyc = cyc.map((v, i) => (i < warm ? null : v));
  for (const v of cyc) if (v) m = Math.max(m, Math.abs(v.cycle), Math.abs(v.trigger));
  const step = (w - 2 * pad) / n;
  const x = (i) => pad + step * (i + 0.5);
  const y = (v) => h / 2 - ((h / 2 - pad) * v) / m;
  ctx.strokeStyle = c.band;
  ctx.setLineDash([4, 4]);
  ctx.beginPath();
  ctx.moveTo(pad, h / 2);
  ctx.lineTo(w - pad, h / 2);
  ctx.stroke();
  ctx.setLineDash([]);
  drawLine(ctx, cyc.map((v) => (v ? v.cycle : null)), x, y, c.cycle, 1.6);
  drawLine(ctx, cyc.map((v) => (v ? v.trigger : null)), x, y, c.trigger, 1.1);
}

// Renko bricks / Kagi lines are drawn in their own time-independent space.
function drawBricks(cv, bricks, kind) {
  const c = colors();
  const { ctx, w, h } = fitCanvas(cv, 200);
  ctx.clearRect(0, 0, w, h);
  const pad = 12;
  if (!bricks.length) {
    ctx.fillStyle = c.text;
    ctx.fillText("No completed " + kind + " yet — lower the size.", pad, h / 2);
    return;
  }
  let lo = Infinity;
  let hi = -Infinity;
  for (const b of bricks) {
    lo = Math.min(lo, b.open, b.close);
    hi = Math.max(hi, b.open, b.close);
  }
  const span = hi - lo || 1;
  const y = (p) => pad + (h - 2 * pad) * (1 - (p - lo) / span);
  const step = (w - 2 * pad) / bricks.length;
  if (kind === "bricks") {
    bricks.forEach((b, i) => {
      ctx.fillStyle = b.dir > 0 ? c.up : c.down;
      const top = y(Math.max(b.open, b.close));
      const bot = y(Math.min(b.open, b.close));
      ctx.fillRect(pad + i * step + 0.5, top, Math.max(1, step - 1), Math.max(1, bot - top));
    });
  } else {
    // Kagi: vertical line per segment, horizontal shoulder/waist joins; thick=yang.
    let px = null;
    bricks.forEach((b, i) => {
      const xx = pad + (i + 0.5) * step;
      // thickness: +1 yang (thick, green), -1 yin (thin, red), 0 not yet set.
      ctx.strokeStyle = b.thick > 0 ? c.up : b.thick < 0 ? c.down : c.band;
      ctx.lineWidth = b.thick > 0 ? 3.5 : 1.4;
      ctx.beginPath();
      if (px !== null) {
        ctx.moveTo(px, y(b.open));
        ctx.lineTo(xx, y(b.open));
      } else {
        ctx.moveTo(xx, y(b.open));
      }
      ctx.lineTo(xx, y(b.close));
      ctx.stroke();
      px = xx;
    });
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

const SLIDERS = {
  "qw-st-period": (v) => v,
  "qw-st-mult": (v) => v.toFixed(1),
  "qw-ema-period": (v) => v,
  "qw-rsi-period": (v) => v,
  "qw-cc-length": (v) => v,
  "qw-harm-swing": (v) => v,
  "qw-renko-box": (v) => v.toFixed(1),
  "qw-kagi-rev": (v) => v.toFixed(1),
};

function render() {
  const v = {};
  for (const [id, fmt] of Object.entries(SLIDERS)) {
    v[id] = +el(id).value;
    el(id + "-val").textContent = fmt(v[id]);
  }
  const harmonics = computeHarmonics(v["qw-harm-swing"]);
  drawMain(
    el("qw-main"),
    computeSuperTrend(v["qw-st-period"], v["qw-st-mult"]),
    computeEma(v["qw-ema-period"]),
    harmonics,
  );
  el("qw-harm-count").textContent = harmonics.length
    ? harmonics.map((p) => `${p.kind} ${p.bull ? "▲" : "▼"}`).join(" · ")
    : "none at this swing strength";
  drawRsi(el("qw-rsi"), computeRsi(v["qw-rsi-period"]));
  drawCycle(el("qw-cycle"), computeCyberCycle(v["qw-cc-length"]));
  drawBricks(el("qw-renko"), computeRenko(v["qw-renko-box"]), "bricks");
  drawBricks(el("qw-kagi"), computeKagi(v["qw-kagi-rev"]), "lines");
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
  for (const id of Object.keys(SLIDERS)) {
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
