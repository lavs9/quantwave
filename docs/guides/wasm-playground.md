# Live Playground — Indicators in Your Browser

This page runs QuantWave's **actual Rust engine** — compiled to WebAssembly — directly in your browser. There is no server and no Python: the same `quantwave-core` streaming code that powers the native and Python builds computes **SuperTrend** and **RSI** here, bit-for-bit identical. Drag the sliders and watch it recompute live over the sample data.

<div id="qw-playground">
<div class="qw-status" id="qw-status">Loading the WebAssembly module…</div>
<div class="qw-controls">
<div class="qw-control"><label>SuperTrend period · <span class="qw-val" id="qw-st-period-val">10</span></label><input type="range" id="qw-st-period" min="3" max="30" step="1" value="10"></div>
<div class="qw-control"><label>SuperTrend multiplier · <span class="qw-val" id="qw-st-mult-val">3.0</span></label><input type="range" id="qw-st-mult" min="1" max="6" step="0.5" value="3"></div>
<div class="qw-control"><label>RSI period · <span class="qw-val" id="qw-rsi-period-val">14</span></label><input type="range" id="qw-rsi-period" min="3" max="30" step="1" value="14"></div>
</div>
<canvas id="qw-main" aria-label="Candlestick chart with SuperTrend overlay"></canvas>
<div class="qw-legend">
<span><span class="qw-swatch" style="background:#26a69a"></span>Up candle</span>
<span><span class="qw-swatch" style="background:#ef5350"></span>Down candle</span>
<span><span class="qw-swatch" style="background:#2e7d32"></span>SuperTrend (up)</span>
<span><span class="qw-swatch" style="background:#c62828"></span>SuperTrend (down)</span>
</div>
<canvas id="qw-rsi" aria-label="RSI subpanel"></canvas>
</div>

## How it works

The [`quantwave-wasm`](https://github.com/lavs9/quantwave/tree/main/quantwave-wasm) crate wraps `quantwave-core`'s streaming `Next<T>` indicators with [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/). The whole engine ships as a **~7 KB gzipped** WebAssembly module. Because it is the same Rust code, the numbers you see above match the native Rust and Python outputs exactly — QuantWave's batch/streaming parity guarantee, extended to a third runtime.

!!! note "What's here today"
    This first playground exposes the simple streaming indicators (SuperTrend, RSI). A broader indicator set, an npm package, and a CI-built bundle are tracked as follow-up work.
