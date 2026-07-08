//! Native O(1) volume indicators: AD, ADOSC.

use crate::indicators::incremental::talib_ema::TalibEma;
use crate::traits::Next;

/// Accumulation/Distribution Line.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct AD {
    cumulative: f64,
}

impl AD {
    pub fn new() -> Self {
        Self { cumulative: 0.0 }
    }
}

impl Next<(f64, f64, f64, f64)> for AD {
    type Output = f64;

    fn next(&mut self, (high, low, close, volume): (f64, f64, f64, f64)) -> Self::Output {
        let hl = high - low;
        let mfm = if hl > 0.0 {
            ((close - low) - (high - close)) / hl
        } else {
            0.0
        };
        self.cumulative += mfm * volume;
        self.cumulative
    }
}

/// Chaikin A/D Oscillator.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct ADOSC {
    pub fastperiod: usize,
    pub slowperiod: usize,
    ad: AD,
    fast_ema: TalibEma,
    slow_ema: TalibEma,
}

impl ADOSC {
    pub fn new(fastperiod: usize, slowperiod: usize) -> Self {
        Self {
            fastperiod,
            slowperiod,
            ad: AD::new(),
            fast_ema: TalibEma::new(fastperiod),
            slow_ema: TalibEma::new(slowperiod),
        }
    }
}

impl Next<(f64, f64, f64, f64)> for ADOSC {
    type Output = f64;

    fn next(&mut self, ohlcv: (f64, f64, f64, f64)) -> Self::Output {
        let ad_val = self.ad.next(ohlcv);
        let fast = self.fast_ema.next(ad_val);
        let slow = self.slow_ema.next(ad_val);
        if fast.is_nan() || slow.is_nan() {
            f64::NAN
        } else {
            fast - slow
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_ad_parity(
            h in prop::collection::vec(10.0..100.0, 1..100),
            l in prop::collection::vec(10.0..100.0, 1..100),
            c in prop::collection::vec(10.0..100.0, 1..100),
            v in prop::collection::vec(1.0..1000.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len()).min(v.len());
            let mut ad = AD::new();
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            let mut vol = Vec::with_capacity(len);
            for i in 0..len {
                let v_h: f64 = h[i];
                let v_l: f64 = l[i];
                let v_c: f64 = c[i];
                let v_v: f64 = v[i];
                high.push(v_h.max(v_l).max(v_c));
                low.push(v_h.min(v_l).min(v_c));
                close.push(v_c);
                vol.push(v_v);
            }
            let streaming: Vec<f64> = (0..len)
                .map(|i| ad.next((high[i], low[i], close[i], vol[i])))
                .collect();
            let batch = talib_rs::volume::ad(&high, &low, &close, &vol)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}
