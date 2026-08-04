//! Native O(1) streaming patterns for Batch C
//! (CDLPIERCING, CDLSEPARATINGLINES, CDLSHOOTINGSTAR, CDLTHRUSTING, CDLKICKING, CDLKICKINGBYLENGTH, CDLHIKKAKE, CDLHIKKAKEMOD, CDLSTICKSANDWICH, CDL2CROWS)
//! Matches `talib_rs::pattern::*`.

use crate::indicators::patterns::candle_settings::*;
use crate::traits::Next;

// =============================================================================
// CDLPIERCING
// =============================================================================
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLPIERCING {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
}

impl CDLPIERCING {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            body_long: RollingCandleAvg::new(BODY_LONG),
        }
    }
}

impl Default for CDLPIERCING {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLPIERCING {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);

        let lookback = BODY_LONG.avg_period + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let b1 = self.window.bar(1);
        let b0 = self.window.bar(0);

        let is_piercing = candle_color(b1.open, b1.close) == -1
            && real_body(b1.open, b1.close) > self.body_long.val(1)
            && candle_color(b0.open, b0.close) == 1
            && real_body(b0.open, b0.close) > self.body_long.val(0)
            && b0.open < b1.low
            && b0.close < b1.open
            && b0.close > b1.close + real_body(b1.open, b1.close) * 0.5;

        if is_piercing { 100.0 } else { 0.0 }
    }
}

// =============================================================================
// CDLSEPARATINGLINES
// =============================================================================
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLSEPARATINGLINES {
    bars_seen: usize,
    window: CandleWindow,
    shadow_vs: RollingCandleAvg,
    body_long: RollingCandleAvg,
    equal: RollingCandleAvg,
}

impl CDLSEPARATINGLINES {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
            body_long: RollingCandleAvg::new(BODY_LONG),
            equal: RollingCandleAvg::new(EQUAL),
        }
    }
}

impl Default for CDLSEPARATINGLINES {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLSEPARATINGLINES {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.shadow_vs.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.equal.push(open, high, low, close);

        let lookback = SHADOW_VERY_SHORT
            .avg_period
            .max(BODY_LONG.avg_period)
            .max(EQUAL.avg_period)
            + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let b1 = self.window.bar(1);
        let b0 = self.window.bar(0);

        let color_prev = candle_color(b1.open, b1.close);
        let color_curr = candle_color(b0.open, b0.close);

        let base = color_prev != color_curr
            && (b0.open - b1.open).abs() <= self.equal.val(1)
            && real_body(b0.open, b0.close) > self.body_long.val(0);

        let bull = base
            && color_curr == 1
            && lower_shadow(b0.open, b0.low, b0.close) < self.shadow_vs.val(0);
        let bear = base
            && color_curr == -1
            && upper_shadow(b0.open, b0.high, b0.close) < self.shadow_vs.val(0);

        if bull {
            100.0
        } else if bear {
            -100.0
        } else {
            0.0
        }
    }
}

// =============================================================================
// CDLSHOOTINGSTAR
// =============================================================================
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLSHOOTINGSTAR {
    bars_seen: usize,
    window: CandleWindow,
    body_short: RollingCandleAvg,
    shadow_long: RollingCandleAvg,
    shadow_vs: RollingCandleAvg,
}

impl CDLSHOOTINGSTAR {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            body_short: RollingCandleAvg::new(BODY_SHORT),
            shadow_long: RollingCandleAvg::new(SHADOW_LONG),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
        }
    }
}

impl Default for CDLSHOOTINGSTAR {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLSHOOTINGSTAR {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_short.push(open, high, low, close);
        self.shadow_long.push(open, high, low, close);
        self.shadow_vs.push(open, high, low, close);

        // SHADOW_LONG.avg_period is 0 and cannot raise the max.
        let lookback = BODY_SHORT.avg_period.max(SHADOW_VERY_SHORT.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let b1 = self.window.bar(1);
        let b0 = self.window.bar(0);

        let gap_up = b0.open.min(b0.close) > b1.open.max(b1.close);

        let is_shooting_star = real_body(b0.open, b0.close) < self.body_short.val(0)
            && upper_shadow(b0.open, b0.high, b0.close) > self.shadow_long.val(0)
            && lower_shadow(b0.open, b0.low, b0.close) < self.shadow_vs.val(0)
            && gap_up;

        if is_shooting_star { -100.0 } else { 0.0 }
    }
}

// =============================================================================
// CDLTHRUSTING
// =============================================================================
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLTHRUSTING {
    bars_seen: usize,
    window: CandleWindow,
    equal: RollingCandleAvg,
    body_long: RollingCandleAvg,
}

impl CDLTHRUSTING {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            equal: RollingCandleAvg::new(EQUAL),
            body_long: RollingCandleAvg::new(BODY_LONG),
        }
    }
}

impl Default for CDLTHRUSTING {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLTHRUSTING {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.equal.push(open, high, low, close);
        self.body_long.push(open, high, low, close);

        let lookback = EQUAL.avg_period.max(BODY_LONG.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let b1 = self.window.bar(1);
        let b0 = self.window.bar(0);

        let is_thrusting = candle_color(b1.open, b1.close) == -1
            && real_body(b1.open, b1.close) > self.body_long.val(1)
            && candle_color(b0.open, b0.close) == 1
            && b0.open < b1.low
            && b0.close > b1.close + self.equal.val(1)
            && b0.close <= b1.close + real_body(b1.open, b1.close) * 0.5;

        if is_thrusting { -100.0 } else { 0.0 }
    }
}

// =============================================================================
// CDLKICKING
// =============================================================================
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLKICKING {
    bars_seen: usize,
    window: CandleWindow,
    shadow_vs: RollingCandleAvg,
    body_long: RollingCandleAvg,
}

impl CDLKICKING {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
            body_long: RollingCandleAvg::new(BODY_LONG),
        }
    }
}

impl Default for CDLKICKING {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLKICKING {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.shadow_vs.push(open, high, low, close);
        self.body_long.push(open, high, low, close);

        let lookback = SHADOW_VERY_SHORT.avg_period.max(BODY_LONG.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let b1 = self.window.bar(1);
        let b0 = self.window.bar(0);

        let color_prev = candle_color(b1.open, b1.close);
        let color_curr = candle_color(b0.open, b0.close);

        if color_prev != color_curr
            && real_body(b1.open, b1.close) > self.body_long.val(1)
            && upper_shadow(b1.open, b1.high, b1.close) < self.shadow_vs.val(1)
            && lower_shadow(b1.open, b1.low, b1.close) < self.shadow_vs.val(1)
            && real_body(b0.open, b0.close) > self.body_long.val(0)
            && upper_shadow(b0.open, b0.high, b0.close) < self.shadow_vs.val(0)
            && lower_shadow(b0.open, b0.low, b0.close) < self.shadow_vs.val(0)
        {
            let bull = color_prev == -1 && color_curr == 1 && b0.open > b1.open;
            let bear = color_prev == 1 && color_curr == -1 && b0.open < b1.open;
            if bull {
                100.0
            } else if bear {
                -100.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

// =============================================================================
// CDLKICKINGBYLENGTH
// =============================================================================
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLKICKINGBYLENGTH {
    bars_seen: usize,
    window: CandleWindow,
    shadow_vs: RollingCandleAvg,
    body_long: RollingCandleAvg,
}

impl CDLKICKINGBYLENGTH {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            shadow_vs: RollingCandleAvg::new(SHADOW_VERY_SHORT),
            body_long: RollingCandleAvg::new(BODY_LONG),
        }
    }
}

impl Default for CDLKICKINGBYLENGTH {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLKICKINGBYLENGTH {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.shadow_vs.push(open, high, low, close);
        self.body_long.push(open, high, low, close);

        let lookback = SHADOW_VERY_SHORT.avg_period.max(BODY_LONG.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let b1 = self.window.bar(1);
        let b0 = self.window.bar(0);

        let color_prev = candle_color(b1.open, b1.close);
        let color_curr = candle_color(b0.open, b0.close);

        if color_prev != color_curr
            && real_body(b1.open, b1.close) > self.body_long.val(1)
            && upper_shadow(b1.open, b1.high, b1.close) < self.shadow_vs.val(1)
            && lower_shadow(b1.open, b1.low, b1.close) < self.shadow_vs.val(1)
            && real_body(b0.open, b0.close) > self.body_long.val(0)
            && upper_shadow(b0.open, b0.high, b0.close) < self.shadow_vs.val(0)
            && lower_shadow(b0.open, b0.low, b0.close) < self.shadow_vs.val(0)
        {
            let has_gap = (color_prev == -1 && color_curr == 1 && b0.open > b1.open)
                || (color_prev == 1 && color_curr == -1 && b0.open < b1.open);
            let curr_longer = real_body(b0.open, b0.close) >= real_body(b1.open, b1.close);
            let color = if curr_longer { color_curr } else { color_prev };

            if has_gap { (color as f64) * 100.0 } else { 0.0 }
        } else {
            0.0
        }
    }
}

// =============================================================================
// CDLHIKKAKE
// =============================================================================
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLHIKKAKE {
    bars_seen: usize,
    window: CandleWindow,
    /// Bars elapsed since the setup bar; `None` once confirmed or expired.
    pattern_age: Option<usize>,
    pattern_result: i32,
    /// High/low of the inside bar (`pattern_idx - 1` in the batch reference),
    /// captured at setup so no backwards indexing is needed while streaming.
    ref_high: f64,
    ref_low: f64,
}

impl CDLHIKKAKE {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            pattern_age: None,
            pattern_result: 0,
            ref_high: 0.0,
            ref_low: 0.0,
        }
    }

    /// Breakout confirmation against the stored inside-bar levels.
    fn confirm(&mut self, close: f64) -> f64 {
        if let Some(age) = self.pattern_age
            && (1..=3).contains(&age)
        {
            {
                if self.pattern_result > 0 && close > self.ref_high {
                    self.pattern_age = None;
                    return (self.pattern_result + 100) as f64;
                } else if self.pattern_result < 0 && close < self.ref_low {
                    self.pattern_age = None;
                    return (self.pattern_result - 100) as f64;
                }
            }
        }
        0.0
    }
}

impl Default for CDLHIKKAKE {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLHIKKAKE {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        const LOOKBACK: usize = 5;
        self.bars_seen += 1;
        self.window.push(open, high, low, close);

        if let Some(age) = self.pattern_age {
            self.pattern_age = Some(age + 1);
        }

        if self.window.len() < 3 {
            return 0.0;
        }

        let b0 = self.window.bar(0);
        let b1 = self.window.bar(1);
        let b2 = self.window.bar(2);

        // The batch reference pre-scans bars before `start`: it updates the setup
        // state but never emits and never runs confirmation during that pre-scan.
        let emitting = self.bars_seen > LOOKBACK;

        let inside = b1.high < b2.high && b1.low > b2.low;
        if inside && b0.high < b1.high && b0.low < b1.low {
            self.pattern_result = 100;
            self.pattern_age = Some(0);
            self.ref_high = b1.high;
            self.ref_low = b1.low;
            return if emitting { 100.0 } else { 0.0 };
        }
        if inside && b0.high > b1.high && b0.low > b1.low {
            self.pattern_result = -100;
            self.pattern_age = Some(0);
            self.ref_high = b1.high;
            self.ref_low = b1.low;
            return if emitting { -100.0 } else { 0.0 };
        }

        if !emitting {
            return 0.0;
        }
        self.confirm(b0.close)
    }
}

// =============================================================================
// CDLHIKKAKEMOD
// =============================================================================
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLHIKKAKEMOD {
    bars_seen: usize,
    window: CandleWindow,
    near_avg: RollingCandleAvg,
    /// Bars elapsed since the setup bar; `None` once confirmed or expired.
    pattern_age: Option<usize>,
    pattern_result: i32,
    ref_high: f64,
    ref_low: f64,
}

impl CDLHIKKAKEMOD {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(4),
            near_avg: RollingCandleAvg::new(NEAR),
            pattern_age: None,
            pattern_result: 0,
            ref_high: 0.0,
            ref_low: 0.0,
        }
    }
}

impl Default for CDLHIKKAKEMOD {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLHIKKAKEMOD {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.near_avg.push(open, high, low, close);

        if let Some(age) = self.pattern_age {
            self.pattern_age = Some(age + 1);
        }

        // C TA-Lib: lookback = max(1, NEAR.avg_period) + 5
        let lookback = 1usize.max(NEAR.avg_period) + 5;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let b0 = self.window.bar(0);
        let b1 = self.window.bar(1);
        let b2 = self.window.bar(2);
        let b3 = self.window.bar(3);

        // Two nested inside bars, then a breakout.
        if b1.high < b2.high && b1.low > b2.low && b2.high < b3.high && b2.low > b3.low {
            let near_avg = self.near_avg.val(2);
            if b0.high < b1.high && b0.low < b1.low && b2.close <= b2.low + near_avg {
                self.pattern_result = 100;
                self.pattern_age = Some(0);
                self.ref_high = b1.high;
                self.ref_low = b1.low;
            } else if b0.high > b1.high && b0.low > b1.low && b2.close >= b2.high - near_avg {
                self.pattern_result = -100;
                self.pattern_age = Some(0);
                self.ref_high = b1.high;
                self.ref_low = b1.low;
            }
        }

        // Confirmation — unlike CDLHIKKAKE this can fire on the setup bar itself,
        // and the setup alone never emits.
        if let Some(age) = self.pattern_age
            && age <= 3
        {
            {
                if self.pattern_result > 0 && b0.close > self.ref_high {
                    self.pattern_age = None;
                    return (self.pattern_result + 100) as f64;
                } else if self.pattern_result < 0 && b0.close < self.ref_low {
                    self.pattern_age = None;
                    return (self.pattern_result - 100) as f64;
                }
            }
        }
        0.0
    }
}

// =============================================================================
// CDLSTICKSANDWICH
// =============================================================================
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLSTICKSANDWICH {
    bars_seen: usize,
    window: CandleWindow,
    equal: RollingCandleAvg,
}

impl CDLSTICKSANDWICH {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            equal: RollingCandleAvg::new(EQUAL),
        }
    }
}

impl Default for CDLSTICKSANDWICH {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDLSTICKSANDWICH {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.equal.push(open, high, low, close);

        let lookback = EQUAL.avg_period + 2;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let b2 = self.window.bar(2);
        let b1 = self.window.bar(1);
        let b0 = self.window.bar(0);

        let is_sandwich = candle_color(b2.open, b2.close) == -1
            && candle_color(b1.open, b1.close) == 1
            && candle_color(b0.open, b0.close) == -1
            && b1.low > b2.close
            && (b0.close - b2.close).abs() <= self.equal.val(2);

        if is_sandwich { 100.0 } else { 0.0 }
    }
}

// =============================================================================
// CDL2CROWS
// =============================================================================
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDL2CROWS {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
}

impl CDL2CROWS {
    pub fn new() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(3),
            body_long: RollingCandleAvg::new(BODY_LONG),
        }
    }
}

impl Default for CDL2CROWS {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<(f64, f64, f64, f64)> for CDL2CROWS {
    type Output = f64;
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.bars_seen += 1;
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);

        let lookback = BODY_LONG.avg_period + 2;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let b2 = self.window.bar(2);
        let b1 = self.window.bar(1);
        let b0 = self.window.bar(0);

        let is_2crows = candle_color(b2.open, b2.close) == 1
            && real_body(b2.open, b2.close) > self.body_long.val(2)
            && candle_color(b1.open, b1.close) == -1
            && b1.open.min(b1.close) > b2.open.max(b2.close) // real body gap up
            && candle_color(b0.open, b0.close) == -1
            && b0.open < b1.open && b0.open > b1.close
            && b0.close > b2.open && b0.close < b2.close;

        if is_2crows { -100.0 } else { 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::patterns::fixtures as fx;
    use crate::test_pattern_parity;

    test_pattern_parity!(
        test_cdlpiercing,
        CDLPIERCING,
        talib_rs::pattern::cdl_piercing,
        fx::piercing
    );
    test_pattern_parity!(
        test_cdlseparatinglines,
        CDLSEPARATINGLINES,
        talib_rs::pattern::cdl_separatinglines
    );
    test_pattern_parity!(
        test_cdlshootingstar,
        CDLSHOOTINGSTAR,
        talib_rs::pattern::cdl_shootingstar,
        fx::shootingstar
    );
    test_pattern_parity!(
        test_cdlthrusting,
        CDLTHRUSTING,
        talib_rs::pattern::cdl_thrusting,
        fx::thrusting
    );
    test_pattern_parity!(
        test_cdlkicking,
        CDLKICKING,
        talib_rs::pattern::cdl_kicking,
        fx::kicking
    );
    test_pattern_parity!(
        test_cdlkickingbylength,
        CDLKICKINGBYLENGTH,
        talib_rs::pattern::cdl_kickingbylength,
        fx::kickingbylength
    );
    test_pattern_parity!(test_cdlhikkake, CDLHIKKAKE, talib_rs::pattern::cdl_hikkake);
    test_pattern_parity!(
        test_cdlhikkakemod,
        CDLHIKKAKEMOD,
        talib_rs::pattern::cdl_hikkakemod,
        fx::hikkakemod
    );
    test_pattern_parity!(
        test_cdlsticksandwich,
        CDLSTICKSANDWICH,
        talib_rs::pattern::cdl_sticksandwich,
        fx::sticksandwich
    );
    test_pattern_parity!(
        test_cdl2crows,
        CDL2CROWS,
        talib_rs::pattern::cdl_2crows,
        fx::two_crows
    );
}
