use crate::traits::Next;
use std::collections::VecDeque;

/// Bill Williams Fractals
/// Identifies a bearish (up) fractal if High[t-2] is greater than High[t-4, t-3, t-1, t].
/// Identifies a bullish (down) fractal if Low[t-2] is less than Low[t-4, t-3, t-1, t].
/// The output is (Bearish, Bullish) meaning (Up Fractal, Down Fractal) at the current bar 
/// which validates the fractal that formed 2 bars ago.
#[derive(Debug, Clone)]
pub struct BillWilliamsFractals {
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
}

impl Default for BillWilliamsFractals {
    fn default() -> Self {
        Self::new()
    }
}

impl BillWilliamsFractals {
    pub fn new() -> Self {
        Self {
            highs: VecDeque::with_capacity(5),
            lows: VecDeque::with_capacity(5),
        }
    }
}

impl Next<(f64, f64)> for BillWilliamsFractals {
    type Output = (bool, bool); // (Bearish/Up, Bullish/Down)

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        self.highs.push_back(high);
        self.lows.push_back(low);

        if self.highs.len() > 5 {
            self.highs.pop_front();
            self.lows.pop_front();
        }

        if self.highs.len() < 5 {
            return (false, false);
        }

        let bearish = self.highs[2] > self.highs[0] &&
                      self.highs[2] > self.highs[1] &&
                      self.highs[2] > self.highs[3] &&
                      self.highs[2] > self.highs[4];

        let bullish = self.lows[2] < self.lows[0] &&
                      self.lows[2] < self.lows[1] &&
                      self.lows[2] < self.lows[3] &&
                      self.lows[2] < self.lows[4];

        (bearish, bullish)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;
    use std::path::Path;
    use proptest::prelude::*;

    #[derive(Debug, Deserialize)]
    struct FractalCase {
        high: Vec<f64>,
        low: Vec<f64>,
        expected_bearish: Vec<bool>,
        expected_bullish: Vec<bool>,
    }

    #[test]
    fn test_fractals_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/fractals.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path.parent().unwrap().join("tests/gold_standard/fractals.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: FractalCase = serde_json::from_str(&content).unwrap();

        let mut fractals = BillWilliamsFractals::new();
        for i in 0..case.high.len() {
            let (bearish, bullish) = fractals.next((case.high[i], case.low[i]));
            assert_eq!(bearish, case.expected_bearish[i]);
            assert_eq!(bullish, case.expected_bullish[i]);
        }
    }

    fn fractals_batch(data: Vec<(f64, f64)>) -> Vec<(bool, bool)> {
        let mut fractals = BillWilliamsFractals::new();
        data.into_iter().map(|x| fractals.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_fractals_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let high = h_f.max(l_f);
                let low = l_f.min(h_f);
                adj_input.push((high, low));
            }
            
            let mut fractals = BillWilliamsFractals::new();
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(fractals.next(val));
            }

            let batch_results = fractals_batch(adj_input);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                assert_eq!(s.0, b.0);
                assert_eq!(s.1, b.1);
            }
        }
    }

    #[test]
    fn test_fractals_basic() {
        let mut f = BillWilliamsFractals::new();
        let h = vec![10.0, 11.0, 15.0, 12.0, 10.0];
        let l = vec![5.0, 6.0, 2.0, 6.0, 7.0];
        
        for i in 0..4 {
            let (bear, bull) = f.next((h[i], l[i]));
            assert!(!bear);
            assert!(!bull);
        }
        
        let (bear, bull) = f.next((h[4], l[4]));
        assert!(bear); // 15.0 > all
        assert!(bull); // 2.0 < all
    }
}
