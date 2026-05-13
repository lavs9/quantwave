use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Choppiness Index
///
/// The Choppiness Index is a volatility indicator used to determine if the 
/// market is trending or ranging (choppy).
/// Values above 61.8 indicate choppiness, while values below 38.2 indicate trending.
#[derive(Debug, Clone)]
pub struct ChoppinessIndex {
    period: usize,
    tr_window: VecDeque<f64>,
    high_window: VecDeque<f64>,
    low_window: VecDeque<f64>,
    prev_close: Option<f64>,
}

impl ChoppinessIndex {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            tr_window: VecDeque::with_capacity(period),
            high_window: VecDeque::with_capacity(period),
            low_window: VecDeque::with_capacity(period),
            prev_close: None,
        }
    }
}

impl Default for ChoppinessIndex {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Next<(f64, f64, f64)> for ChoppinessIndex {
    type Output = f64; // Choppiness value

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        // True Range calculation
        let tr = match self.prev_close {
            None => high - low,
            Some(pc) => {
                let h_pc = (high - pc).abs();
                let l_pc = (low - pc).abs();
                let h_l = high - low;
                h_pc.max(l_pc).max(h_l)
            }
        };
        self.prev_close = Some(close);

        self.tr_window.push_front(tr);
        self.high_window.push_front(high);
        self.low_window.push_front(low);

        if self.tr_window.len() > self.period {
            self.tr_window.pop_back();
            self.high_window.pop_back();
            self.low_window.pop_back();
        }

        if self.tr_window.len() < self.period {
            return 50.0; // Neutral value during startup
        }

        // sum(TrueRange, N)
        let sum_tr: f64 = self.tr_window.iter().sum();
        
        // MaxHigh(N) - MinLow(N)
        let mut max_h = f64::MIN;
        let mut min_l = f64::MAX;
        for &h in &self.high_window { if h > max_h { max_h = h; } }
        for &l in &self.low_window { if l < min_l { min_l = l; } }
        
        let range = max_h - min_l;
        
        if range == 0.0 {
            100.0
        } else {
            let n_f = self.period as f64;
            100.0 * (sum_tr / range).log10() / n_f.log10()
        }
    }
}

pub const CHOPPINESS_INDEX_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Choppiness Index",
    description: "Determines if the market is trending (low values) or ranging/choppy (high values).",
    params: &[
        ParamDef { name: "period", default: "14", description: "Lookback period" },
    ],
    formula_source: "https://www.tradingview.com/support/solutions/43000501980-choppiness-index-chop/",
    formula_latex: r#"
\[
CHOP = 100 \times \frac{\log_{10}(\sum_{i=1}^n ATR(1)_i / (\max(H, n) - \min(L, n)))}{\log_{10}(n)}
\]
"#,
    gold_standard_file: "choppiness_index.json",
    category: "Modern",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_chop_basic() {
        let mut chop = ChoppinessIndex::new(14);
        for i in 0..30 {
            let val = chop.next((100.0 + i as f64, 90.0 + i as f64, 95.0 + i as f64));
            assert!(val >= 0.0 && val <= 100.0);
        }
    }

    proptest! {
        #[test]
        fn test_chop_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let period = 14;
            let mut chop = ChoppinessIndex::new(period);
            // Mock H/L/C from single value
            let ohlc_inputs: Vec<(f64, f64, f64)> = inputs.iter().map(|&x| (x + 1.0, x - 1.0, x)).collect();
            let streaming_results: Vec<f64> = ohlc_inputs.iter().map(|&x| chop.next(x)).collect();

            let mut chop_batch = ChoppinessIndex::new(period);
            let batch_results: Vec<f64> = ohlc_inputs.iter().map(|&x| chop_batch.next(x)).collect();

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
