use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::utils::RingBuffer as VecDeque;
use serde::{Deserialize, Serialize};

/// Policy governing indicator warm-up output before the full window length is seen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WarmupPolicy {
    /// Emit partial-window / first-value seeded outputs from bar 0 (quantwave default).
    #[default]
    Partial,
    /// Emit `f64::NAN` until the full period window is satisfied (TA-Lib convention).
    Nan,
}

/// Simple Moving Average (SMA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMA {
    period: usize,
    window: VecDeque<f64>,
    sum: f64,
    warmup: WarmupPolicy,
}

impl SMA {
    pub fn new(period: usize) -> Self {
        Self::with_warmup(period, WarmupPolicy::Partial)
    }

    pub fn with_warmup(period: usize, warmup: WarmupPolicy) -> Self {
        Self {
            period,
            window: VecDeque::with_capacity(period),
            sum: 0.0,
            warmup,
        }
    }
}

impl From<usize> for SMA {
    fn from(period: usize) -> Self {
        Self::new(period)
    }
}

impl Next<f64> for SMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_back(input);
        self.sum += input;

        if self.window.len() > self.period
            && let Some(oldest) = self.window.pop_front()
        {
            self.sum -= oldest;
        }

        // A single NaN/Inf sample poisons the incremental running sum forever
        // under naive `sum -= oldest`: NaN propagates through every `+=`/`-=`
        // it touches, and subtracting the same NaN back out when it ages out
        // of the window never un-poisons it (see quantwave-1jqv Phase 0
        // audit's `nan_passthrough` finding on plain `sma`). Recompute
        // directly from the window whenever contaminated, so the indicator
        // self-heals exactly when the window does.
        if self.sum.is_nan() || self.sum.is_infinite() {
            self.sum = self.window.iter().sum();
        }

        if self.warmup == WarmupPolicy::Nan && self.window.len() < self.period {
            return f64::NAN;
        }

        self.sum / self.window.len() as f64
    }
}

/// Exponential Moving Average (EMA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMA {
    period: usize,
    alpha: f64,
    current_ema: Option<f64>,
    warmup: WarmupPolicy,
    warmup_window: VecDeque<f64>,
}

impl EMA {
    pub fn new(period: usize) -> Self {
        Self::with_warmup(period, WarmupPolicy::Partial)
    }

    pub fn with_warmup(period: usize, warmup: WarmupPolicy) -> Self {
        Self {
            period,
            alpha: 2.0 / (period as f64 + 1.0),
            current_ema: None,
            warmup,
            warmup_window: VecDeque::with_capacity(period),
        }
    }
}

impl From<usize> for EMA {
    fn from(period: usize) -> Self {
        Self::new(period)
    }
}

impl Next<f64> for EMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if input.is_nan() {
            return f64::NAN;
        }

        if self.warmup == WarmupPolicy::Nan && self.current_ema.is_none() {
            self.warmup_window.push_back(input);
            if self.warmup_window.len() < self.period {
                return f64::NAN;
            }
            let seed: f64 = self.warmup_window.iter().sum::<f64>() / self.period as f64;
            self.current_ema = Some(seed);
            return seed;
        }

        match self.current_ema {
            Some(prev_ema) => {
                let ema = self.alpha * input + (1.0 - self.alpha) * prev_ema;
                self.current_ema = Some(ema);
                ema
            }
            None => {
                self.current_ema = Some(input);
                input
            }
        }
    }
}

/// Weighted Moving Average (WMA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WMA {
    period: usize,
    window: VecDeque<f64>,
}

impl WMA {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            window: VecDeque::with_capacity(period),
        }
    }
}

impl From<usize> for WMA {
    fn from(period: usize) -> Self {
        Self::new(period)
    }
}

impl Next<f64> for WMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_back(input);
        if self.window.len() > self.period {
            self.window.pop_front();
        }

        let mut weight_sum = 0.0;
        let mut weighted_val_sum = 0.0;

        for (i, &val) in self.window.iter().enumerate() {
            let weight = (i + 1) as f64;
            weighted_val_sum += val * weight;
            weight_sum += weight;
        }

        if weight_sum == 0.0 {
            0.0
        } else {
            weighted_val_sum / weight_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_indicator_parity, load_gold_standard};

    #[test]
    fn test_sma_gold_standard() {
        let case = load_gold_standard("sma_5");
        let sma = SMA::new(3); // The expected values in JSON are for SMA(3)
        assert_indicator_parity(sma, &case.input, &case.expected);
    }

    /// Regression for quantwave-1jqv Phase 0 audit: a single NaN sample must
    /// only poison the output while it is inside the window, not forever.
    /// Before the self-heal recompute, `sum -= oldest` never cleared the NaN
    /// out of the running sum once it had been subtracted back out, so the
    /// indicator stayed NaN for the rest of the series.
    #[test]
    fn test_sma_self_heals_after_nan_ages_out_of_window() {
        let mut sma = SMA::new(3);
        assert_eq!(sma.next(1.0), 1.0);
        assert_eq!(sma.next(2.0), 1.5);
        // NaN enters the window: [1.0, 2.0, NaN] -> undefined, correctly NaN.
        assert!(sma.next(f64::NAN).is_nan());
        // NaN still inside the window: [2.0, NaN, 4.0] -> still undefined.
        assert!(sma.next(4.0).is_nan());
        // NaN still inside the window: [NaN, 4.0, 5.0] -> still undefined.
        assert!(sma.next(5.0).is_nan());
        // NaN has aged out of the window: [4.0, 5.0, 6.0] -> must recover to
        // the true mean, not remain NaN forever.
        let recovered = sma.next(6.0);
        assert!(
            !recovered.is_nan(),
            "SMA should self-heal once the NaN ages out of the window, got {recovered}"
        );
        approx::assert_relative_eq!(recovered, 5.0);
        // And normal operation continues correctly afterward.
        approx::assert_relative_eq!(sma.next(9.0), 20.0 / 3.0);
    }

    #[test]
    fn test_ema_basic() {
        let mut ema = EMA::new(3);
        assert_eq!(ema.next(10.0), 10.0);
        approx::assert_relative_eq!(ema.next(12.0), 11.0); // alpha = 0.5. 0.5*12 + 0.5*10 = 11
    }

    #[test]
    fn test_ema_recovers_after_nan() {
        let mut ema = EMA::new(3); // alpha = 0.5
        assert_eq!(ema.next(1.0), 1.0);
        assert_eq!(ema.next(2.0), 1.5);
        assert_eq!(ema.next(3.0), 2.25);
        // NaN enters: must return NaN, but not permanently poison current_ema
        assert!(ema.next(f64::NAN).is_nan());
        // Next finite sample: must resume smoothing from the last valid EMA (2.25)
        let resumed = ema.next(5.0); // alpha * 5.0 + (1 - alpha) * 2.25 = 2.5 + 1.125 = 3.625
        approx::assert_relative_eq!(resumed, 3.625);
    }

    #[test]
    fn test_sma_nan_warmup() {
        let mut sma = SMA::with_warmup(3, WarmupPolicy::Nan);
        assert!(sma.next(1.0).is_nan());
        assert!(sma.next(2.0).is_nan());
        approx::assert_relative_eq!(sma.next(3.0), 2.0);
        approx::assert_relative_eq!(sma.next(4.0), 3.0);
    }

    #[test]
    fn test_ema_nan_warmup() {
        let mut ema = EMA::with_warmup(3, WarmupPolicy::Nan);
        assert!(ema.next(1.0).is_nan());
        assert!(ema.next(2.0).is_nan());
        // Bar 3: seeded with SMA(1, 2, 3) = 2.0
        approx::assert_relative_eq!(ema.next(3.0), 2.0);
        // Bar 4: alpha = 0.5 -> 0.5 * 4.0 + 0.5 * 2.0 = 3.0
        approx::assert_relative_eq!(ema.next(4.0), 3.0);
    }

    #[test]
    fn test_wma_basic() {
        let mut wma = WMA::new(3);
        assert_eq!(wma.next(1.0), 1.0);
        approx::assert_relative_eq!(wma.next(2.0), 1.6666666666, epsilon = 1e-6); // (1*1 + 2*2)/3 = 5/3 = 1.666
        approx::assert_relative_eq!(wma.next(3.0), 2.3333333333, epsilon = 1e-6); // (1*1 + 2*2 + 3*3)/6 = 14/6 = 2.333
        approx::assert_relative_eq!(wma.next(4.0), 3.3333333333, epsilon = 1e-6); // (2*1 + 3*2 + 4*3)/6 = (2+6+12)/6 = 20/6 = 3.333
    }
}

pub const SMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Simple Moving Average",
    description: "The Simple Moving Average calculates the unweighted mean of the previous N data points.",
    usage: "Use as the foundational smoothing module providing SMA, EMA, WMA, and SMMA implementations that power higher-level indicators across the library.",
    keywords: &["moving-average", "smoothing", "classic", "ema"],
    ehlers_summary: "The core smoothing algorithms — SMA, EMA, WMA — are the building blocks of nearly all technical indicators. EMA applies exponential decay weighting (alpha = 2/(n+1)), SMA applies uniform weighting over N bars, and WMA applies linearly increasing weights emphasizing more recent bars.",
    params: &[
        ParamDef {
            name: "period",
            default: "14",
            description: "Smoothing period",
        },
        ParamDef {
            name: "warmup",
            default: "partial",
            description: "Warm-up behavior ('partial' or 'nan')",
        },
    ],
    formula_source: "https://www.investopedia.com/terms/s/sma.asp",
    formula_latex: r#"
\[
SMA = \frac{1}{n} \sum_{i=1}^{n} P_i
\]
"#,
    gold_standard_file: "sma.json",
    category: "Classic",
};

pub const EMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Exponential Moving Average",
    description: "The Exponential Moving Average gives more weight to recent prices.",
    usage: "Use as the foundational smoothing module providing SMA, EMA, WMA, and SMMA implementations that power higher-level indicators across the library.",
    keywords: &["moving-average", "smoothing", "classic", "ema"],
    ehlers_summary: "The core smoothing algorithms — SMA, EMA, WMA — are the building blocks of nearly all technical indicators. EMA applies exponential decay weighting (alpha = 2/(n+1)), SMA applies uniform weighting over N bars, and WMA applies linearly increasing weights emphasizing more recent bars.",
    params: &[
        ParamDef {
            name: "period",
            default: "14",
            description: "Smoothing period",
        },
        ParamDef {
            name: "warmup",
            default: "partial",
            description: "Warm-up behavior ('partial' or 'nan')",
        },
    ],
    formula_source: "https://www.investopedia.com/terms/e/ema.asp",
    formula_latex: r#"
\[
EMA = P_t \times \alpha + EMA_{t-1} \times (1 - \alpha)
\]
"#,
    gold_standard_file: "ema.json",
    category: "Classic",
};

pub const WMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Weighted Moving Average",
    description: "The Weighted Moving Average assigns linearly decreasing weights.",
    usage: "Use as the foundational smoothing module providing SMA, EMA, WMA, and SMMA implementations that power higher-level indicators across the library.",
    keywords: &["moving-average", "smoothing", "classic", "ema"],
    ehlers_summary: "The core smoothing algorithms — SMA, EMA, WMA — are the building blocks of nearly all technical indicators. EMA applies exponential decay weighting (alpha = 2/(n+1)), SMA applies uniform weighting over N bars, and WMA applies linearly increasing weights emphasizing more recent bars.",
    params: &[ParamDef {
        name: "period",
        default: "14",
        description: "Smoothing period",
    }],
    formula_source: "https://www.investopedia.com/articles/technical/060401.asp",
    formula_latex: r#"
\[
WMA = \frac{P_1 \times n + P_2 \times (n-1) + \dots}{n + (n-1) + \dots + 1}
\]
"#,
    gold_standard_file: "wma.json",
    category: "Classic",
};
