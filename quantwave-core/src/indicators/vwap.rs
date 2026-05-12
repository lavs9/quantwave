use crate::indicators::metadata::IndicatorMetadata;
use crate::traits::Next;

#[derive(Debug, Clone)]
pub struct AnchoredVWAP {
    cumulative_tp_v: f64,
    cumulative_v: f64,
}

impl AnchoredVWAP {
    pub fn new() -> Self {
        Self {
            cumulative_tp_v: 0.0,
            cumulative_v: 0.0,
        }
    }
}

impl Next<(f64, f64, bool)> for AnchoredVWAP {
    type Output = f64;

    fn next(&mut self, (price, volume, anchor): (f64, f64, bool)) -> Self::Output {
        if anchor {
            self.cumulative_tp_v = price * volume;
            self.cumulative_v = volume;
        } else {
            self.cumulative_tp_v += price * volume;
            self.cumulative_v += volume;
        }

        if self.cumulative_v == 0.0 {
            0.0
        } else {
            self.cumulative_tp_v / self.cumulative_v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_vwap_no_reset() {
        let mut vwap = AnchoredVWAP::new();
        // bar 1: price 10, vol 100 -> vwap 10
        // bar 2: price 12, vol 200 -> vwap (10*100 + 12*200) / 300 = (1000 + 2400) / 300 = 3400 / 300 = 11.333333
        
        let result1 = vwap.next((10.0, 100.0, false));
        approx::assert_relative_eq!(result1, 10.0);
        
        let result2 = vwap.next((12.0, 200.0, false));
        approx::assert_relative_eq!(result2, 11.3333333333, epsilon = 1e-6);
    }

    #[test]
    fn test_anchored_vwap_reset() {
        let mut vwap = AnchoredVWAP::new();
        
        vwap.next((10.0, 100.0, false));
        vwap.next((12.0, 200.0, false));
        
        // bar 3: price 15, vol 100, anchor=true -> vwap should reset to 15
        let result3 = vwap.next((15.0, 100.0, true));
        approx::assert_relative_eq!(result3, 15.0);
        
        // bar 4: price 16, vol 100, anchor=false -> vwap (15*100 + 16*100) / 200 = 15.5
        let result4 = vwap.next((16.0, 100.0, false));
        approx::assert_relative_eq!(result4, 15.5);
    }
}


pub const VWAP_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Anchored VWAP",
    description: "Volume Weighted Average Price anchored to a specific starting point.",
    params: &[
    ],
    formula_source: "https://www.investopedia.com/terms/v/vwap.asp",
    formula_latex: r#"
\[
VWAP = \frac{\sum (Price \times Volume)}{\sum Volume}
\]
"#,
    gold_standard_file: "vwap.json",
};
