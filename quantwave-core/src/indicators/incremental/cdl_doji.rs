//! Native O(1) streaming CDLDOJI (matches `talib_rs::pattern::cdl_doji`).

use crate::indicators::incremental::utils::RingBuffer;
use crate::traits::Next;

/// Candle setting equivalent to TA-Lib `BODY_DOJI`: HighLow range, avg period 10, factor 0.1.
const LOOKBACK: usize = 10;
const FACTOR_DIV: f64 = 0.1 / LOOKBACK as f64; // 0.01

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLDOJI {
    bar_index: usize,
    hl_sum: f64,
    hl_window: RingBuffer<f64>,
}

impl CDLDOJI {
    pub fn new() -> Self {
        Self {
            bar_index: 0,
            hl_sum: 0.0,
            hl_window: RingBuffer::with_capacity(LOOKBACK),
        }
    }
}

impl Default for CDLDOJI {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLDOJI {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        let idx = self.bar_index;
        self.bar_index += 1;
        let hl = high - low;

        if idx < LOOKBACK {
            self.hl_window.push_back(hl);
            self.hl_sum += hl;
            return 0.0;
        }

        let body = (close - open).abs();
        let thresh = self.hl_sum * FACTOR_DIV;
        let out = 100.0_f64.copysign(thresh - body).max(0.0);

        if let Some(old) = self.hl_window.pop_front() {
            self.hl_sum += hl - old;
        }
        self.hl_window.push_back(hl);

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_cdl_doji_parity(
            o in prop::collection::vec(10.0..100.0, 1..100),
            h in prop::collection::vec(10.0..100.0, 1..100),
            l in prop::collection::vec(10.0..100.0, 1..100),
            c in prop::collection::vec(10.0..100.0, 1..100)
        ) {
            let len = o.len().min(h.len()).min(l.len()).min(c.len());
            if len == 0 { return Ok(()); }

            let mut doji = CDLDOJI::new();
            let streaming: Vec<f64> = (0..len)
                .map(|i| doji.next((o[i], h[i], l[i], c[i])))
                .collect();
            let batch = talib_rs::pattern::cdl_doji(&o[..len], &h[..len], &l[..len], &c[..len])
                .unwrap_or_else(|_| vec![0; len]);

            for (s, b) in streaming.iter().zip(batch.iter()) {
                assert_eq!(*s as i32, *b);
            }
        }
    }
}
