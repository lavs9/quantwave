//! Native O(1) streaming CDLGRAVESTONEDOJI (matches `talib_rs::pattern::cdl_gravestonedoji`).
//! Source: TA-Lib

use crate::traits::Next;
use crate::indicators::patterns::candle_settings::{
    CandleWindow, RollingCandleAvg, BODY_DOJI, SHADOW_VERY_SHORT,
    real_body, upper_shadow, lower_shadow
};

const LOOKBACK: usize = 10; // max(10, 10)

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLGRAVESTONEDOJI {
    bars_seen: usize,
    window: CandleWindow,
    body_doji: RollingCandleAvg,
    shadow_vs: RollingCandleAvg,
}

impl CDLGRAVESTONEDOJI {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(1),
            body_doji: RollingCandleAvg::new(BODY_DOJI),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
        }
    }
}

impl Default for CDLGRAVESTONEDOJI {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLGRAVESTONEDOJI {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_doji.push(open, high, low, close);
        self.shadow_vs.push(open, high, low, close);

        if self.bars_seen <= LOOKBACK {
            return 0.0;
        }

        let curr = self.window.bar(0);

        let out = (real_body(curr.open, curr.close) <= self.body_doji.val(0)
            && lower_shadow(curr.open, curr.low, curr.close) < self.shadow_vs.val(0)
            && upper_shadow(curr.open, curr.high, curr.close) > self.shadow_vs.val(0)) as i32 * 100;

        out as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(
        test_cdlgravestonedoji_parity,
        CDLGRAVESTONEDOJI,
        talib_rs::pattern::cdl_gravestonedoji,
        |o: &mut Vec<f64>, h: &mut Vec<f64>, l: &mut Vec<f64>, c: &mut Vec<f64>| {
            // Provide 11 bars to establish shadow/body averages
            for _ in 0..11 {
                o.push(100.0);
                h.push(110.0);
                l.push(90.0);
                c.push(100.0);
            }
            // A gravestone doji: Open and close are equal and at the low of the bar,
            // with a long upper shadow.
            o.push(100.0);
            h.push(150.0);
            l.push(100.0);
            c.push(100.0);
        }
    );
}
