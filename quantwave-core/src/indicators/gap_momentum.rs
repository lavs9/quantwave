use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::SMA;
use crate::traits::Next;
use std::collections::VecDeque;

/// Gap Momentum
///
/// Introduced by Perry J. Kaufman in "Gap Momentum" (S&C January 2024).
/// This indicator draws inspiration from J. Welles Wilder and Joseph Granville's On-Balance Volume (OBV).
/// It accumulates positive and negative opening gaps over a specified period to derive a cumulative gap ratio.
/// A signal line, computed as a moving average of this ratio, is used to identify momentum shifts.
#[derive(Debug, Clone)]
pub struct GapMomentum {
    period: usize,
    up_gaps: VecDeque<f64>,
    dn_gaps: VecDeque<f64>,
    total_up_gaps: f64,
    total_dn_gaps: f64,
    sma: SMA,
    prev_close: Option<f64>,
}

impl GapMomentum {
    pub fn new(period: usize, signal_period: usize) -> Self {
        Self {
            period,
            up_gaps: VecDeque::with_capacity(period),
            dn_gaps: VecDeque::with_capacity(period),
            total_up_gaps: 0.0,
            total_dn_gaps: 0.0,
            sma: SMA::new(signal_period),
            prev_close: None,
        }
    }
}

impl Next<(f64, f64)> for GapMomentum {
    type Output = (f64, f64); // (GapRatio, Signal)

    fn next(&mut self, (open, close): (f64, f64)) -> Self::Output {
        let gap = match self.prev_close {
            Some(pc) => open - pc,
            None => 0.0,
        };
        self.prev_close = Some(close);

        let up_gap = if gap > 0.0 { gap } else { 0.0 };
        let dn_gap = if gap < 0.0 { -gap } else { 0.0 };

        self.up_gaps.push_back(up_gap);
        self.dn_gaps.push_back(dn_gap);
        self.total_up_gaps += up_gap;
        self.total_dn_gaps += dn_gap;

        if self.up_gaps.len() > self.period {
            if let Some(old_up) = self.up_gaps.pop_front() {
                self.total_up_gaps -= old_up;
            }
            if let Some(old_dn) = self.dn_gaps.pop_front() {
                self.total_dn_gaps -= old_dn;
            }
        }

        let gap_ratio = if self.total_dn_gaps == 0.0 {
            1.0
        } else {
            100.0 * self.total_up_gaps / self.total_dn_gaps
        };

        let signal = self.sma.next(gap_ratio);

        (gap_ratio, signal)
    }
}

pub const GAP_MOMENTUM_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Gap Momentum",
    description: "Accumulates positive and negative opening gaps to derive a cumulative gap ratio, smoothed by a signal line.",
    usage: "Used to identify momentum shifts based on price gaps. Buy when the signal line is rising and sell when it is falling.",
    keywords: &["momentum", "gap", "kaufman", "oscillator"],
    ehlers_summary: "Perry J. Kaufman introduced Gap Momentum as a way to quantify price gaps relative to their cumulative volatility, similar to an On-Balance Volume (OBV) logic applied to opening gaps. It helps traders identify if gap-driven momentum is increasing or decreasing by comparing the sum of upward gaps against downward gaps over a rolling window. — Perry Kaufman, S&C 2024",
    params: &[
        ParamDef {
            name: "period",
            default: "40",
            description: "Rolling window for gap accumulation",
        },
        ParamDef {
            name: "signal_period",
            default: "20",
            description: "Smoothing period for the gap ratio",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JANUARY%202024.html",
    formula_latex: r#"
\[
Gap = Open_t - Close_{t-1}
\]
\[
UpGaps = \sum_{i=0}^{Period-1} \max(0, Gap_{t-i})
\]
\[
DnGaps = \sum_{i=0}^{Period-1} \max(0, -Gap_{t-i})
\]
\[
GapRatio = \begin{cases} 1 & \text{if } DnGaps = 0 \\ 100 \times \frac{UpGaps}{DnGaps} & \text{otherwise} \end{cases}
\]
\[
Signal = SMA(GapRatio, SignalPeriod)
\]
"#,
    gold_standard_file: "gap_momentum.json",
    category: "Momentum",
};

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_gap_momentum_basic() {
        let mut gm = GapMomentum::new(10, 5);
        // Bar 1: Open 10, Close 10. Gap = 0.
        // Bar 2: Open 11, Close 11. Gap = 11 - 10 = 1. UpGap = 1, DnGap = 0. GapRatio = 100 * 1 / 0 -> 1.0 (wait, DnGap is 0)
        // Wait, if DnGaps is 0, GapRatio is 1.0.
        let (gr, sig) = gm.next((10.0, 10.0));
        assert_eq!(gr, 1.0);
        assert_eq!(sig, 1.0);

        let (gr, sig) = gm.next((11.0, 11.0));
        assert_eq!(gr, 1.0); // total_up=1, total_dn=0 -> 1.0
        assert_eq!(sig, 1.0);

        let (gr, _) = gm.next((10.0, 10.0)); // gap = 10 - 11 = -1. total_up=1, total_dn=1 -> 100.0
        assert_eq!(gr, 100.0);
    }

    proptest! {
        #[test]
        fn test_gap_momentum_parity(
            inputs in prop::collection::vec((10.0..20.0, 10.0..20.0), 50..100),
        ) {
            let period = 20;
            let signal_period = 10;
            let mut gm = GapMomentum::new(period, signal_period);

            let mut streaming_results = Vec::with_capacity(inputs.len());
            for &val in &inputs {
                streaming_results.push(gm.next(val));
            }

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut gaps = Vec::with_capacity(inputs.len());
            let mut prev_close = None;
            for &(open, close) in &inputs {
                let gap = prev_close.map(|pc| open - pc).unwrap_or(0.0);
                gaps.push(gap);
                prev_close = Some(close);
            }

            let mut gap_ratios = Vec::with_capacity(inputs.len());
            for i in 0..inputs.len() {
                let start = if i >= period { i - period + 1 } else { 0 };
                let window = &gaps[start..=i];
                let up_gaps: f64 = window.iter().filter(|&&g| g > 0.0).sum();
                let dn_gaps: f64 = window.iter().filter(|&&g| g < 0.0).map(|&g| -g).sum();
                let ratio = if dn_gaps == 0.0 { 1.0 } else { 100.0 * up_gaps / dn_gaps };
                gap_ratios.push(ratio);
            }

            let mut sma = SMA::new(signal_period);
            for ratio in gap_ratios {
                let signal = sma.next(ratio);
                batch_results.push((ratio, signal));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
