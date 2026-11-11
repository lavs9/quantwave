//! Session Levels and Intraday Time Indicators.
//!
//! This module provides streaming indicators anchored to trading sessions:
//! - [`SessionVwap`]: Volume Weighted Average Price with ±1σ, ±2σ, and ±3σ standard deviation bands.
//! - [`OpeningRangeBreakout`]: Toby Crabel's Opening Range Breakout (ORB) with Fibonacci/expansion targets.
//!
//! Both indicators implement the [`Next`] trait and reset accumulation counters whenever
//! `is_session_start == true`.

use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur when configuring session indicators.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SessionLevelsError {
    /// Opening range bars must be at least 1.
    #[error("range_bars must be at least 1, got {0}")]
    InvalidRangeBars(usize),
}

// ============================================================================
// 1. Session VWAP with Standard Deviation Bands
// ============================================================================

/// Standard deviation band identifier for Session VWAP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionVwapBand {
    /// First standard deviation band (±1.0σ).
    Sigma1,
    /// Second standard deviation band (±2.0σ).
    Sigma2,
    /// Third standard deviation band (±3.0σ).
    Sigma3,
}

impl SessionVwapBand {
    /// Returns the standard deviation multiplier for this band.
    #[inline]
    #[must_use]
    pub const fn multiplier(self) -> f64 {
        match self {
            Self::Sigma1 => 1.0,
            Self::Sigma2 => 2.0,
            Self::Sigma3 => 3.0,
        }
    }
}

/// Output state of the Session VWAP indicator.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SessionVwapState {
    /// Volume Weighted Average Price for the session.
    pub vwap: f64,
    /// Upper +1.0 standard deviation band (`vwap + 1.0 * std_dev`).
    pub upper_1: f64,
    /// Lower -1.0 standard deviation band (`vwap - 1.0 * std_dev`).
    pub lower_1: f64,
    /// Upper +2.0 standard deviation band (`vwap + 2.0 * std_dev`).
    pub upper_2: f64,
    /// Lower -2.0 standard deviation band (`vwap - 2.0 * std_dev`).
    pub lower_2: f64,
    /// Upper +3.0 standard deviation band (`vwap + 3.0 * std_dev`).
    pub upper_3: f64,
    /// Lower -3.0 standard deviation band (`vwap - 3.0 * std_dev`).
    pub lower_3: f64,
    /// Session volume-weighted standard deviation: $\sqrt{\frac{\sum V \cdot (TP - VWAP)^2}{\sum V}}$.
    pub std_dev: f64,
    /// Cumulative volume accumulated in the current session.
    pub cumulative_volume: f64,
    /// 0-indexed count of bars processed in the current session.
    pub bar_index: usize,
}

impl SessionVwapState {
    /// Returns the upper band value for the specified sigma multiplier.
    #[inline]
    #[must_use]
    pub fn upper(&self, band: SessionVwapBand) -> f64 {
        match band {
            SessionVwapBand::Sigma1 => self.upper_1,
            SessionVwapBand::Sigma2 => self.upper_2,
            SessionVwapBand::Sigma3 => self.upper_3,
        }
    }

    /// Returns the lower band value for the specified sigma multiplier.
    #[inline]
    #[must_use]
    pub fn lower(&self, band: SessionVwapBand) -> f64 {
        match band {
            SessionVwapBand::Sigma1 => self.lower_1,
            SessionVwapBand::Sigma2 => self.lower_2,
            SessionVwapBand::Sigma3 => self.lower_3,
        }
    }

    /// Returns the `(upper, lower)` band envelope pair for the specified sigma multiplier.
    #[inline]
    #[must_use]
    pub fn band(&self, band: SessionVwapBand) -> (f64, f64) {
        (self.upper(band), self.lower(band))
    }
}

/// Input tuple wrapper for [`SessionVwap`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SessionVwapInput {
    /// Bar high price.
    pub high: f64,
    /// Bar low price.
    pub low: f64,
    /// Bar close price.
    pub close: f64,
    /// Bar traded volume.
    pub volume: f64,
    /// Flag indicating whether this bar begins a new trading session.
    pub is_session_start: bool,
}

impl From<(f64, f64, f64, f64, bool)> for SessionVwapInput {
    #[inline]
    fn from((high, low, close, volume, is_session_start): (f64, f64, f64, f64, bool)) -> Self {
        Self {
            high,
            low,
            close,
            volume,
            is_session_start,
        }
    }
}

impl From<SessionVwapInput> for (f64, f64, f64, f64, bool) {
    #[inline]
    fn from(input: SessionVwapInput) -> Self {
        (
            input.high,
            input.low,
            input.close,
            input.volume,
            input.is_session_start,
        )
    }
}

/// Session Volume Weighted Average Price (VWAP) with Standard Deviation Bands.
///
/// Accumulates volume-weighted typical price across intraday bars and resets
/// whenever `is_session_start` is true.
///
/// Typical Price:
/// $$TP = \frac{\text{High} + \text{Low} + \text{Close}}{3}$$
///
/// Running VWAP:
/// $$VWAP = \frac{\sum (TP \cdot Volume)}{\sum Volume}$$
///
/// Running Volume-Weighted Standard Deviation:
/// $$\sigma = \sqrt{\frac{\sum Volume \cdot (TP - VWAP)^2}{\sum Volume}}$$
///
/// Standard Deviation Bands:
/// $$\text{Upper}_k = VWAP + k \cdot \sigma, \quad \text{Lower}_k = VWAP - k \cdot \sigma \quad (k \in \{1, 2, 3\})$$
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionVwap {
    cumulative_volume: f64,
    vwap: f64,
    m2: f64,
    bar_index: usize,
    initialized: bool,
}

impl Default for SessionVwap {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionVwap {
    /// Creates a new `SessionVwap` state machine.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cumulative_volume: 0.0,
            vwap: 0.0,
            m2: 0.0,
            bar_index: 0,
            initialized: false,
        }
    }

    /// Explicitly resets the session accumulation counters.
    pub fn reset(&mut self) {
        self.cumulative_volume = 0.0;
        self.vwap = 0.0;
        self.m2 = 0.0;
        self.bar_index = 0;
        self.initialized = false;
    }

    /// Returns the total volume accumulated in the current session.
    #[inline]
    #[must_use]
    pub fn cumulative_volume(&self) -> f64 {
        self.cumulative_volume
    }

    /// Returns the current VWAP level for the session.
    #[inline]
    #[must_use]
    pub fn current_vwap(&self) -> f64 {
        self.vwap
    }

    /// Returns the 0-indexed count of bars processed in the current session.
    #[inline]
    #[must_use]
    pub fn current_bar_index(&self) -> usize {
        self.bar_index
    }

    #[inline]
    fn update_internal(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        is_session_start: bool,
    ) -> SessionVwapState {
        let tp = (high + low + close) / 3.0;
        let vol = if volume > 0.0 { volume } else { 0.0 };

        if !self.initialized || is_session_start {
            self.cumulative_volume = vol;
            self.vwap = tp;
            self.m2 = 0.0;
            self.bar_index = 0;
            self.initialized = true;

            SessionVwapState {
                vwap: tp,
                upper_1: tp,
                lower_1: tp,
                upper_2: tp,
                lower_2: tp,
                upper_3: tp,
                lower_3: tp,
                std_dev: 0.0,
                cumulative_volume: vol,
                bar_index: 0,
            }
        } else {
            self.bar_index += 1;

            if vol > 0.0 {
                let old_vol = self.cumulative_volume;
                self.cumulative_volume += vol;
                if self.cumulative_volume > 0.0 {
                    let delta = tp - self.vwap;
                    let r = delta * (vol / self.cumulative_volume);
                    self.vwap += r;
                    self.m2 += old_vol * delta * r;
                }
            }

            let variance = if self.cumulative_volume > 0.0 {
                (self.m2 / self.cumulative_volume).max(0.0)
            } else {
                0.0
            };
            let std_dev = variance.sqrt();

            SessionVwapState {
                vwap: self.vwap,
                upper_1: self.vwap + 1.0 * std_dev,
                lower_1: self.vwap - 1.0 * std_dev,
                upper_2: self.vwap + 2.0 * std_dev,
                lower_2: self.vwap - 2.0 * std_dev,
                upper_3: self.vwap + 3.0 * std_dev,
                lower_3: self.vwap - 3.0 * std_dev,
                std_dev,
                cumulative_volume: self.cumulative_volume,
                bar_index: self.bar_index,
            }
        }
    }
}

impl Next<(f64, f64, f64, f64, bool)> for SessionVwap {
    type Output = SessionVwapState;

    #[inline]
    fn next(
        &mut self,
        (high, low, close, volume, is_session_start): (f64, f64, f64, f64, bool),
    ) -> Self::Output {
        self.update_internal(high, low, close, volume, is_session_start)
    }
}

impl Next<SessionVwapInput> for SessionVwap {
    type Output = SessionVwapState;

    #[inline]
    fn next(&mut self, input: SessionVwapInput) -> Self::Output {
        self.update_internal(
            input.high,
            input.low,
            input.close,
            input.volume,
            input.is_session_start,
        )
    }
}

// ============================================================================
// 2. Opening Range Breakout (ORB)
// ============================================================================

/// Lifecycle phase of an Opening Range Breakout session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrbPhase {
    /// Opening range window is forming; highs and lows are actively accumulated.
    Forming,
    /// Opening range window is locked; breakout triggers and targets are active.
    Active,
    /// Session is closed; no further breakouts are monitored.
    Closed,
}

impl OrbPhase {
    /// Returns `true` if the opening range is currently forming.
    #[inline]
    #[must_use]
    pub const fn is_forming(self) -> bool {
        matches!(self, Self::Forming)
    }

    /// Returns `true` if the opening range is active and monitoring breakouts.
    #[inline]
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns `true` if the session has closed.
    #[inline]
    #[must_use]
    pub const fn is_closed(self) -> bool {
        matches!(self, Self::Closed)
    }
}

/// Breakout status relative to the opening range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrbBreakout {
    /// Price is within the opening range or range is still forming / closed.
    None,
    /// Bar closed strictly above the opening range high.
    BullishBreakout,
    /// Bar closed strictly below the opening range low.
    BearishBreakout,
}

impl OrbBreakout {
    /// Returns `true` if any directional breakout occurred.
    #[inline]
    #[must_use]
    pub const fn is_breakout(self) -> bool {
        matches!(self, Self::BullishBreakout | Self::BearishBreakout)
    }

    /// Returns `true` if a bullish breakout occurred.
    #[inline]
    #[must_use]
    pub const fn is_bullish(self) -> bool {
        matches!(self, Self::BullishBreakout)
    }

    /// Returns `true` if a bearish breakout occurred.
    #[inline]
    #[must_use]
    pub const fn is_bearish(self) -> bool {
        matches!(self, Self::BearishBreakout)
    }
}

/// Output state of the Opening Range Breakout (ORB) indicator.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OrbState {
    /// Current lifecycle phase of the session.
    pub phase: OrbPhase,
    /// Breakout status evaluated on the current bar close.
    pub breakout: OrbBreakout,
    /// Established or in-progress high of the opening range.
    pub range_high: f64,
    /// Established or in-progress low of the opening range.
    pub range_low: f64,
    /// Height of the opening range (`range_high - range_low`).
    pub range_size: f64,
    /// Bullish extension target 1 (`range_high + 0.5 * range_size`).
    pub target_1_bull: f64,
    /// Bearish extension target 1 (`range_low - 0.5 * range_size`).
    pub target_1_bear: f64,
    /// Bullish extension target 2 (`range_high + 1.0 * range_size`).
    pub target_2_bull: f64,
    /// Bearish extension target 2 (`range_low - 1.0 * range_size`).
    pub target_2_bear: f64,
    /// 0-indexed count of bars processed in the current session.
    pub bar_index: usize,
}

impl OrbState {
    /// Returns `true` if the bar closed with a breakout in either direction.
    #[inline]
    #[must_use]
    pub const fn is_breakout(&self) -> bool {
        self.breakout.is_breakout()
    }

    /// Returns `true` if target 1 was reached or exceeded by the provided price.
    #[inline]
    #[must_use]
    pub fn target_1_reached(&self, price: f64) -> bool {
        price >= self.target_1_bull || price <= self.target_1_bear
    }

    /// Returns `true` if target 2 was reached or exceeded by the provided price.
    #[inline]
    #[must_use]
    pub fn target_2_reached(&self, price: f64) -> bool {
        price >= self.target_2_bull || price <= self.target_2_bear
    }
}

/// Input tuple wrapper for [`OpeningRangeBreakout`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OrbInput {
    /// Bar high price.
    pub high: f64,
    /// Bar low price.
    pub low: f64,
    /// Bar close price.
    pub close: f64,
    /// Flag indicating whether this bar begins a new trading session.
    pub is_session_start: bool,
}

impl From<(f64, f64, f64, bool)> for OrbInput {
    #[inline]
    fn from((high, low, close, is_session_start): (f64, f64, f64, bool)) -> Self {
        Self {
            high,
            low,
            close,
            is_session_start,
        }
    }
}

impl From<OrbInput> for (f64, f64, f64, bool) {
    #[inline]
    fn from(input: OrbInput) -> Self {
        (input.high, input.low, input.close, input.is_session_start)
    }
}

/// Opening Range Breakout (ORB) indicator.
///
/// Tracks the high and low established over the first `range_bars` bars of each session
/// and generates directional breakout signals with 0.5x and 1.0x range extension targets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpeningRangeBreakout {
    range_bars: usize,
    max_session_bars: Option<usize>,
    range_high: f64,
    range_low: f64,
    bars_in_session: usize,
    phase: OrbPhase,
    breakout: OrbBreakout,
    initialized: bool,
}

impl Default for OpeningRangeBreakout {
    fn default() -> Self {
        Self::new(15)
    }
}

impl OpeningRangeBreakout {
    /// Creates a new `OpeningRangeBreakout` with the specified number of opening bars.
    ///
    /// If `range_bars == 0`, it defaults to 1.
    #[must_use]
    pub fn new(range_bars: usize) -> Self {
        Self {
            range_bars: range_bars.max(1),
            max_session_bars: None,
            range_high: 0.0,
            range_low: 0.0,
            bars_in_session: 0,
            phase: OrbPhase::Forming,
            breakout: OrbBreakout::None,
            initialized: false,
        }
    }

    /// Try to create a new `OpeningRangeBreakout`, returning an error if `range_bars == 0`.
    pub fn try_new(range_bars: usize) -> Result<Self, SessionLevelsError> {
        if range_bars == 0 {
            return Err(SessionLevelsError::InvalidRangeBars(0));
        }
        Ok(Self::new(range_bars))
    }

    /// Sets an optional session bar cap after which the indicator transitions to [`OrbPhase::Closed`].
    #[must_use]
    pub fn with_max_session_bars(mut self, max_bars: usize) -> Self {
        self.max_session_bars = Some(max_bars);
        self
    }

    /// Number of bars comprising the opening range window.
    #[inline]
    #[must_use]
    pub fn range_bars(&self) -> usize {
        self.range_bars
    }

    /// Optional session bar cap.
    #[inline]
    #[must_use]
    pub fn max_session_bars(&self) -> Option<usize> {
        self.max_session_bars
    }

    /// Explicitly resets the indicator state.
    pub fn reset(&mut self) {
        self.range_high = 0.0;
        self.range_low = 0.0;
        self.bars_in_session = 0;
        self.phase = OrbPhase::Forming;
        self.breakout = OrbBreakout::None;
        self.initialized = false;
    }

    /// Manually closes the active session.
    pub fn close_session(&mut self) {
        self.phase = OrbPhase::Closed;
        self.breakout = OrbBreakout::None;
    }

    #[inline]
    fn update_internal(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        is_session_start: bool,
    ) -> OrbState {
        let h = high.max(low);
        let l = high.min(low);

        if !self.initialized || is_session_start {
            self.bars_in_session = 0;
            self.range_high = h;
            self.range_low = l;
            self.phase = OrbPhase::Forming;
            self.breakout = OrbBreakout::None;
            self.initialized = true;

            let range_size = (self.range_high - self.range_low).max(0.0);
            let state = OrbState {
                phase: OrbPhase::Forming,
                breakout: OrbBreakout::None,
                range_high: self.range_high,
                range_low: self.range_low,
                range_size,
                target_1_bull: self.range_high + 0.5 * range_size,
                target_1_bear: self.range_low - 0.5 * range_size,
                target_2_bull: self.range_high + 1.0 * range_size,
                target_2_bear: self.range_low - 1.0 * range_size,
                bar_index: 0,
            };
            self.bars_in_session = 1;
            state
        } else {
            let current_bar = self.bars_in_session;
            self.bars_in_session += 1;

            if let Some(max_bars) = self.max_session_bars
                && current_bar >= max_bars
            {
                self.phase = OrbPhase::Closed;
                self.breakout = OrbBreakout::None;
                let range_size = (self.range_high - self.range_low).max(0.0);
                return OrbState {
                    phase: OrbPhase::Closed,
                    breakout: OrbBreakout::None,
                    range_high: self.range_high,
                    range_low: self.range_low,
                    range_size,
                    target_1_bull: self.range_high + 0.5 * range_size,
                    target_1_bear: self.range_low - 0.5 * range_size,
                    target_2_bull: self.range_high + 1.0 * range_size,
                    target_2_bear: self.range_low - 1.0 * range_size,
                    bar_index: current_bar,
                };
            }

            if current_bar < self.range_bars {
                self.range_high = self.range_high.max(h);
                self.range_low = self.range_low.min(l);
                self.phase = OrbPhase::Forming;
                self.breakout = OrbBreakout::None;
            } else {
                self.phase = OrbPhase::Active;
                if close > self.range_high {
                    self.breakout = OrbBreakout::BullishBreakout;
                } else if close < self.range_low {
                    self.breakout = OrbBreakout::BearishBreakout;
                } else {
                    self.breakout = OrbBreakout::None;
                }
            }

            let range_size = (self.range_high - self.range_low).max(0.0);
            OrbState {
                phase: self.phase,
                breakout: self.breakout,
                range_high: self.range_high,
                range_low: self.range_low,
                range_size,
                target_1_bull: self.range_high + 0.5 * range_size,
                target_1_bear: self.range_low - 0.5 * range_size,
                target_2_bull: self.range_high + 1.0 * range_size,
                target_2_bear: self.range_low - 1.0 * range_size,
                bar_index: current_bar,
            }
        }
    }
}

impl Next<(f64, f64, f64, bool)> for OpeningRangeBreakout {
    type Output = OrbState;

    #[inline]
    fn next(
        &mut self,
        (high, low, close, is_session_start): (f64, f64, f64, bool),
    ) -> Self::Output {
        self.update_internal(high, low, close, is_session_start)
    }
}

impl Next<OrbInput> for OpeningRangeBreakout {
    type Output = OrbState;

    #[inline]
    fn next(&mut self, input: OrbInput) -> Self::Output {
        self.update_internal(input.high, input.low, input.close, input.is_session_start)
    }
}

// ============================================================================
// 3. Metadata Constants
// ============================================================================

/// Metadata definition for Session VWAP.
pub const SESSION_VWAP_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Session VWAP with Standard Deviation Bands",
    description: "Session-anchored Volume Weighted Average Price (VWAP) with three standard deviation volatility bands, resetting at session boundaries.",
    usage: "Use as an intraday fair value benchmark. The core VWAP serves as the primary institutional execution level, while Sigma 1, 2, and 3 standard deviation bands identify statistical mean reversion boundaries and dynamic support/resistance.",
    keywords: &[
        "vwap",
        "session",
        "intraday",
        "volume",
        "bands",
        "standard-deviation",
        "mean-reversion",
    ],
    ehlers_summary: "Volume Weighted Average Price calculates the cumulative typical price weighted by volume across a trading session. Incorporating standard deviation bands provides dynamic volatility envelopes that highlight statistical value containment and institutional liquidation extremes. — StockCharts / Investopedia",
    params: &[],
    formula_source: "https://chartschool.stockcharts.com/table-of-contents/technical-indicators-and-overlays/technical-overlays/vwap-standard-deviation-bands",
    formula_latex: r#"
\[
TP_t = \frac{H_t + L_t + C_t}{3} \\
VWAP_t = \frac{\sum_{i=1}^t TP_i \cdot V_i}{\sum_{i=1}^t V_i} \\
\sigma_t = \sqrt{\frac{\sum_{i=1}^t V_i \cdot (TP_i - VWAP_t)^2}{\sum_{i=1}^t V_i}} \\
\text{Upper}_k = VWAP_t + k \cdot \sigma_t, \quad \text{Lower}_k = VWAP_t - k \cdot \sigma_t \quad (k \in \{1, 2, 3\})
\]
"#,
    gold_standard_file: "session_vwap.json",
    category: "Volume",
};

/// Metadata definition for Opening Range Breakout (ORB).
pub const ORB_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Opening Range Breakout",
    description: "Tracks the high and low over the opening range bars and detects directional breakouts with extension targets.",
    usage: "Identifies early-session market commitment and momentum expansion. When price breaks out of the established opening range, traders enter in the direction of the break targeting 0.5x and 1.0x range extensions.",
    keywords: &[
        "orb",
        "opening-range",
        "breakout",
        "session",
        "intraday",
        "toby-crabel",
        "trend",
    ],
    ehlers_summary: "Pioneered by Toby Crabel in 'Day Trading with Short Term Price Patterns' (1990), Opening Range Breakout establishes support and resistance during the initial minutes of trading. A decisive breakout beyond the opening boundary often signals directional conviction for the remainder of the trading day. — Toby Crabel / TradingView",
    params: &[ParamDef {
        name: "range_bars",
        default: "15",
        description: "Number of opening bars defining the range window",
    }],
    formula_source: "https://www.tradingview.com/support/solutions/43000696950-opening-range-breakout-orb/",
    formula_latex: r#"
\[
\text{Range High} = \max_{i \in [1, N]}(H_i), \quad \text{Range Low} = \min_{i \in [1, N]}(L_i) \\
\text{Range Size} = \text{Range High} - \text{Range Low} \\
\text{Target}_{1,\text{bull}} = \text{Range High} + 0.5 \cdot \text{Range Size}, \quad \text{Target}_{1,\text{bear}} = \text{Range Low} - 0.5 \cdot \text{Range Size} \\
\text{Target}_{2,\text{bull}} = \text{Range High} + 1.0 \cdot \text{Range Size}, \quad \text{Target}_{2,\text{bear}} = \text{Range Low} - 1.0 \cdot \text{Range Size}
\]
"#,
    gold_standard_file: "orb.json",
    category: "Price Action",
};

// ============================================================================
// 4. Unit Tests and Verification
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use proptest::prelude::*;

    #[test]
    fn test_session_vwap_single_bar() {
        let mut vwap = SessionVwap::new();
        // H = 105.0, L = 95.0, C = 100.0 => TP = 100.0, V = 50.0, is_session_start = true
        let state = vwap.next((105.0, 95.0, 100.0, 50.0, true));

        assert_relative_eq!(state.vwap, 100.0);
        assert_relative_eq!(state.std_dev, 0.0);
        assert_relative_eq!(state.upper_1, 100.0);
        assert_relative_eq!(state.lower_1, 100.0);
        assert_relative_eq!(state.upper_2, 100.0);
        assert_relative_eq!(state.lower_2, 100.0);
        assert_relative_eq!(state.upper_3, 100.0);
        assert_relative_eq!(state.lower_3, 100.0);
        assert_relative_eq!(state.cumulative_volume, 50.0);
        assert_eq!(state.bar_index, 0);
    }

    #[test]
    fn test_session_vwap_multi_bar_accumulation() {
        let mut vwap = SessionVwap::new();

        // Bar 0: TP = 100.0, V = 100.0
        let s0 = vwap.next((105.0, 95.0, 100.0, 100.0, true));
        assert_relative_eq!(s0.vwap, 100.0);
        assert_relative_eq!(s0.std_dev, 0.0);
        assert_eq!(s0.bar_index, 0);

        // Bar 1: TP = 110.0, V = 100.0
        // Expected VWAP = (100 * 100 + 110 * 100) / 200 = 105.0
        // Expected Variance = [100 * (100 - 105)^2 + 100 * (110 - 105)^2] / 200
        //                   = [100 * 25 + 100 * 25] / 200 = 5000 / 200 = 25.0
        // Expected StdDev = sqrt(25.0) = 5.0
        let s1 = vwap.next((115.0, 105.0, 110.0, 100.0, false));
        assert_relative_eq!(s1.vwap, 105.0);
        assert_relative_eq!(s1.std_dev, 5.0);
        assert_relative_eq!(s1.upper_1, 110.0);
        assert_relative_eq!(s1.lower_1, 100.0);
        assert_relative_eq!(s1.upper_2, 115.0);
        assert_relative_eq!(s1.lower_2, 95.0);
        assert_relative_eq!(s1.upper_3, 120.0);
        assert_relative_eq!(s1.lower_3, 90.0);
        assert_relative_eq!(s1.cumulative_volume, 200.0);
        assert_eq!(s1.bar_index, 1);

        // Verify band accessor methods
        assert_relative_eq!(s1.upper(SessionVwapBand::Sigma1), 110.0);
        assert_relative_eq!(s1.lower(SessionVwapBand::Sigma1), 100.0);
        assert_eq!(s1.band(SessionVwapBand::Sigma2), (115.0, 95.0));

        // Bar 2: TP = 120.0, V = 200.0
        // Expected Cumulative Volume = 400.0
        // Expected VWAP = (100*100 + 110*100 + 120*200) / 400 = (10000 + 11000 + 24000) / 400 = 45000 / 400 = 112.5
        // Expected Variance:
        //   100 * (100 - 112.5)^2 = 100 * 156.25 = 15625
        //   100 * (110 - 112.5)^2 = 100 * 6.25   = 625
        //   200 * (120 - 112.5)^2 = 200 * 56.25  = 11250
        //   Sum = 27500 / 400 = 68.75
        // Expected StdDev = sqrt(68.75) = 8.2915619758885
        let s2 = vwap.next((125.0, 115.0, 120.0, 200.0, false));
        assert_relative_eq!(s2.vwap, 112.5);
        assert_relative_eq!(s2.std_dev, 68.75_f64.sqrt(), epsilon = 1e-10);
        assert_relative_eq!(s2.upper_1, 112.5 + 68.75_f64.sqrt(), epsilon = 1e-10);
        assert_relative_eq!(s2.lower_1, 112.5 - 68.75_f64.sqrt(), epsilon = 1e-10);
        assert_relative_eq!(s2.cumulative_volume, 400.0);
        assert_eq!(s2.bar_index, 2);
    }

    #[test]
    fn test_session_vwap_session_reset() {
        let mut vwap = SessionVwap::new();

        vwap.next((105.0, 95.0, 100.0, 100.0, true));
        vwap.next((115.0, 105.0, 110.0, 100.0, false));

        // New session starts: counters must reset cleanly
        let reset_state = vwap.next((205.0, 195.0, 200.0, 50.0, true));
        assert_relative_eq!(reset_state.vwap, 200.0);
        assert_relative_eq!(reset_state.std_dev, 0.0);
        assert_relative_eq!(reset_state.cumulative_volume, 50.0);
        assert_eq!(reset_state.bar_index, 0);

        // Second bar in new session
        let next_state = vwap.next((215.0, 205.0, 210.0, 50.0, false));
        assert_relative_eq!(next_state.vwap, 205.0);
        assert_relative_eq!(next_state.std_dev, 5.0);
        assert_relative_eq!(next_state.cumulative_volume, 100.0);
        assert_eq!(next_state.bar_index, 1);
    }

    #[test]
    fn test_session_vwap_zero_volume_handling() {
        let mut vwap = SessionVwap::new();

        // Session start with zero volume
        let s0 = vwap.next((105.0, 95.0, 100.0, 0.0, true));
        assert_relative_eq!(s0.vwap, 100.0);
        assert_relative_eq!(s0.std_dev, 0.0);
        assert_relative_eq!(s0.cumulative_volume, 0.0);

        // Next bar also zero volume
        let s1 = vwap.next((108.0, 98.0, 103.0, 0.0, false));
        assert_relative_eq!(s1.vwap, 100.0);
        assert_relative_eq!(s1.std_dev, 0.0);
        assert_relative_eq!(s1.cumulative_volume, 0.0);
    }

    #[test]
    fn test_orb_range_formation_and_phase_transition() {
        let mut orb = OpeningRangeBreakout::new(3);

        // Bar 0 (Session Start): H = 105, L = 95, C = 100
        let s0 = orb.next((105.0, 95.0, 100.0, true));
        assert_eq!(s0.phase, OrbPhase::Forming);
        assert_eq!(s0.breakout, OrbBreakout::None);
        assert_relative_eq!(s0.range_high, 105.0);
        assert_relative_eq!(s0.range_low, 95.0);
        assert_relative_eq!(s0.range_size, 10.0);
        assert_relative_eq!(s0.target_1_bull, 110.0);
        assert_relative_eq!(s0.target_1_bear, 90.0);
        assert_relative_eq!(s0.target_2_bull, 115.0);
        assert_relative_eq!(s0.target_2_bear, 85.0);
        assert_eq!(s0.bar_index, 0);

        // Bar 1: H = 108, L = 94, C = 102
        let s1 = orb.next((108.0, 94.0, 102.0, false));
        assert_eq!(s1.phase, OrbPhase::Forming);
        assert_eq!(s1.breakout, OrbBreakout::None);
        assert_relative_eq!(s1.range_high, 108.0);
        assert_relative_eq!(s1.range_low, 94.0);
        assert_relative_eq!(s1.range_size, 14.0);
        assert_eq!(s1.bar_index, 1);

        // Bar 2: H = 107, L = 96, C = 101 (inside existing bounds)
        let s2 = orb.next((107.0, 96.0, 101.0, false));
        assert_eq!(s2.phase, OrbPhase::Forming);
        assert_eq!(s2.breakout, OrbBreakout::None);
        assert_relative_eq!(s2.range_high, 108.0);
        assert_relative_eq!(s2.range_low, 94.0);
        assert_eq!(s2.bar_index, 2);

        // Bar 3: Range complete -> transitions to Active!
        // H = 106, L = 98, C = 105 (inside range)
        let s3 = orb.next((106.0, 98.0, 105.0, false));
        assert_eq!(s3.phase, OrbPhase::Active);
        assert_eq!(s3.breakout, OrbBreakout::None);
        assert_relative_eq!(s3.range_high, 108.0); // Frozen!
        assert_relative_eq!(s3.range_low, 94.0); // Frozen!
        assert_relative_eq!(s3.range_size, 14.0);
        assert_relative_eq!(s3.target_1_bull, 115.0);
        assert_relative_eq!(s3.target_1_bear, 87.0);
        assert_relative_eq!(s3.target_2_bull, 122.0);
        assert_relative_eq!(s3.target_2_bear, 80.0);
        assert_eq!(s3.bar_index, 3);
    }

    #[test]
    fn test_orb_bullish_and_bearish_breakouts() {
        let mut orb = OpeningRangeBreakout::new(2);

        // Form 2-bar opening range: High = 110, Low = 90
        orb.next((105.0, 95.0, 100.0, true));
        orb.next((110.0, 90.0, 102.0, false));

        // Bar 2: Active, close = 112 > 110 => BullishBreakout
        let s_bull = orb.next((115.0, 108.0, 112.0, false));
        assert_eq!(s_bull.phase, OrbPhase::Active);
        assert_eq!(s_bull.breakout, OrbBreakout::BullishBreakout);
        assert!(s_bull.is_breakout());
        assert!(s_bull.breakout.is_bullish());
        assert!(!s_bull.breakout.is_bearish());

        // Target checking
        assert_relative_eq!(s_bull.range_size, 20.0);
        assert_relative_eq!(s_bull.target_1_bull, 120.0);
        assert!(!s_bull.target_1_reached(112.0));
        assert!(s_bull.target_1_reached(120.0));
        assert!(s_bull.target_1_reached(125.0));

        // Bar 3: Active, close = 88 < 90 => BearishBreakout
        let s_bear = orb.next((95.0, 85.0, 88.0, false));
        assert_eq!(s_bear.phase, OrbPhase::Active);
        assert_eq!(s_bear.breakout, OrbBreakout::BearishBreakout);
        assert!(s_bear.is_breakout());
        assert!(s_bear.breakout.is_bearish());
        assert!(!s_bear.breakout.is_bullish());

        // Bar 4: Active, close = 100 (inside 90..110) => None
        let s_none = orb.next((102.0, 98.0, 100.0, false));
        assert_eq!(s_none.phase, OrbPhase::Active);
        assert_eq!(s_none.breakout, OrbBreakout::None);
        assert!(!s_none.is_breakout());
    }

    #[test]
    fn test_orb_session_reset() {
        let mut orb = OpeningRangeBreakout::new(2);

        orb.next((105.0, 95.0, 100.0, true));
        orb.next((110.0, 90.0, 102.0, false));
        let active = orb.next((115.0, 105.0, 112.0, false));
        assert_eq!(active.phase, OrbPhase::Active);

        // New session begins
        let reset_state = orb.next((205.0, 195.0, 200.0, true));
        assert_eq!(reset_state.phase, OrbPhase::Forming);
        assert_eq!(reset_state.breakout, OrbBreakout::None);
        assert_relative_eq!(reset_state.range_high, 205.0);
        assert_relative_eq!(reset_state.range_low, 195.0);
        assert_relative_eq!(reset_state.range_size, 10.0);
        assert_eq!(reset_state.bar_index, 0);
    }

    #[test]
    fn test_orb_max_session_bars_closed() {
        let mut orb = OpeningRangeBreakout::new(2).with_max_session_bars(4);

        orb.next((105.0, 95.0, 100.0, true)); // bar 0
        orb.next((108.0, 94.0, 102.0, false)); // bar 1
        let s2 = orb.next((112.0, 106.0, 111.0, false)); // bar 2: Active
        assert_eq!(s2.phase, OrbPhase::Active);
        let s3 = orb.next((115.0, 107.0, 114.0, false)); // bar 3: Active
        assert_eq!(s3.phase, OrbPhase::Active);

        // bar 4: Reached max session bars (4) -> Closed
        let s4 = orb.next((120.0, 110.0, 118.0, false));
        assert_eq!(s4.phase, OrbPhase::Closed);
        assert_eq!(s4.breakout, OrbBreakout::None);
        assert!(s4.phase.is_closed());
    }

    #[test]
    fn test_orb_try_new_invalid_range_bars() {
        assert_eq!(
            OpeningRangeBreakout::try_new(0),
            Err(SessionLevelsError::InvalidRangeBars(0))
        );
        assert!(OpeningRangeBreakout::try_new(5).is_ok());
    }

    #[test]
    fn test_metadata_consistency() {
        assert_eq!(
            SESSION_VWAP_METADATA.name,
            "Session VWAP with Standard Deviation Bands"
        );
        assert_eq!(
            SESSION_VWAP_METADATA.gold_standard_file,
            "session_vwap.json"
        );
        assert_eq!(SESSION_VWAP_METADATA.category, "Volume");

        assert_eq!(ORB_METADATA.name, "Opening Range Breakout");
        assert_eq!(ORB_METADATA.gold_standard_file, "orb.json");
        assert_eq!(ORB_METADATA.category, "Price Action");
    }

    // ========================================================================
    // Property-Based Tests (Streaming vs Batch Parity)
    // ========================================================================

    fn session_vwap_batch(data: &[(f64, f64, f64, f64, bool)]) -> Vec<SessionVwapState> {
        let mut results = Vec::with_capacity(data.len());
        let mut session_bars: Vec<(f64, f64)> = Vec::new(); // (TP, Volume)
        let mut bar_index = 0;

        for &(high, low, close, volume, is_session_start) in data {
            let tp = (high + low + close) / 3.0;
            let vol = if volume > 0.0 { volume } else { 0.0 };

            if session_bars.is_empty() || is_session_start {
                session_bars.clear();
                bar_index = 0;
            } else {
                bar_index += 1;
            }

            session_bars.push((tp, vol));

            let total_vol: f64 = session_bars.iter().map(|&(_, v)| v).sum();
            let vwap = if total_vol > 0.0 {
                session_bars.iter().map(|&(p, v)| p * v).sum::<f64>() / total_vol
            } else {
                tp
            };

            let variance = if total_vol > 0.0 {
                let sum_sq_diff: f64 = session_bars
                    .iter()
                    .map(|&(p, v)| v * (p - vwap).powi(2))
                    .sum();
                (sum_sq_diff / total_vol).max(0.0)
            } else {
                0.0
            };
            let std_dev = variance.sqrt();

            results.push(SessionVwapState {
                vwap,
                upper_1: vwap + 1.0 * std_dev,
                lower_1: vwap - 1.0 * std_dev,
                upper_2: vwap + 2.0 * std_dev,
                lower_2: vwap - 2.0 * std_dev,
                upper_3: vwap + 3.0 * std_dev,
                lower_3: vwap - 3.0 * std_dev,
                std_dev,
                cumulative_volume: total_vol,
                bar_index,
            });
        }

        results
    }

    fn orb_batch(data: &[(f64, f64, f64, bool)], range_bars: usize) -> Vec<OrbState> {
        let range_bars = range_bars.max(1);
        let mut results = Vec::with_capacity(data.len());
        let mut session_highs: Vec<f64> = Vec::new();
        let mut session_lows: Vec<f64> = Vec::new();

        for &(high, low, close, is_session_start) in data {
            let h = high.max(low);
            let l = high.min(low);

            if session_highs.is_empty() || is_session_start {
                session_highs.clear();
                session_lows.clear();
            }

            let bar_idx = session_highs.len();
            session_highs.push(h);
            session_lows.push(l);

            let forming_window = (bar_idx + 1).min(range_bars);
            let range_high = session_highs[..forming_window]
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            let range_low = session_lows[..forming_window]
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min);
            let range_size = (range_high - range_low).max(0.0);

            let phase = if bar_idx < range_bars {
                OrbPhase::Forming
            } else {
                OrbPhase::Active
            };

            let breakout = if phase == OrbPhase::Active {
                if close > range_high {
                    OrbBreakout::BullishBreakout
                } else if close < range_low {
                    OrbBreakout::BearishBreakout
                } else {
                    OrbBreakout::None
                }
            } else {
                OrbBreakout::None
            };

            results.push(OrbState {
                phase,
                breakout,
                range_high,
                range_low,
                range_size,
                target_1_bull: range_high + 0.5 * range_size,
                target_1_bear: range_low - 0.5 * range_size,
                target_2_bull: range_high + 1.0 * range_size,
                target_2_bear: range_low - 1.0 * range_size,
                bar_index: bar_idx,
            });
        }

        results
    }

    proptest! {
        #[test]
        fn test_session_vwap_parity(
            prices in prop::collection::vec((10.0..500.0f64, 10.0..500.0f64, 10.0..500.0f64, 1.0..1000.0f64, any::<bool>()), 1..100)
        ) {
            let mut sanitized_input: Vec<(f64, f64, f64, f64, bool)> = Vec::with_capacity(prices.len());
            for (i, (h_raw, l_raw, c_raw, vol, is_start)) in prices.into_iter().enumerate() {
                let high: f64 = h_raw.max(l_raw).max(c_raw);
                let low: f64 = h_raw.min(l_raw).min(c_raw);
                let is_session_start = if i == 0 { true } else { is_start };
                sanitized_input.push((high, low, c_raw, vol, is_session_start));
            }

            let mut indicator = SessionVwap::new();
            let mut streaming_results = Vec::with_capacity(sanitized_input.len());
            for &val in &sanitized_input {
                streaming_results.push(indicator.next(val));
            }

            let batch_results = session_vwap_batch(&sanitized_input);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                assert_relative_eq!(s.vwap, b.vwap, epsilon = 1e-8);
                assert_relative_eq!(s.std_dev, b.std_dev, epsilon = 1e-8);
                assert_relative_eq!(s.upper_1, b.upper_1, epsilon = 1e-8);
                assert_relative_eq!(s.lower_1, b.lower_1, epsilon = 1e-8);
                assert_relative_eq!(s.upper_2, b.upper_2, epsilon = 1e-8);
                assert_relative_eq!(s.lower_2, b.lower_2, epsilon = 1e-8);
                assert_relative_eq!(s.upper_3, b.upper_3, epsilon = 1e-8);
                assert_relative_eq!(s.lower_3, b.lower_3, epsilon = 1e-8);
                assert_relative_eq!(s.cumulative_volume, b.cumulative_volume, epsilon = 1e-8);
                assert_eq!(s.bar_index, b.bar_index);
            }
        }

        #[test]
        fn test_orb_parity(
            bars in prop::collection::vec((10.0..500.0f64, 10.0..500.0f64, 10.0..500.0f64, any::<bool>()), 1..100),
            range_bars in 1..20usize,
        ) {
            let mut sanitized_input: Vec<(f64, f64, f64, bool)> = Vec::with_capacity(bars.len());
            for (i, (h_raw, l_raw, c_raw, is_start)) in bars.into_iter().enumerate() {
                let high: f64 = h_raw.max(l_raw).max(c_raw);
                let low: f64 = h_raw.min(l_raw).min(c_raw);
                let is_session_start = if i == 0 { true } else { is_start };
                sanitized_input.push((high, low, c_raw, is_session_start));
            }

            let mut indicator = OpeningRangeBreakout::new(range_bars);
            let mut streaming_results = Vec::with_capacity(sanitized_input.len());
            for &val in &sanitized_input {
                streaming_results.push(indicator.next(val));
            }

            let batch_results = orb_batch(&sanitized_input, range_bars);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                assert_eq!(s.phase, b.phase);
                assert_eq!(s.breakout, b.breakout);
                assert_relative_eq!(s.range_high, b.range_high, epsilon = 1e-10);
                assert_relative_eq!(s.range_low, b.range_low, epsilon = 1e-10);
                assert_relative_eq!(s.range_size, b.range_size, epsilon = 1e-10);
                assert_relative_eq!(s.target_1_bull, b.target_1_bull, epsilon = 1e-10);
                assert_relative_eq!(s.target_1_bear, b.target_1_bear, epsilon = 1e-10);
                assert_relative_eq!(s.target_2_bull, b.target_2_bull, epsilon = 1e-10);
                assert_relative_eq!(s.target_2_bear, b.target_2_bear, epsilon = 1e-10);
                assert_eq!(s.bar_index, b.bar_index);
            }
        }
    }
}
