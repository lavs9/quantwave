//! Native O(1) streaming CDLDOJI (matches `talib_rs::pattern::cdl_doji`).

use crate::indicators::patterns::candle_settings::{BODY_DOJI, RollingCandleAvg};
use crate::traits::Next;

const LOOKBACK: usize = 10;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLDOJI {
    bars_seen: usize,
    hl_avg: RollingCandleAvg,
}

impl CDLDOJI {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            hl_avg: RollingCandleAvg::new(BODY_DOJI),
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
        self.bars_seen += 1;
        self.hl_avg.push(open, high, low, close);

        if self.bars_seen <= LOOKBACK {
            return 0.0;
        }

        let body = (close - open).abs();
        let thresh = self.hl_avg.val(0);

        100.0_f64.copysign(thresh - body).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(test_cdl_doji_parity, CDLDOJI, talib_rs::pattern::cdl_doji);
}
