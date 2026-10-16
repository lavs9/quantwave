# Live Playground — Indicators in Your Browser

This page runs QuantWave's **actual Rust engine**, compiled to WebAssembly, directly in your browser. There is no server and no Python. The same `quantwave-core` streaming code that powers the native and Python builds computes **SuperTrend, EMA, RSI, Ehlers' CyberCycle, Renko and Kagi bars, and the harmonic pattern scanner** here, bit-for-bit identical. Drag the sliders and watch it recompute live over the sample data.

<div id="qw-playground">
<div class="qw-status" id="qw-status">Loading the WebAssembly module…</div>
<div class="qw-controls">
<div class="qw-control"><label>SuperTrend period · <span class="qw-val" id="qw-st-period-val">10</span></label><input type="range" id="qw-st-period" min="3" max="30" step="1" value="10"></div>
<div class="qw-control"><label>SuperTrend multiplier · <span class="qw-val" id="qw-st-mult-val">3.0</span></label><input type="range" id="qw-st-mult" min="1" max="6" step="0.5" value="3"></div>
<div class="qw-control"><label>EMA period · <span class="qw-val" id="qw-ema-period-val">20</span></label><input type="range" id="qw-ema-period" min="3" max="60" step="1" value="20"></div>
<div class="qw-control"><label>Harmonic swing strength · <span class="qw-val" id="qw-harm-swing-val">2</span></label><input type="range" id="qw-harm-swing" min="1" max="5" step="1" value="2"></div>
</div>
<canvas id="qw-main" aria-label="Candlestick chart with SuperTrend, EMA and harmonic pattern overlays"></canvas>
<div class="qw-legend">
<span><span class="qw-swatch" style="background:#26a69a"></span>Up candle</span>
<span><span class="qw-swatch" style="background:#ef5350"></span>Down candle</span>
<span><span class="qw-swatch" style="background:#2e7d32"></span>SuperTrend (up)</span>
<span><span class="qw-swatch" style="background:#c62828"></span>SuperTrend (down)</span>
<span><span class="qw-swatch" style="background:#f9a825"></span>EMA</span>
<span>Harmonic patterns: <span id="qw-harm-count">…</span></span>
</div>
<div class="qw-controls">
<div class="qw-control"><label>RSI period · <span class="qw-val" id="qw-rsi-period-val">14</span></label><input type="range" id="qw-rsi-period" min="3" max="30" step="1" value="14"></div>
</div>
<canvas id="qw-rsi" aria-label="RSI subpanel"></canvas>
<div class="qw-controls">
<div class="qw-control"><label>CyberCycle length · <span class="qw-val" id="qw-cc-length-val">14</span></label><input type="range" id="qw-cc-length" min="4" max="40" step="1" value="14"></div>
</div>
<canvas id="qw-cycle" aria-label="Ehlers CyberCycle subpanel"></canvas>
<div class="qw-legend">
<span><span class="qw-swatch" style="background:#00897b"></span>CyberCycle</span>
<span><span class="qw-swatch" style="background:#8e24aa"></span>Trigger</span>
</div>
<div class="qw-controls">
<div class="qw-control"><label>Renko box size · <span class="qw-val" id="qw-renko-box-val">1.5</span></label><input type="range" id="qw-renko-box" min="0.5" max="5" step="0.5" value="1.5"></div>
<div class="qw-control"><label>Kagi reversal · <span class="qw-val" id="qw-kagi-rev-val">1.0</span></label><input type="range" id="qw-kagi-rev" min="0.5" max="4" step="0.5" value="1"></div>
</div>
<canvas id="qw-renko" aria-label="Renko bricks"></canvas>
<canvas id="qw-kagi" aria-label="Kagi lines"></canvas>
</div>

## How it works

The [`quantwave-wasm`](https://github.com/lavs9/quantwave/tree/main/quantwave-wasm) crate wraps `quantwave-core`'s streaming `Next<T>` indicators with [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/). The whole engine ships as a **~20 KB gzipped** WebAssembly module. Because it is the same Rust code, the numbers you see above match the native Rust and Python outputs exactly. A node test checks that bit for bit against `quantwave-core` for every indicator on this page. That's QuantWave's batch/streaming parity guarantee, extended to a third runtime.

Rebuild locally with `./scripts/build_wasm_playground.sh`, then `mkdocs serve` and open `/guides/wasm-playground/`.

!!! note "What's here today"
    A curated, chart-friendly subset. Full indicator coverage, an npm package, and a CI-built wasm bundle are tracked as follow-up work.
