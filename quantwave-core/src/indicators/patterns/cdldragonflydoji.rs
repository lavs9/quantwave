//! Native O(1) streaming CDLDRAGONFLYDOJI (matches `talib_rs::pattern::cdl_dragonflydoji`).
//! Source: TA-Lib

use crate::indicators::patterns::candle_settings::{
    BODY_DOJI, CandleWindow, RollingCandleAvg, SHADOW_VERY_SHORT, lower_shadow, real_body,
    upper_shadow,
};
use crate::traits::Next;

const LOOKBACK: usize = 10; // max(10, 10)

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLDRAGONFLYDOJI {
    bars_seen: usize,
    window: CandleWindow,
    body_doji: RollingCandleAvg,
    shadow_vs: RollingCandleAvg,
}

impl CDLDRAGONFLYDOJI {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(1),
            body_doji: RollingCandleAvg::new(BODY_DOJI),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
        }
    }
}

impl Default for CDLDRAGONFLYDOJI {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLDRAGONFLYDOJI {
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
            && upper_shadow(curr.open, curr.high, curr.close) < self.shadow_vs.val(0)
            && lower_shadow(curr.open, curr.low, curr.close) > self.shadow_vs.val(0))
            as i32
            * 100;

        out as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(
        test_cdldragonflydoji_parity,
        CDLDRAGONFLYDOJI,
        talib_rs::pattern::cdl_dragonflydoji,
        |o: &mut Vec<f64>, h: &mut Vec<f64>, l: &mut Vec<f64>, c: &mut Vec<f64>| {
            // Provide 11 bars of generic positive drift to establish shadow/body averages
            for _ in 0..11 {
                o.push(100.0);
                h.push(110.0);
                l.push(90.0);
                c.push(100.0);
            }
            // A dragonfly doji: Open and close are equal and at the high of the bar,
            // with a long lower shadow.
            o.push(100.0);
            h.push(100.0);
            l.push(50.0);
            c.push(100.0);
        }
    );
}
