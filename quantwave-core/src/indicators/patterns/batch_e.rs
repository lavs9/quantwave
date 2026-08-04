//! Native O(1) streaming implementations for Batch E patterns.
//! Matches `talib_rs::pattern::*`.

use crate::indicators::patterns::candle_settings::{
    BODY_DOJI, BODY_LONG, BODY_SHORT, CandleWindow, EQUAL, NEAR, RollingCandleAvg,
    SHADOW_VERY_SHORT, candle_color, lower_shadow, real_body, upper_shadow,
};
use crate::traits::Next;

#[inline(always)]
fn real_body_gap_up(window: &CandleWindow, i: usize, j: usize) -> bool {
    window.bar(i).open.min(window.bar(i).close) > window.bar(j).open.max(window.bar(j).close)
}

#[inline(always)]
fn real_body_gap_down(window: &CandleWindow, i: usize, j: usize) -> bool {
    window.bar(i).open.max(window.bar(i).close) < window.bar(j).open.min(window.bar(j).close)
}

const CDLGAPSIDESIDEWHITE_LOOKBACK: usize = 5 + 2; // NEAR(5) + 2

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLGAPSIDESIDEWHITE {
    bars_seen: usize,
    window: CandleWindow,
    near_avg: RollingCandleAvg,
}
impl CDLGAPSIDESIDEWHITE {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            near_avg: RollingCandleAvg::new(NEAR),
        }
    }
}
impl Default for CDLGAPSIDESIDEWHITE {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLGAPSIDESIDEWHITE {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.near_avg.push(open, high, low, close);

        if self.bars_seen <= CDLGAPSIDESIDEWHITE_LOOKBACK {
            return 0.0;
        }

        let c1 = candle_color(self.window.bar(1).open, self.window.bar(1).close);
        let c0 = candle_color(self.window.bar(0).open, self.window.bar(0).close);

        let near_same = (real_body(self.window.bar(1).open, self.window.bar(1).close)
            - real_body(self.window.bar(0).open, self.window.bar(0).close))
        .abs()
            < self.near_avg.val(1);

        let bull = real_body_gap_up(&self.window, 1, 2)
            && c1 == 1
            && c0 == 1
            && self.window.bar(0).open < self.window.bar(1).close
            && self.window.bar(0).open > self.window.bar(1).open
            && self.window.bar(0).close > self.window.bar(1).close
            && near_same;

        let bear = real_body_gap_down(&self.window, 1, 2)
            && c1 == -1
            && c0 == -1
            && self.window.bar(0).open > self.window.bar(1).close
            && self.window.bar(0).open < self.window.bar(1).open
            && self.window.bar(0).close < self.window.bar(1).close
            && near_same;

        if bull || bear {
            (candle_color(self.window.bar(2).open, self.window.bar(2).close) * 100) as f64
        } else {
            0.0
        }
    }
}

const CDLIDENTICAL3CROWS_LOOKBACK: usize = 10 + 2; // SHADOW_VERY_SHORT(10), EQUAL(5) + 2 = 12

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLIDENTICAL3CROWS {
    bars_seen: usize,
    window: CandleWindow,
    shadow_avg: RollingCandleAvg,
    equal_avg: RollingCandleAvg,
}
impl CDLIDENTICAL3CROWS {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            shadow_avg: RollingCandleAvg::new(SHADOW_VERY_SHORT),
            equal_avg: RollingCandleAvg::new(EQUAL),
        }
    }
}
impl Default for CDLIDENTICAL3CROWS {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLIDENTICAL3CROWS {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.shadow_avg.push(open, high, low, close);
        self.equal_avg.push(open, high, low, close);

        if self.bars_seen <= CDLIDENTICAL3CROWS_LOOKBACK {
            return 0.0;
        }

        let cond = candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1
            && candle_color(self.window.bar(1).open, self.window.bar(1).close) == -1
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == -1
            && self.window.bar(1).close < self.window.bar(2).close
            && self.window.bar(0).close < self.window.bar(1).close
            && lower_shadow(
                self.window.bar(2).open,
                self.window.bar(2).low,
                self.window.bar(2).close,
            ) < self.shadow_avg.val(2)
            && lower_shadow(
                self.window.bar(1).open,
                self.window.bar(1).low,
                self.window.bar(1).close,
            ) < self.shadow_avg.val(1)
            && lower_shadow(
                self.window.bar(0).open,
                self.window.bar(0).low,
                self.window.bar(0).close,
            ) < self.shadow_avg.val(0)
            && (self.window.bar(1).open - self.window.bar(2).close).abs() <= self.equal_avg.val(2)
            && (self.window.bar(0).open - self.window.bar(1).close).abs() <= self.equal_avg.val(1);

        if cond { -100.0 } else { 0.0 }
    }
}

const CDLSTALLEDPATTERN_LOOKBACK: usize = 10 + 2;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLSTALLEDPATTERN {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    body_short: RollingCandleAvg,
    shadow_vs: RollingCandleAvg,
    near: RollingCandleAvg,
}
impl CDLSTALLEDPATTERN {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_long: RollingCandleAvg::new(BODY_LONG),
            body_short: RollingCandleAvg::new(BODY_SHORT),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
            near: RollingCandleAvg::new(NEAR),
        }
    }
}
impl Default for CDLSTALLEDPATTERN {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLSTALLEDPATTERN {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.body_short.push(open, high, low, close);
        self.shadow_vs.push(open, high, low, close);
        self.near.push(open, high, low, close);

        if self.bars_seen <= CDLSTALLEDPATTERN_LOOKBACK {
            return 0.0;
        }

        let cond = candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1
            && candle_color(self.window.bar(1).open, self.window.bar(1).close) == 1
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1
            && self.window.bar(1).close > self.window.bar(2).close
            && self.window.bar(0).close > self.window.bar(1).close
            && real_body(self.window.bar(2).open, self.window.bar(2).close) > self.body_long.val(2)
            && real_body(self.window.bar(1).open, self.window.bar(1).close) > self.body_long.val(1)
            && upper_shadow(
                self.window.bar(1).open,
                self.window.bar(1).high,
                self.window.bar(1).close,
            ) < self.shadow_vs.val(1)
            && self.window.bar(1).open > self.window.bar(2).open
            && self.window.bar(1).open <= self.window.bar(2).close + self.near.val(2)
            && real_body(self.window.bar(0).open, self.window.bar(0).close)
                < self.body_short.val(0)
            && self.window.bar(0).open
                >= self.window.bar(1).close
                    - real_body(self.window.bar(0).open, self.window.bar(0).close)
                    - self.near.val(1);

        if cond { -100.0 } else { 0.0 }
    }
}

const CDLTASUKIGAP_LOOKBACK: usize = 5 + 2;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLTASUKIGAP {
    bars_seen: usize,
    window: CandleWindow,
    near: RollingCandleAvg,
}
impl CDLTASUKIGAP {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            near: RollingCandleAvg::new(NEAR),
        }
    }
}
impl Default for CDLTASUKIGAP {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLTASUKIGAP {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.near.push(open, high, low, close);

        if self.bars_seen <= CDLTASUKIGAP_LOOKBACK {
            return 0.0;
        }

        let c1 = candle_color(self.window.bar(1).open, self.window.bar(1).close);
        let c0 = candle_color(self.window.bar(0).open, self.window.bar(0).close);

        let near_same = (real_body(self.window.bar(1).open, self.window.bar(1).close)
            - real_body(self.window.bar(0).open, self.window.bar(0).close))
        .abs()
            < self.near.val(1);

        let bull = real_body_gap_up(&self.window, 1, 2)
            && c1 == 1
            && c0 == -1
            && self.window.bar(0).open < self.window.bar(1).close
            && self.window.bar(0).open > self.window.bar(1).open
            && self.window.bar(0).close < self.window.bar(1).open
            && self.window.bar(0).close > self.window.bar(2).open.max(self.window.bar(2).close)
            && near_same;

        let bear = real_body_gap_down(&self.window, 1, 2)
            && c1 == -1
            && c0 == 1
            && self.window.bar(0).open < self.window.bar(1).open
            && self.window.bar(0).open > self.window.bar(1).close
            && self.window.bar(0).close > self.window.bar(1).open
            && self.window.bar(0).close < self.window.bar(2).open.min(self.window.bar(2).close)
            && near_same;

        if bull || bear { (c1 * 100) as f64 } else { 0.0 }
    }
}

const CDLTRISTAR_LOOKBACK: usize = 10 + 2;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLTRISTAR {
    bars_seen: usize,
    window: CandleWindow,
    body_doji: RollingCandleAvg,
}
impl CDLTRISTAR {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_doji: RollingCandleAvg::new(BODY_DOJI),
        }
    }
}
impl Default for CDLTRISTAR {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLTRISTAR {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_doji.push(open, high, low, close);

        if self.bars_seen <= CDLTRISTAR_LOOKBACK {
            return 0.0;
        }

        let base = real_body(self.window.bar(2).open, self.window.bar(2).close)
            <= self.body_doji.val(2)
            && real_body(self.window.bar(1).open, self.window.bar(1).close)
                <= self.body_doji.val(1)
            && real_body(self.window.bar(0).open, self.window.bar(0).close)
                <= self.body_doji.val(0);

        let bear =
            base && real_body_gap_up(&self.window, 1, 2) && !real_body_gap_up(&self.window, 0, 1);
        let bull = base
            && real_body_gap_down(&self.window, 1, 2)
            && !real_body_gap_down(&self.window, 0, 1);

        (bull as i32 * 100 - bear as i32 * 100) as f64
    }
}

const CDLUNIQUE3RIVER_LOOKBACK: usize = 10 + 2;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLUNIQUE3RIVER {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    body_short: RollingCandleAvg,
}
impl CDLUNIQUE3RIVER {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_long: RollingCandleAvg::new(BODY_LONG),
            body_short: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}
impl Default for CDLUNIQUE3RIVER {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLUNIQUE3RIVER {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.body_short.push(open, high, low, close);

        if self.bars_seen <= CDLUNIQUE3RIVER_LOOKBACK {
            return 0.0;
        }

        let cond = candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1
            && real_body(self.window.bar(2).open, self.window.bar(2).close) > self.body_long.val(2)
            && candle_color(self.window.bar(1).open, self.window.bar(1).close) == -1
            && self.window.bar(1).open.max(self.window.bar(1).close)
                < self.window.bar(2).open.max(self.window.bar(2).close)
            && self.window.bar(1).open.min(self.window.bar(1).close)
                > self.window.bar(2).open.min(self.window.bar(2).close)
            && self.window.bar(1).low < self.window.bar(2).low
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1
            && real_body(self.window.bar(0).open, self.window.bar(0).close)
                < self.body_short.val(0)
            && self.window.bar(0).close < self.window.bar(1).close;

        if cond { 100.0 } else { 0.0 }
    }
}

const CDLUPSIDEGAP2CROWS_LOOKBACK: usize = 10 + 2;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLUPSIDEGAP2CROWS {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    body_short: RollingCandleAvg,
}
impl CDLUPSIDEGAP2CROWS {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_long: RollingCandleAvg::new(BODY_LONG),
            body_short: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}
impl Default for CDLUPSIDEGAP2CROWS {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLUPSIDEGAP2CROWS {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.body_short.push(open, high, low, close);

        if self.bars_seen <= CDLUPSIDEGAP2CROWS_LOOKBACK {
            return 0.0;
        }

        let cond = candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1
            && real_body(self.window.bar(2).open, self.window.bar(2).close) > self.body_long.val(2)
            && candle_color(self.window.bar(1).open, self.window.bar(1).close) == -1
            && real_body(self.window.bar(1).open, self.window.bar(1).close)
                <= self.body_short.val(1)
            && real_body_gap_up(&self.window, 1, 2)
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == -1
            && self.window.bar(0).open > self.window.bar(1).open
            && self.window.bar(0).close < self.window.bar(1).close
            && self.window.bar(0).close > self.window.bar(2).close;

        if cond { -100.0 } else { 0.0 }
    }
}

const CDLXSIDEGAP3METHODS_LOOKBACK: usize = 2;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLXSIDEGAP3METHODS {
    bars_seen: usize,
    window: CandleWindow,
}
impl CDLXSIDEGAP3METHODS {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
        }
    }
}
impl Default for CDLXSIDEGAP3METHODS {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLXSIDEGAP3METHODS {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);

        if self.bars_seen <= CDLXSIDEGAP3METHODS_LOOKBACK {
            return 0.0;
        }

        let c2 = candle_color(self.window.bar(2).open, self.window.bar(2).close);
        let c1 = candle_color(self.window.bar(1).open, self.window.bar(1).close);
        let c0 = candle_color(self.window.bar(0).open, self.window.bar(0).close);

        let opens_within = self.window.bar(0).open
            > self.window.bar(1).open.min(self.window.bar(1).close)
            && self.window.bar(0).open < self.window.bar(1).open.max(self.window.bar(1).close);
        let closes_within = self.window.bar(0).close
            > self.window.bar(2).open.min(self.window.bar(2).close)
            && self.window.bar(0).close < self.window.bar(2).open.max(self.window.bar(2).close);

        let base = c2 == c1 && c0 != c2 && opens_within && closes_within;
        let bull = base && c2 == 1 && real_body_gap_up(&self.window, 1, 2);
        let bear = base && c2 == -1 && real_body_gap_down(&self.window, 1, 2);

        (bull as i32 * 100 - bear as i32 * 100) as f64
    }
}

const CDLBREAKAWAY_LOOKBACK: usize = 10 + 4;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLBREAKAWAY {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    body_short: RollingCandleAvg,
}
impl CDLBREAKAWAY {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(5),
            body_long: RollingCandleAvg::new(BODY_LONG),
            body_short: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}
impl Default for CDLBREAKAWAY {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLBREAKAWAY {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.body_short.push(open, high, low, close);

        if self.bars_seen <= CDLBREAKAWAY_LOOKBACK {
            return 0.0;
        }

        let c4 = candle_color(self.window.bar(4).open, self.window.bar(4).close);
        let c0 = candle_color(self.window.bar(0).open, self.window.bar(0).close);

        if real_body(self.window.bar(4).open, self.window.bar(4).close) > self.body_long.val(4)
            && real_body(self.window.bar(0).open, self.window.bar(0).close) > self.body_long.val(0)
        {
            let bull = c4 == -1
                && candle_color(self.window.bar(3).open, self.window.bar(3).close) == -1
                && real_body_gap_down(&self.window, 3, 4)
                && candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1
                && self.window.bar(2).close < self.window.bar(3).close
                && candle_color(self.window.bar(1).open, self.window.bar(1).close) == -1
                && self.window.bar(1).close < self.window.bar(2).close
                && c0 == 1
                && self.window.bar(0).close > self.window.bar(3).open.max(self.window.bar(3).close)
                && self.window.bar(0).close < self.window.bar(4).open.min(self.window.bar(4).close);

            let bear = c4 == 1
                && candle_color(self.window.bar(3).open, self.window.bar(3).close) == 1
                && real_body_gap_up(&self.window, 3, 4)
                && candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1
                && self.window.bar(2).close > self.window.bar(3).close
                && candle_color(self.window.bar(1).open, self.window.bar(1).close) == 1
                && self.window.bar(1).close > self.window.bar(2).close
                && c0 == -1
                && self.window.bar(0).close < self.window.bar(3).open.min(self.window.bar(3).close)
                && self.window.bar(0).close > self.window.bar(4).open.max(self.window.bar(4).close);

            (bull as i32 * 100 - bear as i32 * 100) as f64
        } else {
            0.0
        }
    }
}

const CDLCONCEALBABYSWALL_LOOKBACK: usize = 10 + 3;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLCONCEALBABYSWALL {
    bars_seen: usize,
    window: CandleWindow,
    shadow_vs: RollingCandleAvg,
}
impl CDLCONCEALBABYSWALL {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(4),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
        }
    }
}
impl Default for CDLCONCEALBABYSWALL {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLCONCEALBABYSWALL {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.shadow_vs.push(open, high, low, close);

        if self.bars_seen <= CDLCONCEALBABYSWALL_LOOKBACK {
            return 0.0;
        }

        let cond = candle_color(self.window.bar(3).open, self.window.bar(3).close) == -1
            && candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1
            && candle_color(self.window.bar(1).open, self.window.bar(1).close) == -1
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == -1
            && lower_shadow(
                self.window.bar(3).open,
                self.window.bar(3).low,
                self.window.bar(3).close,
            ) == 0.0
            && upper_shadow(
                self.window.bar(3).open,
                self.window.bar(3).high,
                self.window.bar(3).close,
            ) == 0.0
            && lower_shadow(
                self.window.bar(2).open,
                self.window.bar(2).low,
                self.window.bar(2).close,
            ) == 0.0
            && upper_shadow(
                self.window.bar(2).open,
                self.window.bar(2).high,
                self.window.bar(2).close,
            ) == 0.0
            && real_body_gap_down(&self.window, 2, 3)
            && lower_shadow(
                self.window.bar(1).open,
                self.window.bar(1).low,
                self.window.bar(1).close,
            ) > self.shadow_vs.val(1)
            && self.window.bar(1).high > self.window.bar(2).close
            && self.window.bar(0).open > self.window.bar(1).high
            && self.window.bar(0).close < self.window.bar(1).low;

        if cond { 100.0 } else { 0.0 }
    }
}

const CDLLADDERBOTTOM_LOOKBACK: usize = 10 + 4;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLLADDERBOTTOM {
    bars_seen: usize,
    window: CandleWindow,
    shadow_vs: RollingCandleAvg,
}
impl CDLLADDERBOTTOM {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(5),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
        }
    }
}
impl Default for CDLLADDERBOTTOM {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLLADDERBOTTOM {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.shadow_vs.push(open, high, low, close);

        if self.bars_seen <= CDLLADDERBOTTOM_LOOKBACK {
            return 0.0;
        }

        let cond = candle_color(self.window.bar(4).open, self.window.bar(4).close) == -1
            && candle_color(self.window.bar(3).open, self.window.bar(3).close) == -1
            && candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1
            && candle_color(self.window.bar(1).open, self.window.bar(1).close) == -1
            && self.window.bar(3).close < self.window.bar(4).close
            && self.window.bar(2).close < self.window.bar(3).close
            && upper_shadow(
                self.window.bar(1).open,
                self.window.bar(1).high,
                self.window.bar(1).close,
            ) > self.shadow_vs.val(1)
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1
            && self.window.bar(0).open > self.window.bar(1).open
            && self.window.bar(0).close > self.window.bar(1).high;

        if cond { 100.0 } else { 0.0 }
    }
}

const CDLMATHOLD_LOOKBACK: usize = 10 + 4;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLMATHOLD {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    body_short: RollingCandleAvg,
}
impl CDLMATHOLD {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(5),
            body_long: RollingCandleAvg::new(BODY_LONG),
            body_short: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}
impl Default for CDLMATHOLD {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLMATHOLD {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.body_short.push(open, high, low, close);

        if self.bars_seen <= CDLMATHOLD_LOOKBACK {
            return 0.0;
        }

        let penetration = 0.5;
        let cond = real_body(self.window.bar(4).open, self.window.bar(4).close)
            > self.body_long.val(4)
            && real_body(self.window.bar(3).open, self.window.bar(3).close)
                < self.body_short.val(3)
            && real_body(self.window.bar(2).open, self.window.bar(2).close)
                < self.body_short.val(2)
            && real_body(self.window.bar(1).open, self.window.bar(1).close)
                < self.body_short.val(1)
            && candle_color(self.window.bar(4).open, self.window.bar(4).close) == 1
            && candle_color(self.window.bar(3).open, self.window.bar(3).close) == -1
            && candle_color(self.window.bar(0).open, self.window.bar(0).close) == 1
            && real_body_gap_up(&self.window, 3, 4)
            && self.window.bar(2).open.min(self.window.bar(2).close) < self.window.bar(4).close
            && self.window.bar(1).open.min(self.window.bar(1).close) < self.window.bar(4).close
            && self.window.bar(2).open.min(self.window.bar(2).close)
                > self.window.bar(4).close
                    - real_body(self.window.bar(4).open, self.window.bar(4).close) * penetration
            && self.window.bar(1).open.min(self.window.bar(1).close)
                > self.window.bar(4).close
                    - real_body(self.window.bar(4).open, self.window.bar(4).close) * penetration
            && self.window.bar(2).open.max(self.window.bar(2).close) < self.window.bar(3).open
            && self.window.bar(1).open.max(self.window.bar(1).close)
                < self.window.bar(2).open.max(self.window.bar(2).close)
            && self.window.bar(0).open > self.window.bar(1).close
            && self.window.bar(0).close
                > self
                    .window
                    .bar(3)
                    .high
                    .max(self.window.bar(2).high)
                    .max(self.window.bar(1).high);

        if cond { 100.0 } else { 0.0 }
    }
}

const CDLRISEFALL3METHODS_LOOKBACK: usize = 10 + 4;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLRISEFALL3METHODS {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    body_short: RollingCandleAvg,
}
impl CDLRISEFALL3METHODS {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(5),
            body_long: RollingCandleAvg::new(BODY_LONG),
            body_short: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}
impl Default for CDLRISEFALL3METHODS {
    fn default() -> Self {
        Self::new()
    }
}
impl Next<(f64, f64, f64, f64)> for CDLRISEFALL3METHODS {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.body_short.push(open, high, low, close);

        if self.bars_seen <= CDLRISEFALL3METHODS_LOOKBACK {
            return 0.0;
        }

        let c4 = candle_color(self.window.bar(4).open, self.window.bar(4).close);
        let c0 = candle_color(self.window.bar(0).open, self.window.bar(0).close);

        if real_body(self.window.bar(4).open, self.window.bar(4).close) > self.body_long.val(4)
            && real_body(self.window.bar(0).open, self.window.bar(0).close) > self.body_long.val(0)
        {
            let mid_short = real_body(self.window.bar(3).open, self.window.bar(3).close)
                < self.body_short.val(3)
                && real_body(self.window.bar(2).open, self.window.bar(2).close)
                    < self.body_short.val(2)
                && real_body(self.window.bar(1).open, self.window.bar(1).close)
                    < self.body_short.val(1);

            let bull = c4 == 1
                && mid_short
                && candle_color(self.window.bar(3).open, self.window.bar(3).close) == -1
                && candle_color(self.window.bar(2).open, self.window.bar(2).close) == -1
                && candle_color(self.window.bar(1).open, self.window.bar(1).close) == -1
                && self.window.bar(3).close < self.window.bar(4).close
                && self.window.bar(2).close < self.window.bar(3).close
                && self.window.bar(1).close < self.window.bar(2).close
                && self.window.bar(3).low > self.window.bar(4).low
                && self.window.bar(2).low > self.window.bar(4).low
                && self.window.bar(1).low > self.window.bar(4).low
                && self.window.bar(3).high < self.window.bar(4).high
                && self.window.bar(2).high < self.window.bar(4).high
                && self.window.bar(1).high < self.window.bar(4).high
                && c0 == 1
                && self.window.bar(0).open > self.window.bar(1).close
                && self.window.bar(0).close > self.window.bar(4).close;

            let bear = c4 == -1
                && mid_short
                && candle_color(self.window.bar(3).open, self.window.bar(3).close) == 1
                && candle_color(self.window.bar(2).open, self.window.bar(2).close) == 1
                && candle_color(self.window.bar(1).open, self.window.bar(1).close) == 1
                && self.window.bar(3).close > self.window.bar(4).close
                && self.window.bar(2).close > self.window.bar(3).close
                && self.window.bar(1).close > self.window.bar(2).close
                && self.window.bar(3).high < self.window.bar(4).high
                && self.window.bar(2).high < self.window.bar(4).high
                && self.window.bar(1).high < self.window.bar(4).high
                && self.window.bar(3).low > self.window.bar(4).low
                && self.window.bar(2).low > self.window.bar(4).low
                && self.window.bar(1).low > self.window.bar(4).low
                && c0 == -1
                && self.window.bar(0).open < self.window.bar(1).close
                && self.window.bar(0).close < self.window.bar(4).close;

            (bull as i32 * 100 - bear as i32 * 100) as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    crate::test_pattern_parity!(
        test_cdlgapsidesidewhite,
        CDLGAPSIDESIDEWHITE,
        talib_rs::pattern::cdl_gapsidesidewhite
    );
    crate::test_pattern_parity!(
        test_cdlidentical3crows,
        CDLIDENTICAL3CROWS,
        talib_rs::pattern::cdl_identical3crows
    );
    crate::test_pattern_parity!(
        test_cdlstalledpattern,
        CDLSTALLEDPATTERN,
        talib_rs::pattern::cdl_stalledpattern
    );
    crate::test_pattern_parity!(
        test_cdltasukigap,
        CDLTASUKIGAP,
        talib_rs::pattern::cdl_tasukigap
    );
    crate::test_pattern_parity!(test_cdltristar, CDLTRISTAR, talib_rs::pattern::cdl_tristar);
    crate::test_pattern_parity!(
        test_cdlunique3river,
        CDLUNIQUE3RIVER,
        talib_rs::pattern::cdl_unique3river
    );
    crate::test_pattern_parity!(
        test_cdlupsidegap2crows,
        CDLUPSIDEGAP2CROWS,
        talib_rs::pattern::cdl_upsidegap2crows
    );
    crate::test_pattern_parity!(
        test_cdlxsidegap3methods,
        CDLXSIDEGAP3METHODS,
        talib_rs::pattern::cdl_xsidegap3methods
    );
    crate::test_pattern_parity!(
        test_cdlbreakaway,
        CDLBREAKAWAY,
        talib_rs::pattern::cdl_breakaway
    );
    crate::test_pattern_parity!(
        test_cdlconcealbabyswall,
        CDLCONCEALBABYSWALL,
        talib_rs::pattern::cdl_concealbabyswall
    );
    crate::test_pattern_parity!(
        test_cdlladderbottom,
        CDLLADDERBOTTOM,
        talib_rs::pattern::cdl_ladderbottom
    );
    crate::test_pattern_parity!(test_cdlmathold, CDLMATHOLD, talib_rs::pattern::cdl_mathold);
    crate::test_pattern_parity!(
        test_cdlrisefall3methods,
        CDLRISEFALL3METHODS,
        talib_rs::pattern::cdl_risefall3methods
    );
}
