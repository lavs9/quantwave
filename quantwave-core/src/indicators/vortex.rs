use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Vortex Indicator
/// VI+ = Sum(VM+) / Sum(TR)
/// VI- = Sum(VM-) / Sum(TR)
/// where:
/// VM+ = |High - prevLow|
/// VM- = |Low - prevHigh|
/// TR = True Range
#[derive(Debug, Clone)]
pub struct VortexIndicator {
    period: usize,
    vm_plus: VecDeque<f64>,
    vm_minus: VecDeque<f64>,
    tr: VecDeque<f64>,
    sum_vm_plus: f64,
    sum_vm_minus: f64,
    sum_tr: f64,
    prev_high: Option<f64>,
    prev_low: Option<f64>,
    prev_close: Option<f64>,
}

impl VortexIndicator {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            vm_plus: VecDeque::with_capacity(period),
            vm_minus: VecDeque::with_capacity(period),
            tr: VecDeque::with_capacity(period),
            sum_vm_plus: 0.0,
            sum_vm_minus: 0.0,
            sum_tr: 0.0,
            prev_high: None,
            prev_low: None,
            prev_close: None,
        }
    }
}

impl Next<(f64, f64, f64)> for VortexIndicator {
    type Output = (f64, f64);

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let (vmp, vmm, tr) = match (self.prev_high, self.prev_low, self.prev_close) {
            (Some(ph), Some(pl), Some(pc)) => {
                let vmp = (high - pl).abs();
                let vmm = (low - ph).abs();
                let tr = (high - low).max((high - pc).abs()).max((low - pc).abs());
                (vmp, vmm, tr)
            }
            _ => (0.0, 0.0, 0.0), // Warmup
        };

        self.vm_plus.push_back(vmp);
        self.vm_minus.push_back(vmm);
        self.tr.push_back(tr);
        self.sum_vm_plus += vmp;
        self.sum_vm_minus += vmm;
        self.sum_tr += tr;

        if self.vm_plus.len() > self.period {
            if let Some(old_vmp) = self.vm_plus.pop_front() {
                self.sum_vm_plus -= old_vmp;
            }
            if let Some(old_vmm) = self.vm_minus.pop_front() {
                self.sum_vm_minus -= old_vmm;
            }
            if let Some(old_tr) = self.tr.pop_front() {
                self.sum_tr -= old_tr;
            }
        }

        self.prev_high = Some(high);
        self.prev_low = Some(low);
        self.prev_close = Some(close);

        if self.sum_tr == 0.0 {
            (1.0, 1.0)
        } else {
            (self.sum_vm_plus / self.sum_tr, self.sum_vm_minus / self.sum_tr)
        }
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
    struct VortexCase {
        high: Vec<f64>,
        low: Vec<f64>,
        close: Vec<f64>,
        expected_plus: Vec<f64>,
        expected_minus: Vec<f64>,
    }

    #[test]
    fn test_vortex_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/vortex_14.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path.parent().unwrap().join("tests/gold_standard/vortex_14.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: VortexCase = serde_json::from_str(&content).unwrap();

        let mut vi = VortexIndicator::new(14);
        for i in 0..case.high.len() {
            let (plus, minus) = vi.next((case.high[i], case.low[i], case.close[i]));
            approx::assert_relative_eq!(plus, case.expected_plus[i], epsilon = 1e-6);
            approx::assert_relative_eq!(minus, case.expected_minus[i], epsilon = 1e-6);
        }
    }

    fn vortex_batch(data: Vec<(f64, f64, f64)>, period: usize) -> Vec<(f64, f64)> {
        let mut vi = VortexIndicator::new(period);
        data.into_iter().map(|x| vi.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_vortex_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, c) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(l_f).max(c_f);
                let low = l_f.min(h_f).min(c_f);
                adj_input.push((high, low, c_f));
            }

            let period = 14;
            let mut vi = VortexIndicator::new(period);
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(vi.next(val));
            }

            let batch_results = vortex_batch(adj_input, period);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_vortex_basic() {
        let mut vi = VortexIndicator::new(3);
        // Bar 0: H=10, L=8, C=9. No prev. vmp=0, vmm=0, tr=0. Output (1,1)
        // Bar 1: H=12, L=10, C=11. Prev H=10, L=8, C=9.
        // vmp = |12-8|=4, vmm=|10-10|=0, tr=max(2, |12-9|=3, |10-9|=1)=3
        // sum_vmp=4, sum_vmm=0, sum_tr=3. Output (4/3, 0/3) = (1.333, 0)
        
        let (p0, m0) = vi.next((10.0, 8.0, 9.0));
        assert_eq!(p0, 1.0);
        assert_eq!(m0, 1.0);

        let (p1, m1) = vi.next((12.0, 10.0, 11.0));
        approx::assert_relative_eq!(p1, 1.3333333333, epsilon = 1e-6);
        assert_eq!(m1, 0.0);
    }
}


pub const VORTEX_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Vortex Indicator",
    description: "The Vortex Indicator helps identify the start of a new trend or the continuation of an existing one.",
    params: &[
        ParamDef { name: "period", default: "14", description: "Period" },
    ],
    formula_source: "https://www.investopedia.com/terms/v/vortex-indicator-vi.asp",
    formula_latex: r#"
\[
VI+ = \frac{\sum VM+}{\sum TR} \\ VI- = \frac{\sum VM-}{\sum TR}
\]
"#,
    gold_standard_file: "vortex.json",
    category: "Classic",
};
