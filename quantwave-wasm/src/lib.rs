//! # quantwave-wasm
//!
//! WebAssembly bindings that expose a curated subset of
//! [`quantwave-core`](https://docs.rs/quantwave-core)'s streaming indicators to
//! JavaScript, so QuantWave can compute indicators **in the browser** with no
//! Python and no server. This is the v1 tracer bullet (bead `quantwave-stb8`):
//! the curated, chart-friendly subset — **SuperTrend**, **RSI**, **EMA**,
//! Ehlers' **CyberCycle**, **Renko** and **Kagi** bar builders, and the
//! **harmonic pattern scanner**. Full coverage, an npm package and a CI wasm
//! build are follow-up work.
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
use quantwave_core::indicators::cyber_cycle::CyberCycle as CoreCyberCycle;
use quantwave_core::indicators::incremental::rsi::RSI as CoreRsi;
use quantwave_core::traits::Next;
use quantwave_core::{
    EMA as CoreEma, HarmonicKind, HarmonicPattern, HarmonicPatternScanner as CoreHarmonic,
    KagiBuilder as CoreKagi, RenkoBuilder as CoreRenko,
};
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

/// Streaming EMA, callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct Ema {
    inner: CoreEma,
}

#[wasm_bindgen]
impl Ema {
    #[wasm_bindgen(constructor)]
    pub fn new(period: usize) -> Ema {
        Ema {
            inner: CoreEma::new(period),
        }
    }

    /// Feed one value; returns the current EMA.
    pub fn next(&mut self, value: f64) -> f64 {
        self.inner.next(value)
    }
}

/// Streaming Ehlers CyberCycle, callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct CyberCycle {
    inner: CoreCyberCycle,
}

#[wasm_bindgen]
impl CyberCycle {
    #[wasm_bindgen(constructor)]
    pub fn new(length: usize) -> CyberCycle {
        CyberCycle {
            inner: CoreCyberCycle::new(length),
        }
    }

    /// Feed one price; returns `[cycle, trigger]`.
    pub fn next(&mut self, price: f64) -> Vec<f64> {
        let (cycle, trigger) = self.inner.next(price);
        vec![cycle, trigger]
    }
}

/// Streaming Renko brick builder.
#[wasm_bindgen]
pub struct Renko {
    inner: CoreRenko,
}

#[wasm_bindgen]
impl Renko {
    #[wasm_bindgen(constructor)]
    pub fn new(box_size: f64) -> Renko {
        Renko {
            inner: CoreRenko::new(box_size),
        }
    }

    /// Feed one close; returns the bricks it completed, flattened as
    /// `[open, close, direction, open, close, direction, ...]` (may be empty).
    pub fn next(&mut self, close: f64) -> Vec<f64> {
        self.inner
            .next(close)
            .iter()
            .flat_map(|b| [b.open, b.close, f64::from(b.direction)])
            .collect()
    }
}

/// Streaming Kagi line builder.
#[wasm_bindgen]
pub struct Kagi {
    inner: CoreKagi,
}

#[wasm_bindgen]
impl Kagi {
    #[wasm_bindgen(constructor)]
    pub fn new(reversal: f64) -> Kagi {
        Kagi {
            inner: CoreKagi::new(reversal),
        }
    }

    /// Feed one price; returns `[open, close, direction, thickness]` when the
    /// price completes a Kagi line, else an empty array.
    pub fn next(&mut self, price: f64) -> Vec<f64> {
        match self.inner.next(price) {
            Some(l) => vec![
                l.open,
                l.close,
                f64::from(l.direction),
                f64::from(l.thickness),
            ],
            None => Vec::new(),
        }
    }
}

const HARMONIC_KINDS: [HarmonicKind; 8] = [
    HarmonicKind::AbCd,
    HarmonicKind::AlternateAbCd,
    HarmonicKind::FiveZero,
    HarmonicKind::Gartley,
    HarmonicKind::Bat,
    HarmonicKind::Butterfly,
    HarmonicKind::Crab,
    HarmonicKind::AlternateBat,
];

/// Number of `f64` fields per pattern in [`HarmonicScanner::next`] output.
pub const HARMONIC_STRIDE: usize = 15;

fn encode_pattern(p: &HarmonicPattern) -> [f64; HARMONIC_STRIDE] {
    let kind = HARMONIC_KINDS
        .iter()
        .position(|k| *k == p.kind)
        .map_or(f64::NAN, |i| i as f64);
    [
        kind,
        if p.is_bull { 1.0 } else { 0.0 },
        p.x_bar.map_or(f64::NAN, |b| b as f64),
        p.x_price.unwrap_or(f64::NAN),
        p.a_bar as f64,
        p.a_price,
        p.b_bar as f64,
        p.b_price,
        p.c_bar as f64,
        p.c_price,
        p.d_bar as f64,
        p.d_price,
        p.score,
        p.prz_low,
        p.prz_high,
    ]
}

/// Name of a harmonic pattern kind code returned by [`HarmonicScanner::next`].
#[wasm_bindgen(js_name = harmonicKindName)]
pub fn harmonic_kind_name(code: usize) -> String {
    HARMONIC_KINDS
        .get(code)
        .map_or("unknown", |k| k.as_str())
        .to_string()
}

/// Streaming harmonic pattern scanner (AB=CD, 5-0, Gartley family).
#[wasm_bindgen]
pub struct HarmonicScanner {
    inner: CoreHarmonic,
}

#[wasm_bindgen]
impl HarmonicScanner {
    /// `swing_strength` pivot confirmation bars on each side.
    #[wasm_bindgen(constructor)]
    pub fn new(swing_strength: usize) -> HarmonicScanner {
        HarmonicScanner {
            inner: CoreHarmonic::new(swing_strength),
        }
    }

    /// Fields per pattern in the flattened output of [`HarmonicScanner::next`].
    pub fn stride() -> usize {
        HARMONIC_STRIDE
    }

    /// Feed one `(high, low)` bar; returns newly completed patterns flattened,
    /// [`HARMONIC_STRIDE`] values each: `kind, is_bull, x_bar, x_price, a_bar,
    /// a_price, b_bar, b_price, c_bar, c_price, d_bar, d_price, score, prz_low,
    /// prz_high` (`x_*` is NaN when the pattern has no X point).
    pub fn next(&mut self, high: f64, low: f64) -> Vec<f64> {
        self.inner
            .next((high, low))
            .iter()
            .flat_map(encode_pattern)
            .collect()
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

    #[test]
    fn new_shims_match_core() {
        let mut ema = Ema::new(3);
        let mut core_ema = CoreEma::new(3);
        let mut cc = CyberCycle::new(5);
        let mut core_cc = CoreCyberCycle::new(5);
        let mut renko = Renko::new(0.5);
        let mut core_renko = CoreRenko::new(0.5);
        let mut kagi = Kagi::new(0.5);
        let mut core_kagi = CoreKagi::new(0.5);
        for &(_, _, c) in HLC {
            assert_eq!(ema.next(c).to_bits(), core_ema.next(c).to_bits());
            let (cy, tr) = core_cc.next(c);
            let out = cc.next(c);
            assert_eq!(
                (out[0].to_bits(), out[1].to_bits()),
                (cy.to_bits(), tr.to_bits())
            );
            let want: Vec<f64> = core_renko
                .next(c)
                .iter()
                .flat_map(|b| [b.open, b.close, f64::from(b.direction)])
                .collect();
            assert_eq!(renko.next(c), want);
            let want: Vec<f64> = core_kagi.next(c).map_or(vec![], |l| {
                vec![
                    l.open,
                    l.close,
                    f64::from(l.direction),
                    f64::from(l.thickness),
                ]
            });
            assert_eq!(kagi.next(c), want);
        }
    }

    #[test]
    fn harmonic_shim_matches_core_and_finds_patterns() {
        let mut shim = HarmonicScanner::new(2);
        let mut core = CoreHarmonic::new(2);
        let mut found = 0;
        for i in 0..400 {
            let t = i as f64;
            let mid = 100.0 + 10.0 * (t / 9.0).sin() + 4.0 * (t / 3.7).sin();
            let (h, l) = (mid + 0.6, mid - 0.6);
            let want: Vec<f64> = core.next((h, l)).iter().flat_map(encode_pattern).collect();
            let got = shim.next(h, l);
            assert_eq!(got.len(), want.len());
            for (g, w) in got.iter().zip(&want) {
                assert_eq!(g.to_bits(), w.to_bits());
            }
            found += got.len() / HARMONIC_STRIDE;
        }
        assert!(found > 0, "fixture should produce at least one pattern");
        assert_eq!(harmonic_kind_name(3), "gartley");
        assert_eq!(harmonic_kind_name(99), "unknown");
    }
}
