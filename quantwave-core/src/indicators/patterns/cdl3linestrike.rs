//! Native O(1) streaming CDL3LINESTRIKE (matches `talib_rs::pattern::cdl_3linestrike`).

use crate::indicators::patterns::candle_settings::{candle_color, CandleWindow, RollingCandleAvg, NEAR};
use crate::traits::Next;

const LOOKBACK: usize = 5 + 3; // NEAR.avg_period (5) + 3

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDL3LINESTRIKE {
    bars_seen: usize,
    window: CandleWindow,
    near_avg: RollingCandleAvg,
}

impl CDL3LINESTRIKE {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(4), // Needs i, i-1, i-2, i-3 => capacity 4
            near_avg: RollingCandleAvg::new(NEAR),
        }
    }
}

impl Default for CDL3LINESTRIKE {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDL3LINESTRIKE {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        
        self.window.push(open, high, low, close);
        self.near_avg.push(open, high, low, close);

        if self.bars_seen <= LOOKBACK {
            return 0.0;
        }


        let c3 = candle_color(self.window.bar(3).open, self.window.bar(3).close);
        let c2 = candle_color(self.window.bar(2).open, self.window.bar(2).close);
        let c1 = candle_color(self.window.bar(1).open, self.window.bar(1).close);
        let c0 = candle_color(self.window.bar(0).open, self.window.bar(0).close);

        if c3 == c2 && c2 == c1 && c0 != c1 {
            let progressive = if c3 == 1 {
                self.window.bar(2).close > self.window.bar(3).close && self.window.bar(1).close > self.window.bar(2).close
            } else {
                self.window.bar(2).close < self.window.bar(3).close && self.window.bar(1).close < self.window.bar(2).close
            };

            let opens_near = if c3 == 1 {
                self.window.bar(2).open >= self.window.bar(3).open.min(self.window.bar(3).close)
                    && self.window.bar(2).open <= self.window.bar(3).close + self.near_avg.val(3)
                    && self.window.bar(1).open >= self.window.bar(2).open.min(self.window.bar(2).close)
                    && self.window.bar(1).open <= self.window.bar(2).close + self.near_avg.val(2)
            } else {
                self.window.bar(2).open <= self.window.bar(3).open.max(self.window.bar(3).close)
                    && self.window.bar(2).open >= self.window.bar(3).close - self.near_avg.val(3)
                    && self.window.bar(1).open <= self.window.bar(2).open.max(self.window.bar(2).close)
                    && self.window.bar(1).open >= self.window.bar(2).close - self.near_avg.val(2)
            };

            let strike = if c3 == 1 {
                self.window.bar(0).open >= self.window.bar(1).close && self.window.bar(0).close <= self.window.bar(3).open
            } else {
                self.window.bar(0).open <= self.window.bar(1).close && self.window.bar(0).close >= self.window.bar(3).open
            };

            if progressive && opens_near && strike {
                return (c3 * 100) as f64;
            }
        }
        
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(test_cdl3linestrike_parity, CDL3LINESTRIKE, talib_rs::pattern::cdl_3linestrike);
}
