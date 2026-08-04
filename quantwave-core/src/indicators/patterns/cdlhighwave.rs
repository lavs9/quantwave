//! Native O(1) streaming CDLHIGHWAVE (matches `talib_rs::pattern::cdl_highwave`).
//! Source: TA-Lib

use crate::indicators::patterns::candle_settings::{
    BODY_SHORT, CandleWindow, RollingCandleAvg, SHADOW_VERY_LONG, candle_color, lower_shadow,
    real_body, upper_shadow,
};
use crate::traits::Next;

const LOOKBACK: usize = 10; // max(10, 0)

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLHIGHWAVE {
    bars_seen: usize,
    window: CandleWindow,
    body_short: RollingCandleAvg,
    shadow_vl: RollingCandleAvg,
}

impl CDLHIGHWAVE {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(1),
            body_short: RollingCandleAvg::new(BODY_SHORT),
            shadow_vl: RollingCandleAvg::new(SHADOW_VERY_LONG),
        }
    }
}

impl Default for CDLHIGHWAVE {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLHIGHWAVE {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_short.push(open, high, low, close);
        self.shadow_vl.push(open, high, low, close);

        if self.bars_seen <= LOOKBACK {
            return 0.0;
        }

        let curr = self.window.bar(0);

        let out = (real_body(curr.open, curr.close) < self.body_short.val(0)
            && upper_shadow(curr.open, curr.high, curr.close) > self.shadow_vl.val(0)
            && lower_shadow(curr.open, curr.low, curr.close) > self.shadow_vl.val(0))
            as i32
            * candle_color(curr.open, curr.close)
            * 100;

        out as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(
        test_cdlhighwave_parity,
        CDLHIGHWAVE,
        talib_rs::pattern::cdl_highwave
    );
}
