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
use quantwave_core::indicators::ichimoku::IchimokuCloud as CoreIchimoku;
use quantwave_core::indicators::incremental::bbands::BBANDS as CoreBbands;
use quantwave_core::indicators::incremental::dmi::{ADX as CoreAdx, ADXR as CoreAdxr};
use quantwave_core::indicators::incremental::macd::MACD as CoreMacd;
use quantwave_core::indicators::incremental::rsi::RSI as CoreRsi;
use quantwave_core::indicators::incremental::stoch::STOCH as CoreStoch;
use quantwave_core::indicators::kalman::KalmanFilter as CoreKalman;
use quantwave_core::indicators::roofing_filter::RoofingFilter as CoreRoofingFilter;
use quantwave_core::indicators::super_smoother::SuperSmoother as CoreSuperSmoother;
use quantwave_core::indicators::ttm_squeeze::TTMSqueeze as CoreTtmSqueeze;
use quantwave_core::indicators::volatility::TaATR as CoreAtr;
use quantwave_core::traits::Next;
use quantwave_core::{
    EMA as CoreEma, HarmonicKind, HarmonicPattern, HarmonicPatternScanner as CoreHarmonic,
    KagiBuilder as CoreKagi, MaType, MarketStructure as CoreMarketStructure,
    RenkoBuilder as CoreRenko, SMA as CoreSma, WMA as CoreWma,
};
use wasm_bindgen::prelude::*;

/// Generates a 1-input (`f64`), 1-output (`f64`) `#[wasm_bindgen]` wrapper
/// with a `(period: usize)` constructor.
macro_rules! wasm_1in_1out {
    ($(#[$meta:meta])* $name:ident, $core:ty) => {
        $(#[$meta])*
        #[wasm_bindgen]
        pub struct $name {
            inner: $core,
        }

        #[wasm_bindgen]
        impl $name {
            #[wasm_bindgen(constructor)]
            pub fn new(period: usize) -> $name {
                $name {
                    inner: <$core>::new(period),
                }
            }

            /// Feed one price sample; returns the updated indicator value.
            pub fn next(&mut self, value: f64) -> f64 {
                self.inner.next(value)
            }
        }
    };
}

/// Generates a 3-input `(high, low, close)`, 1-output (`f64`) `#[wasm_bindgen]`
/// wrapper with a `(period: usize)` constructor.
macro_rules! wasm_hlc_1out {
    ($(#[$meta:meta])* $name:ident, $core:ty) => {
        $(#[$meta])*
        #[wasm_bindgen]
        pub struct $name {
            inner: $core,
        }

        #[wasm_bindgen]
        impl $name {
            #[wasm_bindgen(constructor)]
            pub fn new(period: usize) -> $name {
                $name {
                    inner: <$core>::new(period),
                }
            }

            /// Feed one `(high, low, close)` bar; returns the updated indicator value.
            pub fn next(&mut self, high: f64, low: f64, close: f64) -> f64 {
                self.inner.next((high, low, close))
            }
        }
    };
}

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

wasm_1in_1out!(
    /// Streaming RSI, callable bar-by-bar from JS.
    Rsi,
    CoreRsi
);
wasm_1in_1out!(
    /// Streaming EMA, callable bar-by-bar from JS.
    Ema,
    CoreEma
);
wasm_1in_1out!(
    /// Streaming SMA, callable bar-by-bar from JS.
    Sma,
    CoreSma
);
wasm_1in_1out!(
    /// Streaming WMA, callable bar-by-bar from JS.
    Wma,
    CoreWma
);
wasm_1in_1out!(
    /// Streaming Ehlers SuperSmoother filter, callable bar-by-bar from JS.
    SuperSmoother,
    CoreSuperSmoother
);

wasm_hlc_1out!(
    /// Streaming TA-Lib Wilder ATR, callable bar-by-bar from JS.
    Atr,
    CoreAtr
);
wasm_hlc_1out!(
    /// Streaming ADX, callable bar-by-bar from JS.
    Adx,
    CoreAdx
);
wasm_hlc_1out!(
    /// Streaming ADXR (TA-Lib `period - 1` lag), callable bar-by-bar from JS.
    Adxr,
    CoreAdxr
);

/// Streaming MACD, callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct Macd {
    inner: CoreMacd,
}

#[wasm_bindgen]
impl Macd {
    #[wasm_bindgen(constructor)]
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Macd {
        Macd {
            inner: CoreMacd::new(fast_period, slow_period, signal_period),
        }
    }

    /// Feed one price; returns `[macd, signal, histogram]`.
    pub fn next(&mut self, value: f64) -> Vec<f64> {
        let (macd, signal, hist) = self.inner.next(value);
        vec![macd, signal, hist]
    }
}

/// Streaming Bollinger Bands (SMA basis), callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct Bbands {
    inner: CoreBbands,
}

#[wasm_bindgen]
impl Bbands {
    #[wasm_bindgen(constructor)]
    pub fn new(period: usize, nb_dev_up: f64, nb_dev_dn: f64) -> Bbands {
        Bbands {
            inner: CoreBbands::new(period, nb_dev_up, nb_dev_dn, MaType::Sma),
        }
    }

    /// Feed one price; returns `[upper, middle, lower]`.
    pub fn next(&mut self, value: f64) -> Vec<f64> {
        let (upper, middle, lower) = self.inner.next(value);
        vec![upper, middle, lower]
    }
}

/// Streaming Slow Stochastic (SMA smoothing), callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct Stoch {
    inner: CoreStoch,
}

#[wasm_bindgen]
impl Stoch {
    #[wasm_bindgen(constructor)]
    pub fn new(fastk_period: usize, slowk_period: usize, slowd_period: usize) -> Stoch {
        Stoch {
            inner: CoreStoch::new(
                fastk_period,
                slowk_period,
                MaType::Sma,
                slowd_period,
                MaType::Sma,
            ),
        }
    }

    /// Feed one `(high, low, close)` bar; returns `[slow_k, slow_d]`.
    pub fn next(&mut self, high: f64, low: f64, close: f64) -> Vec<f64> {
        let (k, d) = self.inner.next((high, low, close));
        vec![k, d]
    }
}

/// Streaming Ichimoku Cloud, callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct Ichimoku {
    inner: CoreIchimoku,
}

#[wasm_bindgen]
impl Ichimoku {
    #[wasm_bindgen(constructor)]
    pub fn new(tenkan_period: usize, kijun_period: usize, senkou_b_period: usize) -> Ichimoku {
        Ichimoku {
            inner: CoreIchimoku::new(tenkan_period, kijun_period, senkou_b_period),
        }
    }

    /// Feed one `(high, low)` bar; returns `[tenkan, kijun, senkou_a, senkou_b]`.
    pub fn next(&mut self, high: f64, low: f64) -> Vec<f64> {
        let (tenkan, kijun, senkou_a, senkou_b) = self.inner.next((high, low));
        vec![tenkan, kijun, senkou_a, senkou_b]
    }
}

/// Streaming Ehlers Roofing Filter, callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct RoofingFilter {
    inner: CoreRoofingFilter,
}

#[wasm_bindgen]
impl RoofingFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(hp_period: usize, ss_period: usize) -> RoofingFilter {
        RoofingFilter {
            inner: CoreRoofingFilter::new(hp_period, ss_period),
        }
    }

    /// Feed one price; returns the filtered value.
    pub fn next(&mut self, value: f64) -> f64 {
        self.inner.next(value)
    }
}

/// Streaming 1D Kalman Filter, callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct KalmanFilter {
    inner: CoreKalman,
}

#[wasm_bindgen]
impl KalmanFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(q: f64, r: f64) -> KalmanFilter {
        KalmanFilter {
            inner: CoreKalman::new(q, r),
        }
    }

    /// Feed one price; returns the filtered estimate.
    pub fn next(&mut self, value: f64) -> f64 {
        self.inner.next(value)
    }
}

/// Streaming TTM Squeeze, callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct TtmSqueeze {
    inner: CoreTtmSqueeze,
}

#[wasm_bindgen]
impl TtmSqueeze {
    #[wasm_bindgen(constructor)]
    pub fn new(period: usize, multiplier_bb: f64, multiplier_kc: f64) -> TtmSqueeze {
        TtmSqueeze {
            inner: CoreTtmSqueeze::new(period, multiplier_bb, multiplier_kc),
        }
    }

    /// Feed one `(high, low, close)` bar; returns `[histogram, is_squeezed]`
    /// where `is_squeezed` is `1.0` when squeezed, else `0.0`.
    pub fn next(&mut self, high: f64, low: f64, close: f64) -> Vec<f64> {
        let (hist, sq) = self.inner.next((high, low, close));
        vec![hist, if sq { 1.0 } else { 0.0 }]
    }
}

/// Streaming Market Structure (Swings + BOS), callable bar-by-bar from JS.
#[wasm_bindgen]
pub struct MarketStructure {
    inner: CoreMarketStructure,
}

#[wasm_bindgen]
impl MarketStructure {
    #[wasm_bindgen(constructor)]
    pub fn new(swing_strength: usize) -> MarketStructure {
        MarketStructure {
            inner: CoreMarketStructure::new(swing_strength),
        }
    }

    /// Feed one `(high, low)` bar; returns the 12-field flattened state vector:
    /// `[bias, last_high_price, last_high_bar, last_low_price, last_low_bar,
    ///   has_flip, flip_bearish, flip_price, flip_bar, flip_strength,
    ///   swing_depth, bar_index]`.
    pub fn next(&mut self, high: f64, low: f64) -> Vec<f64> {
        let st = self.inner.next((high, low));
        vec![
            f64::from(st.bias_u32()),
            st.last_high_price(),
            st.last_high_bar() as f64,
            st.last_low_price(),
            st.last_low_bar() as f64,
            if st.has_flip() { 1.0 } else { 0.0 },
            if st.flip_bearish() { 1.0 } else { 0.0 },
            st.flip_price(),
            st.flip_bar() as f64,
            f64::from(st.flip_strength()),
            st.swing_depth() as f64,
            st.bar_index as f64,
        ]
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

/// Name of a harmonic pattern kind code returned by [`HarmonicScanner::next`]
/// (the `HarmonicKind::as_str` value). Any code that is not a known kind index,
/// including NaN, negative or fractional values, returns `"unknown"`.
#[wasm_bindgen(js_name = harmonicKindName)]
pub fn harmonic_kind_name(code: f64) -> String {
    let known = code.is_finite() && code >= 0.0 && code.fract() == 0.0;
    let kind = if known {
        HARMONIC_KINDS.get(code as usize)
    } else {
        None
    };
    kind.map_or("unknown", |k| k.as_str()).to_string()
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

    /// The probe's LCG random walk (multiplicative steps, high/low band).
    pub(crate) fn random_walk(seed: u64, n: usize) -> Vec<(f64, f64)> {
        let mut state = seed * 7919;
        let mut next = || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((state >> 11) as f64) / ((1u64 << 53) as f64)
        };
        let mut p = 100.0;
        (0..n)
            .map(|_| {
                p *= 1.0 + (next() - 0.5) * 0.03;
                let w = p * 0.004 * (1.0 + next());
                (p + w, p - w)
            })
            .collect()
    }

    /// (seed, swing_strength) series that together produce every harmonic
    /// kind in both directions.
    const HARMONIC_SERIES: [(u64, usize); 3] = [(6, 1), (4, 3), (5, 2)];

    /// Expected 15-field encoding built straight from the core struct, with the
    /// kind taken by *name* (`HarmonicKind::as_str`), independent of `encode_pattern`.
    fn expected_fields(p: &HarmonicPattern) -> (&'static str, [f64; 14]) {
        (
            p.kind.as_str(),
            [
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
            ],
        )
    }

    #[test]
    fn harmonic_shim_all_fields_and_kinds_match_core() {
        use std::collections::BTreeSet;
        let mut seen = BTreeSet::new();
        for (seed, ss) in HARMONIC_SERIES {
            let bars = random_walk(seed, 3000);
            let mut shim = HarmonicScanner::new(ss);
            let mut core = CoreHarmonic::new(ss);
            for &(h, l) in &bars {
                let want = core.next((h, l));
                let got = shim.next(h, l);
                assert_eq!(got.len(), want.len() * HARMONIC_STRIDE);
                for (k, p) in want.iter().enumerate() {
                    let g = &got[k * HARMONIC_STRIDE..(k + 1) * HARMONIC_STRIDE];
                    let (kind, fields) = expected_fields(p);
                    assert_eq!(harmonic_kind_name(g[0]), kind, "seed {seed} ss {ss}");
                    for (j, (gv, wv)) in g[1..].iter().zip(fields.iter()).enumerate() {
                        assert_eq!(gv.to_bits(), wv.to_bits(), "{kind} field {}", j + 1);
                    }
                    if matches!(p.kind, HarmonicKind::AbCd | HarmonicKind::AlternateAbCd) {
                        assert!(
                            g[2].is_nan() && g[3].is_nan(),
                            "{kind} must have no X point"
                        );
                    }
                    seen.insert((kind, p.is_bull));
                }
            }
        }
        let kinds = [
            "abcd",
            "alternate_abcd",
            "5-0",
            "gartley",
            "bat",
            "butterfly",
            "crab",
            "alternate_bat",
        ];
        for k in kinds {
            for bull in [true, false] {
                assert!(
                    seen.contains(&(k, bull)),
                    "missing fixture for {k} bull={bull}"
                );
            }
        }
    }

    #[test]
    fn harmonic_bar_indices_are_zero_based() {
        let bars = random_walk(6, 3000);
        let mut shim = HarmonicScanner::new(1);
        let mut checked = 0;
        for &(h, l) in &bars {
            let got = shim.next(h, l);
            for p in got.chunks(HARMONIC_STRIDE) {
                // Every pivot price is the high or low of bars[bar], 0-based.
                for j in [4, 6, 8, 10] {
                    let (bar, price) = (p[j] as usize, p[j + 1]);
                    assert!(bars[bar].0 == price || bars[bar].1 == price);
                }
                checked += 1;
            }
        }
        assert!(checked > 0);
    }

    #[test]
    fn harmonic_no_match_series_is_empty() {
        let mut shim = HarmonicScanner::new(2);
        for i in 0..500 {
            let p = 100.0 + 0.1 * i as f64; // monotonic: no swings, no patterns
            assert!(shim.next(p + 0.5, p - 0.5).is_empty());
        }
    }

    #[test]
    fn harmonic_kind_name_rejects_non_codes() {
        assert_eq!(harmonic_kind_name(0.0), "abcd");
        assert_eq!(harmonic_kind_name(3.0), "gartley");
        assert_eq!(harmonic_kind_name(7.0), "alternate_bat");
        for bad in [f64::NAN, -1.0, 1.5, 8.0, f64::INFINITY] {
            assert_eq!(harmonic_kind_name(bad), "unknown", "{bad}");
        }
    }
}
