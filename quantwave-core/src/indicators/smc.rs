//! Structure & SMC (Smart Money Concepts) Suite
//!
//! High-performance, streaming-first implementation of Smart Money Concepts (SMC)
//! and Market Structure analytics for algorithmic trading and quantitative research.
//!
//! # Included Detectors
//!
//! 1. [`Choch`] / [`ChochDetector`]:
//!    Market Structure Break (BOS) and Change of Character (CHoCH) detector.
//!    Tracks swing highs and swing lows via a symmetric sliding window, determines
//!    structural trend (Bullish, Bearish, Undetermined), and emits rich break events:
//!    - **BOS (Break of Structure)**: Continuation of current trend (e.g., closing above
//!      previous swing high in an uptrend, or below previous swing low in a downtrend).
//!    - **CHoCH (Change of Character)**: Reversal of structure (e.g., closing below
//!      previous swing low in an uptrend, or above previous swing high in a downtrend).
//!
//! 2. [`OrderBlock`] / [`OrderBlockDetector`]:
//!    Institutional Order Block (OB) detector with lifecycle mitigation tracking:
//!    - **Bullish Order Block**: The last down-closing candle prior to a strong bullish
//!      breakout/displacement. Zone covers `[low, high]` (or body).
//!    - **Bearish Order Block**: The last up-closing candle prior to a strong bearish
//!      breakdown/displacement. Zone covers `[low, high]` (or body).
//!    - Tracks incoming price action against active order blocks: unmitigated, partially
//!      mitigated (tested), and fully mitigated (violated / invalidated).
//!
//! 3. [`FairValueGap`] / [`FairValueGapDetector`]:
//!    3-bar price imbalance detector (Fair Value Gap / FVG):
//!    - **Bullish FVG**: `low[t] > high[t - 2]`. Imbalance zone is `[high[t - 2], low[t]]`.
//!    - **Bearish FVG**: `high[t] < low[t - 2]`. Imbalance zone is `[high[t], low[t - 2]]`.
//!    - Tracks retest penetration and full mitigation as subsequent candles fill the imbalance.
//!
//! # Formulas and References
//!
//! - **CHoCH / BOS**:
//!   Canonical SMC market structure:
//!   $$
//!   \begin{aligned}
//!   \text{SwingHigh}_t &= \operatorname{high}_t \ge \operatorname{high}_{t \pm i}, \quad \forall i \in [1, L] \\
//!   \text{SwingLow}_t  &= \operatorname{low}_t \le \operatorname{low}_{t \pm i}, \quad \forall i \in [1, L] \\
//!   \text{BOS}_{\text{bull}}  &= \text{Close} > \text{SwingHigh} \quad (\text{Trend} = \text{Bullish}) \\
//!   \text{CHoCH}_{\text{bear}} &= \text{Close} < \text{SwingLow} \quad (\text{Trend} = \text{Bullish} \to \text{Bearish})
//!   \end{aligned}
//!   $$
//!   Source: <https://www.tradingview.com/support/solutions/43000703076-market-structure/>
//!
//! - **Order Blocks**:
//!   Institutional footprints preceding displacement moves:
//!   $$
//!   \text{Bullish OB} = \text{Candle}_{t-1} (\text{Close} \le \text{Open}) \land \text{Close}_t > \text{High}_{t-1}
//!   $$
//!   Source: <https://www.tradingview.com/support/solutions/43000703077-order-blocks/>
//!
//! - **Fair Value Gaps**:
//!   3-bar price imbalance / liquidity voids:
//!   $$
//!   \text{Bullish FVG} = \text{Low}_t > \text{High}_{t-2}, \quad \text{Bearish FVG} = \text{High}_t < \text{Low}_{t-2}
//!   $$
//!   Source: <https://www.tradingview.com/support/solutions/43000703078-fair-value-gaps/>

use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

// ============================================================================
// Metadata Definitions
// ============================================================================

pub const CHOCH_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "choch",
    description: "Detects Market Structure Breaks (BOS) and Changes of Character (CHoCH) using swing highs and lows.",
    usage: "Identifies trend continuations (BOS) and trend reversals (CHoCH) in market structure for Smart Money Concepts (SMC) trading strategies.",
    keywords: &["smc", "market structure", "choch", "bos", "swings", "trend"],
    ehlers_summary: "",
    params: &[ParamDef {
        name: "swing_strength",
        default: "5",
        description: "Number of bars required on each side to confirm a swing high or swing low.",
    }],
    formula_source: "https://www.tradingview.com/support/solutions/43000703076-market-structure/",
    formula_latex: r"\text{BOS} = \text{Close} > \text{SwingHigh}_{\text{bull}} \lor \text{Close} < \text{SwingLow}_{\text{bear}}, \quad \text{CHoCH} = \text{Close} < \text{SwingLow}_{\text{bull}} \lor \text{Close} > \text{SwingHigh}_{\text{bear}}",
    gold_standard_file: "choch.json",
    category: "Price Action / SMC",
};

pub const ORDER_BLOCK_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "order_block",
    description: "Detects Bullish and Bearish Order Blocks (institutional entry zones) and tracks their mitigation.",
    usage: "Identifies institutional order blocks preceding energetic displacements and monitors subsequent mitigation/invalidation.",
    keywords: &[
        "smc",
        "order block",
        "ob",
        "smart money",
        "mitigation",
        "institutional",
    ],
    ehlers_summary: "",
    params: &[ParamDef {
        name: "max_active_blocks",
        default: "20",
        description: "Maximum number of unmitigated or partially mitigated active order blocks retained in memory.",
    }],
    formula_source: "https://www.tradingview.com/support/solutions/43000703077-order-blocks/",
    formula_latex: r"\text{Bullish OB} = \text{Candle}_{t-1} \text{ (down)} \land \text{Close}_t > \text{High}_{t-1}, \quad \text{Bearish OB} = \text{Candle}_{t-1} \text{ (up)} \land \text{Close}_t < \text{Low}_{t-1}",
    gold_standard_file: "order_block.json",
    category: "Price Action / SMC",
};

pub const FAIR_VALUE_GAP_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "fair_value_gap",
    description: "Detects 3-bar Fair Value Gaps (price imbalances) and tracks their mitigation and fill levels.",
    usage: "Identifies liquidity imbalances where price moves impulsively, leaving untraded zones that act as magnets for price retests.",
    keywords: &[
        "smc",
        "fvg",
        "fair value gap",
        "imbalance",
        "liquidity",
        "smart money",
    ],
    ehlers_summary: "",
    params: &[
        ParamDef {
            name: "min_gap_size",
            default: "0.0",
            description: "Minimum price distance required between outer bars to register a valid Fair Value Gap.",
        },
        ParamDef {
            name: "max_active_gaps",
            default: "20",
            description: "Maximum number of unmitigated or partially mitigated fair value gaps retained in memory.",
        },
    ],
    formula_source: "https://www.tradingview.com/support/solutions/43000703078-fair-value-gaps/",
    formula_latex: r"\text{Bullish FVG} = \text{Low}_t > \text{High}_{t-2}, \quad \text{Bearish FVG} = \text{High}_t < \text{Low}_{t-2}",
    gold_standard_file: "fair_value_gap.json",
    category: "Price Action / SMC",
};

// ============================================================================
// Shared Domain Types & Enums
// ============================================================================

/// Structural market trend derived from swing highs and swing lows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructureTrend {
    /// Consecutive higher highs and higher lows, or confirmed bullish break.
    Bullish,
    /// Consecutive lower lows and lower highs, or confirmed bearish break.
    Bearish,
    /// Insufficient swing structure to determine definitive trend direction.
    Undetermined,
}

impl StructureTrend {
    #[inline]
    pub fn is_bullish(self) -> bool {
        self == Self::Bullish
    }

    #[inline]
    pub fn is_bearish(self) -> bool {
        self == Self::Bearish
    }

    #[inline]
    pub fn is_undetermined(self) -> bool {
        self == Self::Undetermined
    }
}

/// Category of structure break.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructureBreakType {
    /// Break of Structure: trend continuation break.
    Bos,
    /// Change of Character: trend reversal break.
    Choch,
}

/// Direction of structure break.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructureBreakDirection {
    /// Price breaks upward through resistance / swing high.
    Bullish,
    /// Price breaks downward through support / swing low.
    Bearish,
}

/// Type of swing extreme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwingType {
    /// Local peak.
    High,
    /// Local trough.
    Low,
}

/// Confirmed swing extreme with bar index and price level.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SmcSwingPoint {
    /// Absolute bar index where the swing extreme occurred.
    pub bar: usize,
    /// Price of the swing high or swing low.
    pub price: f64,
    /// Type of swing (High or Low).
    pub swing_type: SwingType,
}

/// Structural break event emitted when price closes beyond an active swing level.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StructureBreakEvent {
    /// Break classification (BOS or CHoCH).
    pub break_type: StructureBreakType,
    /// Direction of the breakout.
    pub direction: StructureBreakDirection,
    /// Price level of the broken swing point.
    pub level: f64,
    /// Bar index of the broken swing point.
    pub broken_swing_bar: usize,
    /// Bar index that triggered the confirmation break.
    pub trigger_bar: usize,
    /// Close price of the triggering bar.
    pub trigger_price: f64,
}

/// Mitigation status for order blocks and fair value gaps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MitigationStatus {
    /// Zone has not been touched by subsequent price action.
    Unmitigated,
    /// Price has penetrated into the zone without completely violating/filling it.
    PartiallyMitigated,
    /// Zone has been fully penetrated, filled, or invalidated.
    FullyMitigated,
}

// ============================================================================
// 1. Choch / ChochDetector Indicator
// ============================================================================

/// Snapshot output of the Market Structure (CHoCH / BOS) detector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChochState {
    /// Current structural trend bias.
    pub trend: StructureTrend,
    /// Most recent structure break event (if any break has occurred).
    pub last_break: Option<StructureBreakEvent>,
    /// Most recently confirmed swing high.
    pub last_swing_high: Option<SmcSwingPoint>,
    /// Most recently confirmed swing low.
    pub last_swing_low: Option<SmcSwingPoint>,
    /// Current bar index.
    pub bar_index: usize,
}

impl ChochState {
    /// Returns the break event if one occurred on this exact bar.
    #[inline]
    pub fn current_break(&self) -> Option<StructureBreakEvent> {
        self.last_break.filter(|b| b.trigger_bar == self.bar_index)
    }

    #[inline]
    pub fn is_bullish(&self) -> bool {
        self.trend.is_bullish()
    }

    #[inline]
    pub fn is_bearish(&self) -> bool {
        self.trend.is_bearish()
    }
}

/// Market Structure detector tracking Swings, BOS, and CHoCH.
#[derive(Debug, Clone)]
pub struct Choch {
    swing_strength: usize,
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
    bar_index: usize,
    trend: StructureTrend,
    last_swing_high: Option<SmcSwingPoint>,
    prev_swing_high: Option<SmcSwingPoint>,
    last_swing_low: Option<SmcSwingPoint>,
    prev_swing_low: Option<SmcSwingPoint>,
    broken_high: bool,
    broken_low: bool,
    last_break: Option<StructureBreakEvent>,
}

pub type ChochDetector = Choch;

impl Choch {
    /// Construct a new `Choch` detector with specified swing strength.
    ///
    /// `swing_strength` defines the number of bars on either side required to confirm
    /// a swing high or low. Clamped to a minimum of 1.
    pub fn new(swing_strength: usize) -> Self {
        let strength = swing_strength.max(1);
        Self {
            swing_strength: strength,
            highs: VecDeque::with_capacity(2 * strength + 1),
            lows: VecDeque::with_capacity(2 * strength + 1),
            bar_index: 0,
            trend: StructureTrend::Undetermined,
            last_swing_high: None,
            prev_swing_high: None,
            last_swing_low: None,
            prev_swing_low: None,
            broken_high: false,
            broken_low: false,
            last_break: None,
        }
    }

    /// Reset internal state machine.
    pub fn reset(&mut self) {
        self.highs.clear();
        self.lows.clear();
        self.bar_index = 0;
        self.trend = StructureTrend::Undetermined;
        self.last_swing_high = None;
        self.prev_swing_high = None;
        self.last_swing_low = None;
        self.prev_swing_low = None;
        self.broken_high = false;
        self.broken_low = false;
        self.last_break = None;
    }

    /// Helper to process a slice of `(high, low, close)` bars.
    pub fn process_batch(&mut self, bars: &[(f64, f64, f64)]) -> Vec<ChochState> {
        let mut results = Vec::with_capacity(bars.len());
        for &bar in bars {
            results.push(self.next(bar));
        }
        results
    }
}

impl Default for Choch {
    fn default() -> Self {
        Self::new(5)
    }
}

impl Next<(f64, f64, f64)> for Choch {
    type Output = ChochState;

    fn next(&mut self, input: (f64, f64, f64)) -> Self::Output {
        let (high, low, close) = input;

        // Fallback for non-finite prices
        if !high.is_finite() || !low.is_finite() || !close.is_finite() {
            return ChochState {
                trend: self.trend,
                last_break: self.last_break,
                last_swing_high: self.last_swing_high,
                last_swing_low: self.last_swing_low,
                bar_index: self.bar_index,
            };
        }

        let curr_bar = self.bar_index;
        let window_len = 2 * self.swing_strength + 1;

        // Maintain sliding window for swing detection
        self.highs.push_back(high);
        self.lows.push_back(low);
        if self.highs.len() > window_len {
            self.highs.pop_front();
            self.lows.pop_front();
        }

        // 1. Swing Confirmation at candidate bar: curr_bar - swing_strength
        if self.highs.len() == window_len {
            let candidate_bar = curr_bar - self.swing_strength;
            let center_high = self.highs[self.swing_strength];
            let center_low = self.lows[self.swing_strength];

            let mut is_swing_high = true;
            let mut is_swing_low = true;

            for i in 0..window_len {
                if i == self.swing_strength {
                    continue;
                }
                if i < self.swing_strength {
                    if self.highs[i] > center_high {
                        is_swing_high = false;
                    }
                    if self.lows[i] < center_low {
                        is_swing_low = false;
                    }
                } else {
                    // Strict inequality on right side prevents duplicate consecutive swings
                    if self.highs[i] >= center_high {
                        is_swing_high = false;
                    }
                    if self.lows[i] <= center_low {
                        is_swing_low = false;
                    }
                }
            }

            if is_swing_high {
                let point = SmcSwingPoint {
                    bar: candidate_bar,
                    price: center_high,
                    swing_type: SwingType::High,
                };
                self.prev_swing_high = self.last_swing_high;
                self.last_swing_high = Some(point);
                self.broken_high = false;
            }

            if is_swing_low {
                let point = SmcSwingPoint {
                    bar: candidate_bar,
                    price: center_low,
                    swing_type: SwingType::Low,
                };
                self.prev_swing_low = self.last_swing_low;
                self.last_swing_low = Some(point);
                self.broken_low = false;
            }

            // Derive initial trend if Undetermined from consecutive swing points
            if self.trend == StructureTrend::Undetermined
                && let (Some(sh), Some(psh), Some(sl), Some(psl)) = (
                    self.last_swing_high,
                    self.prev_swing_high,
                    self.last_swing_low,
                    self.prev_swing_low,
                )
            {
                if sh.price > psh.price && sl.price > psl.price {
                    self.trend = StructureTrend::Bullish;
                } else if sh.price < psh.price && sl.price < psl.price {
                    self.trend = StructureTrend::Bearish;
                }
            }
        }

        // 2. Break Detection on current bar close
        let mut triggered_break = None;

        // Check break above last active swing high
        if let Some(sh) = self.last_swing_high
            && !self.broken_high
            && close > sh.price
        {
            let (b_type, b_dir) = match self.trend {
                StructureTrend::Bullish => {
                    (StructureBreakType::Bos, StructureBreakDirection::Bullish)
                }
                StructureTrend::Bearish => {
                    self.trend = StructureTrend::Bullish;
                    (StructureBreakType::Choch, StructureBreakDirection::Bullish)
                }
                StructureTrend::Undetermined => {
                    self.trend = StructureTrend::Bullish;
                    (StructureBreakType::Choch, StructureBreakDirection::Bullish)
                }
            };
            self.broken_high = true;
            let event = StructureBreakEvent {
                break_type: b_type,
                direction: b_dir,
                level: sh.price,
                broken_swing_bar: sh.bar,
                trigger_bar: curr_bar,
                trigger_price: close,
            };
            triggered_break = Some(event);
            self.last_break = Some(event);
        }

        // Check break below last active swing low (if high was not broken on same bar)
        if triggered_break.is_none()
            && let Some(sl) = self.last_swing_low
            && !self.broken_low
            && close < sl.price
        {
            let (b_type, b_dir) = match self.trend {
                StructureTrend::Bearish => {
                    (StructureBreakType::Bos, StructureBreakDirection::Bearish)
                }
                StructureTrend::Bullish => {
                    self.trend = StructureTrend::Bearish;
                    (StructureBreakType::Choch, StructureBreakDirection::Bearish)
                }
                StructureTrend::Undetermined => {
                    self.trend = StructureTrend::Bearish;
                    (StructureBreakType::Choch, StructureBreakDirection::Bearish)
                }
            };
            self.broken_low = true;
            let event = StructureBreakEvent {
                break_type: b_type,
                direction: b_dir,
                level: sl.price,
                broken_swing_bar: sl.bar,
                trigger_bar: curr_bar,
                trigger_price: close,
            };
            self.last_break = Some(event);
        }

        self.bar_index += 1;

        ChochState {
            trend: self.trend,
            last_break: self.last_break,
            last_swing_high: self.last_swing_high,
            last_swing_low: self.last_swing_low,
            bar_index: curr_bar,
        }
    }
}

/// Compute `Choch` over a batch of `(high, low, close)` bars.
pub fn choch_batch(inputs: &[(f64, f64, f64)], swing_strength: usize) -> Vec<ChochState> {
    let mut detector = Choch::new(swing_strength);
    detector.process_batch(inputs)
}

// ============================================================================
// 2. OrderBlock / OrderBlockDetector Indicator
// ============================================================================

/// Order block classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderBlockType {
    /// Bullish order block: down-candle before upward displacement.
    Bullish,
    /// Bearish order block: up-candle before downward displacement.
    Bearish,
}

/// Calculation mode for the order block price zone boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderBlockZoneMode {
    /// Full candle wick range: `[low, high]`.
    Wick,
    /// Real body range: `[min(open, close), max(open, close)]`.
    Body,
}

/// Structural representation of an identified Order Block.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OrderBlockZone {
    /// Monotonically increasing unique ID.
    pub id: u64,
    /// Bullish or Bearish type.
    pub block_type: OrderBlockType,
    /// Bar index of the order block candle (the down/up candle).
    pub bar_index: usize,
    /// Bar index where the breakout candle confirmed the displacement.
    pub trigger_bar: usize,
    /// Open price of the order block candle.
    pub open: f64,
    /// High price of the order block candle.
    pub high: f64,
    /// Low price of the order block candle.
    pub low: f64,
    /// Close price of the order block candle.
    pub close: f64,
    /// Upper price boundary of the zone.
    pub zone_high: f64,
    /// Lower price boundary of the zone.
    pub zone_low: f64,
    /// Mitigation status.
    pub mitigation: MitigationStatus,
    /// Bar index where mitigation occurred (if any).
    pub mitigation_bar: Option<usize>,
}

impl OrderBlockZone {
    #[inline]
    pub fn is_unmitigated(&self) -> bool {
        self.mitigation == MitigationStatus::Unmitigated
    }

    #[inline]
    pub fn is_partially_mitigated(&self) -> bool {
        self.mitigation == MitigationStatus::PartiallyMitigated
    }

    #[inline]
    pub fn is_fully_mitigated(&self) -> bool {
        self.mitigation == MitigationStatus::FullyMitigated
    }
}

/// Snapshot output of the Order Block detector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderBlockState {
    /// Currently active (unmitigated or partially mitigated) order blocks.
    pub active_blocks: Vec<OrderBlockZone>,
    /// Most recently formed order block (if any).
    pub latest_formed: Option<OrderBlockZone>,
    /// Current bar index.
    pub bar_index: usize,
}

impl OrderBlockState {
    /// Returns the order block formed on this exact bar, if any.
    #[inline]
    pub fn current_formed(&self) -> Option<&OrderBlockZone> {
        self.latest_formed
            .as_ref()
            .filter(|b| b.trigger_bar == self.bar_index)
    }
}

/// Tunable parameters for Order Block detection.
#[derive(Debug, Clone)]
pub struct OrderBlockConfig {
    /// Maximum number of active order blocks retained in memory.
    pub max_active_blocks: usize,
    /// Zone calculation mode (Wick vs Body).
    pub zone_mode: OrderBlockZoneMode,
    /// Minimum displacement ratio: `displacement_body / prev_candle_range`. Default: 0.0.
    pub min_displacement_ratio: f64,
}

impl Default for OrderBlockConfig {
    fn default() -> Self {
        Self {
            max_active_blocks: 20,
            zone_mode: OrderBlockZoneMode::Wick,
            min_displacement_ratio: 0.0,
        }
    }
}

/// Institutional Order Block detector with lifecycle mitigation tracking.
#[derive(Debug, Clone)]
pub struct OrderBlock {
    config: OrderBlockConfig,
    active_blocks: Vec<OrderBlockZone>,
    latest_formed: Option<OrderBlockZone>,
    prev_candle: Option<(f64, f64, f64, f64, usize)>, // (open, high, low, close, bar_index)
    bar_index: usize,
    next_id: u64,
}

pub type OrderBlockDetector = OrderBlock;

impl OrderBlock {
    /// Construct a new `OrderBlock` detector with capacity limit.
    pub fn new(max_active_blocks: usize) -> Self {
        Self::with_config(OrderBlockConfig {
            max_active_blocks: max_active_blocks.max(1),
            ..OrderBlockConfig::default()
        })
    }

    /// Construct with full configuration.
    pub fn with_config(config: OrderBlockConfig) -> Self {
        let max_active = config.max_active_blocks.max(1);
        Self {
            config: OrderBlockConfig {
                max_active_blocks: max_active,
                ..config
            },
            active_blocks: Vec::with_capacity(max_active),
            latest_formed: None,
            prev_candle: None,
            bar_index: 0,
            next_id: 1,
        }
    }

    /// Reset internal state machine.
    pub fn reset(&mut self) {
        self.active_blocks.clear();
        self.latest_formed = None;
        self.prev_candle = None;
        self.bar_index = 0;
        self.next_id = 1;
    }

    /// Helper to process a batch of `(open, high, low, close)` bars.
    pub fn process_batch(&mut self, bars: &[(f64, f64, f64, f64)]) -> Vec<OrderBlockState> {
        let mut results = Vec::with_capacity(bars.len());
        for &bar in bars {
            results.push(self.next(bar));
        }
        results
    }
}

impl Default for OrderBlock {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Next<(f64, f64, f64, f64)> for OrderBlock {
    type Output = OrderBlockState;

    fn next(&mut self, input: (f64, f64, f64, f64)) -> Self::Output {
        let (open, high, low, close) = input;

        // Fallback for non-finite values
        if !open.is_finite() || !high.is_finite() || !low.is_finite() || !close.is_finite() {
            return OrderBlockState {
                active_blocks: self.active_blocks.clone(),
                latest_formed: self.latest_formed,
                bar_index: self.bar_index,
            };
        }

        let curr_bar = self.bar_index;

        // 1. Prune blocks fully mitigated in previous bars (retaining them on the mitigation bar)
        self.active_blocks
            .retain(|b| b.mitigation != MitigationStatus::FullyMitigated);

        // 2. Update mitigation status of active blocks with incoming bar
        for block in &mut self.active_blocks {
            match block.block_type {
                OrderBlockType::Bullish => {
                    if low <= block.zone_high {
                        if close < block.zone_low {
                            // Violated / fully invalidated
                            block.mitigation = MitigationStatus::FullyMitigated;
                            block.mitigation_bar = Some(curr_bar);
                        } else if block.mitigation == MitigationStatus::Unmitigated {
                            // Tested / penetrated into zone
                            block.mitigation = MitigationStatus::PartiallyMitigated;
                            block.mitigation_bar = Some(curr_bar);
                        }
                    }
                }
                OrderBlockType::Bearish => {
                    if high >= block.zone_low {
                        if close > block.zone_high {
                            // Violated / fully invalidated
                            block.mitigation = MitigationStatus::FullyMitigated;
                            block.mitigation_bar = Some(curr_bar);
                        } else if block.mitigation == MitigationStatus::Unmitigated {
                            // Tested / penetrated into zone
                            block.mitigation = MitigationStatus::PartiallyMitigated;
                            block.mitigation_bar = Some(curr_bar);
                        }
                    }
                }
            }
        }

        // 3. Detect new Order Block formation from previous candle -> current candle displacement
        if let Some((p_open, p_high, p_low, p_close, p_bar)) = self.prev_candle {
            let prev_range = (p_high - p_low).max(1e-12);

            // Bullish OB: prior down-candle, current candle closes above prior high
            if p_close < p_open && close > open && close > p_high {
                let displacement_body = close - open;
                if displacement_body / prev_range >= self.config.min_displacement_ratio {
                    let (z_low, z_high) = match self.config.zone_mode {
                        OrderBlockZoneMode::Wick => (p_low, p_high),
                        OrderBlockZoneMode::Body => (p_close, p_open),
                    };

                    let new_block = OrderBlockZone {
                        id: self.next_id,
                        block_type: OrderBlockType::Bullish,
                        bar_index: p_bar,
                        trigger_bar: curr_bar,
                        open: p_open,
                        high: p_high,
                        low: p_low,
                        close: p_close,
                        zone_high: z_high,
                        zone_low: z_low,
                        mitigation: MitigationStatus::Unmitigated,
                        mitigation_bar: None,
                    };
                    self.next_id += 1;
                    self.latest_formed = Some(new_block);
                    self.active_blocks.push(new_block);
                }
            }
            // Bearish OB: prior up-candle, current candle closes below prior low
            else if p_close > p_open && close < open && close < p_low {
                let displacement_body = open - close;
                if displacement_body / prev_range >= self.config.min_displacement_ratio {
                    let (z_low, z_high) = match self.config.zone_mode {
                        OrderBlockZoneMode::Wick => (p_low, p_high),
                        OrderBlockZoneMode::Body => (p_open, p_close),
                    };

                    let new_block = OrderBlockZone {
                        id: self.next_id,
                        block_type: OrderBlockType::Bearish,
                        bar_index: p_bar,
                        trigger_bar: curr_bar,
                        open: p_open,
                        high: p_high,
                        low: p_low,
                        close: p_close,
                        zone_high: z_high,
                        zone_low: z_low,
                        mitigation: MitigationStatus::Unmitigated,
                        mitigation_bar: None,
                    };
                    self.next_id += 1;
                    self.latest_formed = Some(new_block);
                    self.active_blocks.push(new_block);
                }
            }
        }

        // Enforce maximum active blocks capacity
        while self.active_blocks.len() > self.config.max_active_blocks {
            self.active_blocks.remove(0);
        }

        self.prev_candle = Some((open, high, low, close, curr_bar));
        self.bar_index += 1;

        OrderBlockState {
            active_blocks: self.active_blocks.clone(),
            latest_formed: self.latest_formed,
            bar_index: curr_bar,
        }
    }
}

/// Compute `OrderBlock` over a batch of `(open, high, low, close)` bars.
pub fn order_block_batch(
    inputs: &[(f64, f64, f64, f64)],
    max_active_blocks: usize,
) -> Vec<OrderBlockState> {
    let mut detector = OrderBlock::new(max_active_blocks);
    detector.process_batch(inputs)
}

// ============================================================================
// 3. FairValueGap / FairValueGapDetector Indicator
// ============================================================================

/// Direction of price imbalance for Fair Value Gaps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FvgDirection {
    /// Bullish imbalance: upward impulse gap between high[t-2] and low[t].
    Bullish,
    /// Bearish imbalance: downward impulse gap between low[t-2] and high[t].
    Bearish,
}

/// Structural representation of an identified Fair Value Gap.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FairValueGapZone {
    /// Monotonically increasing unique ID.
    pub id: u64,
    /// Bullish or Bearish direction.
    pub direction: FvgDirection,
    /// Bar index where the gap was confirmed (bar t).
    pub bar_index: usize,
    /// Upper boundary of the imbalance zone.
    pub zone_high: f64,
    /// Lower boundary of the imbalance zone.
    pub zone_low: f64,
    /// Gap width: `zone_high - zone_low`.
    pub gap_size: f64,
    /// Mitigation status.
    pub mitigation: MitigationStatus,
    /// Bar index where mitigation occurred (if any).
    pub mitigation_bar: Option<usize>,
}

impl FairValueGapZone {
    #[inline]
    pub fn is_unmitigated(&self) -> bool {
        self.mitigation == MitigationStatus::Unmitigated
    }

    #[inline]
    pub fn is_partially_mitigated(&self) -> bool {
        self.mitigation == MitigationStatus::PartiallyMitigated
    }

    #[inline]
    pub fn is_fully_mitigated(&self) -> bool {
        self.mitigation == MitigationStatus::FullyMitigated
    }
}

/// Snapshot output of the Fair Value Gap detector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FvgState {
    /// Currently active (unmitigated or partially mitigated) fair value gaps.
    pub active_gaps: Vec<FairValueGapZone>,
    /// Most recently formed fair value gap (if any).
    pub latest_gap: Option<FairValueGapZone>,
    /// Current bar index.
    pub bar_index: usize,
}

impl FvgState {
    /// Returns the fair value gap formed on this exact bar, if any.
    #[inline]
    pub fn current_gap(&self) -> Option<&FairValueGapZone> {
        self.latest_gap
            .as_ref()
            .filter(|g| g.bar_index == self.bar_index)
    }
}

/// 3-bar Fair Value Gap (FVG) detector with lifecycle fill tracking.
#[derive(Debug, Clone)]
pub struct FairValueGap {
    min_gap_size: f64,
    max_active_gaps: usize,
    active_gaps: Vec<FairValueGapZone>,
    latest_gap: Option<FairValueGapZone>,
    history: VecDeque<(f64, f64, f64, usize)>, // (high, low, close, bar_index)
    bar_index: usize,
    next_id: u64,
}

pub type FairValueGapDetector = FairValueGap;

impl FairValueGap {
    /// Construct a new `FairValueGap` detector.
    ///
    /// - `min_gap_size`: Minimum price gap required between outer bars to register an FVG.
    /// - `max_active_gaps`: Maximum number of unmitigated/partially mitigated gaps retained.
    pub fn new(min_gap_size: f64, max_active_gaps: usize) -> Self {
        let max_active = max_active_gaps.max(1);
        Self {
            min_gap_size: min_gap_size.max(0.0),
            max_active_gaps: max_active,
            active_gaps: Vec::with_capacity(max_active),
            latest_gap: None,
            history: VecDeque::with_capacity(3),
            bar_index: 0,
            next_id: 1,
        }
    }

    /// Reset internal state machine.
    pub fn reset(&mut self) {
        self.active_gaps.clear();
        self.latest_gap = None;
        self.history.clear();
        self.bar_index = 0;
        self.next_id = 1;
    }

    /// Helper to process a batch of `(high, low, close)` bars.
    pub fn process_batch(&mut self, bars: &[(f64, f64, f64)]) -> Vec<FvgState> {
        let mut results = Vec::with_capacity(bars.len());
        for &bar in bars {
            results.push(self.next(bar));
        }
        results
    }
}

impl Default for FairValueGap {
    fn default() -> Self {
        Self::new(0.0, 20)
    }
}

impl Next<(f64, f64, f64)> for FairValueGap {
    type Output = FvgState;

    fn next(&mut self, input: (f64, f64, f64)) -> Self::Output {
        let (high, low, close) = input;

        // Fallback for non-finite values
        if !high.is_finite() || !low.is_finite() || !close.is_finite() {
            return FvgState {
                active_gaps: self.active_gaps.clone(),
                latest_gap: self.latest_gap,
                bar_index: self.bar_index,
            };
        }

        let curr_bar = self.bar_index;

        // 1. Prune gaps fully mitigated in previous bars
        self.active_gaps
            .retain(|g| g.mitigation != MitigationStatus::FullyMitigated);

        // 2. Update mitigation status of active gaps with current bar's price action
        for gap in &mut self.active_gaps {
            match gap.direction {
                FvgDirection::Bullish => {
                    if low < gap.zone_high {
                        if low <= gap.zone_low {
                            // Fully filled
                            gap.mitigation = MitigationStatus::FullyMitigated;
                            gap.mitigation_bar = Some(curr_bar);
                        } else if gap.mitigation == MitigationStatus::Unmitigated {
                            // Partially filled
                            gap.mitigation = MitigationStatus::PartiallyMitigated;
                            gap.mitigation_bar = Some(curr_bar);
                        }
                    }
                }
                FvgDirection::Bearish => {
                    if high > gap.zone_low {
                        if high >= gap.zone_high {
                            // Fully filled
                            gap.mitigation = MitigationStatus::FullyMitigated;
                            gap.mitigation_bar = Some(curr_bar);
                        } else if gap.mitigation == MitigationStatus::Unmitigated {
                            // Partially filled
                            gap.mitigation = MitigationStatus::PartiallyMitigated;
                            gap.mitigation_bar = Some(curr_bar);
                        }
                    }
                }
            }
        }

        // 3. Detect new 3-bar Fair Value Gap
        if self.history.len() >= 2 {
            let (h_prev2, l_prev2, _, _) = self.history[self.history.len() - 2];

            // Bullish FVG: low[t] > high[t - 2]
            if low > h_prev2 {
                let gap_size = low - h_prev2;
                if gap_size >= self.min_gap_size && gap_size > 0.0 {
                    let new_gap = FairValueGapZone {
                        id: self.next_id,
                        direction: FvgDirection::Bullish,
                        bar_index: curr_bar,
                        zone_high: low,
                        zone_low: h_prev2,
                        gap_size,
                        mitigation: MitigationStatus::Unmitigated,
                        mitigation_bar: None,
                    };
                    self.next_id += 1;
                    self.latest_gap = Some(new_gap);
                    self.active_gaps.push(new_gap);
                }
            }
            // Bearish FVG: high[t] < low[t - 2]
            else if high < l_prev2 {
                let gap_size = l_prev2 - high;
                if gap_size >= self.min_gap_size && gap_size > 0.0 {
                    let new_gap = FairValueGapZone {
                        id: self.next_id,
                        direction: FvgDirection::Bearish,
                        bar_index: curr_bar,
                        zone_high: l_prev2,
                        zone_low: high,
                        gap_size,
                        mitigation: MitigationStatus::Unmitigated,
                        mitigation_bar: None,
                    };
                    self.next_id += 1;
                    self.latest_gap = Some(new_gap);
                    self.active_gaps.push(new_gap);
                }
            }
        }

        // Enforce maximum active gaps capacity
        while self.active_gaps.len() > self.max_active_gaps {
            self.active_gaps.remove(0);
        }

        // Maintain 2 prior bars in history
        self.history.push_back((high, low, close, curr_bar));
        if self.history.len() > 2 {
            self.history.pop_front();
        }

        self.bar_index += 1;

        FvgState {
            active_gaps: self.active_gaps.clone(),
            latest_gap: self.latest_gap,
            bar_index: curr_bar,
        }
    }
}

/// Compute `FairValueGap` over a batch of `(high, low, close)` bars.
pub fn fair_value_gap_batch(
    inputs: &[(f64, f64, f64)],
    min_gap_size: f64,
    max_active_gaps: usize,
) -> Vec<FvgState> {
    let mut detector = FairValueGap::new(min_gap_size, max_active_gaps);
    detector.process_batch(inputs)
}

// ============================================================================
// Unit Tests & Proptests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_choch_swing_detection_basic() {
        let mut choch = Choch::new(2);

        // Create a clear swing high at bar 2 (price 120.0)
        // Bars:
        // 0: (100, 95, 98)
        // 1: (110, 105, 108)
        // 2: (120, 115, 118) -> Swing High (strength 2)
        // 3: (112, 107, 110)
        // 4: (108, 102, 105) -> Confirms swing high at bar 2
        let bars = vec![
            (100.0, 95.0, 98.0),
            (110.0, 105.0, 108.0),
            (120.0, 115.0, 118.0),
            (112.0, 107.0, 110.0),
            (108.0, 102.0, 105.0),
        ];

        let mut states = Vec::new();
        for bar in bars {
            states.push(choch.next(bar));
        }

        let state_4 = &states[4];
        assert!(state_4.last_swing_high.is_some());
        let sh = state_4.last_swing_high.unwrap();
        assert_eq!(sh.bar, 2);
        assert!((sh.price - 120.0).abs() < 1e-6);
        assert_eq!(sh.swing_type, SwingType::High);
    }

    #[test]
    fn test_choch_bullish_bos_and_bearish_choch() {
        let mut choch = Choch::new(2);

        // Sequence:
        // 0..4: establish swing high at bar 2 (120.0) and swing low at bar 0 (95.0)
        let bars = vec![
            (100.0, 95.0, 98.0),
            (110.0, 105.0, 108.0),
            (120.0, 115.0, 118.0), // Swing High at bar 2
            (112.0, 107.0, 110.0),
            (108.0, 102.0, 105.0), // Confirms bar 2 as Swing High at bar 4
        ];
        for bar in bars {
            choch.next(bar);
        }

        // Bar 5: Closes above swing high (125 > 120)
        // Trend was Undetermined, closes above swing high -> CHoCH / establishes Bullish trend
        let state_5 = choch.next((126.0, 118.0, 125.0));
        assert_eq!(state_5.trend, StructureTrend::Bullish);
        assert!(state_5.last_break.is_some());
        let brk5 = state_5.last_break.unwrap();
        assert_eq!(brk5.direction, StructureBreakDirection::Bullish);
        assert_eq!(brk5.broken_swing_bar, 2);
        assert_eq!(brk5.trigger_bar, 5);

        // Now trend is Bullish!
        // Form a new swing high at bar 7 (140.0) and swing low at bar 5 (118.0)
        let bars_bull = vec![
            (130.0, 120.0, 128.0), // 6
            (140.0, 130.0, 138.0), // 7: Swing High (140.0)
            (135.0, 128.0, 132.0), // 8
            (132.0, 124.0, 126.0), // 9: Confirms bar 7 as Swing High
        ];
        for bar in bars_bull {
            choch.next(bar);
        }

        // Bar 10: Closes above 140.0 (145.0) -> Bullish BOS!
        let state_10 = choch.next((146.0, 138.0, 145.0));
        assert_eq!(state_10.trend, StructureTrend::Bullish);
        let brk10 = state_10.last_break.unwrap();
        assert_eq!(brk10.break_type, StructureBreakType::Bos);
        assert_eq!(brk10.direction, StructureBreakDirection::Bullish);
        assert_eq!(brk10.broken_swing_bar, 7);
        assert_eq!(brk10.trigger_bar, 10);

        // Now establish a swing low in the uptrend, then break it -> Bearish CHoCH!
        let bars_reversal = vec![
            (144.0, 136.0, 140.0), // 11
            (138.0, 125.0, 128.0), // 12: Swing Low (125.0)
            (135.0, 127.0, 132.0), // 13
            (134.0, 126.0, 130.0), // 14: Confirms swing low at bar 12
        ];
        for bar in bars_reversal {
            choch.next(bar);
        }

        // Bar 15: Close collapses below swing low (120.0 < 125.0) -> Bearish CHoCH!
        let state_15 = choch.next((126.0, 118.0, 120.0));
        assert_eq!(state_15.trend, StructureTrend::Bearish);
        let brk15 = state_15.last_break.unwrap();
        assert_eq!(brk15.break_type, StructureBreakType::Choch);
        assert_eq!(brk15.direction, StructureBreakDirection::Bearish);
        assert_eq!(brk15.broken_swing_bar, 12);
        assert_eq!(brk15.trigger_bar, 15);
    }

    #[test]
    fn test_order_block_bullish_and_bearish_lifecycle() {
        let mut ob = OrderBlock::new(10);

        // Bar 0: Down candle (Open 100, High 102, Low 95, Close 96)
        let s0 = ob.next((100.0, 102.0, 95.0, 96.0));
        assert!(s0.active_blocks.is_empty());

        // Bar 1: Strong displacement up candle (Open 97, High 110, Low 96, Close 108)
        // Close 108 > High(bar 0) 102 -> Bullish Order Block formed!
        let s1 = ob.next((97.0, 110.0, 96.0, 108.0));
        assert_eq!(s1.active_blocks.len(), 1);
        let active = &s1.active_blocks[0];
        assert_eq!(active.block_type, OrderBlockType::Bullish);
        assert_eq!(active.bar_index, 0);
        assert_eq!(active.trigger_bar, 1);
        assert!((active.zone_high - 102.0).abs() < 1e-6);
        assert!((active.zone_low - 95.0).abs() < 1e-6);
        assert_eq!(active.mitigation, MitigationStatus::Unmitigated);

        // Bar 2: Price retraces and dips into the zone: Low 100 <= 102, Close 101 >= 95
        // -> PartiallyMitigated!
        let s2 = ob.next((107.0, 108.0, 100.0, 101.0));
        assert_eq!(s2.active_blocks.len(), 1);
        assert_eq!(
            s2.active_blocks[0].mitigation,
            MitigationStatus::PartiallyMitigated
        );
        assert_eq!(s2.active_blocks[0].mitigation_bar, Some(2));

        // Bar 3: Price collapses completely through the zone: Close 92 < 95
        // -> FullyMitigated!
        let s3 = ob.next((101.0, 102.0, 91.0, 92.0));
        assert_eq!(s3.active_blocks.len(), 1);
        assert_eq!(
            s3.active_blocks[0].mitigation,
            MitigationStatus::FullyMitigated
        );
        assert_eq!(s3.active_blocks[0].mitigation_bar, Some(3));

        // Bar 4: The fully mitigated block should now be pruned from active_blocks
        let s4 = ob.next((92.0, 94.0, 89.0, 90.0));
        assert!(s4.active_blocks.is_empty());
    }

    #[test]
    fn test_order_block_bearish_formation_and_mitigation() {
        let mut ob = OrderBlock::new(10);

        // Bar 0: Up candle (Open 100, High 106, Low 99, Close 105)
        ob.next((100.0, 106.0, 99.0, 105.0));

        // Bar 1: Strong displacement down candle (Open 104, High 105, Low 90, Close 92)
        // Close 92 < Low(bar 0) 99 -> Bearish Order Block formed!
        let s1 = ob.next((104.0, 105.0, 90.0, 92.0));
        assert_eq!(s1.active_blocks.len(), 1);
        let block = &s1.active_blocks[0];
        assert_eq!(block.block_type, OrderBlockType::Bearish);
        assert_eq!(block.bar_index, 0);
        assert!((block.zone_low - 99.0).abs() < 1e-6);
        assert!((block.zone_high - 106.0).abs() < 1e-6);
        assert_eq!(block.mitigation, MitigationStatus::Unmitigated);

        // Bar 2: Retraces up into zone: High 101 >= 99, Close 100 <= 106 -> PartiallyMitigated
        let s2 = ob.next((93.0, 101.0, 92.0, 100.0));
        assert_eq!(
            s2.active_blocks[0].mitigation,
            MitigationStatus::PartiallyMitigated
        );

        // Bar 3: Breaks through top of zone: Close 108 > 106 -> FullyMitigated
        let s3 = ob.next((100.0, 110.0, 99.0, 108.0));
        assert_eq!(
            s3.active_blocks[0].mitigation,
            MitigationStatus::FullyMitigated
        );

        // Bar 4: Pruned
        let s4 = ob.next((108.0, 112.0, 107.0, 111.0));
        assert!(s4.active_blocks.is_empty());
    }

    #[test]
    fn test_fair_value_gap_bullish_and_bearish() {
        let mut fvg = FairValueGap::new(0.0, 10);

        // 3-bar Bullish FVG:
        // Bar 0: High 100, Low 90, Close 95
        // Bar 1: High 120, Low 98, Close 118 (impulsive expansion)
        // Bar 2: High 130, Low 105, Close 128
        // Imbalance: Low(bar 2) 105 > High(bar 0) 100 -> Zone [100.0, 105.0]
        let s0 = fvg.next((100.0, 90.0, 95.0));
        assert!(s0.active_gaps.is_empty());

        let s1 = fvg.next((120.0, 98.0, 118.0));
        assert!(s1.active_gaps.is_empty());

        let s2 = fvg.next((130.0, 105.0, 128.0));
        assert_eq!(s2.active_gaps.len(), 1);
        let gap = &s2.active_gaps[0];
        assert_eq!(gap.direction, FvgDirection::Bullish);
        assert!((gap.zone_low - 100.0).abs() < 1e-6);
        assert!((gap.zone_high - 105.0).abs() < 1e-6);
        assert!((gap.gap_size - 5.0).abs() < 1e-6);
        assert_eq!(gap.mitigation, MitigationStatus::Unmitigated);

        // Bar 3: Retests into gap: Low 102 < 105, Low > 100 -> PartiallyMitigated
        let s3 = fvg.next((125.0, 102.0, 120.0));
        assert_eq!(s3.active_gaps.len(), 1);
        assert_eq!(
            s3.active_gaps[0].mitigation,
            MitigationStatus::PartiallyMitigated
        );
        assert_eq!(s3.active_gaps[0].mitigation_bar, Some(3));

        // Bar 4: Fills gap completely: Low 98 <= 100 -> FullyMitigated
        let s4 = fvg.next((120.0, 98.0, 105.0));
        assert_eq!(s4.active_gaps.len(), 1);
        assert_eq!(
            s4.active_gaps[0].mitigation,
            MitigationStatus::FullyMitigated
        );

        // Bar 5: Fully mitigated gap is pruned
        let s5 = fvg.next((110.0, 104.0, 108.0));
        assert!(s5.active_gaps.is_empty());

        // Now test Bearish FVG:
        // Bar 5: High 110, Low 104, Close 108
        // Bar 6: High 106, Low 85, Close 88
        // Bar 7: High 101, Low 80, Close 82
        // Imbalance: High(bar 7) 101 < Low(bar 5) 104 -> Zone [101.0, 104.0]
        fvg.next((106.0, 85.0, 88.0)); // bar 6
        let s7 = fvg.next((101.0, 80.0, 82.0)); // bar 7
        assert_eq!(s7.active_gaps.len(), 1);
        let b_gap = &s7.active_gaps[0];
        assert_eq!(b_gap.direction, FvgDirection::Bearish);
        assert!((b_gap.zone_low - 101.0).abs() < 1e-6);
        assert!((b_gap.zone_high - 104.0).abs() < 1e-6);
        assert_eq!(b_gap.mitigation, MitigationStatus::Unmitigated);

        // Retest into bearish gap: High 102 > 101, High < 104 -> PartiallyMitigated
        let s8 = fvg.next((102.0, 85.0, 90.0));
        assert_eq!(
            s8.active_gaps[0].mitigation,
            MitigationStatus::PartiallyMitigated
        );

        // Fill bearish gap completely: High 105 >= 104 -> FullyMitigated
        let s9 = fvg.next((105.0, 88.0, 95.0));
        assert_eq!(
            s9.active_gaps[0].mitigation,
            MitigationStatus::FullyMitigated
        );
    }

    #[test]
    fn test_batch_equivalence() {
        let bars = vec![
            (100.0, 90.0, 95.0),
            (110.0, 95.0, 108.0),
            (120.0, 105.0, 115.0),
            (115.0, 100.0, 102.0),
            (105.0, 92.0, 94.0),
            (130.0, 102.0, 128.0),
        ];

        let mut streaming = Choch::new(2);
        let streaming_results: Vec<_> = bars.iter().map(|&b| streaming.next(b)).collect();
        let batch_results = choch_batch(&bars, 2);

        assert_eq!(streaming_results, batch_results);

        let ohlc_bars = vec![
            (100.0, 102.0, 95.0, 96.0),
            (97.0, 110.0, 96.0, 108.0),
            (107.0, 108.0, 100.0, 101.0),
            (101.0, 102.0, 91.0, 92.0),
        ];

        let mut ob_streaming = OrderBlock::new(10);
        let ob_stream_results: Vec<_> = ohlc_bars.iter().map(|&b| ob_streaming.next(b)).collect();
        let ob_batch_results = order_block_batch(&ohlc_bars, 10);

        assert_eq!(ob_stream_results, ob_batch_results);

        let mut fvg_streaming = FairValueGap::new(0.0, 10);
        let fvg_stream_results: Vec<_> = bars.iter().map(|&b| fvg_streaming.next(b)).collect();
        let fvg_batch_results = fair_value_gap_batch(&bars, 0.0, 10);

        assert_eq!(fvg_stream_results, fvg_batch_results);
    }

    proptest! {
        #[test]
        fn test_choch_parity(
            highs in prop::collection::vec(10.0..500.0f64, 10..80),
            low_offsets in prop::collection::vec(1.0..10.0f64, 10..80),
            close_pcts in prop::collection::vec(0.0..1.0f64, 10..80),
        ) {
            let n = highs.len().min(low_offsets.len()).min(close_pcts.len());
            let mut bars = Vec::with_capacity(n);
            for i in 0..n {
                let h = highs[i];
                let l = (h - low_offsets[i]).max(0.1);
                let c = l + (h - l) * close_pcts[i];
                bars.push((h, l, c));
            }

            let mut streaming = Choch::new(3);
            let mut stream_outs = Vec::with_capacity(n);
            for &bar in &bars {
                stream_outs.push(streaming.next(bar));
            }

            let batch_outs = choch_batch(&bars, 3);
            prop_assert_eq!(stream_outs, batch_outs);
        }

        #[test]
        fn test_order_block_parity(
            prices in prop::collection::vec((10.0..500.0f64, 10.0..500.0f64, 10.0..500.0f64, 10.0..500.0f64), 5..60),
        ) {
            let mut ohlc = Vec::with_capacity(prices.len());
            for (p0, p1, p2, p3) in prices {
                let open = p0;
                let close = p1;
                let high = open.max(close).max(p2);
                let low = open.min(close).min(p3).max(0.01);
                ohlc.push((open, high, low, close));
            }

            let mut streaming = OrderBlock::new(15);
            let stream_outs: Vec<_> = ohlc.iter().map(|&b| streaming.next(b)).collect();
            let batch_outs = order_block_batch(&ohlc, 15);

            prop_assert_eq!(stream_outs, batch_outs);
        }

        #[test]
        fn test_fair_value_gap_parity(
            highs in prop::collection::vec(10.0..500.0f64, 5..60),
            low_offsets in prop::collection::vec(1.0..10.0f64, 5..60),
            close_pcts in prop::collection::vec(0.0..1.0f64, 5..60),
        ) {
            let n = highs.len().min(low_offsets.len()).min(close_pcts.len());
            let mut bars = Vec::with_capacity(n);
            for i in 0..n {
                let h = highs[i];
                let l = (h - low_offsets[i]).max(0.1);
                let c = l + (h - l) * close_pcts[i];
                bars.push((h, l, c));
            }

            let mut streaming = FairValueGap::new(0.5, 10);
            let stream_outs: Vec<_> = bars.iter().map(|&b| streaming.next(b)).collect();
            let batch_outs = fair_value_gap_batch(&bars, 0.5, 10);

            prop_assert_eq!(stream_outs, batch_outs);
        }
    }
}
