//! Native O(1) streaming CDLRICKSHAWMAN (matches `talib_rs::pattern::cdl_rickshawman`).
//! Source: TA-Lib

use crate::traits::Next;
use crate::indicators::patterns::candle_settings::{
    CandleWindow, RollingCandleAvg, BODY_DOJI, SHADOW_LONG, NEAR,
    real_body, upper_shadow, lower_shadow
};

const LOOKBACK: usize = 10; // max(10, 0, 5)

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLRICKSHAWMAN {
    bars_seen: usize,
    window: CandleWindow,
    body_doji: RollingCandleAvg,
    shadow_long: RollingCandleAvg,
    near: RollingCandleAvg,
}

impl CDLRICKSHAWMAN {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(1),
            body_doji: RollingCandleAvg::new(BODY_DOJI),
            shadow_long: RollingCandleAvg::new(SHADOW_LONG),
            near: RollingCandleAvg::new(NEAR),
        }
    }
}

impl Default for CDLRICKSHAWMAN {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLRICKSHAWMAN {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_doji.push(open, high, low, close);
        self.shadow_long.push(open, high, low, close);
        self.near.push(open, high, low, close);

        if self.bars_seen <= LOOKBACK {
            return 0.0;
        }

        let curr = self.window.bar(0);

        let mid = curr.low + (curr.high - curr.low) / 2.0;
        let near_avg = self.near.val(0);

        let out = (real_body(curr.open, curr.close) <= self.body_doji.val(0)
            && lower_shadow(curr.open, curr.low, curr.close) > self.shadow_long.val(0)
            && upper_shadow(curr.open, curr.high, curr.close) > self.shadow_long.val(0)
            && curr.open.min(curr.close) <= mid + near_avg
            && curr.open.max(curr.close) >= mid - near_avg) as i32 * 100;

        out as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(test_cdlrickshawman_parity, CDLRICKSHAWMAN, talib_rs::pattern::cdl_rickshawman);
}
