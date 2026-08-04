use crate::indicators::patterns::candle_settings::*;
use crate::traits::Next;

macro_rules! real_body_gap_up {
    ($curr:expr, $prev:expr) => {
        $curr.open.min($curr.close) > $prev.open.max($prev.close)
    };
}

macro_rules! real_body_gap_down {
    ($curr:expr, $prev:expr) => {
        $curr.open.max($curr.close) < $prev.open.min($prev.close)
    };
}

/// Native O(1) streaming CDLENGULFING (matches `talib_rs::pattern::cdl_engulfing`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLENGULFING {
    bars_seen: usize,
    window: CandleWindow,
}
impl CDLENGULFING {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLENGULFING {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLENGULFING {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.bars_seen += 1;
        if self.bars_seen <= 1 {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let bull = candle_color(prev.open, prev.close) == -1
            && candle_color(curr.open, curr.close) == 1
            && curr.close >= prev.open
            && curr.open <= prev.close;

        let bear = candle_color(prev.open, prev.close) == 1
            && candle_color(curr.open, curr.close) == -1
            && curr.open >= prev.close
            && curr.close <= prev.open;

        (bull as i32 * 100 - bear as i32 * 100) as f64
    }
}

/// Native O(1) streaming CDLCOUNTERATTACK (matches `talib_rs::pattern::cdl_counterattack`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLCOUNTERATTACK {
    bars_seen: usize,
    window: CandleWindow,
    equal: RollingCandleAvg,
    body_long: RollingCandleAvg,
}
impl CDLCOUNTERATTACK {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLCOUNTERATTACK {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            equal: RollingCandleAvg::new(EQUAL),
            body_long: RollingCandleAvg::new(BODY_LONG),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLCOUNTERATTACK {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.equal.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = EQUAL.avg_period.max(BODY_LONG.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let is_counterattack = candle_color(prev.open, prev.close)
            != candle_color(curr.open, curr.close)
            && real_body(prev.open, prev.close) > self.body_long.val(1)
            && real_body(curr.open, curr.close) > self.body_long.val(0)
            && (curr.close - prev.close).abs() <= self.equal.val(1);

        (is_counterattack as i32 * candle_color(curr.open, curr.close) * 100) as f64
    }
}

/// Native O(1) streaming CDLDARKCLOUDCOVER (matches `talib_rs::pattern::cdl_darkcloudcover`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLDARKCLOUDCOVER {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
}
impl CDLDARKCLOUDCOVER {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLDARKCLOUDCOVER {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            body_long: RollingCandleAvg::new(BODY_LONG),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLDARKCLOUDCOVER {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = BODY_LONG.avg_period + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);
        let penetration = 0.5;

        let is_dark_cloud_cover = candle_color(prev.open, prev.close) == 1
            && real_body(prev.open, prev.close) > self.body_long.val(1)
            && candle_color(curr.open, curr.close) == -1
            && curr.open > prev.high
            && curr.close > prev.open
            && curr.close < prev.close - real_body(prev.open, prev.close) * penetration;

        (is_dark_cloud_cover as i32 * -100) as f64
    }
}

/// Native O(1) streaming CDLDOJISTAR (matches `talib_rs::pattern::cdl_dojistar`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLDOJISTAR {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    body_doji: RollingCandleAvg,
}
impl CDLDOJISTAR {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLDOJISTAR {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            body_long: RollingCandleAvg::new(BODY_LONG),
            body_doji: RollingCandleAvg::new(BODY_DOJI),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLDOJISTAR {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.body_doji.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = BODY_DOJI.avg_period.max(BODY_LONG.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let base = real_body(prev.open, prev.close) > self.body_long.val(1)
            && real_body(curr.open, curr.close) <= self.body_doji.val(0);

        let bear =
            base && candle_color(prev.open, prev.close) == 1 && real_body_gap_up!(curr, prev);
        let bull =
            base && candle_color(prev.open, prev.close) == -1 && real_body_gap_down!(curr, prev);

        (bull as i32 * 100 - bear as i32 * 100) as f64
    }
}

/// Native O(1) streaming CDLHANGINGMAN (matches `talib_rs::pattern::cdl_hangingman`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLHANGINGMAN {
    bars_seen: usize,
    window: CandleWindow,
    body_short: RollingCandleAvg,
    shadow_long: RollingCandleAvg,
    shadow_very_short: RollingCandleAvg,
    near: RollingCandleAvg,
}
impl CDLHANGINGMAN {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLHANGINGMAN {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            body_short: RollingCandleAvg::new(BODY_SHORT),
            shadow_long: RollingCandleAvg::new(SHADOW_LONG),
            shadow_very_short: RollingCandleAvg::new(SHADOW_VERY_SHORT),
            near: RollingCandleAvg::new(NEAR),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLHANGINGMAN {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.body_short.push(open, high, low, close);
        self.shadow_long.push(open, high, low, close);
        self.shadow_very_short.push(open, high, low, close);
        self.near.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = *[
            BODY_SHORT.avg_period,
            SHADOW_LONG.avg_period,
            SHADOW_VERY_SHORT.avg_period,
            NEAR.avg_period,
        ]
        .iter()
        .max()
        .unwrap()
            + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let is_hanging_man = real_body(curr.open, curr.close) < self.body_short.val(0)
            && lower_shadow(curr.open, curr.low, curr.close) > self.shadow_long.val(0)
            && upper_shadow(curr.open, curr.high, curr.close) < self.shadow_very_short.val(0)
            && curr.open.min(curr.close) >= prev.high - self.near.val(1);

        (is_hanging_man as i32 * -100) as f64
    }
}

/// Native O(1) streaming CDLHARAMI (matches `talib_rs::pattern::cdl_harami`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLHARAMI {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    body_short: RollingCandleAvg,
}
impl CDLHARAMI {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLHARAMI {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            body_long: RollingCandleAvg::new(BODY_LONG),
            body_short: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLHARAMI {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.body_short.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let is_harami = real_body(prev.open, prev.close) > self.body_long.val(1)
            && real_body(curr.open, curr.close) <= self.body_short.val(0)
            && curr.open.max(curr.close) < prev.open.max(prev.close)
            && curr.open.min(curr.close) > prev.open.min(prev.close);

        (is_harami as i32 * -candle_color(prev.open, prev.close) * 100) as f64
    }
}

/// Native O(1) streaming CDLHARAMICROSS (matches `talib_rs::pattern::cdl_haramicross`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLHARAMICROSS {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    body_doji: RollingCandleAvg,
}
impl CDLHARAMICROSS {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLHARAMICROSS {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            body_long: RollingCandleAvg::new(BODY_LONG),
            body_doji: RollingCandleAvg::new(BODY_DOJI),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLHARAMICROSS {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.body_doji.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = BODY_DOJI.avg_period.max(BODY_LONG.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let is_harami_cross = real_body(prev.open, prev.close) > self.body_long.val(1)
            && real_body(curr.open, curr.close) <= self.body_doji.val(0)
            && curr.open.max(curr.close) < prev.open.max(prev.close)
            && curr.open.min(curr.close) > prev.open.min(prev.close);

        (is_harami_cross as i32 * -candle_color(prev.open, prev.close) * 100) as f64
    }
}

/// Native O(1) streaming CDLHOMINGPIGEON (matches `talib_rs::pattern::cdl_homingpigeon`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLHOMINGPIGEON {
    bars_seen: usize,
    window: CandleWindow,
    body_long: RollingCandleAvg,
    body_short: RollingCandleAvg,
}
impl CDLHOMINGPIGEON {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLHOMINGPIGEON {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            body_long: RollingCandleAvg::new(BODY_LONG),
            body_short: RollingCandleAvg::new(BODY_SHORT),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLHOMINGPIGEON {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.body_short.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = BODY_SHORT.avg_period.max(BODY_LONG.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let is_homing_pigeon = candle_color(prev.open, prev.close) == -1
            && candle_color(curr.open, curr.close) == -1
            && real_body(prev.open, prev.close) > self.body_long.val(1)
            && real_body(curr.open, curr.close) <= self.body_short.val(0)
            && curr.open < prev.open
            && curr.close > prev.close;

        (is_homing_pigeon as i32 * 100) as f64
    }
}

/// Native O(1) streaming CDLINNECK (matches `talib_rs::pattern::cdl_inneck`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLINNECK {
    bars_seen: usize,
    window: CandleWindow,
    equal: RollingCandleAvg,
    body_long: RollingCandleAvg,
}
impl CDLINNECK {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLINNECK {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            equal: RollingCandleAvg::new(EQUAL),
            body_long: RollingCandleAvg::new(BODY_LONG),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLINNECK {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.equal.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = EQUAL.avg_period.max(BODY_LONG.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let is_inneck = candle_color(prev.open, prev.close) == -1
            && real_body(prev.open, prev.close) > self.body_long.val(1)
            && candle_color(curr.open, curr.close) == 1
            && curr.open < prev.low
            && curr.close >= prev.close
            && curr.close <= prev.close + self.equal.val(1);

        (is_inneck as i32 * -100) as f64
    }
}

/// Native O(1) streaming CDLINVERTEDHAMMER (matches `talib_rs::pattern::cdl_invertedhammer`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLINVERTEDHAMMER {
    bars_seen: usize,
    window: CandleWindow,
    body_short: RollingCandleAvg,
    shadow_long: RollingCandleAvg,
    shadow_very_short: RollingCandleAvg,
}
impl CDLINVERTEDHAMMER {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLINVERTEDHAMMER {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            body_short: RollingCandleAvg::new(BODY_SHORT),
            shadow_long: RollingCandleAvg::new(SHADOW_LONG),
            shadow_very_short: RollingCandleAvg::new(SHADOW_VERY_SHORT),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLINVERTEDHAMMER {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.body_short.push(open, high, low, close);
        self.shadow_long.push(open, high, low, close);
        self.shadow_very_short.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = *[
            BODY_SHORT.avg_period,
            SHADOW_LONG.avg_period,
            SHADOW_VERY_SHORT.avg_period,
        ]
        .iter()
        .max()
        .unwrap()
            + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let is_inverted_hammer = real_body(curr.open, curr.close) < self.body_short.val(0)
            && upper_shadow(curr.open, curr.high, curr.close) > self.shadow_long.val(0)
            && lower_shadow(curr.open, curr.low, curr.close) < self.shadow_very_short.val(0)
            && real_body_gap_down!(curr, prev);

        (is_inverted_hammer as i32 * 100) as f64
    }
}

/// Native O(1) streaming CDLMATCHINGLOW (matches `talib_rs::pattern::cdl_matchinglow`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLMATCHINGLOW {
    bars_seen: usize,
    window: CandleWindow,
    equal: RollingCandleAvg,
}
impl CDLMATCHINGLOW {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLMATCHINGLOW {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            equal: RollingCandleAvg::new(EQUAL),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLMATCHINGLOW {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.equal.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = EQUAL.avg_period + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let is_matching_low = candle_color(prev.open, prev.close) == -1
            && candle_color(curr.open, curr.close) == -1
            && (curr.close - prev.close).abs() <= self.equal.val(1);

        (is_matching_low as i32 * 100) as f64
    }
}

/// Native O(1) streaming CDLONNECK (matches `talib_rs::pattern::cdl_onneck`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CDLONNECK {
    bars_seen: usize,
    window: CandleWindow,
    equal: RollingCandleAvg,
    body_long: RollingCandleAvg,
}
impl CDLONNECK {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for CDLONNECK {
    fn default() -> Self {
        Self {
            bars_seen: 0,
            window: CandleWindow::new(2),
            equal: RollingCandleAvg::new(EQUAL),
            body_long: RollingCandleAvg::new(BODY_LONG),
        }
    }
}
impl Next<(f64, f64, f64, f64)> for CDLONNECK {
    type Output = f64;
    #[inline]
    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        self.window.push(open, high, low, close);
        self.equal.push(open, high, low, close);
        self.body_long.push(open, high, low, close);
        self.bars_seen += 1;

        let lookback = EQUAL.avg_period.max(BODY_LONG.avg_period) + 1;
        if self.bars_seen <= lookback {
            return 0.0;
        }

        let curr = self.window.bar(0);
        let prev = self.window.bar(1);

        let is_onneck = candle_color(prev.open, prev.close) == -1
            && real_body(prev.open, prev.close) > self.body_long.val(1)
            && candle_color(curr.open, curr.close) == 1
            && curr.open < prev.low
            && (curr.close - prev.low).abs() <= self.equal.val(1);

        (is_onneck as i32 * -100) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::test_pattern_parity!(
        test_cdl_engulfing_parity,
        CDLENGULFING,
        talib_rs::pattern::cdl_engulfing
    );
    crate::test_pattern_parity!(
        test_cdl_counterattack_parity,
        CDLCOUNTERATTACK,
        talib_rs::pattern::cdl_counterattack
    );
    crate::test_pattern_parity!(
        test_cdl_darkcloudcover_parity,
        CDLDARKCLOUDCOVER,
        talib_rs::pattern::cdl_darkcloudcover
    );
    crate::test_pattern_parity!(
        test_cdl_dojistar_parity,
        CDLDOJISTAR,
        talib_rs::pattern::cdl_dojistar
    );
    crate::test_pattern_parity!(
        test_cdl_hangingman_parity,
        CDLHANGINGMAN,
        talib_rs::pattern::cdl_hangingman
    );
    crate::test_pattern_parity!(
        test_cdl_harami_parity,
        CDLHARAMI,
        talib_rs::pattern::cdl_harami
    );
    crate::test_pattern_parity!(
        test_cdl_haramicross_parity,
        CDLHARAMICROSS,
        talib_rs::pattern::cdl_haramicross
    );
    crate::test_pattern_parity!(
        test_cdl_homingpigeon_parity,
        CDLHOMINGPIGEON,
        talib_rs::pattern::cdl_homingpigeon
    );
    crate::test_pattern_parity!(
        test_cdl_inneck_parity,
        CDLINNECK,
        talib_rs::pattern::cdl_inneck
    );
    crate::test_pattern_parity!(
        test_cdl_invertedhammer_parity,
        CDLINVERTEDHAMMER,
        talib_rs::pattern::cdl_invertedhammer
    );
    crate::test_pattern_parity!(
        test_cdl_matchinglow_parity,
        CDLMATCHINGLOW,
        talib_rs::pattern::cdl_matchinglow
    );
    crate::test_pattern_parity!(
        test_cdl_onneck_parity,
        CDLONNECK,
        talib_rs::pattern::cdl_onneck
    );
}
