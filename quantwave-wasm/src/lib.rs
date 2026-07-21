//! # quantwave-wasm
//!
//! WebAssembly bindings that expose a curated subset of
//! [`quantwave-core`](https://docs.rs/quantwave-core)'s streaming indicators to
//! JavaScript, so QuantWave can compute indicators **in the browser** with no
//! Python and no server. This is the v1 tracer bullet (bead `quantwave-stb8`):
//! the simple `Next<Input>` streaming indicators only — **SuperTrend** and
//! **RSI**. Bar-shaping and pattern detectors (Renko/Kagi/harmonics) are
//! intentionally out of scope here.
//!
//! Each indicator is a small `#[wasm_bindgen]` struct that wraps the
//! corresponding `quantwave-core` type and forwards `next()` bar-by-bar, so the
//! browser output is bit-identical to the native Rust and Python paths.
//!
//! ```ignore
//! import init, { SuperTrend, Rsi } from "quantwave-wasm";
//! await init();
//! const st = new SuperTrend(10, 3.0);
//! const [value, direction] = st.next(high, low, close);
//! ```

use quantwave_core::SuperTrend as CoreSuperTrend;
use quantwave_core::indicators::incremental::rsi::RSI as CoreRsi;
use quantwave_core::traits::Next;
use wasm_bindgen::prelude::*;

/// Streaming SuperTrend, callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct SuperTrend {
    inner: CoreSuperTrend,
}

#[wasm_bindgen]
impl SuperTrend {
    /// `period` ATR lookback, `multiplier` band width.
    #[wasm_bindgen(constructor)]
    pub fn new(period: usize, multiplier: f64) -> SuperTrend {
        SuperTrend {
            inner: CoreSuperTrend::new(period, multiplier),
        }
    }

    /// Feed one `(high, low, close)` bar; returns `[supertrend_value, direction]`
    /// where `direction` is +1 (uptrend) or -1 (downtrend).
    pub fn next(&mut self, high: f64, low: f64, close: f64) -> Vec<f64> {
        let (value, direction) = self.inner.next((high, low, close));
        vec![value, direction as f64]
    }
}

/// Streaming RSI, callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct Rsi {
    inner: CoreRsi,
}

#[wasm_bindgen]
impl Rsi {
    /// `period` Wilder smoothing lookback.
    #[wasm_bindgen(constructor)]
    pub fn new(period: usize) -> Rsi {
        Rsi {
            inner: CoreRsi::new(period),
        }
    }

    /// Feed one `close`; returns the current RSI value.
    pub fn next(&mut self, close: f64) -> f64 {
        self.inner.next(close)
    }
}

#[cfg(test)]
mod tests {
    //! Native-target tests (crate is also built as `rlib`). They confirm the
    //! wasm shim forwards to `quantwave-core` without altering the numbers —
    //! the same parity contract the wasm/node build must uphold.
    use super::*;

    const HLC: &[(f64, f64, f64)] = &[
        (10.0, 9.0, 9.5),
        (11.0, 9.5, 10.8),
        (12.0, 10.0, 11.5),
        (11.5, 10.5, 10.7),
        (13.0, 10.8, 12.9),
        (13.5, 12.0, 12.2),
        (12.5, 11.0, 11.3),
        (14.0, 11.5, 13.8),
    ];

    #[test]
    fn supertrend_shim_matches_core() {
        let mut shim = SuperTrend::new(3, 3.0);
        let mut core = CoreSuperTrend::new(3, 3.0);
        for &(h, l, c) in HLC {
            let out = shim.next(h, l, c);
            let (value, direction) = core.next((h, l, c));
            assert_eq!(out, vec![value, direction as f64]);
        }
    }

    #[test]
    fn rsi_shim_matches_core() {
        let mut shim = Rsi::new(4);
        let mut core = CoreRsi::new(4);
        for &(_, _, c) in HLC {
            let (a, b) = (shim.next(c), core.next(c));
            // Warmup yields NaN; bit-identical means equal bits (NaN==NaN here).
            assert_eq!(a.to_bits(), b.to_bits());
        }
    }
}
