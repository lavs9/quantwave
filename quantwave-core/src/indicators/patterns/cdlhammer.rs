//! Native O(1) streaming CDLHAMMER (matches `talib_rs::pattern::cdl_hammer`).
//! Source: TA-Lib

use crate::indicators::patterns::candle_settings::{
    BODY_SHORT, CandleWindow, NEAR, RollingCandleAvg, SHADOW_LONG, SHADOW_VERY_SHORT, lower_shadow,
    real_body, upper_shadow,
};
use crate::traits::Next;

const LOOKBACK: usize = 11; // max(10, 0, 10, 5) + 1

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLHAMMER {
    bars_seen: usize,
    window: CandleWindow,
    body_short: RollingCandleAvg,
    shadow_long: RollingCandleAvg,
    shadow_vs: RollingCandleAvg,
    near: RollingCandleAvg,
}

impl CDLHAMMER {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            body_short: RollingCandleAvg::new(BODY_SHORT),
            shadow_long: RollingCandleAvg::new(SHADOW_LONG),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
            near: RollingCandleAvg::new(NEAR),
        }
    }
}

impl Default for CDLHAMMER {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLHAMMER {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_short.push(open, high, low, close);
        self.shadow_long.push(open, high, low, close);
        self.shadow_vs.push(open, high, low, close);
        self.near.push(open, high, low, close);

        if self.bars_seen <= LOOKBACK {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let out = (real_body(curr.open, curr.close) < self.body_short.val(0)
            && lower_shadow(curr.open, curr.low, curr.close) > self.shadow_long.val(0)
            && upper_shadow(curr.open, curr.high, curr.close) < self.shadow_vs.val(0)
            && curr.open.min(curr.close) <= prev.low + self.near.val(1)) as i32
            * 100;

        out as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(
        test_cdlhammer_parity,
        CDLHAMMER,
        talib_rs::pattern::cdl_hammer
    );
}
