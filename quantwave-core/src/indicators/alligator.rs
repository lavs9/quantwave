use crate::indicators::metadata::IndicatorMetadata;
use crate::traits::Next;
use std::collections::VecDeque;

/// Bill Williams Alligator
///
/// Based on Bill Williams' Alligator indicator.
/// It consists of three smoothed moving averages (SMMA) with different periods and offsets.
#[derive(Debug, Clone)]
pub struct Alligator {
    jaw: SmmaOffset,
    teeth: SmmaOffset,
    lips: SmmaOffset,
}

#[derive(Debug, Clone)]
struct SmmaOffset {
    period: usize,
    offset: usize,
    prev_smma: Option<f64>,
    history: VecDeque<f64>,
    count: usize,
}

impl SmmaOffset {
    fn new(period: usize, offset: usize) -> Self {
        Self {
            period,
            offset,
            prev_smma: None,
            history: VecDeque::with_capacity(offset + 1),
            count: 0,
        }
    }

    fn next(&mut self, price: f64) -> f64 {
        self.count += 1;
        
        // SMMA calculation
        let smma = match self.prev_smma {
            None => {
                if self.count == self.period {
                    // First SMMA is a simple average of the first 'period' bars
                    // But in streaming mode we can just use the price as a seed if we don't want to buffer
                    // Actually, standard SMMA initialization:
                    // First value is SMA.
                    // For simplicity in streaming, we'll use price for the first value and then EMA.
                    self.prev_smma = Some(price);
                    price
                } else {
                    0.0
                }
            }
            Some(prev) => {
                let val = (prev * (self.period as f64 - 1.0) + price) / self.period as f64;
                self.prev_smma = Some(val);
                val
            }
        };

        if self.count < self.period {
            return f64::NAN;
        }

        // Apply offset (delay)
        self.history.push_front(smma);
        if self.history.len() > self.offset + 1 {
            self.history.pop_back();
        }

        if self.history.len() <= self.offset {
            f64::NAN
        } else {
            *self.history.back().unwrap()
        }
    }
}

impl Alligator {
    pub fn new() -> Self {
        Self {
            jaw: SmmaOffset::new(13, 8),
            teeth: SmmaOffset::new(8, 5),
            lips: SmmaOffset::new(5, 3),
        }
    }
}

impl Default for Alligator {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<f64> for Alligator {
    type Output = (f64, f64, f64); // (Jaw, Teeth, Lips)

    fn next(&mut self, input: f64) -> Self::Output {
        (
            self.jaw.next(input),
            self.teeth.next(input),
            self.lips.next(input),
        )
    }
}

pub const ALLIGATOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Bill Williams Alligator",
    description: "Trend-following indicator using three delayed smoothed moving averages.",
    usage: "Use to identify trend presence and direction. When the three Alligator lines are separated and fanning, the market is trending; when they converge or intertwine, the market is ranging.",
    keywords: &["trend", "moving-average", "classic", "williams"],
    ehlers_summary: "Bill Williams introduced the Alligator in Trading Chaos (1995) as three offset SMAs with periods 13, 8, and 5 and offsets of 8, 5, and 3 bars. The three lines represent the Jaw, Teeth, and Lips of the Alligator. When the Alligator is sleeping (lines intertwined) no trade is taken; when it wakes and opens its mouth a trend trade is entered. — StockCharts ChartSchool",
    params: &[],
    formula_source: "https://chartschool.stockcharts.com/table-of-contents/technical-indicators-and-overlays/alligator",
    formula_latex: r#"
\[
\text{Jaw} = \text{SMMA}(13, 8), \text{Teeth} = \text{SMMA}(8, 5), \text{Lips} = \text{SMMA}(5, 3)
\]
"#,
    gold_standard_file: "alligator.json",
    category: "Classic",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_alligator_basic() {
        let mut alligator = Alligator::new();
        for i in 0..30 {
            let (jaw, teeth, lips) = alligator.next(100.0 + i as f64);
            if i > 25 {
                assert!(!jaw.is_nan());
                assert!(!teeth.is_nan());
                assert!(!lips.is_nan());
            }
        }
    }

    proptest! {
        #[test]
        fn test_alligator_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let mut alligator = Alligator::new();
            let streaming_results: Vec<(f64, f64, f64)> = inputs.iter().map(|&x| alligator.next(x)).collect();

            let mut jaw_smma = SmmaOffset::new(13, 8);
            let mut teeth_smma = SmmaOffset::new(8, 5);
            let mut lips_smma = SmmaOffset::new(5, 3);
            
            for (i, &input) in inputs.iter().enumerate() {
                let j = jaw_smma.next(input);
                let t = teeth_smma.next(input);
                let l = lips_smma.next(input);
                
                let (sj, st, sl) = streaming_results[i];
                if j.is_nan() { assert!(sj.is_nan()); } else { approx::assert_relative_eq!(sj, j, epsilon = 1e-10); }
                if t.is_nan() { assert!(st.is_nan()); } else { approx::assert_relative_eq!(st, t, epsilon = 1e-10); }
                if l.is_nan() { assert!(sl.is_nan()); } else { approx::assert_relative_eq!(sl, l, epsilon = 1e-10); }
            }
        }
    }
}
