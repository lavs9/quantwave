//! Native O(1) streaming patterns for Batch D (three-bar patterns).

use crate::indicators::patterns::candle_settings::*;
use crate::traits::Next;

/// CDL3BLACKCROWS
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDL3BLACKCROWS {
    bars_seen: usize,
    window: CandleWindow,
    shadow_very_short_avg: RollingCandleAvg,
}

impl CDL3BLACKCROWS {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(4),
            shadow_very_short_avg: RollingCandleAvg::new(SHADOW_VERY_SHORT),
        }
    }
}

impl Default for CDL3BLACKCROWS {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDL3BLACKCROWS {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.shadow_very_short_avg.push(open, high, low, close);

        if self.bars_seen <= SHADOW_VERY_SHORT.avg_period + 3 {
            return 0.0;
        }

        if candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1 && candle_color(self.window.bar(1).open, self.window.bar(1).close) == -1 && candle_color(self.window.bar(0).open, self.window.bar(0).close) == -1
            && self.window.bar(1).close < self.window.bar(2).close && self.window.bar(0).close < self.window.bar(1).close
            && self.window.bar(2).open <= self.window.bar(3).open.max(self.window.bar(3).close)
            && self.window.bar(1).open <= self.window.bar(2).open && self.window.bar(1).open >= self.window.bar(2).close
            && self.window.bar(0).open <= self.window.bar(1).open && self.window.bar(0).open >= self.window.bar(1).close
            && lower_shadow(self.window.bar(2).open, self.window.bar(2).low, self.window.bar(2).close) < self.shadow_very_short_avg.val(2)
            && lower_shadow(self.window.bar(1).open, self.window.bar(1).low, self.window.bar(1).close) < self.shadow_very_short_avg.val(1)
            && lower_shadow(self.window.bar(0).open, self.window.bar(0).low, self.window.bar(0).close) < self.shadow_very_short_avg.val(0) {
            return -100.0;
        }
        0.0
    }
}

/// CDL3INSIDE
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDL3INSIDE {
    bars_seen: usize,
    window: CandleWindow,
    body_long_avg: RollingCandleAvg,
    body_short_avg: RollingCandleAvg,
}

impl CDL3INSIDE {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_long_avg: RollingCandleAvg::new(BODY_LONG),
            body_short_avg: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}

impl Default for CDL3INSIDE {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDL3INSIDE {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long_avg.push(open, high, low, close);
        self.body_short_avg.push(open, high, low, close);

        if self.bars_seen <= BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 2 {
            return 0.0;
        }

        if real_body(self.window.bar(2).open, self.window.bar(2).close) > self.body_long_avg.val(2) && real_body(self.window.bar(1).open, self.window.bar(1).close) <= self.body_short_avg.val(1)
            && self.window.bar(1).open.max(self.window.bar(1).close) < self.window.bar(2).open.max(self.window.bar(2).close) && self.window.bar(1).open.min(self.window.bar(1).close) > self.window.bar(2).open.min(self.window.bar(2).close) {
            if candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1 && candle_color(self.window.bar(0).open, self.window.bar(0).close) == -1 && self.window.bar(0).close < self.window.bar(2).open {
                return -100.0;
            } else if candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1 && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1 && self.window.bar(0).close > self.window.bar(2).open {
                return 100.0;
            }
        }
        0.0
    }
}

/// CDL3OUTSIDE
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDL3OUTSIDE {
    bars_seen: usize,
    window: CandleWindow,
}

impl CDL3OUTSIDE {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
        }
    }
}

impl Default for CDL3OUTSIDE {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDL3OUTSIDE {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);

        if self.bars_seen <= 2 {
            return 0.0;
        }

        let bull = candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1 && candle_color(self.window.bar(1).open, self.window.bar(1).close) == 1 && self.window.bar(1).close >= self.window.bar(2).open && self.window.bar(1).open <= self.window.bar(2).close && self.window.bar(0).close > self.window.bar(1).close;
        let bear = candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1 && candle_color(self.window.bar(1).open, self.window.bar(1).close) == -1 && self.window.bar(1).open >= self.window.bar(2).close && self.window.bar(1).close <= self.window.bar(2).open && self.window.bar(0).close < self.window.bar(1).close;
        if bull { return 100.0; }
        if bear { return -100.0; }
        0.0
    }
}

/// CDL3STARSINSOUTH
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDL3STARSINSOUTH {
    bars_seen: usize,
    window: CandleWindow,
    body_long_avg: RollingCandleAvg,
    shadow_long_avg: RollingCandleAvg,
}

impl CDL3STARSINSOUTH {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_long_avg: RollingCandleAvg::new(BODY_LONG),
            shadow_long_avg: RollingCandleAvg::new(SHADOW_LONG),
        }
    }
}

impl Default for CDL3STARSINSOUTH {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDL3STARSINSOUTH {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long_avg.push(open, high, low, close);
        self.shadow_long_avg.push(open, high, low, close);

        if self.bars_seen <= BODY_LONG.avg_period.max(SHADOW_LONG.avg_period) + 2 {
            return 0.0;
        }

        if candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1 && candle_color(self.window.bar(1).open, self.window.bar(1).close) == -1 && candle_color(self.window.bar(0).open, self.window.bar(0).close) == -1
            && real_body(self.window.bar(2).open, self.window.bar(2).close) > self.body_long_avg.val(2) && lower_shadow(self.window.bar(2).open, self.window.bar(2).low, self.window.bar(2).close) > self.shadow_long_avg.val(2)
            && self.window.bar(1).open.min(self.window.bar(1).close) > self.window.bar(2).open.min(self.window.bar(2).close) && self.window.bar(1).open.max(self.window.bar(1).close) < self.window.bar(2).open.max(self.window.bar(2).close) && self.window.bar(1).low < self.window.bar(2).low
            && self.window.bar(0).open.min(self.window.bar(0).close) > self.window.bar(1).open.min(self.window.bar(1).close) && self.window.bar(0).open.max(self.window.bar(0).close) < self.window.bar(1).open.max(self.window.bar(1).close) && lower_shadow(self.window.bar(0).open, self.window.bar(0).low, self.window.bar(0).close) == 0.0 {
            return 100.0;
        }
        0.0
    }
}

/// CDL3WHITESOLDIERS
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDL3WHITESOLDIERS {
    bars_seen: usize,
    window: CandleWindow,
    near_avg: RollingCandleAvg,
    shadow_very_short_avg: RollingCandleAvg,
}

impl CDL3WHITESOLDIERS {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(4),
            near_avg: RollingCandleAvg::new(NEAR),
            shadow_very_short_avg: RollingCandleAvg::new(SHADOW_VERY_SHORT),
        }
    }
}

impl Default for CDL3WHITESOLDIERS {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDL3WHITESOLDIERS {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.near_avg.push(open, high, low, close);
        self.shadow_very_short_avg.push(open, high, low, close);

        if self.bars_seen <= SHADOW_VERY_SHORT.avg_period.max(NEAR.avg_period) + 3 {
            return 0.0;
        }

        if candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1 && candle_color(self.window.bar(1).open, self.window.bar(1).close) == 1 && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1
            && self.window.bar(1).close > self.window.bar(2).close && self.window.bar(0).close > self.window.bar(1).close
            && upper_shadow(self.window.bar(2).open, self.window.bar(2).high, self.window.bar(2).close) < self.shadow_very_short_avg.val(2) && upper_shadow(self.window.bar(1).open, self.window.bar(1).high, self.window.bar(1).close) < self.shadow_very_short_avg.val(1) && upper_shadow(self.window.bar(0).open, self.window.bar(0).high, self.window.bar(0).close) < self.shadow_very_short_avg.val(0)
            && self.window.bar(1).open > self.window.bar(2).open && self.window.bar(1).open <= self.window.bar(2).close + self.near_avg.val(1)
            && self.window.bar(0).open > self.window.bar(1).open && self.window.bar(0).open <= self.window.bar(1).close + self.near_avg.val(0) {
            return 100.0;
        }
        0.0
    }
}

/// CDLABANDONEDBABY
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLABANDONEDBABY {
    bars_seen: usize,
    window: CandleWindow,
    body_doji_avg: RollingCandleAvg,
    body_long_avg: RollingCandleAvg,
    body_short_avg: RollingCandleAvg,
}

impl CDLABANDONEDBABY {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_doji_avg: RollingCandleAvg::new(BODY_DOJI),
            body_long_avg: RollingCandleAvg::new(BODY_LONG),
            body_short_avg: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}

impl Default for CDLABANDONEDBABY {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLABANDONEDBABY {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_doji_avg.push(open, high, low, close);
        self.body_long_avg.push(open, high, low, close);
        self.body_short_avg.push(open, high, low, close);

        if self.bars_seen <= BODY_DOJI.avg_period.max(BODY_LONG.avg_period).max(BODY_SHORT.avg_period) + 2 {
            return 0.0;
        }

        let penetration = 0.3;
        let base = real_body(self.window.bar(2).open, self.window.bar(2).close) > self.body_long_avg.val(2) && real_body(self.window.bar(1).open, self.window.bar(1).close) <= self.body_doji_avg.val(1) && real_body(self.window.bar(0).open, self.window.bar(0).close) > self.body_short_avg.val(0);
        let bull = base && candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1 && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1 && self.window.bar(1).high < self.window.bar(2).low && self.window.bar(0).low > self.window.bar(1).high && self.window.bar(0).close > self.window.bar(2).close + real_body(self.window.bar(2).open, self.window.bar(2).close) * penetration;
        let bear = base && candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1 && candle_color(self.window.bar(0).open, self.window.bar(0).close) == -1 && self.window.bar(1).low > self.window.bar(2).high && self.window.bar(0).high < self.window.bar(1).low && self.window.bar(0).close < self.window.bar(2).close - real_body(self.window.bar(2).open, self.window.bar(2).close) * penetration;
        if bull { return 100.0; }
        if bear { return -100.0; }
        0.0
    }
}

/// CDLADVANCEBLOCK
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLADVANCEBLOCK {
    bars_seen: usize,
    window: CandleWindow,
    far_avg: RollingCandleAvg,
    near_avg: RollingCandleAvg,
    shadow_long_avg: RollingCandleAvg,
}

impl CDLADVANCEBLOCK {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            far_avg: RollingCandleAvg::new(FAR),
            near_avg: RollingCandleAvg::new(NEAR),
            shadow_long_avg: RollingCandleAvg::new(SHADOW_LONG),
        }
    }
}

impl Default for CDLADVANCEBLOCK {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLADVANCEBLOCK {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.far_avg.push(open, high, low, close);
        self.near_avg.push(open, high, low, close);
        self.shadow_long_avg.push(open, high, low, close);

        if self.bars_seen <= SHADOW_LONG.avg_period.max(NEAR.avg_period).max(FAR.avg_period) + 2 {
            return 0.0;
        }

        let _base = candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1 && candle_color(self.window.bar(1).open, self.window.bar(1).close) == 1 && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1
            && self.window.bar(1).close > self.window.bar(2).close && self.window.bar(0).close > self.window.bar(1).close
            && self.window.bar(1).open > self.window.bar(2).open && self.window.bar(1).open <= self.window.bar(2).close + self.near_avg.val(1)
            && self.window.bar(0).open > self.window.bar(1).open && self.window.bar(0).open <= self.window.bar(1).close + self.near_avg.val(0)
            && real_body(self.window.bar(2).open, self.window.bar(2).close) > self.shadow_long_avg.val(2) * 0.0 // not used directly this way
            ;
        // The original logic checks short shadow and long body using near, far and shadow averages:
        let weakness = candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1 && candle_color(self.window.bar(1).open, self.window.bar(1).close) == 1 && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1
            && self.window.bar(1).close > self.window.bar(2).close && self.window.bar(0).close > self.window.bar(1).close
            && self.window.bar(1).open > self.window.bar(2).open && self.window.bar(1).open <= self.window.bar(2).close + self.near_avg.val(1)
            && self.window.bar(0).open > self.window.bar(1).open && self.window.bar(0).open <= self.window.bar(1).close + self.near_avg.val(0)
            && (
                (real_body(self.window.bar(1).open, self.window.bar(1).close) < real_body(self.window.bar(2).open, self.window.bar(2).close) - self.far_avg.val(1) && real_body(self.window.bar(0).open, self.window.bar(0).close) < real_body(self.window.bar(1).open, self.window.bar(1).close) + self.near_avg.val(0))
                || (real_body(self.window.bar(0).open, self.window.bar(0).close) < real_body(self.window.bar(1).open, self.window.bar(1).close) && real_body(self.window.bar(1).open, self.window.bar(1).close) < real_body(self.window.bar(2).open, self.window.bar(2).close) && (upper_shadow(self.window.bar(0).open, self.window.bar(0).high, self.window.bar(0).close) > self.shadow_long_avg.val(0) || upper_shadow(self.window.bar(1).open, self.window.bar(1).high, self.window.bar(1).close) > self.shadow_long_avg.val(1)))
                || (real_body(self.window.bar(0).open, self.window.bar(0).close) < real_body(self.window.bar(1).open, self.window.bar(1).close) - self.far_avg.val(0))
            );
        if weakness { return -100.0; }
        0.0
    }
}

/// CDLEVENINGDOJISTAR
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLEVENINGDOJISTAR {
    bars_seen: usize,
    window: CandleWindow,
    body_doji_avg: RollingCandleAvg,
    body_long_avg: RollingCandleAvg,
    body_short_avg: RollingCandleAvg,
}

impl CDLEVENINGDOJISTAR {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_doji_avg: RollingCandleAvg::new(BODY_DOJI),
            body_long_avg: RollingCandleAvg::new(BODY_LONG),
            body_short_avg: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}

impl Default for CDLEVENINGDOJISTAR {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLEVENINGDOJISTAR {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_doji_avg.push(open, high, low, close);
        self.body_long_avg.push(open, high, low, close);
        self.body_short_avg.push(open, high, low, close);

        if self.bars_seen <= BODY_DOJI.avg_period.max(BODY_LONG.avg_period).max(BODY_SHORT.avg_period) + 2 {
            return 0.0;
        }

        let penetration = 0.3;
        if candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1 && real_body(self.window.bar(2).open, self.window.bar(2).close) > self.body_long_avg.val(2)
            && real_body(self.window.bar(1).open, self.window.bar(1).close) <= self.body_doji_avg.val(1) && (self.window.bar(1).open.min(self.window.bar(1).close) > self.window.bar(2).open.max(self.window.bar(2).close))
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == -1 && real_body(self.window.bar(0).open, self.window.bar(0).close) > self.body_short_avg.val(0)
            && self.window.bar(0).close < self.window.bar(2).close - real_body(self.window.bar(2).open, self.window.bar(2).close) * penetration {
            return -100.0;
        }
        0.0
    }
}

/// CDLEVENINGSTAR
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLEVENINGSTAR {
    bars_seen: usize,
    window: CandleWindow,
    body_long_avg: RollingCandleAvg,
    body_short_avg: RollingCandleAvg,
}

impl CDLEVENINGSTAR {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_long_avg: RollingCandleAvg::new(BODY_LONG),
            body_short_avg: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}

impl Default for CDLEVENINGSTAR {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLEVENINGSTAR {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long_avg.push(open, high, low, close);
        self.body_short_avg.push(open, high, low, close);

        if self.bars_seen <= BODY_LONG.avg_period.max(BODY_SHORT.avg_period) + 2 {
            return 0.0;
        }

        let penetration = 0.3;
        if candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1 && real_body(self.window.bar(2).open, self.window.bar(2).close) > self.body_long_avg.val(2)
            && real_body(self.window.bar(1).open, self.window.bar(1).close) <= self.body_short_avg.val(1) && (self.window.bar(1).open.min(self.window.bar(1).close) > self.window.bar(2).open.max(self.window.bar(2).close))
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == -1 && real_body(self.window.bar(0).open, self.window.bar(0).close) > self.body_short_avg.val(0)
            && self.window.bar(0).close < self.window.bar(2).close - real_body(self.window.bar(2).open, self.window.bar(2).close) * penetration {
            return -100.0;
        }
        0.0
    }
}

/// CDLMORNINGDOJISTAR
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLMORNINGDOJISTAR {
    bars_seen: usize,
    window: CandleWindow,
    body_doji_avg: RollingCandleAvg,
    body_long_avg: RollingCandleAvg,
    body_short_avg: RollingCandleAvg,
}

impl CDLMORNINGDOJISTAR {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_doji_avg: RollingCandleAvg::new(BODY_DOJI),
            body_long_avg: RollingCandleAvg::new(BODY_LONG),
            body_short_avg: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}

impl Default for CDLMORNINGDOJISTAR {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLMORNINGDOJISTAR {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_doji_avg.push(open, high, low, close);
        self.body_long_avg.push(open, high, low, close);
        self.body_short_avg.push(open, high, low, close);

        if self.bars_seen <= BODY_DOJI.avg_period.max(BODY_LONG.avg_period).max(BODY_SHORT.avg_period) + 2 {
            return 0.0;
        }

        let penetration = 0.3;
        if candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1 && real_body(self.window.bar(2).open, self.window.bar(2).close) > self.body_long_avg.val(2)
            && real_body(self.window.bar(1).open, self.window.bar(1).close) <= self.body_doji_avg.val(1) && (self.window.bar(1).open.max(self.window.bar(1).close) < self.window.bar(2).open.min(self.window.bar(2).close))
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1 && real_body(self.window.bar(0).open, self.window.bar(0).close) > self.body_short_avg.val(0)
            && self.window.bar(0).close > self.window.bar(2).close + real_body(self.window.bar(2).open, self.window.bar(2).close) * penetration {
            return 100.0;
        }
        0.0
    }
}

/// CDLMORNINGSTAR
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLMORNINGSTAR {
    bars_seen: usize,
    window: CandleWindow,
    body_long_avg: RollingCandleAvg,
    body_short_avg: RollingCandleAvg,
}

impl CDLMORNINGSTAR {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_long_avg: RollingCandleAvg::new(BODY_LONG),
            body_short_avg: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}

impl Default for CDLMORNINGSTAR {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLMORNINGSTAR {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long_avg.push(open, high, low, close);
        self.body_short_avg.push(open, high, low, close);

        if self.bars_seen <= BODY_LONG.avg_period.max(BODY_SHORT.avg_period) + 2 {
            return 0.0;
        }

        let penetration = 0.3;
        if candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1 && real_body(self.window.bar(2).open, self.window.bar(2).close) > self.body_long_avg.val(2)
            && real_body(self.window.bar(1).open, self.window.bar(1).close) <= self.body_short_avg.val(1) && (self.window.bar(1).open.max(self.window.bar(1).close) < self.window.bar(2).open.min(self.window.bar(2).close))
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1 && real_body(self.window.bar(0).open, self.window.bar(0).close) > self.body_short_avg.val(0)
            && self.window.bar(0).close > self.window.bar(2).close + real_body(self.window.bar(2).open, self.window.bar(2).close) * penetration {
            return 100.0;
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(test_cdl3blackcrows_parity, CDL3BLACKCROWS, talib_rs::pattern::cdl_3blackcrows);
    crate::test_pattern_parity!(test_cdl3inside_parity, CDL3INSIDE, talib_rs::pattern::cdl_3inside);
    crate::test_pattern_parity!(test_cdl3outside_parity, CDL3OUTSIDE, talib_rs::pattern::cdl_3outside);
    crate::test_pattern_parity!(test_cdl3starsinsouth_parity, CDL3STARSINSOUTH, talib_rs::pattern::cdl_3starsinsouth);
    crate::test_pattern_parity!(test_cdl3whitesoldiers_parity, CDL3WHITESOLDIERS, talib_rs::pattern::cdl_3whitesoldiers);
    crate::test_pattern_parity!(test_cdlabandonedbaby_parity, CDLABANDONEDBABY, talib_rs::pattern::cdl_abandonedbaby);
    crate::test_pattern_parity!(test_cdladvanceblock_parity, CDLADVANCEBLOCK, talib_rs::pattern::cdl_advanceblock);
    crate::test_pattern_parity!(test_cdleveningdojistar_parity, CDLEVENINGDOJISTAR, talib_rs::pattern::cdl_eveningdojistar);
    crate::test_pattern_parity!(test_cdleveningstar_parity, CDLEVENINGSTAR, talib_rs::pattern::cdl_eveningstar);
    crate::test_pattern_parity!(test_cdlmorningdojistar_parity, CDLMORNINGDOJISTAR, talib_rs::pattern::cdl_morningdojistar);
    crate::test_pattern_parity!(test_cdlmorningstar_parity, CDLMORNINGSTAR, talib_rs::pattern::cdl_morningstar);
}
