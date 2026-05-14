use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Fractal Adaptive Moving Average (FRAMA)
/// As described by John Ehlers.
///
/// The FRAMA uses the fractal dimension of prices to dynamically adapt its smoothing
/// constant (alpha). It rapidly follows major changes in price and slows down when
/// prices are in congestion.
///
/// The `length` parameter specifies the period `N`. If an odd length is provided,
/// it will be automatically converted to an even number (by adding 1) because the
/// fractal dimension calculation requires splitting the period into two equal halves.
#[derive(Debug, Clone)]
pub struct FRAMA {
    length: usize,
    half_length: usize,
    high_history: VecDeque<f64>,
    low_history: VecDeque<f64>,
    filt: f64,
    initialized: bool,
}

impl FRAMA {
    pub fn new(mut length: usize) -> Self {
        // Ehlers notes that N must be an even number.
        if !length.is_multiple_of(2) {
            length += 1;
        }
        let half_length = length / 2;

        Self {
            length,
            half_length,
            high_history: VecDeque::with_capacity(length),
            low_history: VecDeque::with_capacity(length),
            filt: 0.0,
            initialized: false,
        }
    }
}

impl Next<(f64, f64, f64)> for FRAMA {
    type Output = f64; // The filtered value (FRAMA)

    fn next(&mut self, (high, low, price): (f64, f64, f64)) -> Self::Output {
        if self.high_history.len() == self.length {
            self.high_history.pop_back();
            self.low_history.pop_back();
        }

        self.high_history.push_front(high);
        self.low_history.push_front(low);

        // Not enough data to compute the fractal dimension
        if self.high_history.len() < self.length {
            self.filt = price;
            return self.filt;
        }

        // Calculate Highest High and Lowest Low over different periods
        let mut hh1 = f64::MIN;
        let mut ll1 = f64::MAX;
        for i in 0..self.half_length {
            hh1 = hh1.max(self.high_history[i]);
            ll1 = ll1.min(self.low_history[i]);
        }
        let n1 = (hh1 - ll1) / (self.half_length as f64);

        let mut hh2 = f64::MIN;
        let mut ll2 = f64::MAX;
        for i in self.half_length..self.length {
            hh2 = hh2.max(self.high_history[i]);
            ll2 = ll2.min(self.low_history[i]);
        }
        let n2 = (hh2 - ll2) / (self.half_length as f64);

        let mut hh3 = f64::MIN;
        let mut ll3 = f64::MAX;
        for i in 0..self.length {
            hh3 = hh3.max(self.high_history[i]);
            ll3 = ll3.min(self.low_history[i]);
        }
        let n3 = (hh3 - ll3) / (self.length as f64);

        let mut dimen = 1.0;
        if n1 > 0.0 && n2 > 0.0 && n3 > 0.0 {
            dimen = ((n1 + n2).ln() - n3.ln()) / std::f64::consts::LN_2;
        }

        let mut alpha = (-4.6 * (dimen - 1.0)).exp();
        alpha = alpha.clamp(0.01, 1.0);

        if !self.initialized {
            self.filt = price;
            self.initialized = true;
        } else {
            self.filt = alpha * price + (1.0 - alpha) * self.filt;
        }

        self.filt
    }
}

pub const FRAMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Fractal Adaptive Moving Average",
    description: "An adaptive moving average that uses the fractal dimension of prices to dynamically change its smoothing constant.",
    usage: "Use as an adaptive moving average that slows dramatically during consolidation and speeds up during trending phases. Outperforms fixed-period MAs in ranging markets by avoiding false crossovers.",
    keywords: &["moving-average", "adaptive", "fractal", "smoothing"],
    ehlers_summary: "The Fractal Adaptive Moving Average uses the fractal dimension of recent price action to adapt its smoothing constant. During trending markets the fractal dimension approaches 1 (a line) producing a fast-reacting EMA; during ranging markets the dimension approaches 2 (a plane) slowing the average dramatically to filter chop.",
    params: &[ParamDef {
        name: "length",
        default: "16",
        description: "Length (must be an even number; odd values will be incremented by 1).",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/FRAMA.pdf",
    formula_latex: r#"
\[
D = \frac{\log(N_1 + N_2) - \log(N_3)}{\log(2)}
\]
\[
\alpha = \exp(-4.6 (D - 1))
\]
\[
\text{FRAMA}_t = \alpha P_t + (1 - \alpha) \text{FRAMA}_{t-1}
\]
"#,
    gold_standard_file: "frama.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn frama_batch(data: &[(f64, f64, f64)], length: usize) -> Vec<f64> {
        let mut indicator = FRAMA::new(length);
        data.iter().map(|&x| indicator.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_frama_parity(input in prop::collection::vec((0.1..100.0, 0.1..100.0, 0.1..100.0), 1..100)) {
            // Adjust input so High >= Low
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, p) in input {
                let h_f64: f64 = h;
                let l_f64: f64 = l;
                let high = h_f64.max(l_f64);
                let low = h_f64.min(l_f64);
                adj_input.push((high, low, p));
            }

            let length = 16;
            let mut streaming_ind = FRAMA::new(length);
            let streaming_results: Vec<f64> = adj_input.iter().map(|&x| streaming_ind.next(x)).collect();
            let batch_results = frama_batch(&adj_input, length);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(*s, *b, epsilon = 1e-6);
            }
        }
    }
}
