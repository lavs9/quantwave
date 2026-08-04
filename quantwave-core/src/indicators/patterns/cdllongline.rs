//! Native O(1) streaming CDLLONGLINE (matches `talib_rs::pattern::cdl_longline`).
//! Source: TA-Lib

use crate::indicators::patterns::candle_settings::{
    BODY_LONG, CandleWindow, RollingCandleAvg, SHADOW_SHORT, candle_color, lower_shadow, real_body,
    upper_shadow,
};
use crate::traits::Next;

const LOOKBACK: usize = 10; // max(10, 10)

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLLONGLINE {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    shadow_short: RollingCandleAvg,
}

impl CDLLONGLINE {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(1),
            body_long: RollingCandleAvg::new(BODY_LONG),
            shadow_short: RollingCandleAvg::new(SHADOW_SHORT),
        }
    }
}

impl Default for CDLLONGLINE {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLLONGLINE {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.shadow_short.push(open, high, low, close);

        if self.bars_seen <= LOOKBACK {
            return 0.0;
        }

        let curr = self.window.bar(0);

        let out = (real_body(curr.open, curr.close) > self.body_long.val(0)
            && upper_shadow(curr.open, curr.high, curr.close) < self.shadow_short.val(0)
            && lower_shadow(curr.open, curr.low, curr.close) < self.shadow_short.val(0))
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
        test_cdllongline_parity,
        CDLLONGLINE,
        talib_rs::pattern::cdl_longline
    );
}
