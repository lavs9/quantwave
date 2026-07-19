//! Harmonic pattern detection (AB=CD, Alternate AB=CD, 5-0).
//!
//! Harmonic patterns are Fibonacci-ratio-constrained price structures. This
//! module implements three of them, built on the shared swing foundation
//! ([`MarketStructure`], Part 21) exactly like [`GeometricPatternScanner`]:
//! confirmed swing pivots are collected into an alternating sequence, and the
//! most recent 4 (AB=CD) or 5 (5-0) pivots are tested against the pattern's
//! ratio gates. A pattern is emitted only once its completion pivot `D` is a
//! *confirmed* swing — which lags the pivot by `swing_strength` bars — so
//! detection never uses information from beyond `D` (anti-lookahead).
//!
//! # Attribution
//!
//! Harmonic Trading is the work of **Scott M. Carney** (HarmonicTrader.com).
//! Carney named and defined these patterns; "Harmonic Trading" and several
//! pattern names are his trademarks. The ratio definitions here follow his
//! primary sources and are implemented for interoperability with attribution —
//! no source text is reproduced:
//!
//! - AB=CD reciprocal ratio table (C retracement {0.382, 0.50, 0.618, 0.707,
//!   0.786, 0.886} ↔ BC projection {2.618/2.24, 2.0, 1.618, 1.41, 1.27, 1.13}):
//!   Carney, *Harmonic Trading: Volume One* (2010), Ch. 4 "The AB=CD Pattern".
//! - Alternate AB=CD (CD = 1.27 or 1.618 × AB): *Volume One* Ch. 4 /
//!   harmonictrader.com "Alternate ABCD Pattern".
//! - 5-0 pattern (B = 1.13–1.618 of XA; C = 1.618–2.24 of AB; D = 50% of BC and
//!   the reciprocal AB=CD): Carney, *Harmonic Trading: Volume Two* (2010),
//!   Ch. 3 "New Harmonic Patterns", pp. 78–79 / harmonictrader.com "5-0".
//!
//! Signal output is a rich [`HarmonicPattern`] struct (pivots, measured ratios,
//! fit score, PRZ), kept separate from any visualization per the project's PA
//! design philosophy.

use crate::indicators::market_structure::{MarketStructure, SwingPoint};
use crate::traits::Next;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Which harmonic pattern was detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmonicKind {
    /// AB=CD: CD leg equals AB in length.
    AbCd,
    /// Alternate AB=CD: CD = 1.27 × AB or 1.618 × AB.
    AlternateAbCd,
    /// 5-0: five-point structure completing at a 50% BC retrace / reciprocal AB=CD.
    FiveZero,
}

impl HarmonicKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            HarmonicKind::AbCd => "abcd",
            HarmonicKind::AlternateAbCd => "alternate_abcd",
            HarmonicKind::FiveZero => "5-0",
        }
    }

    fn discriminant(&self) -> u8 {
        match self {
            HarmonicKind::AbCd => 0,
            HarmonicKind::AlternateAbCd => 1,
            HarmonicKind::FiveZero => 2,
        }
    }
}

/// A detected harmonic pattern. Points are labelled X, A, B, C, D; `x_*` is
/// populated only for the 5-0 (four-point AB=CD patterns have no X). `is_bull`
/// is true when the pattern completes at a low (a buy setup).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarmonicPattern {
    pub id: u32,
    pub kind: HarmonicKind,
    pub is_bull: bool,
    pub x_bar: Option<usize>,
    pub x_price: Option<f64>,
    pub a_bar: usize,
    pub a_price: f64,
    pub b_bar: usize,
    pub b_price: f64,
    pub c_bar: usize,
    pub c_price: f64,
    pub d_bar: usize,
    pub d_price: f64,
    /// Ratio-fit quality in [0, 1]; 1.0 is a textbook-exact pattern.
    pub score: f64,
    /// 5-0 only: |AB| / |XA| (the B-point extension of XA).
    pub xa_ext: Option<f64>,
    /// |BC| / |AB| — a retracement (< 1) for AB=CD, an extension (> 1) for 5-0.
    pub bc_ab: f64,
    /// |CD| / |AB| — 1.0 for AB=CD, 1.27/1.618 for Alternate, ~1.0 for 5-0.
    pub cd_ab: f64,
    /// |CD| / |BC| — ~0.5 for the 5-0 completion.
    pub cd_bc: f64,
    /// Potential Reversal Zone (price band where the pattern projects to complete).
    pub prz_low: f64,
    pub prz_high: f64,
    /// Vertical extent of the pattern (max − min pivot price) in ATR units.
    pub size_atr: f64,
}

/// Tunable detection thresholds.
#[derive(Debug, Clone)]
pub struct HarmonicConfig {
    /// Relative tolerance on equality ratios and range-gate edges (e.g. 0.10 = ±10%).
    pub ratio_tolerance: f64,
    /// Minimum fit score to emit a pattern.
    pub min_score: f64,
    /// Minimum pattern extent in ATR units (0.0 disables the filter).
    pub min_size_atr: f64,
    pub atr_period: usize,
    pub detect_abcd: bool,
    pub detect_alternate_abcd: bool,
    pub detect_5_0: bool,
}

impl Default for HarmonicConfig {
    fn default() -> Self {
        Self {
            ratio_tolerance: 0.10,
            min_score: 0.5,
            min_size_atr: 0.0,
            atr_period: 14,
            detect_abcd: true,
            detect_alternate_abcd: true,
            detect_5_0: true,
        }
    }
}

/// 1.0 when `measured == ideal`, decreasing linearly to 0 at `±tol` relative.
fn ratio_fit(measured: f64, ideal: f64, tol: f64) -> f64 {
    if ideal == 0.0 || tol <= 0.0 {
        return 0.0;
    }
    let err = (measured / ideal - 1.0).abs();
    (1.0 - err / tol).clamp(0.0, 1.0)
}

/// Whether `x` lies within `[lo, hi]` after expanding the edges by `tol` relative.
fn in_band(x: f64, lo: f64, hi: f64, tol: f64) -> bool {
    x >= lo * (1.0 - tol) && x <= hi * (1.0 + tol)
}

fn leg(a: f64, b: f64) -> f64 {
    (a - b).abs()
}

/// Streaming harmonic pattern scanner. Feed `(high, low)` per bar; each call
/// returns the patterns (usually none) whose completion pivot confirmed on that
/// bar.
#[derive(Debug, Clone)]
pub struct HarmonicPatternScanner {
    ms: MarketStructure,
    config: HarmonicConfig,
    bar_index: usize,
    prev_close: f64,
    have_prev_close: bool,
    atr: f64,
    swings: Vec<SwingPoint>,
    /// Bar of the last high/low pivot already ingested, so each confirmed pivot
    /// is appended exactly once (not re-read every bar while it stays current).
    last_high_bar: Option<usize>,
    last_low_bar: Option<usize>,
    seen: HashSet<(u8, usize, usize, usize, usize)>,
    next_id: u32,
}

impl HarmonicPatternScanner {
    pub fn new(swing_strength: usize) -> Self {
        Self::with_config(swing_strength, HarmonicConfig::default())
    }

    pub fn with_config(swing_strength: usize, config: HarmonicConfig) -> Self {
        Self {
            ms: MarketStructure::new(swing_strength),
            config,
            bar_index: 0,
            prev_close: 0.0,
            have_prev_close: false,
            atr: 1.0,
            swings: Vec::with_capacity(64),
            last_high_bar: None,
            last_low_bar: None,
            seen: HashSet::new(),
            next_id: 1,
        }
    }

    fn update_atr(&mut self, high: f64, low: f64) {
        let prev_close = if self.have_prev_close {
            self.prev_close
        } else {
            (high + low) / 2.0
        };
        let tr = (high - low)
            .max((high - prev_close).abs())
            .max((low - prev_close).abs());
        let p = self.config.atr_period.max(1);
        if self.bar_index <= p {
            // Seed with a simple mean until the window fills.
            let n = self.bar_index as f64;
            self.atr = (self.atr * (n - 1.0) + tr) / n.max(1.0);
        } else {
            let alpha = 1.0 / p as f64;
            self.atr = self.atr * (1.0 - alpha) + tr * alpha;
        }
        self.atr = self.atr.max(1e-8);
    }

    /// Append a confirmed swing, keeping the sequence strictly alternating (a
    /// same-direction pivot replaces the previous one when it is more extreme).
    fn ingest_swing(&mut self, sp: &SwingPoint) {
        if let Some(last) = self.swings.last() {
            if last.bar == sp.bar && last.is_high == sp.is_high {
                return;
            }
            if last.is_high == sp.is_high {
                let more_extreme = (sp.is_high && sp.price >= last.price)
                    || (!sp.is_high && sp.price <= last.price);
                if more_extreme {
                    self.swings.pop();
                } else {
                    return;
                }
            }
        }
        self.swings.push(sp.clone());
        if self.swings.len() > 64 {
            self.swings.drain(0..16);
        }
    }

    fn size_atr(&self, prices: &[f64]) -> f64 {
        let hi = prices.iter().cloned().fold(f64::MIN, f64::max);
        let lo = prices.iter().cloned().fold(f64::MAX, f64::min);
        (hi - lo) / self.atr
    }

    fn emit(&mut self, mut p: HarmonicPattern) -> Option<HarmonicPattern> {
        if p.score < self.config.min_score || p.size_atr < self.config.min_size_atr {
            return None;
        }
        let key = (
            p.kind.discriminant(),
            p.x_bar.unwrap_or(usize::MAX),
            p.a_bar,
            p.c_bar,
            p.d_bar,
        );
        if !self.seen.insert(key) {
            return None;
        }
        p.id = self.next_id;
        self.next_id += 1;
        Some(p)
    }

    /// Test the last four swings (A, B, C, D) as a (possibly Alternate) AB=CD.
    fn classify_abcd(
        &self,
        a: &SwingPoint,
        b: &SwingPoint,
        c: &SwingPoint,
        d: &SwingPoint,
    ) -> Option<HarmonicPattern> {
        // Pivots must be strictly time-ordered (MarketStructure can occasionally
        // report a high and a low on the same bar for degenerate windows).
        if !(a.bar < b.bar && b.bar < c.bar && c.bar < d.bar) {
            return None;
        }
        let ab = leg(a.price, b.price);
        let bc = leg(b.price, c.price);
        let cd = leg(c.price, d.price);
        if ab <= 0.0 || bc <= 0.0 || cd <= 0.0 {
            return None;
        }
        let bc_ab = bc / ab;
        let cd_ab = cd / ab;
        let tol = self.config.ratio_tolerance;
        // C must be a retracement of AB (0.382–0.886 per the reciprocal table).
        if !in_band(bc_ab, 0.382, 0.886, tol) {
            return None;
        }
        let is_bull = !d.is_high;

        let (kind, target) = if self.config.detect_abcd && ratio_fit(cd_ab, 1.0, tol) > 0.0 {
            (HarmonicKind::AbCd, 1.0)
        } else if self.config.detect_alternate_abcd {
            let nearest = if (cd_ab - 1.27).abs() <= (cd_ab - 1.618).abs() {
                1.27
            } else {
                1.618
            };
            if ratio_fit(cd_ab, nearest, tol) > 0.0 {
                (HarmonicKind::AlternateAbCd, nearest)
            } else {
                return None;
            }
        } else {
            return None;
        };

        let score = ratio_fit(cd_ab, target, tol);
        // PRZ: the AB=CD completion price (target × AB projected from C), banded.
        let sign = if is_bull { -1.0 } else { 1.0 };
        let d_proj = c.price + sign * target * ab;
        let band = tol * ab;
        Some(HarmonicPattern {
            id: 0,
            kind,
            is_bull,
            x_bar: None,
            x_price: None,
            a_bar: a.bar,
            a_price: a.price,
            b_bar: b.bar,
            b_price: b.price,
            c_bar: c.bar,
            c_price: c.price,
            d_bar: d.bar,
            d_price: d.price,
            score,
            xa_ext: None,
            bc_ab,
            cd_ab,
            cd_bc: cd / bc,
            prz_low: d_proj - band,
            prz_high: d_proj + band,
            size_atr: self.size_atr(&[a.price, b.price, c.price, d.price]),
        })
    }

    /// Test the last five swings (X, A, B, C, D) as a 5-0.
    fn classify_5_0(
        &self,
        x: &SwingPoint,
        a: &SwingPoint,
        b: &SwingPoint,
        c: &SwingPoint,
        d: &SwingPoint,
    ) -> Option<HarmonicPattern> {
        if !self.config.detect_5_0 {
            return None;
        }
        if !(x.bar < a.bar && a.bar < b.bar && b.bar < c.bar && c.bar < d.bar) {
            return None;
        }
        let xa = leg(x.price, a.price);
        let ab = leg(a.price, b.price);
        let bc = leg(b.price, c.price);
        let cd = leg(c.price, d.price);
        if xa <= 0.0 || ab <= 0.0 || bc <= 0.0 || cd <= 0.0 {
            return None;
        }
        let xa_ext = ab / xa;
        let bc_ab = bc / ab;
        let cd_bc = cd / bc;
        let cd_ab = cd / ab;
        let tol = self.config.ratio_tolerance;
        // B = 1.13–1.618 of XA; C = 1.618–2.24 of AB; D = 50% of BC.
        if !in_band(xa_ext, 1.13, 1.618, tol)
            || !in_band(bc_ab, 1.618, 2.24, tol)
            || ratio_fit(cd_bc, 0.5, tol) == 0.0
        {
            return None;
        }
        let is_bull = !d.is_high;
        // D is defined by both the 50% BC retrace and the reciprocal AB=CD; score
        // rewards proximity to both (the reciprocal on a looser band, as it only
        // "complements" the PRZ per Vol. 2).
        let score = 0.5 * ratio_fit(cd_bc, 0.5, tol) + 0.5 * ratio_fit(cd_ab, 1.0, tol * 1.5);
        // PRZ spans the two projections from C: 50% of BC and the reciprocal AB=CD.
        let sign = if is_bull { -1.0 } else { 1.0 };
        let lvl_50 = c.price + sign * 0.5 * bc;
        let lvl_abcd = c.price + sign * ab;
        Some(HarmonicPattern {
            id: 0,
            kind: HarmonicKind::FiveZero,
            is_bull,
            x_bar: Some(x.bar),
            x_price: Some(x.price),
            a_bar: a.bar,
            a_price: a.price,
            b_bar: b.bar,
            b_price: b.price,
            c_bar: c.bar,
            c_price: c.price,
            d_bar: d.bar,
            d_price: d.price,
            score,
            xa_ext: Some(xa_ext),
            bc_ab,
            cd_ab,
            cd_bc,
            prz_low: lvl_50.min(lvl_abcd),
            prz_high: lvl_50.max(lvl_abcd),
            size_atr: self.size_atr(&[x.price, a.price, b.price, c.price, d.price]),
        })
    }

    fn detect(&mut self) -> Vec<HarmonicPattern> {
        let mut out = Vec::new();
        let n = self.swings.len();
        if n >= 5 {
            let (x, a, b, c, d) = (
                self.swings[n - 5].clone(),
                self.swings[n - 4].clone(),
                self.swings[n - 3].clone(),
                self.swings[n - 2].clone(),
                self.swings[n - 1].clone(),
            );
            if let Some(p) = self.classify_5_0(&x, &a, &b, &c, &d)
                && let Some(p) = self.emit(p)
            {
                out.push(p);
            }
        }
        if n >= 4 {
            let (a, b, c, d) = (
                self.swings[n - 4].clone(),
                self.swings[n - 3].clone(),
                self.swings[n - 2].clone(),
                self.swings[n - 1].clone(),
            );
            if let Some(p) = self.classify_abcd(&a, &b, &c, &d)
                && let Some(p) = self.emit(p)
            {
                out.push(p);
            }
        }
        out
    }
}

impl Next<(f64, f64)> for HarmonicPatternScanner {
    type Output = Vec<HarmonicPattern>;

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        self.bar_index += 1;
        self.update_atr(high, low);
        self.prev_close = (high + low) / 2.0;
        self.have_prev_close = true;

        let state = self.ms.next((high, low));
        // Ingest a pivot only when newly confirmed (its bar changed since last
        // seen), so each pivot enters the sequence once, in confirmation order.
        if let Some(sh) = state.last_swing_high.clone()
            && self.last_high_bar != Some(sh.bar)
        {
            self.last_high_bar = Some(sh.bar);
            self.ingest_swing(&sh);
        }
        if let Some(sl) = state.last_swing_low.clone()
            && self.last_low_bar != Some(sl.bar)
        {
            self.last_low_bar = Some(sl.bar);
            self.ingest_swing(&sl);
        }
        self.detect()
    }
}

/// Batch harmonic detection: fold [`HarmonicPatternScanner`] over `(high, low)`
/// bars, returning every detected pattern in completion order.
pub fn harmonic_patterns_batch(
    bars: &[(f64, f64)],
    swing_strength: usize,
    config: HarmonicConfig,
) -> Vec<HarmonicPattern> {
    let mut s = HarmonicPatternScanner::with_config(swing_strength, config);
    let mut out = Vec::new();
    for &(h, l) in bars {
        out.extend(s.next((h, l)));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const STRENGTH: usize = 2;
    const SPACING: usize = 6;

    /// Build an OHLC `(high, low)` series whose confirmed swings land exactly on
    /// the given pivots. Prices ramp piecewise-linearly between pivots so each is
    /// a strict local extreme detectable by `MarketStructure` at `STRENGTH`.
    fn series_from_pivots(pivots: &[(f64, bool)]) -> Vec<(f64, f64)> {
        let eps = 0.01;
        let start = STRENGTH + 1;
        let n = start + (pivots.len() - 1) * SPACING + STRENGTH + 2;
        let bars: Vec<usize> = (0..pivots.len()).map(|i| start + i * SPACING).collect();
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            // Interpolate the "mid path" through the pivot prices at this bar.
            let mid = if i <= bars[0] {
                pivots[0].0
            } else if i >= *bars.last().unwrap() {
                pivots.last().unwrap().0
            } else {
                let k = bars.iter().position(|&b| b > i).unwrap();
                let (b0, b1) = (bars[k - 1], bars[k]);
                let (v0, v1) = (pivots[k - 1].0, pivots[k].0);
                v0 + (v1 - v0) * (i - b0) as f64 / (b1 - b0) as f64
            };
            out.push((mid + eps, mid - eps));
        }
        for (idx, &(price, is_high)) in pivots.iter().enumerate() {
            let bar = bars[idx];
            if is_high {
                out[bar] = (price, price - 2.0 * eps);
            } else {
                out[bar] = (price + 2.0 * eps, price);
            }
        }
        out
    }

    fn run(pivots: &[(f64, bool)]) -> Vec<HarmonicPattern> {
        harmonic_patterns_batch(
            &series_from_pivots(pivots),
            STRENGTH,
            HarmonicConfig::default(),
        )
    }

    #[test]
    fn gold_bullish_abcd_exact() {
        // Bullish AB=CD: A high, B low, C high (0.618 retrace of AB), D low with
        // CD = AB. A=110, B=100 (AB=10), C=106.18 (0.618 up), D=96.18 (CD=10).
        let pats = run(&[
            (110.0, true),
            (100.0, false),
            (106.18, true),
            (96.18, false),
        ]);
        let abcd: Vec<_> = pats
            .iter()
            .filter(|p| p.kind == HarmonicKind::AbCd)
            .collect();
        assert_eq!(abcd.len(), 1, "exactly one AB=CD");
        let p = abcd[0];
        assert!(p.is_bull);
        assert!(p.score > 0.95, "near-exact score, got {}", p.score);
        assert!((p.cd_ab - 1.0).abs() < 0.02);
        assert!((p.bc_ab - 0.618).abs() < 0.02);
        assert_eq!(p.x_bar, None);
    }

    #[test]
    fn gold_alternate_abcd_1_27() {
        // Same A,B,C but CD = 1.27 × AB → D = 106.18 − 12.7 = 93.48.
        let pats = run(&[
            (110.0, true),
            (100.0, false),
            (106.18, true),
            (93.48, false),
        ]);
        let alt: Vec<_> = pats
            .iter()
            .filter(|p| p.kind == HarmonicKind::AlternateAbCd)
            .collect();
        assert_eq!(alt.len(), 1, "one Alternate AB=CD");
        assert!((alt[0].cd_ab - 1.27).abs() < 0.03);
        assert!(alt[0].score > 0.9);
    }

    #[test]
    fn gold_bullish_5_0_exact() {
        // 5-0: X low, A high, B low (AB = 1.618×XA), C high (BC = 2.0×AB), D low
        // (CD = 0.5×BC = AB, both projections agree → score ~1).
        // X=100, A=110 (XA=10), B=93.82 (AB=16.18=1.618×10), C=126.18 (BC=32.36
        // =2.0×AB), D=110.0 (CD=16.18=0.5×BC=AB).
        let pats = run(&[
            (100.0, false),
            (110.0, true),
            (93.82, false),
            (126.18, true),
            (110.0, false),
        ]);
        let five: Vec<_> = pats
            .iter()
            .filter(|p| p.kind == HarmonicKind::FiveZero)
            .collect();
        assert_eq!(five.len(), 1, "one 5-0");
        let p = five[0];
        assert!(p.is_bull);
        assert!(p.x_bar.is_some());
        assert!((p.cd_bc - 0.5).abs() < 0.03, "cd_bc {}", p.cd_bc);
        assert!(p.score > 0.9, "score {}", p.score);
    }

    #[test]
    fn perturbed_abcd_rejected() {
        // CD = 1.5 × AB is neither 1.0 (AB=CD) nor 1.27/1.618 (Alternate) within
        // 10% → no pattern. C=106.18, D = 106.18 − 15 = 91.18.
        let pats = run(&[
            (110.0, true),
            (100.0, false),
            (106.18, true),
            (91.18, false),
        ]);
        assert!(
            pats.iter().all(|p| p.kind == HarmonicKind::FiveZero),
            "no AB=CD family should match cd_ab=1.5"
        );
    }

    #[test]
    fn bad_retrace_rejected() {
        // C retraces AB by only 0.2 (< 0.382 gate) → not an AB=CD candidate.
        // A=110, B=100, C=102 (0.2), D=92 (CD=10).
        let pats = run(&[(110.0, true), (100.0, false), (102.0, true), (92.0, false)]);
        assert!(pats.iter().all(|p| p.kind != HarmonicKind::AbCd));
    }

    #[test]
    fn detection_is_anti_lookahead() {
        // The reported completion bar D must exist before detection: emission bar
        // (index in the folded stream) is >= d_bar for every pattern.
        let series = series_from_pivots(&[
            (110.0, true),
            (100.0, false),
            (106.18, true),
            (96.18, false),
        ]);
        let mut s = HarmonicPatternScanner::new(STRENGTH);
        for (i, &(h, l)) in series.iter().enumerate() {
            for p in s.next((h, l)) {
                assert!(
                    i >= p.d_bar,
                    "pattern emitted at bar {} before its D at {}",
                    i,
                    p.d_bar
                );
            }
        }
    }

    #[test]
    fn bearish_abcd_detected() {
        // Mirror of the bullish case: A low, B high, C low, D high.
        let pats = run(&[(90.0, false), (100.0, true), (93.82, false), (103.82, true)]);
        let abcd: Vec<_> = pats
            .iter()
            .filter(|p| p.kind == HarmonicKind::AbCd)
            .collect();
        assert_eq!(abcd.len(), 1);
        assert!(!abcd[0].is_bull);
    }

    proptest! {
        // On arbitrary bars, every emitted pattern must satisfy the universal
        // invariants: anti-lookahead (detection bar >= D), ordered/labelled
        // pivots, a score above threshold, and ratios consistent with its kind.
        #[test]
        fn emitted_patterns_are_well_formed(
            hl in prop::collection::vec((0.0f64..1000.0, 0.0f64..1000.0), 20..400),
        ) {
            let bars: Vec<(f64, f64)> = hl
                .into_iter()
                .map(|(a, b)| (a.max(b), a.min(b)))
                .collect();
            let mut s = HarmonicPatternScanner::new(STRENGTH);
            let cfg = HarmonicConfig::default();
            for (i, &(h, l)) in bars.iter().enumerate() {
                for p in s.next((h, l)) {
                    prop_assert!(i >= p.d_bar, "lookahead: emitted at {} for D {}", i, p.d_bar);
                    prop_assert!(p.a_bar < p.b_bar && p.b_bar < p.c_bar && p.c_bar < p.d_bar);
                    prop_assert!(p.score >= cfg.min_score);
                    prop_assert!(p.prz_low <= p.prz_high);
                    match p.kind {
                        HarmonicKind::AbCd => {
                            prop_assert!(in_band(p.bc_ab, 0.382, 0.886, cfg.ratio_tolerance));
                            prop_assert!(ratio_fit(p.cd_ab, 1.0, cfg.ratio_tolerance) > 0.0);
                            prop_assert!(p.x_bar.is_none());
                        }
                        HarmonicKind::AlternateAbCd => {
                            prop_assert!(in_band(p.bc_ab, 0.382, 0.886, cfg.ratio_tolerance));
                            let f127 = ratio_fit(p.cd_ab, 1.27, cfg.ratio_tolerance);
                            let f1618 = ratio_fit(p.cd_ab, 1.618, cfg.ratio_tolerance);
                            prop_assert!(f127 > 0.0 || f1618 > 0.0);
                        }
                        HarmonicKind::FiveZero => {
                            prop_assert!(p.x_bar.is_some());
                            prop_assert!(in_band(p.bc_ab, 1.618, 2.24, cfg.ratio_tolerance));
                            prop_assert!(ratio_fit(p.cd_bc, 0.5, cfg.ratio_tolerance) > 0.0);
                        }
                    }
                }
            }
        }
    }
}
