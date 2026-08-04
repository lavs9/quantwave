//! Native O(1) MAVP — variable-period SMA per bar (TA-Lib parity).

use crate::indicators::ma_type::MaType;
use crate::traits::Next;

/// Moving average with variable period — matches `talib_rs::overlap::mavp`.
///
/// `matype` is retained for API compatibility but does not change the result:
/// the batch implementation also always uses SMA ("变周期场景下无法高效使用其他
/// MA" — a variable window makes the recursive MA families ill-defined), so
/// honouring it here would *break* batch↔streaming parity rather than fix it.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MAVP {
    pub minperiod: usize,
    pub maxperiod: usize,
    pub matype: MaType,
    prices: Vec<f64>,
}

impl MAVP {
    pub fn new(minperiod: usize, maxperiod: usize, matype: MaType) -> Self {
        Self {
            minperiod,
            maxperiod,
            matype,
            prices: Vec::new(),
        }
    }
}

impl Next<(f64, f64)> for MAVP {
    type Output = f64;

    fn next(&mut self, (price, period): (f64, f64)) -> Self::Output {
        let _ = self.matype;
        self.prices.push(price);
        let i = self.prices.len() - 1;
        let maxp = self.maxperiod;
        if maxp == 0 || i < maxp - 1 {
            return f64::NAN;
        }
        let p = (period.round() as usize).clamp(self.minperiod, self.maxperiod);
        if i + 1 >= p {
            let start = i + 1 - p;
            let sum: f64 = self.prices[start..=i].iter().sum();
            sum / p as f64
        } else {
            f64::NAN
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_mavp_parity(
            h in prop::collection::vec(10.0..100.0, 10..100),
            l in prop::collection::vec(10.0..100.0, 10..100)
        ) {
            let len = h.len().min(l.len());
            let in1: Vec<f64> = (0..len)
                .map(|i| {
                    let hi: f64 = h[i];
                    let lo: f64 = l[i];
                    hi.max(lo)
                })
                .collect();
            let in2: Vec<f64> = (0..len)
                .map(|i| {
                    let hi: f64 = h[i];
                    let lo: f64 = l[i];
                    hi.min(lo)
                })
                .collect();
            let minperiod = 2usize;
            let maxperiod = 30usize;
            let matype = MaType::Sma;
            let mut mavp = MAVP::new(minperiod, maxperiod, matype);
            let streaming: Vec<f64> = (0..len)
                .map(|i| mavp.next((in1[i], in2[i])))
                .collect();
            let batch = talib_rs::overlap::mavp(&in1, &in2, minperiod, maxperiod, matype.into())
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }
}
