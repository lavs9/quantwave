use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Simple Moving Average (SMA)
#[derive(Debug, Clone)]
pub struct SMA {
    period: usize,
    window: VecDeque<f64>,
    sum: f64,
}

impl SMA {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            window: VecDeque::with_capacity(period),
            sum: 0.0,
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

        if self.window.len() > self.period {
            if let Some(oldest) = self.window.pop_front() {
                self.sum -= oldest;
            }
        }

        self.sum / self.window.len() as f64
    }
}

/// Exponential Moving Average (EMA)
#[derive(Debug, Clone)]
pub struct EMA {
    _period: usize,
    alpha: f64,
    current_ema: Option<f64>,
}

impl EMA {
    pub fn new(period: usize) -> Self {
        Self {
            _period: period,
            alpha: 2.0 / (period as f64 + 1.0),
            current_ema: None,
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
#[derive(Debug, Clone)]
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

    #[test]
    fn test_ema_basic() {
        let mut ema = EMA::new(3);
        assert_eq!(ema.next(10.0), 10.0);
        approx::assert_relative_eq!(ema.next(12.0), 11.0); // alpha = 0.5. 0.5*12 + 0.5*10 = 11
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
    params: &[ParamDef {
        name: "period",
        default: "14",
        description: "Smoothing period",
    }],
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
    params: &[ParamDef {
        name: "period",
        default: "14",
        description: "Smoothing period",
    }],
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
