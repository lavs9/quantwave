//! Native O(1) streaming CDLBELTHOLD (matches `talib_rs::pattern::cdl_belthold`).
//! Source: TA-Lib

use crate::indicators::patterns::candle_settings::{
    BODY_LONG, CandleWindow, RollingCandleAvg, SHADOW_VERY_SHORT, candle_color, lower_shadow,
    real_body, upper_shadow,
};
use crate::traits::Next;

const LOOKBACK: usize = 10; // max(10, 10)

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLBELTHOLD {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    shadow_vs: RollingCandleAvg,
}

impl CDLBELTHOLD {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(1),
            body_long: RollingCandleAvg::new(BODY_LONG),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
        }
    }
}

impl Default for CDLBELTHOLD {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLBELTHOLD {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.shadow_vs.push(open, high, low, close);

        if self.bars_seen <= LOOKBACK {
            return 0.0;
        }

        let curr = self.window.bar(0);

        let long_body = real_body(curr.open, curr.close) > self.body_long.val(0);
        let bull = long_body
            && candle_color(curr.open, curr.close) == 1
            && lower_shadow(curr.open, curr.low, curr.close) < self.shadow_vs.val(0);
        let bear = long_body
            && candle_color(curr.open, curr.close) == -1
            && upper_shadow(curr.open, curr.high, curr.close) < self.shadow_vs.val(0);

        ((bull as i32) * 100 - (bear as i32) * 100) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(
        test_cdlbelthold_parity,
        CDLBELTHOLD,
        talib_rs::pattern::cdl_belthold
    );
}
