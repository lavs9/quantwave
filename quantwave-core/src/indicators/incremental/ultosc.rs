//! Native streaming Ultimate Oscillator — TA-Lib parity (`talib_rs::momentum::ultosc`).

use crate::traits::Next;

/// Ultimate Oscillator — default periods 7, 14, 28.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct ULTOSC {
    pub timeperiod1: usize,
    pub timeperiod2: usize,
    pub timeperiod3: usize,
    max_period: usize,
    prev_close: Option<f64>,
    bp: Vec<f64>,
    tr: Vec<f64>,
    sum_bp1: f64,
    sum_tr1: f64,
    sum_bp2: f64,
    sum_tr2: f64,
    sum_bp3: f64,
    sum_tr3: f64,
    initialized: bool,
}

impl ULTOSC {
    pub fn new(timeperiod1: usize, timeperiod2: usize, timeperiod3: usize) -> Self {
        let max_period = timeperiod1.max(timeperiod2).max(timeperiod3);
        Self {
            timeperiod1,
            timeperiod2,
            timeperiod3,
            max_period,
            prev_close: None,
            bp: vec![0.0],
            tr: vec![0.0],
            sum_bp1: 0.0,
            sum_tr1: 0.0,
            sum_bp2: 0.0,
            sum_tr2: 0.0,
            sum_bp3: 0.0,
            sum_tr3: 0.0,
            initialized: false,
        }
    }

    #[inline]
    fn ultosc_value(&self) -> f64 {
        let avg1 = if self.sum_tr1 > 0.0 {
            self.sum_bp1 / self.sum_tr1
        } else {
            0.0
        };
        let avg2 = if self.sum_tr2 > 0.0 {
            self.sum_bp2 / self.sum_tr2
        } else {
            0.0
        };
        let avg3 = if self.sum_tr3 > 0.0 {
            self.sum_bp3 / self.sum_tr3
        } else {
            0.0
        };
        100.0 * (4.0 * avg1 + 2.0 * avg2 + avg3) / 7.0
    }

    fn init_sums(&mut self, i: usize) {
        let p1 = self.timeperiod1;
        let p2 = self.timeperiod2;
        let p3 = self.timeperiod3;
        self.sum_bp1 = self.bp[(i + 1 - p1)..=i].iter().sum();
        self.sum_tr1 = self.tr[(i + 1 - p1)..=i].iter().sum();
        self.sum_bp2 = self.bp[(i + 1 - p2)..=i].iter().sum();
        self.sum_tr2 = self.tr[(i + 1 - p2)..=i].iter().sum();
        self.sum_bp3 = self.bp[(i + 1 - p3)..=i].iter().sum();
        self.sum_tr3 = self.tr[(i + 1 - p3)..=i].iter().sum();
        self.initialized = true;
    }

    fn slide_sums(&mut self, i: usize) {
        let p1 = self.timeperiod1;
        let p2 = self.timeperiod2;
        let p3 = self.timeperiod3;
        self.sum_bp1 += self.bp[i] - self.bp[i - p1];
        self.sum_tr1 += self.tr[i] - self.tr[i - p1];
        self.sum_bp2 += self.bp[i] - self.bp[i - p2];
        self.sum_tr2 += self.tr[i] - self.tr[i - p2];
        self.sum_bp3 += self.bp[i] - self.bp[i - p3];
        self.sum_tr3 += self.tr[i] - self.tr[i - p3];
    }
}

impl Next<(f64, f64, f64)> for ULTOSC {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let max_period = self.max_period;

        let Some(prev_close) = self.prev_close else {
            self.prev_close = Some(close);
            return f64::NAN;
        };

        let true_low = low.min(prev_close);
        let true_high = high.max(prev_close);
        let bp_val = close - true_low;
        let tr_val = true_high - true_low;

        self.bp.push(bp_val);
        self.tr.push(tr_val);
        self.prev_close = Some(close);

        let i = self.bp.len() - 1;
        if i < max_period {
            return f64::NAN;
        }

        if !self.initialized {
            self.init_sums(i);
            return self.ultosc_value();
        }

        self.slide_sums(i);
        self.ultosc_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn ordered_hlc(h: &[f64], l: &[f64], c: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let len = h.len().min(l.len()).min(c.len());
        let mut high = Vec::with_capacity(len);
        let mut low = Vec::with_capacity(len);
        let mut close = Vec::with_capacity(len);
        for i in 0..len {
            let vh = h[i];
            let vl = l[i];
            let vc = c[i];
            high.push(vh.max(vl).max(vc));
            low.push(vh.min(vl).min(vc));
            close.push(vc);
        }
        (high, low, close)
    }

    proptest! {
        #[test]
        fn test_ultosc_parity(
            h in prop::collection::vec(1.0..100.0, 10..100),
            l in prop::collection::vec(1.0..100.0, 10..100),
            c in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let (high, low, close) = ordered_hlc(&h, &l, &c);
            let len = high.len();
            if len == 0 { return Ok(()); }
            let p1 = 7;
            let p2 = 14;
            let p3 = 28;
            let mut ult = ULTOSC::new(p1, p2, p3);
            let streaming: Vec<f64> =
                (0..len).map(|i| ult.next((high[i], low[i], close[i]))).collect();
            let batch = talib_rs::momentum::ultosc(&high, &low, &close, p1, p2, p3)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else if !b.is_nan() {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }
}
