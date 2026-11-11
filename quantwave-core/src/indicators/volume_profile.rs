//! Levels & Auction Theory: Volume Profile (POC, VAH, VAL)
//!
//! # Concept & Theoretical Foundation
//! Volume Profile is an Auction Market Theory study originally conceived by
//! J. Peter Steidlmayer (1984, Chicago Board of Trade Market Profile) and later
//! adapted into modern volume-at-price histograms.
//!
//! While standard volume indicators plot volume over time (x-axis), Volume Profile
//! plots traded volume across discrete price buckets (y-axis) over a rolling lookback
//! window. This reveals:
//! - **Point of Control (POC)**: The price level with the highest traded volume in
//!   the window, representing maximum market consensus / acceptance.
//! - **Value Area (VA)**: The price range surrounding the POC that contains a specified
//!   percentage of total volume (conventionally 70%, reflecting one standard deviation
//!   in a normal distribution).
//! - **Value Area High (VAH)**: The upper boundary of the Value Area.
//! - **Value Area Low (VAL)**: The lower boundary of the Value Area.
//! - **Profile High / Low**: The highest and lowest prices traded across the window.
//!
//! # Formula References
//! - J. Peter Steidlmayer & Kevin Koy (1986). *Markets & Market Logic*.
//! - CBOT Market Profile Handbook (1984, CME Group).
//! - TradingView Visible Range Volume Profile (VPVR):
//!   <https://www.tradingview.com/support/solutions/43000703076-visible-range-volume-profile/>

use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Calculation method used to expand the Value Area (VAH and VAL) from the Point of Control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ValueAreaMethod {
    /// Modern greedy single-bin comparison (standard in TradingView and modern platforms).
    ///
    /// Starting at the POC bin, compares the volume of the next adjacent bin above with
    /// the next adjacent bin below. Expands into whichever has higher volume until the
    /// accumulated volume reaches the target percentage (`value_area_pct`).
    #[default]
    Greedy,
    /// Steidlmayer classic two-row comparison (CBOT Market Profile 1986).
    ///
    /// Compares the sum of volume in the next two rows above versus the next two rows below.
    /// Expands into the side with greater combined volume.
    SteidlmayerTwoRow,
    /// Symmetrical expansion in both directions simultaneously from the POC.
    Symmetrical,
}

/// Strategy for allocating a bar's volume across overlapping price bins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VolumeAllocationMethod {
    /// Allocates volume proportionally to the exact geometric overlap between the bar's
    /// `[low, high]` price range and each price bin.
    #[default]
    ProportionalOverlap,
    /// Distributes volume equally among all bins overlapped by `[low, high]`.
    UniformAcrossBins,
    /// Allocates the bar's entire volume into the price bin containing the close price.
    ClosePrice,
    /// Allocates the bar's entire volume into the price bin containing the typical price `(high + low + close) / 3`.
    TypicalPrice,
}

/// A single OHLCV bar input for Volume Profile computation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BarData {
    /// Bar high price.
    pub high: f64,
    /// Bar low price.
    pub low: f64,
    /// Bar close price.
    pub close: f64,
    /// Bar volume.
    pub volume: f64,
}

impl BarData {
    /// Creates a new `BarData` instance with validated prices and volume.
    ///
    /// Ensures `high >= low` and volume is non-negative and finite.
    pub fn new(high: f64, low: f64, close: f64, volume: f64) -> Self {
        let (high, low) = if high < low { (low, high) } else { (high, low) };
        let volume = if volume.is_finite() && volume > 0.0 {
            volume
        } else {
            0.0
        };
        Self {
            high,
            low,
            close,
            volume,
        }
    }
}

impl Default for BarData {
    fn default() -> Self {
        Self {
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
        }
    }
}

impl From<(f64, f64, f64, f64)> for BarData {
    #[inline]
    fn from((high, low, close, volume): (f64, f64, f64, f64)) -> Self {
        Self::new(high, low, close, volume)
    }
}

impl From<(f64, f64, f64)> for BarData {
    #[inline]
    fn from((high, low, volume): (f64, f64, f64)) -> Self {
        let close = (high + low) * 0.5;
        Self::new(high, low, close, volume)
    }
}

impl From<(f64, f64)> for BarData {
    #[inline]
    fn from((price, volume): (f64, f64)) -> Self {
        Self::new(price, price, price, volume)
    }
}

/// A discrete price bin within the volume profile histogram.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PriceBin {
    /// Lower price bound of the bin (inclusive).
    pub lower: f64,
    /// Upper price bound of the bin (exclusive, except for uppermost bin).
    pub upper: f64,
    /// Midpoint / center price of the bin.
    pub center: f64,
    /// Accumulated volume in this bin.
    pub volume: f64,
}

/// Output state emitted by `VolumeProfile` for each bar.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VolumeProfileState {
    /// Point of Control (POC): center price of the bin with the highest volume.
    pub poc: f64,
    /// Value Area High (VAH): upper price boundary of the Value Area.
    pub vah: f64,
    /// Value Area Low (VAL): lower price boundary of the Value Area.
    pub val: f64,
    /// Highest price observed in the rolling window.
    pub profile_high: f64,
    /// Lowest price observed in the rolling window.
    pub profile_low: f64,
    /// Total traded volume accumulated across the rolling window.
    pub total_volume: f64,
    /// Zero-based index of the most recently processed bar.
    pub bar_index: usize,
}

impl VolumeProfileState {
    /// Creates an empty/uninitialized state with `NaN` prices and zero volume.
    pub fn empty(bar_index: usize) -> Self {
        Self {
            poc: f64::NAN,
            vah: f64::NAN,
            val: f64::NAN,
            profile_high: f64::NAN,
            profile_low: f64::NAN,
            total_volume: 0.0,
            bar_index,
        }
    }

    /// Price span of the Value Area (`vah - val`).
    #[inline]
    pub fn value_area_width(&self) -> f64 {
        (self.vah - self.val).max(0.0)
    }

    /// Price span of the entire profile (`profile_high - profile_low`).
    #[inline]
    pub fn profile_range(&self) -> f64 {
        (self.profile_high - self.profile_low).max(0.0)
    }

    /// Returns `true` if `price` is currently inside the Value Area `[val, vah]`.
    #[inline]
    pub fn is_inside_value_area(&self, price: f64) -> bool {
        price >= self.val && price <= self.vah
    }
}

/// Auction Market Theory Volume Profile indicator.
///
/// Tracks volume distribution across price bins over a rolling lookback window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeProfile {
    period: usize,
    bins: usize,
    value_area_pct: f64,
    va_method: ValueAreaMethod,
    alloc_method: VolumeAllocationMethod,
    window: VecDeque<BarData>,
    bar_count: usize,
}

impl VolumeProfile {
    /// Creates a new `VolumeProfile` with default 70% Value Area and Greedy expansion.
    ///
    /// # Arguments
    /// * `period` - Sliding window size in bars (minimum 1).
    /// * `bins` - Number of price buckets in the histogram (minimum 1).
    pub fn new(period: usize, bins: usize) -> Self {
        let period = period.max(1);
        Self {
            period,
            bins: bins.max(1),
            value_area_pct: 0.70,
            va_method: ValueAreaMethod::Greedy,
            alloc_method: VolumeAllocationMethod::ProportionalOverlap,
            window: VecDeque::with_capacity(period),
            bar_count: 0,
        }
    }

    /// Sets the target percentage of total volume to include in the Value Area.
    ///
    /// Value is clamped between 0.0 and 1.0 (default 0.70).
    pub fn with_value_area_pct(mut self, pct: f64) -> Self {
        self.value_area_pct = pct.clamp(0.0, 1.0);
        self
    }

    /// Sets the Value Area expansion method and volume allocation method.
    pub fn with_methods(
        mut self,
        va_method: ValueAreaMethod,
        alloc_method: VolumeAllocationMethod,
    ) -> Self {
        self.va_method = va_method;
        self.alloc_method = alloc_method;
        self
    }

    /// Creates a `VolumeProfile` with full parameter configuration.
    pub fn with_params(
        period: usize,
        bins: usize,
        value_area_pct: f64,
        va_method: ValueAreaMethod,
        alloc_method: VolumeAllocationMethod,
    ) -> Self {
        Self::new(period, bins)
            .with_value_area_pct(value_area_pct)
            .with_methods(va_method, alloc_method)
    }

    /// Sliding window size in bars.
    #[inline]
    pub fn period(&self) -> usize {
        self.period
    }

    /// Number of histogram price bins.
    #[inline]
    pub fn bins(&self) -> usize {
        self.bins
    }

    /// Configured Value Area percentage (e.g. 0.70).
    #[inline]
    pub fn value_area_pct(&self) -> f64 {
        self.value_area_pct
    }

    /// Configured Value Area expansion method.
    #[inline]
    pub fn value_area_method(&self) -> ValueAreaMethod {
        self.va_method
    }

    /// Configured volume allocation method.
    #[inline]
    pub fn allocation_method(&self) -> VolumeAllocationMethod {
        self.alloc_method
    }

    /// Number of bars currently in the rolling window.
    #[inline]
    pub fn window_len(&self) -> usize {
        self.window.len()
    }

    /// Total number of bars ingested since initialization or last reset.
    #[inline]
    pub fn bar_count(&self) -> usize {
        self.bar_count
    }

    /// Returns `true` if the rolling window has reached full capacity (`>= period`).
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.window.len() >= self.period
    }

    /// Resets the sliding window and bar counter to initial state.
    pub fn reset(&mut self) {
        self.window.clear();
        self.bar_count = 0;
    }

    /// Ingests a new bar and computes the updated `VolumeProfileState`.
    pub fn next_bar(&mut self, bar: impl Into<BarData>) -> VolumeProfileState {
        let bar = bar.into();
        self.bar_count = self.bar_count.wrapping_add(1);
        let current_bar_idx = self.bar_count.saturating_sub(1);

        self.window.push_back(bar);
        if self.window.len() > self.period {
            self.window.pop_front();
        }

        if self.window.is_empty() {
            return VolumeProfileState::empty(current_bar_idx);
        }

        // Find min low, max high, and total volume across window
        let mut min_low = f64::INFINITY;
        let mut max_high = f64::NEG_INFINITY;
        let mut total_vol = 0.0;

        for b in &self.window {
            if b.low < min_low {
                min_low = b.low;
            }
            if b.high > max_high {
                max_high = b.high;
            }
            total_vol += b.volume;
        }

        if !min_low.is_finite() || !max_high.is_finite() {
            return VolumeProfileState::empty(current_bar_idx);
        }

        // Handle flat market edge-case (min_low == max_high)
        if (max_high - min_low).abs() < 1e-12 {
            return VolumeProfileState {
                poc: min_low,
                vah: min_low,
                val: min_low,
                profile_high: min_low,
                profile_low: min_low,
                total_volume: total_vol,
                bar_index: current_bar_idx,
            };
        }

        let bin_size = (max_high - min_low) / (self.bins as f64);
        let mut histogram = vec![0.0; self.bins];

        // Allocate each bar's volume into histogram bins
        for b in &self.window {
            allocate_bar_volume(
                b,
                min_low,
                bin_size,
                self.bins,
                self.alloc_method,
                &mut histogram,
            );
        }

        // Identify POC (bin with maximum volume)
        let mut max_v = -1.0;
        let mut poc_idx = 0;
        for (i, &v) in histogram.iter().enumerate() {
            if v > max_v {
                max_v = v;
                poc_idx = i;
            }
        }

        let poc = min_low + (poc_idx as f64 + 0.5) * bin_size;

        // Calculate Value Area (VAH and VAL)
        let (va_low_idx, va_high_idx) = compute_value_area_indices(
            &histogram,
            poc_idx,
            total_vol,
            self.value_area_pct,
            self.va_method,
        );

        let val_calc = min_low + (va_low_idx as f64) * bin_size;
        let vah_calc = (min_low + ((va_high_idx + 1) as f64) * bin_size).min(max_high);

        // Invariant guard: val <= poc <= vah unconditionally
        let val = val_calc.min(poc);
        let vah = vah_calc.max(poc);

        VolumeProfileState {
            poc,
            vah,
            val,
            profile_high: max_high,
            profile_low: min_low,
            total_volume: total_vol,
            bar_index: current_bar_idx,
        }
    }

    /// Computes and returns the full discrete price bins for the current window.
    pub fn current_bins(&self) -> Vec<PriceBin> {
        if self.window.is_empty() {
            return Vec::new();
        }

        let mut min_low = f64::INFINITY;
        let mut max_high = f64::NEG_INFINITY;

        for b in &self.window {
            if b.low < min_low {
                min_low = b.low;
            }
            if b.high > max_high {
                max_high = b.high;
            }
        }

        if !min_low.is_finite() || !max_high.is_finite() {
            return Vec::new();
        }

        let bin_size = if (max_high - min_low).abs() < 1e-12 {
            0.0
        } else {
            (max_high - min_low) / (self.bins as f64)
        };

        let mut histogram = vec![0.0; self.bins];
        for b in &self.window {
            allocate_bar_volume(
                b,
                min_low,
                bin_size.max(1e-12),
                self.bins,
                self.alloc_method,
                &mut histogram,
            );
        }

        (0..self.bins)
            .map(|i| {
                let lower = min_low + (i as f64) * bin_size;
                let upper = if i == self.bins - 1 {
                    max_high
                } else {
                    lower + bin_size
                };
                let center = lower + 0.5 * bin_size;
                PriceBin {
                    lower,
                    upper,
                    center,
                    volume: histogram[i],
                }
            })
            .collect()
    }
}

/// Helper function to allocate a bar's volume across price bins.
fn allocate_bar_volume(
    bar: &BarData,
    min_low: f64,
    bin_size: f64,
    num_bins: usize,
    method: VolumeAllocationMethod,
    histogram: &mut [f64],
) {
    if bar.volume <= 0.0 || !bar.volume.is_finite() || bin_size <= 0.0 {
        return;
    }
    let num_bins_f = num_bins as f64;

    match method {
        VolumeAllocationMethod::ProportionalOverlap => {
            let bar_range = bar.high - bar.low;
            if bar_range <= 1e-12 || !bar_range.is_finite() {
                // Point-like bar: allocate to the single bin containing the midpoint
                let mid = (bar.high + bar.low) * 0.5;
                let bin_idx = ((mid - min_low) / bin_size)
                    .floor()
                    .clamp(0.0, num_bins_f - 1.0) as usize;
                histogram[bin_idx] += bar.volume;
            } else {
                let start_idx = ((bar.low - min_low) / bin_size)
                    .floor()
                    .clamp(0.0, num_bins_f - 1.0) as usize;
                let end_idx = ((bar.high - min_low) / bin_size)
                    .floor()
                    .clamp(0.0, num_bins_f - 1.0) as usize;

                for i in start_idx..=end_idx {
                    let bin_lower = min_low + (i as f64) * bin_size;
                    let bin_upper = bin_lower + bin_size;
                    let overlap_start = bar.low.max(bin_lower);
                    let overlap_end = bar.high.min(bin_upper);
                    let overlap = (overlap_end - overlap_start).max(0.0);
                    if overlap > 0.0 {
                        let frac = overlap / bar_range;
                        histogram[i] += bar.volume * frac;
                    }
                }
            }
        }
        VolumeAllocationMethod::UniformAcrossBins => {
            let start_idx = ((bar.low - min_low) / bin_size)
                .floor()
                .clamp(0.0, num_bins_f - 1.0) as usize;
            let end_idx = ((bar.high - min_low) / bin_size)
                .floor()
                .clamp(0.0, num_bins_f - 1.0) as usize;
            let count = (end_idx - start_idx + 1) as f64;
            let vol_per_bin = bar.volume / count;
            for i in start_idx..=end_idx {
                histogram[i] += vol_per_bin;
            }
        }
        VolumeAllocationMethod::ClosePrice => {
            let bin_idx = ((bar.close - min_low) / bin_size)
                .floor()
                .clamp(0.0, num_bins_f - 1.0) as usize;
            histogram[bin_idx] += bar.volume;
        }
        VolumeAllocationMethod::TypicalPrice => {
            let tp = (bar.high + bar.low + bar.close) / 3.0;
            let bin_idx = ((tp - min_low) / bin_size)
                .floor()
                .clamp(0.0, num_bins_f - 1.0) as usize;
            histogram[bin_idx] += bar.volume;
        }
    }
}

/// Helper function to determine the lower and upper bin indices of the Value Area.
fn compute_value_area_indices(
    histogram: &[f64],
    poc_idx: usize,
    total_vol: f64,
    value_area_pct: f64,
    method: ValueAreaMethod,
) -> (usize, usize) {
    let num_bins = histogram.len();
    if num_bins == 0 {
        return (0, 0);
    }
    if total_vol <= 0.0 || value_area_pct <= 0.0 {
        return (poc_idx, poc_idx);
    }

    let target_vol = total_vol * value_area_pct;
    let mut va_low_idx = poc_idx;
    let mut va_high_idx = poc_idx;
    let mut current_va_vol = histogram[poc_idx];

    match method {
        ValueAreaMethod::Greedy => {
            while current_va_vol < target_vol && (va_low_idx > 0 || va_high_idx + 1 < num_bins) {
                let can_go_up = va_high_idx + 1 < num_bins;
                let can_go_down = va_low_idx > 0;

                if can_go_up && can_go_down {
                    let vol_up = histogram[va_high_idx + 1];
                    let vol_down = histogram[va_low_idx - 1];

                    if vol_up > vol_down {
                        va_high_idx += 1;
                        current_va_vol += vol_up;
                    } else if vol_down > vol_up {
                        va_low_idx -= 1;
                        current_va_vol += vol_down;
                    } else {
                        // Tie: expand in both directions
                        va_high_idx += 1;
                        va_low_idx -= 1;
                        current_va_vol += vol_up + vol_down;
                    }
                } else if can_go_up {
                    va_high_idx += 1;
                    current_va_vol += histogram[va_high_idx];
                } else if can_go_down {
                    va_low_idx -= 1;
                    current_va_vol += histogram[va_low_idx];
                } else {
                    break;
                }
            }
        }
        ValueAreaMethod::SteidlmayerTwoRow => {
            while current_va_vol < target_vol && (va_low_idx > 0 || va_high_idx + 1 < num_bins) {
                let can_go_up = va_high_idx + 1 < num_bins;
                let can_go_down = va_low_idx > 0;

                if can_go_up && can_go_down {
                    let mut sum_up = histogram[va_high_idx + 1];
                    if va_high_idx + 2 < num_bins {
                        sum_up += histogram[va_high_idx + 2];
                    }

                    let mut sum_down = histogram[va_low_idx - 1];
                    if va_low_idx >= 2 {
                        sum_down += histogram[va_low_idx - 2];
                    }

                    if sum_up > sum_down {
                        va_high_idx += 1;
                        current_va_vol += histogram[va_high_idx];
                        if current_va_vol < target_vol && va_high_idx + 1 < num_bins {
                            va_high_idx += 1;
                            current_va_vol += histogram[va_high_idx];
                        }
                    } else if sum_down > sum_up {
                        va_low_idx -= 1;
                        current_va_vol += histogram[va_low_idx];
                        if current_va_vol < target_vol && va_low_idx > 0 {
                            va_low_idx -= 1;
                            current_va_vol += histogram[va_low_idx];
                        }
                    } else {
                        va_high_idx += 1;
                        va_low_idx -= 1;
                        current_va_vol += histogram[va_high_idx] + histogram[va_low_idx];
                    }
                } else if can_go_up {
                    va_high_idx += 1;
                    current_va_vol += histogram[va_high_idx];
                } else if can_go_down {
                    va_low_idx -= 1;
                    current_va_vol += histogram[va_low_idx];
                } else {
                    break;
                }
            }
        }
        ValueAreaMethod::Symmetrical => {
            while current_va_vol < target_vol && (va_low_idx > 0 || va_high_idx + 1 < num_bins) {
                let mut added_any = false;
                if va_high_idx + 1 < num_bins {
                    va_high_idx += 1;
                    current_va_vol += histogram[va_high_idx];
                    added_any = true;
                }
                if va_low_idx > 0 {
                    va_low_idx -= 1;
                    current_va_vol += histogram[va_low_idx];
                    added_any = true;
                }
                if !added_any {
                    break;
                }
            }
        }
    }

    (va_low_idx, va_high_idx)
}

// =============================================================================
// Universal Indicator Next Implementations
// =============================================================================

/// Full OHLCV input: `(high, low, close, volume)`.
impl Next<(f64, f64, f64, f64)> for VolumeProfile {
    type Output = VolumeProfileState;

    #[inline]
    fn next(&mut self, (high, low, close, volume): (f64, f64, f64, f64)) -> Self::Output {
        self.next_bar((high, low, close, volume))
    }
}

/// Convenience HLV input: `(high, low, volume)`. Close defaults to `(high + low) / 2`.
impl Next<(f64, f64, f64)> for VolumeProfile {
    type Output = VolumeProfileState;

    #[inline]
    fn next(&mut self, (high, low, volume): (f64, f64, f64)) -> Self::Output {
        self.next_bar((high, low, volume))
    }
}

/// Direct `BarData` input.
impl Next<BarData> for VolumeProfile {
    type Output = VolumeProfileState;

    #[inline]
    fn next(&mut self, bar: BarData) -> Self::Output {
        self.next_bar(bar)
    }
}

/// Reference `&BarData` input.
impl Next<&BarData> for VolumeProfile {
    type Output = VolumeProfileState;

    #[inline]
    fn next(&mut self, bar: &BarData) -> Self::Output {
        self.next_bar(*bar)
    }
}

/// Reference `&(f64, f64, f64, f64)` input.
impl Next<&(f64, f64, f64, f64)> for VolumeProfile {
    type Output = VolumeProfileState;

    #[inline]
    fn next(&mut self, input: &(f64, f64, f64, f64)) -> Self::Output {
        self.next_bar(*input)
    }
}

/// Legacy single price & volume input: `(price, volume)`.
///
/// Returns the Point of Control (POC) as `f64` for backward compatibility with PyO3 bindings.
impl Next<(f64, f64)> for VolumeProfile {
    type Output = f64;

    #[inline]
    fn next(&mut self, (price, volume): (f64, f64)) -> Self::Output {
        self.next_bar((price, volume)).poc
    }
}

// =============================================================================
// Metadata Registry Constant
// =============================================================================

pub const VOLUME_PROFILE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Volume Profile",
    description: "Calculates price distribution of traded volume, Point of Control (POC), and Value Area High/Low (VAH/VAL) over a sliding window.",
    usage: "Identifies institutional high-volume acceptance nodes (POC) and unfair value boundaries (VAH/VAL) for mean-reversion, breakout targets, and dynamic support/resistance in Auction Market Theory.",
    keywords: &[
        "volume",
        "volume-profile",
        "poc",
        "point-of-control",
        "vah",
        "val",
        "value-area",
        "auction-market-theory",
        "steidlmayer",
        "market-profile",
        "support-resistance",
    ],
    ehlers_summary: "Volume Profile displays trading activity over a rolling window at specified price levels. Originating from J. Peter Steidlmayer's Market Profile (1984), the Point of Control (POC) marks the price of maximum liquidity and acceptance, while the Value Area encompasses 70% of total volume traded to delineate fair market value.",
    params: &[
        ParamDef {
            name: "period",
            default: "200",
            description: "Sliding window size in bars",
        },
        ParamDef {
            name: "bins",
            default: "50",
            description: "Number of discrete price bins in the histogram",
        },
        ParamDef {
            name: "value_area_pct",
            default: "0.70",
            description: "Target fraction of total window volume contained in the Value Area [0.0, 1.0]",
        },
    ],
    formula_source: "https://www.tradingview.com/support/solutions/43000703076-visible-range-volume-profile/",
    formula_latex: r#"
\begin{aligned}
\Delta P &= \frac{P_{\max} - P_{\min}}{N_{\text{bins}}} \\
\text{Bin}(P) &= \min\left(N_{\text{bins}}-1, \; \left\lfloor \frac{P - P_{\min}}{\Delta P} \right\rfloor \right) \\
\text{POC} &= P_{\min} + \left( \arg\max_{i} V_i + 0.5 \right) \cdot \Delta P \\
\sum_{i=i_{\text{VAL}}}^{i_{\text{VAH}}} V_i &\ge \alpha \cdot V_{\text{total}}, \quad \text{where } \alpha = 0.70 \\
\text{VAL} &= P_{\min} + i_{\text{VAL}} \cdot \Delta P \\
\text{VAH} &= P_{\min} + (i_{\text{VAH}} + 1) \cdot \Delta P
\end{aligned}
"#,
    gold_standard_file: "volume_profile.json",
    category: "Volume",
};

// =============================================================================
// Tests & Proptests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use proptest::prelude::*;

    #[test]
    fn test_volume_profile_basic_backward_compatibility() {
        let mut vp = VolumeProfile::new(10, 5);
        // All volume at 100
        for _ in 0..5 {
            vp.next((100.0, 10.0));
        }
        // Some volume at 110
        let res = vp.next((110.0, 5.0));
        assert!((100.0..=105.0).contains(&res)); // POC should still be in the 100 bin

        // More volume at 110
        vp.next((110.0, 20.0));
        vp.next((110.0, 20.0));
        let res2 = vp.next((110.0, 20.0));
        assert!(res2 >= 105.0); // POC should shift to higher bin
    }

    #[test]
    fn test_single_bar_profile() {
        let mut vp = VolumeProfile::new(10, 5);
        let state = vp.next((105.0, 95.0, 100.0, 1000.0));

        assert_eq!(state.bar_index, 0);
        assert_relative_eq!(state.total_volume, 1000.0, epsilon = 1e-6);
        assert_relative_eq!(state.profile_high, 105.0, epsilon = 1e-6);
        assert_relative_eq!(state.profile_low, 95.0, epsilon = 1e-6);
        assert!(state.val <= state.poc);
        assert!(state.poc <= state.vah);
        assert!(state.val >= state.profile_low);
        assert!(state.vah <= state.profile_high);
    }

    #[test]
    fn test_flat_market() {
        let mut vp = VolumeProfile::new(5, 10);
        for _ in 0..5 {
            let state = vp.next((100.0, 100.0, 100.0, 50.0));
            assert_relative_eq!(state.poc, 100.0, epsilon = 1e-6);
            assert_relative_eq!(state.vah, 100.0, epsilon = 1e-6);
            assert_relative_eq!(state.val, 100.0, epsilon = 1e-6);
            assert_relative_eq!(state.profile_high, 100.0, epsilon = 1e-6);
            assert_relative_eq!(state.profile_low, 100.0, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_poc_selection_skewed_distribution() {
        // 10 bins over range [100, 110]. Bins are width 1.0:
        // [100,101), [101,102), ..., [107,108), [108,109), [109,110]
        let mut vp = VolumeProfile::new(10, 10);

        // Put light volume across lower prices
        vp.next((101.0, 100.0, 100.5, 10.0));
        vp.next((102.0, 101.0, 101.5, 10.0));
        vp.next((103.0, 102.0, 102.5, 10.0));

        // Put massive volume around [107, 108]
        vp.next((108.0, 107.0, 107.5, 500.0));

        // Put light volume at top
        let state = vp.next((110.0, 109.0, 109.5, 10.0));

        // POC center should be in bin [107, 108], i.e., 107.5
        assert_relative_eq!(state.poc, 107.5, epsilon = 1e-6);
        assert!(state.val <= state.poc);
        assert!(state.poc <= state.vah);
        assert!(state.is_inside_value_area(107.5));
    }

    #[test]
    fn test_value_area_containment_invariant() {
        let mut vp = VolumeProfile::new(20, 15).with_value_area_pct(0.70);

        let data = [
            (102.0, 98.0, 100.0, 150.0),
            (104.0, 99.0, 103.0, 200.0),
            (106.0, 101.0, 105.0, 350.0),
            (108.0, 103.0, 107.0, 500.0),
            (107.0, 102.0, 104.0, 400.0),
            (105.0, 100.0, 101.0, 300.0),
            (103.0, 97.0, 98.0, 250.0),
            (101.0, 96.0, 99.0, 180.0),
            (105.0, 98.0, 103.0, 220.0),
            (110.0, 104.0, 109.0, 600.0),
        ];

        for (h, l, c, v) in data {
            let state = vp.next((h, l, c, v));
            assert!(
                state.val <= state.poc,
                "VAL ({}) must be <= POC ({})",
                state.val,
                state.poc
            );
            assert!(
                state.poc <= state.vah,
                "POC ({}) must be <= VAH ({})",
                state.poc,
                state.vah
            );
            assert!(
                state.val >= state.profile_low,
                "VAL ({}) must be >= profile_low ({})",
                state.val,
                state.profile_low
            );
            assert!(
                state.vah <= state.profile_high,
                "VAH ({}) must be <= profile_high ({})",
                state.vah,
                state.profile_high
            );
            assert!(state.value_area_width() >= 0.0);
            assert!(state.profile_range() >= state.value_area_width());
        }
    }

    #[test]
    fn test_value_area_methods_comparison() {
        let bars = vec![
            (110.0, 100.0, 105.0, 100.0),
            (115.0, 105.0, 110.0, 250.0),
            (120.0, 110.0, 115.0, 500.0),
            (125.0, 115.0, 120.0, 300.0),
            (118.0, 108.0, 112.0, 150.0),
        ];

        let methods = [
            ValueAreaMethod::Greedy,
            ValueAreaMethod::SteidlmayerTwoRow,
            ValueAreaMethod::Symmetrical,
        ];

        for method in methods {
            let mut vp = VolumeProfile::new(10, 20)
                .with_methods(method, VolumeAllocationMethod::ProportionalOverlap);

            for &bar in &bars {
                let state = vp.next(bar);
                assert!(state.val <= state.poc);
                assert!(state.poc <= state.vah);
            }
        }
    }

    #[test]
    fn test_volume_allocation_methods() {
        let alloc_methods = [
            VolumeAllocationMethod::ProportionalOverlap,
            VolumeAllocationMethod::UniformAcrossBins,
            VolumeAllocationMethod::ClosePrice,
            VolumeAllocationMethod::TypicalPrice,
        ];

        for alloc in alloc_methods {
            let mut vp = VolumeProfile::new(5, 10).with_methods(ValueAreaMethod::Greedy, alloc);

            let s1 = vp.next((110.0, 100.0, 105.0, 200.0));
            assert_relative_eq!(s1.total_volume, 200.0, epsilon = 1e-6);

            let s2 = vp.next((115.0, 105.0, 110.0, 300.0));
            assert_relative_eq!(s2.total_volume, 500.0, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_sliding_window_eviction() {
        let period = 3;
        let mut vp = VolumeProfile::new(period, 10);

        // Bars 1..3 in high range [200, 210]
        vp.next((210.0, 200.0, 205.0, 100.0));
        vp.next((210.0, 200.0, 205.0, 100.0));
        let s3 = vp.next((210.0, 200.0, 205.0, 100.0));
        assert!(s3.profile_low >= 200.0);

        // Bars 4..6 in low range [100, 110]
        vp.next((110.0, 100.0, 105.0, 100.0));
        vp.next((110.0, 100.0, 105.0, 100.0));
        let s6 = vp.next((110.0, 100.0, 105.0, 100.0));

        // The old bars [200, 210] must have been evicted from the sliding window
        assert_relative_eq!(s6.profile_high, 110.0, epsilon = 1e-6);
        assert_relative_eq!(s6.profile_low, 100.0, epsilon = 1e-6);
        assert_relative_eq!(s6.total_volume, 300.0, epsilon = 1e-6);
    }

    #[test]
    fn test_current_bins_inspection() {
        let mut vp = VolumeProfile::new(10, 5);
        vp.next((110.0, 100.0, 105.0, 100.0));

        let bins = vp.current_bins();
        assert_eq!(bins.len(), 5);

        let total_binned_vol: f64 = bins.iter().map(|b| b.volume).sum();
        assert_relative_eq!(total_binned_vol, 100.0, epsilon = 1e-6);

        for (i, b) in bins.iter().enumerate() {
            assert!(b.lower < b.upper);
            assert_relative_eq!(b.center, (b.lower + b.upper) * 0.5, epsilon = 1e-6);
            if i > 0 {
                assert_relative_eq!(b.lower, bins[i - 1].upper, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_convenience_trait_inputs() {
        let mut vp = VolumeProfile::new(5, 10);

        // Test 4-tuple
        let s4 = vp.next((105.0, 95.0, 100.0, 50.0));
        assert_eq!(s4.bar_index, 0);

        // Test 3-tuple
        let s3 = vp.next((106.0, 96.0, 50.0));
        assert_eq!(s3.bar_index, 1);

        // Test BarData struct
        let bar = BarData::new(107.0, 97.0, 102.0, 60.0);
        let sb = vp.next(bar);
        assert_eq!(sb.bar_index, 2);

        // Test &BarData
        let sref = vp.next(&bar);
        assert_eq!(sref.bar_index, 3);
    }

    #[test]
    fn test_reset() {
        let mut vp = VolumeProfile::new(5, 10);
        vp.next((105.0, 95.0, 100.0, 50.0));
        assert_eq!(vp.window_len(), 1);
        assert_eq!(vp.bar_count(), 1);

        vp.reset();
        assert_eq!(vp.window_len(), 0);
        assert_eq!(vp.bar_count(), 0);
        assert!(vp.current_bins().is_empty());
    }

    // Batch simulation function for parity assertion
    fn volume_profile_batch(
        bars: &[(f64, f64, f64, f64)],
        period: usize,
        bins: usize,
    ) -> Vec<VolumeProfileState> {
        let mut vp = VolumeProfile::new(period, bins);
        bars.iter().map(|&bar| vp.next(bar)).collect()
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        #[test]
        fn proptest_streaming_batch_parity(
            bars in prop::collection::vec(
                (50.0..150.0, 1.0..50.0, 0.0..100.0, 10.0..1000.0)
                    .prop_map(|(base, span, offset, vol)| {
                        let low = base;
                        let high = base + span;
                        let close = low + (high - low) * (offset / 100.0);
                        (high, low, close, vol)
                    }),
                1..60
            )
        ) {
            let period = 10;
            let bins = 20;

            // Batch run
            let batch_results = volume_profile_batch(&bars, period, bins);

            // Streaming run
            let mut streaming_vp = VolumeProfile::new(period, bins);
            let mut streaming_results = Vec::with_capacity(bars.len());
            for &bar in &bars {
                streaming_results.push(streaming_vp.next(bar));
            }

            // Assert bit-for-bit parity
            prop_assert_eq!(batch_results.len(), streaming_results.len());
            for (b, s) in batch_results.iter().zip(streaming_results.iter()) {
                prop_assert_eq!(b, s);

                // Invariant checks
                prop_assert!(s.val <= s.poc, "VAL ({}) must be <= POC ({})", s.val, s.poc);
                prop_assert!(s.poc <= s.vah, "POC ({}) must be <= VAH ({})", s.poc, s.vah);
                prop_assert!(s.val >= s.profile_low - 1e-6);
                prop_assert!(s.vah <= s.profile_high + 1e-6);
                prop_assert!(s.total_volume >= 0.0);
            }
        }
    }
}
