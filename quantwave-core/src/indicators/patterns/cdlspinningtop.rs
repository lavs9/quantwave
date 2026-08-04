//! Native O(1) streaming CDLSPINNINGTOP (matches `talib_rs::pattern::cdl_spinningtop`).
//! Source: TA-Lib

use crate::traits::Next;
use crate::indicators::patterns::candle_settings::{
    CandleWindow, RollingCandleAvg, BODY_SHORT,
    real_body, upper_shadow, lower_shadow, candle_color
};

const LOOKBACK: usize = 10;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLSPINNINGTOP {
    bars_seen: usize,
    window: CandleWindow,
    body_short: RollingCandleAvg,
}

impl CDLSPINNINGTOP {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(1),
            body_short: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}

impl Default for CDLSPINNINGTOP {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLSPINNINGTOP {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_short.push(open, high, low, close);

        if self.bars_seen <= LOOKBACK {
            return 0.0;
        }

        let curr = self.window.bar(0);

        let out = (real_body(curr.open, curr.close) < self.body_short.val(0)
            && upper_shadow(curr.open, curr.high, curr.close) > real_body(curr.open, curr.close)
            && lower_shadow(curr.open, curr.low, curr.close) > real_body(curr.open, curr.close)) as i32 * candle_color(curr.open, curr.close) * 100;

        out as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(test_cdlspinningtop_parity, CDLSPINNINGTOP, talib_rs::pattern::cdl_spinningtop);
}
