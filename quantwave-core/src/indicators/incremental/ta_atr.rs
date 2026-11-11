//! Native O(1) ATR — TA-Lib and classic Wilder smoothing on true range.

use crate::traits::Next;
use serde::{Deserialize, Serialize};

/// Seeding and warm-up convention for Average True Range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AtrSeed {
    /// TA-Lib convention (default): True Range is undefined on bar 0.
    /// Emits `NaN` for bars 0..period-1 (the first `period` bars).
    /// Seeds at bar `period` with the simple average of `TR[1..period]`.
    #[default]
    TaLib,
    /// Classic J. Welles Wilder Jr. (1978) specification:
    /// Bar 0 True Range is `High[0] - Low[0]`.
    /// Emits `NaN` for bars 0..period-2 (the first `period - 1` bars).
    /// Seeds at bar `period - 1` with the simple average of `TR[0..period-1]`.
    Wilder1978,
}

impl std::str::FromStr for AtrSeed {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "talib" | "standard" | "ta_lib" | "ta-lib" => Ok(AtrSeed::TaLib),
            "wilder1978" | "wilder_1978" | "wilder-1978" | "classic" | "wilder" | "book" => {
                Ok(AtrSeed::Wilder1978)
            }
            other => Err(format!(
                "unknown ATR seed policy: '{other}' (expected 'talib' or 'wilder1978'/'classic')"
            )),
        }
    }
}

/// Average True Range — matches `talib_rs::volatility::atr` by default, or Wilder 1978 specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct TaATR {
    pub timeperiod: usize,
    period_f: f64,
    prev_close: Option<f64>,
    bars_seen: usize,
    warmup_tr_count: usize,
    warmup_sum: f64,
    atr: f64,
    seed: AtrSeed,
}

impl TaATR {
    pub fn new(timeperiod: usize) -> Self {
        Self::with_seed(timeperiod, AtrSeed::TaLib)
    }

    pub fn with_seed(timeperiod: usize, seed: AtrSeed) -> Self {
        Self {
            timeperiod,
            period_f: timeperiod as f64,
            prev_close: None,
            bars_seen: 0,
            warmup_tr_count: 0,
            warmup_sum: 0.0,
            atr: 0.0,
            seed,
        }
    }

    pub fn classic(timeperiod: usize) -> Self {
        Self::with_seed(timeperiod, AtrSeed::Wilder1978)
    }

    #[inline]
    pub fn seed(&self) -> AtrSeed {
        self.seed
    }

    #[inline]
    fn true_range(&self, high: f64, low: f64, prev_close: f64) -> f64 {
        let hl = high - low;
        let hc = (high - prev_close).abs();
        let lc = (low - prev_close).abs();
        hl.max(hc).max(lc)
    }
}

impl Next<(f64, f64, f64)> for TaATR {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let period = self.timeperiod;
        if period < 1 {
            return f64::NAN;
        }
        if high.is_nan() || low.is_nan() || close.is_nan() {
            return f64::NAN;
        }

        match self.seed {
            AtrSeed::TaLib => {
                if self.bars_seen == 0 {
                    self.prev_close = Some(close);
                    self.bars_seen = 1;
                    return f64::NAN;
                }

                let pc = self.prev_close.unwrap_or(close);
                let tr = self.true_range(high, low, pc);
                self.prev_close = Some(close);
                self.bars_seen += 1;

                if self.warmup_tr_count < period {
                    self.warmup_tr_count += 1;
                    self.warmup_sum += tr;
                    if self.warmup_tr_count < period {
                        return f64::NAN;
                    }
                    self.atr = self.warmup_sum / self.period_f;
                    return self.atr;
                }

                self.atr = (self.atr * (self.period_f - 1.0) + tr) / self.period_f;
                self.atr
            }
            AtrSeed::Wilder1978 => {
                let tr = match self.prev_close {
                    Some(pc) => self.true_range(high, low, pc),
                    None => high - low,
                };
                self.prev_close = Some(close);
                self.bars_seen += 1;

                if self.warmup_tr_count < period {
                    self.warmup_tr_count += 1;
                    self.warmup_sum += tr;
                    if self.warmup_tr_count < period {
                        return f64::NAN;
                    }
                    self.atr = self.warmup_sum / self.period_f;
                    return self.atr;
                }

                self.atr = (self.atr * (self.period_f - 1.0) + tr) / self.period_f;
                self.atr
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_ta_atr_parity(
            h in prop::collection::vec(1.0..100.0, 1..100),
            l in prop::collection::vec(1.0..100.0, 1..100),
            c in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len());
            if len == 0 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            for i in 0..len {
                let v_h: f64 = h[i];
                let v_l: f64 = l[i];
                let v_c: f64 = c[i];
                high.push(v_h.max(v_l).max(v_c));
                low.push(v_h.min(v_l).min(v_c));
                close.push(v_c);
            }

            let period = 14;
            let mut ta_atr = TaATR::new(period);
            let streaming_results: Vec<f64> =
                (0..len).map(|i| ta_atr.next((high[i], low[i], close[i]))).collect();
            let batch_results = talib_rs::volatility::atr(&high, &low, &close, period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }

    #[test]
    fn test_wilder1978_classic_formula() {
        // Synthetic 20-bar dataset
        let high: Vec<f64> = vec![
            10.0, 11.0, 12.0, 11.5, 12.5, 13.0, 12.8, 13.5, 14.0, 13.8, 14.2, 14.5, 14.1, 14.8,
            15.0, 15.2, 15.1, 15.5, 15.8, 16.0,
        ];
        let low: Vec<f64> = vec![
            8.0, 9.0, 10.0, 9.5, 10.5, 11.0, 10.8, 11.5, 12.0, 11.8, 12.2, 12.5, 12.1, 12.8, 13.0,
            13.2, 13.1, 13.5, 13.8, 14.0,
        ];
        let close: Vec<f64> = vec![
            9.0, 10.5, 11.0, 10.0, 12.0, 11.5, 12.5, 13.0, 12.5, 13.5, 13.0, 14.0, 13.5, 14.5,
            14.0, 14.8, 14.2, 15.0, 14.5, 15.5,
        ];

        // Wilder 1978 hand calculation for period = 14:
        // TR[0] = H[0] - L[0] = 10.0 - 8.0 = 2.0
        let mut tr = [0.0; 20];
        tr[0] = high[0] - low[0];
        for i in 1..20 {
            let hl = high[i] - low[i];
            let hc = (high[i] - close[i - 1]).abs();
            let lc = (low[i] - close[i - 1]).abs();
            tr[i] = hl.max(hc).max(lc);
        }

        let mut expected_atr = [f64::NAN; 20];
        // Bars 0..12 are NaN.
        // Bar 13 (14th bar): mean of TR[0..14]
        let sum_14: f64 = tr[0..14].iter().sum();
        expected_atr[13] = sum_14 / 14.0;
        for i in 14..20 {
            expected_atr[i] = (expected_atr[i - 1] * 13.0 + tr[i]) / 14.0;
        }

        let mut atr = TaATR::classic(14);
        let mut actual = Vec::with_capacity(20);
        for i in 0..20 {
            actual.push(atr.next((high[i], low[i], close[i])));
        }

        for i in 0..13 {
            assert!(actual[i].is_nan(), "Bar {i} must be NaN under Wilder 1978");
        }
        for i in 13..20 {
            approx::assert_relative_eq!(actual[i], expected_atr[i], epsilon = 1e-12);
        }
    }

    #[test]
    fn test_wilder1978_talib_convergence() {
        // 500-bar series where differences decay below 1e-9 by bar 400
        let n = 500;
        let mut high = Vec::with_capacity(n);
        let mut low = Vec::with_capacity(n);
        let mut close = Vec::with_capacity(n);
        let mut p = 100.0;
        for i in 0..n {
            let step = (i as f64 * 0.1).sin() * 2.0;
            p += step;
            high.push(p + 1.5);
            low.push(p - 1.5);
            close.push(p);
        }

        let mut atr_talib = TaATR::new(14);
        let mut atr_classic = TaATR::classic(14);

        let mut diff_400 = 0.0;
        for i in 0..n {
            let v_t = atr_talib.next((high[i], low[i], close[i]));
            let v_c = atr_classic.next((high[i], low[i], close[i]));
            if i == 400 {
                diff_400 = (v_t - v_c).abs() / v_t;
            }
        }

        assert!(
            diff_400 < 1e-9,
            "Relative difference at bar 400 must be < 1e-9, got {diff_400}"
        );
    }
}
