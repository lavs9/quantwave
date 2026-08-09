//! Numerically stable O(1)-amortised rolling variance over a fixed window.
//!
//! # Why this exists (quantwave-qkft)
//!
//! The obvious sliding-window variance keeps `Σx` and `Σx²` and evaluates
//! `E[X²] - E[X]²`. That is O(1) per bar but catastrophically cancels when the
//! window mean is large relative to the spread: both terms are ~`mean²` while
//! the answer is ~`σ²`, so the relative error is roughly `eps * (mean/σ)²`.
//! On raw prices that is harmless (~1e-11), but on a slowly-varying series such
//! as log-price (`lnP` in 1..9 with per-bar moves of ~1e-4) the ratio is ~1e10
//! and the result drifts ~1e-6 relative away from a two-pass reference.
//!
//! # The algorithm
//!
//! Shifted-data variance with a deterministic periodic exact refresh:
//!
//! * We keep an `origin` K and accumulate `Σ(x-K)` and `Σ(x-K)²`. Because K sits
//!   inside the current window, `x-K` is on the order of the window *spread*,
//!   not the window *level*, so the `Σd² - (Σd)²/n` subtraction cancels
//!   quantities of order `n·σ²` rather than `n·mean²`. The mean-magnitude
//!   cancellation disappears entirely.
//! * Add and remove are the exact algebraic inverses of each other, so a single
//!   evict/insert pair introduces no algorithmic drift beyond one rounding.
//! * Rounding drift and origin staleness (a trending series eventually walks
//!   away from K) are bounded by recomputing `origin`, `Σd` and `Σd²` exactly
//!   from the window every `period` pushes. One O(period) pass per `period`
//!   bars is O(1) amortised, and the window has fully turned over since the
//!   previous refresh, so no rounding error ever survives a whole window.
//!
//! # Determinism / batch-streaming parity
//!
//! The refresh cadence is driven purely by a push counter, i.e. by bar index —
//! never by data values, timers, or window content. Batch and streaming both
//! drive this same accumulator through the same `Next` implementations, so the
//! refresh lands on the same bars in both paths and the outputs stay
//! bit-identical. There is no non-deterministic "recompute when error looks
//! large" heuristic here, deliberately.
//!
//! # Tradeoff
//!
//! Per-bar cost is O(1) amortised but not O(1) worst case: one bar in every
//! `period` does an O(period) refresh. A truly O(1)-worst-case removal-based
//! rolling variance cannot bound drift indefinitely, so bounded periodic
//! recomputation is the intentional price of correctness.

use crate::utils::RingBuffer;

/// Sliding-window accumulator for population mean/variance.
#[derive(Debug, Clone)]
pub struct RollingVariance {
    period: usize,
    buf: RingBuffer<f64>,
    /// Shift reference; always a value near the current window level.
    origin: f64,
    /// `Σ(x - origin)` over the window.
    sum_d: f64,
    /// `Σ(x - origin)²` over the window.
    sum_d2: f64,
    /// Pushes since the last exact refresh.
    since_refresh: usize,
}

impl RollingVariance {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            buf: RingBuffer::with_capacity(period.max(1)),
            origin: 0.0,
            sum_d: 0.0,
            sum_d2: 0.0,
            since_refresh: 0,
        }
    }

    /// Number of samples currently in the window.
    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Window contents, oldest first.
    pub fn values(&self) -> Vec<f64> {
        self.buf.iter().cloned().collect()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.buf.iter()
    }

    /// Push a sample, evicting the oldest once the window is full.
    ///
    /// Returns `true` once the window holds a full `period` samples.
    pub fn push(&mut self, v: f64) -> bool {
        if self.period == 0 {
            return false;
        }
        if self.buf.len() >= self.period
            && let Some(old) = self.buf.pop_front()
        {
            let d = old - self.origin;
            self.sum_d -= d;
            self.sum_d2 -= d * d;
        }
        self.buf.push_back(v);
        let d = v - self.origin;
        self.sum_d += d;
        self.sum_d2 += d * d;

        self.since_refresh += 1;
        // Deterministic, index-driven refresh:
        //  * during warmup (partial window) refresh every bar — that is at most
        //    `period` O(period) passes in total, once, at the start of the
        //    stream, and it keeps warmup output exactly two-pass. Without it
        //    the origin would still be its initial 0.0 and the warmup bars
        //    would carry the very cancellation this type exists to remove.
        //  * once full, refresh exactly once per full window turnover.
        if self.buf.len() < self.period || self.since_refresh >= self.period {
            self.refresh();
        }

        self.buf.len() >= self.period
    }

    /// Recompute `origin`/`sum_d`/`sum_d2` exactly from the window.
    ///
    /// `origin` becomes the window mean, which makes `sum_d ≈ 0` and leaves
    /// `sum_d2` as a pure sum of squared deviations — the two-pass form.
    fn refresh(&mut self) {
        self.since_refresh = 0;
        let n = self.buf.len();
        if n == 0 {
            self.origin = 0.0;
            self.sum_d = 0.0;
            self.sum_d2 = 0.0;
            return;
        }
        let mut total = 0.0;
        for &v in self.buf.iter() {
            total += v;
        }
        let k = total / n as f64;
        let mut sum_d = 0.0;
        let mut sum_d2 = 0.0;
        for &v in self.buf.iter() {
            let d = v - k;
            sum_d += d;
            sum_d2 += d * d;
        }
        self.origin = k;
        self.sum_d = sum_d;
        self.sum_d2 = sum_d2;
    }

    /// Sum of the window's raw values.
    ///
    /// Reconstructed from the shifted accumulator, so it carries the same
    /// (small) error as the accumulator itself rather than an independently
    /// drifting `Σx`.
    #[inline]
    pub fn sum(&self) -> f64 {
        self.sum_d + self.origin * self.buf.len() as f64
    }

    /// Window mean (population), or `NaN` for an empty window.
    #[inline]
    pub fn mean(&self) -> f64 {
        let n = self.buf.len();
        if n == 0 {
            return f64::NAN;
        }
        self.origin + self.sum_d / n as f64
    }

    /// Population variance over the current window, clamped at zero.
    #[inline]
    pub fn variance(&self) -> f64 {
        let n = self.buf.len();
        if n == 0 {
            return f64::NAN;
        }
        let n_f = n as f64;
        // Σ(x-µ)² = Σd² - (Σd)²/n, with d = x - origin and origin ≈ µ.
        ((self.sum_d2 - self.sum_d * self.sum_d / n_f) / n_f).max(0.0)
    }

    /// Population standard deviation over the current window.
    #[inline]
    pub fn stddev(&self) -> f64 {
        self.variance().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{log_price_series, reference_stddev};

    /// The historical one-pass `E[X²] - E[X]²` accumulator, kept here purely so
    /// the regression test can demonstrate it failing the gate the new
    /// accumulator passes.
    fn one_pass_stddev(w: &[f64]) -> f64 {
        let n = w.len() as f64;
        let sum: f64 = w.iter().sum();
        let sum_sq: f64 = w.iter().map(|&x| x * x).sum();
        let mean = sum / n;
        (sum_sq / n - mean * mean).max(0.0).sqrt()
    }

    #[test]
    fn stable_on_log_price_where_one_pass_fails() {
        let period = 20;
        let data = log_price_series(600);

        let mut acc = RollingVariance::new(period);
        let mut worst_new: f64 = 0.0;
        let mut worst_old: f64 = 0.0;

        for (i, &x) in data.iter().enumerate() {
            let full = acc.push(x);
            if !full {
                continue;
            }
            let window = &data[i + 1 - period..=i];
            let reference = reference_stddev(window);
            assert!(reference > 0.0);

            let rel_new = ((acc.stddev() - reference) / reference).abs();
            let rel_old = ((one_pass_stddev(window) - reference) / reference).abs();
            worst_new = worst_new.max(rel_new);
            worst_old = worst_old.max(rel_old);
        }

        println!("worst rel err: one-pass {worst_old:e}, stable {worst_new:e}");
        // The bug: the old accumulator blows through the strict 1e-7 gate.
        assert!(
            worst_old > 1e-7,
            "one-pass should fail the 1e-7 gate on log-price, got {worst_old:e}"
        );
        // The fix: the new accumulator is at reference precision.
        assert!(
            worst_new < 1e-12,
            "stable accumulator rel err {worst_new:e} (one-pass was {worst_old:e})"
        );
    }

    /// The accumulator must not accumulate drift over long streams: every
    /// value it emits stays at reference precision no matter how many bars
    /// have gone by.
    #[test]
    fn no_drift_over_long_stream() {
        let period = 30;
        let data = log_price_series(50_000);
        let mut acc = RollingVariance::new(period);
        let mut worst: f64 = 0.0;
        for (i, &x) in data.iter().enumerate() {
            if !acc.push(x) {
                continue;
            }
            if i % 97 != 0 {
                continue;
            }
            let reference = reference_stddev(&data[i + 1 - period..=i]);
            if reference > 0.0 {
                worst = worst.max(((acc.stddev() - reference) / reference).abs());
            }
        }
        assert!(worst < 1e-12, "drift {worst:e} after 50k bars");
    }

    /// Raw prices must not regress either.
    #[test]
    fn matches_reference_on_raw_prices() {
        let period = 14;
        let data: Vec<f64> = (0..500)
            .map(|i| 2980.0 + (i as f64 * 0.31).sin() * 40.0)
            .collect();
        let mut acc = RollingVariance::new(period);
        for (i, &x) in data.iter().enumerate() {
            if !acc.push(x) {
                continue;
            }
            let reference = reference_stddev(&data[i + 1 - period..=i]);
            approx::assert_relative_eq!(acc.stddev(), reference, max_relative = 1e-12);
        }
    }

    #[test]
    fn constant_window_is_exactly_zero() {
        let mut acc = RollingVariance::new(10);
        for _ in 0..50 {
            acc.push(1234.5678);
        }
        assert_eq!(acc.stddev(), 0.0);
        assert_eq!(acc.variance(), 0.0);
    }

    #[test]
    fn sum_and_mean_track_the_window() {
        let mut acc = RollingVariance::new(4);
        for x in [1.0, 2.0, 3.0, 4.0, 5.0] {
            acc.push(x);
        }
        // window is [2,3,4,5]
        approx::assert_relative_eq!(acc.sum(), 14.0, max_relative = 1e-12);
        approx::assert_relative_eq!(acc.mean(), 3.5, max_relative = 1e-12);
        assert_eq!(acc.len(), 4);
        assert_eq!(acc.values(), vec![2.0, 3.0, 4.0, 5.0]);
    }

    /// The refresh is index-driven, so two accumulators fed the same bars in
    /// the same order agree bit-for-bit — the property batch/streaming parity
    /// rests on.
    #[test]
    fn refresh_cadence_is_deterministic() {
        let data = log_price_series(1_000);
        let mut a = RollingVariance::new(20);
        let mut b = RollingVariance::new(20);
        for &x in &data {
            a.push(x);
            b.push(x);
            assert_eq!(a.stddev().to_bits(), b.stddev().to_bits());
        }
    }
}
