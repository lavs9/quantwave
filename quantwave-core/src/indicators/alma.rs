use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::WarmupPolicy;
use crate::traits::Next;
use crate::utils::RingBuffer as VecDeque;

#[derive(Debug, Clone)]
pub struct ALMA {
    period: usize,
    _offset: f64,
    _sigma: f64,
    window: VecDeque<f64>,
    weights: Vec<f64>,
    warmup: WarmupPolicy,
}

impl ALMA {
    pub fn new(period: usize, offset: f64, sigma: f64) -> Self {
        Self::with_warmup(period, offset, sigma, WarmupPolicy::Partial)
    }

    pub fn with_warmup(period: usize, offset: f64, sigma: f64, warmup: WarmupPolicy) -> Self {
        let m = offset * (period as f64 - 1.0);
        let s = period as f64 / sigma;
        let mut weights = Vec::with_capacity(period);
        let mut sum_w = 0.0;

        for i in 0..period {
            let weight = (-((i as f64 - m).powi(2) / (2.0 * s.powi(2)))).exp();
            weights.push(weight);
            sum_w += weight;
        }

        // Normalize weights
        for w in weights.iter_mut() {
            *w /= sum_w;
        }

        Self {
            period,
            _offset: offset,
            _sigma: sigma,
            window: VecDeque::with_capacity(period),
            weights,
            warmup,
        }
    }
}

impl Next<f64> for ALMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if input.is_nan() {
            return f64::NAN;
        }
        self.window.push_back(input);
        if self.window.len() > self.period {
            self.window.pop_front();
        }

        if self.window.len() < self.period {
            if self.warmup == WarmupPolicy::Nan {
                return f64::NAN;
            }
            let mut sum_w = 0.0;
            let mut weighted_val_sum = 0.0;
            for (i, &val) in self.window.iter().enumerate() {
                let weight = self.weights[i + self.period - self.window.len()];
                weighted_val_sum += val * weight;
                sum_w += weight;
            }
            if sum_w == 0.0 {
                0.0
            } else {
                weighted_val_sum / sum_w
            }
        } else {
            let mut weighted_val_sum = 0.0;
            for (i, &val) in self.window.iter().enumerate() {
                weighted_val_sum += val * self.weights[i];
            }
            weighted_val_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        assert_indicator_parity, check_batch_streaming_parity, load_gold_standard,
    };
    use proptest::prelude::*;

    #[test]
    fn test_alma_gold_standard() {
        let case = load_gold_standard("alma_9_085_6");
        let alma = ALMA::new(9, 0.85, 6.0);
        assert_indicator_parity(alma, &case.input, &case.expected);
    }

    fn alma_batch(data: Vec<f64>, period: usize, offset: f64, sigma: f64) -> Vec<f64> {
        let mut alma = ALMA::new(period, offset, sigma);
        data.into_iter().map(|x| alma.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_alma_parity(input in prop::collection::vec(0.0..1000.0, 1..100)) {
            let period = 9;
            let offset = 0.85;
            let sigma = 6.0;
            let indicator = ALMA::new(period, offset, sigma);
            check_batch_streaming_parity(input, indicator, |data| alma_batch(data, period, offset, sigma));
        }
    }

    #[test]
    fn test_alma_basic() {
        let mut alma = ALMA::new(9, 0.85, 6.0);
        for i in 1..20 {
            let val = alma.next(i as f64);
            if i >= 9 {
                assert!(val > 0.0);
            }
        }
    }

    #[test]
    fn test_alma_warmup_nan() {
        let mut alma = ALMA::with_warmup(5, 0.85, 6.0, WarmupPolicy::Nan);
        for i in 1..5 {
            assert!(alma.next(i as f64).is_nan());
        }
        let val5 = alma.next(5.0);
        assert!(!val5.is_nan());
        assert!(val5 > 0.0);
    }

    #[test]
    fn test_alma_nan_input_recovery() {
        let mut alma = ALMA::new(5, 0.85, 6.0);
        for i in 1..5 {
            alma.next(i as f64);
        }
        assert!(alma.next(f64::NAN).is_nan());
        // Ring buffer flushes the NaN after 5 bars
        for _ in 0..5 {
            alma.next(10.0);
        }
        let clean = alma.next(10.0);
        assert!(!clean.is_nan());
        assert!((clean - 10.0).abs() < 1e-4);
    }
}

pub const ALMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Arnaud Legoux Moving Average",
    description: "ALMA is designed to reduce lag while providing high smoothness. During warmup (< period bars), partial-window Gaussian weights are normalized over available bars to emit finite values immediately from bar 0.",
    usage: "Use as a low-latency moving average that reduces lag compared to EMA while controlling overshoot through the Gaussian offset parameter. Well-suited for momentum systems. Supports WarmupPolicy (Partial default, or Nan).",
    keywords: &["moving-average", "smoothing", "low-latency", "adaptive"],
    ehlers_summary: "The Arnaud Legoux Moving Average applies a Gaussian-shaped weight distribution offset toward the recent end of the lookback window. The sigma parameter controls weight spread and the offset parameter controls how far the Gaussian peak is positioned from the current bar, enabling a lag-accuracy trade-off unavailable in standard MAs.",
    params: &[
        ParamDef {
            name: "period",
            default: "9",
            description: "Period",
        },
        ParamDef {
            name: "offset",
            default: "0.85",
            description: "Offset",
        },
        ParamDef {
            name: "sigma",
            default: "6.0",
            description: "Sigma",
        },
    ],
    formula_source: "https://www.prorealcode.com/prorealtime-indicators/arnaud-legoux-moving-average-alma/",
    formula_latex: r#"
\[
ALMA = \sum (W_i \times P_i) / \sum W_i
\]
"#,
    gold_standard_file: "alma.json",
    category: "Classic",
};
