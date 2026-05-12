use crate::indicators::metadata::IndicatorMetadata;
use crate::traits::Next;

/// Standard Pivot Points
/// Uses the High, Low, and Close of the previous period to calculate 
/// the current period's Support and Resistance levels.
/// Output: (P, R1, S1, R2, S2)
#[derive(Debug, Clone, Default)]
pub struct PivotPoints {
    prev_high: Option<f64>,
    prev_low: Option<f64>,
    prev_close: Option<f64>,
}

impl PivotPoints {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Next<(f64, f64, f64)> for PivotPoints {
    type Output = (f64, f64, f64, f64, f64);

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let (p, r1, s1, r2, s2) = match (self.prev_high, self.prev_low, self.prev_close) {
            (Some(ph), Some(pl), Some(pc)) => {
                let p = (ph + pl + pc) / 3.0;
                let r1 = (p * 2.0) - pl;
                let s1 = (p * 2.0) - ph;
                let r2 = p + (ph - pl);
                let s2 = p - (ph - pl);
                (p, r1, s1, r2, s2)
            }
            _ => (0.0, 0.0, 0.0, 0.0, 0.0), // Warmup
        };

        self.prev_high = Some(high);
        self.prev_low = Some(low);
        self.prev_close = Some(close);

        (p, r1, s1, r2, s2)
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
    struct PivotCase {
        high: Vec<f64>,
        low: Vec<f64>,
        close: Vec<f64>,
        expected_p: Vec<f64>,
        expected_r1: Vec<f64>,
        expected_s1: Vec<f64>,
        expected_r2: Vec<f64>,
        expected_s2: Vec<f64>,
    }

    #[test]
    fn test_pivot_points_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/pivot_points.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path.parent().unwrap().join("tests/gold_standard/pivot_points.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: PivotCase = serde_json::from_str(&content).unwrap();

        let mut pivot = PivotPoints::new();
        for i in 0..case.high.len() {
            let (p, r1, s1, r2, s2) = pivot.next((case.high[i], case.low[i], case.close[i]));
            approx::assert_relative_eq!(p, case.expected_p[i], epsilon = 1e-6);
            approx::assert_relative_eq!(r1, case.expected_r1[i], epsilon = 1e-6);
            approx::assert_relative_eq!(s1, case.expected_s1[i], epsilon = 1e-6);
            approx::assert_relative_eq!(r2, case.expected_r2[i], epsilon = 1e-6);
            approx::assert_relative_eq!(s2, case.expected_s2[i], epsilon = 1e-6);
        }
    }

    fn pivot_batch(data: Vec<(f64, f64, f64)>) -> Vec<(f64, f64, f64, f64, f64)> {
        let mut pivot = PivotPoints::new();
        data.into_iter().map(|x| pivot.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_pivot_points_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, c) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(l_f).max(c_f);
                let low = l_f.min(h_f).min(c_f);
                adj_input.push((high, low, c_f));
            }
            
            let mut pivot = PivotPoints::new();
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(pivot.next(val));
            }

            let batch_results = pivot_batch(adj_input);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-6);
                approx::assert_relative_eq!(s.2, b.2, epsilon = 1e-6);
                approx::assert_relative_eq!(s.3, b.3, epsilon = 1e-6);
                approx::assert_relative_eq!(s.4, b.4, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_pivot_points_basic() {
        let mut pivot = PivotPoints::new();
        let (p0, _, _, _, _) = pivot.next((12.0, 8.0, 10.0));
        assert_eq!(p0, 0.0);
        let (p1, r1, s1, r2, s2) = pivot.next((14.0, 9.0, 11.0));
        assert_eq!(p1, 10.0); // (12+8+10)/3
        assert_eq!(r1, 12.0); // 20 - 8
        assert_eq!(s1, 8.0); // 20 - 12
        assert_eq!(r2, 14.0); // 10 + 4
        assert_eq!(s2, 6.0); // 10 - 4
    }
}


pub const PIVOT_POINTS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Pivot Points",
    description: "Pivot Points are used to determine overall trend over different time frames.",
    params: &[
    ],
    formula_source: "https://www.investopedia.com/terms/p/pivotpoint.asp",
    formula_latex: r#"
\[
P = \frac{H + L + C}{3}
\]
"#,
    gold_standard_file: "pivot_points.json",
};
