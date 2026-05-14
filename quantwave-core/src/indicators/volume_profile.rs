use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Volume Profile (POC)
///
/// Tracks the volume distribution over a sliding window and returns the
/// Point of Control (POC) - the price level with the highest volume.
#[derive(Debug, Clone)]
pub struct VolumeProfile {
    period: usize,
    bins: usize,
    window: VecDeque<(f64, f64)>, // (price, volume)
}

impl VolumeProfile {
    pub fn new(period: usize, bins: usize) -> Self {
        Self {
            period,
            bins: bins.max(1),
            window: VecDeque::with_capacity(period),
        }
    }
}

impl Next<(f64, f64)> for VolumeProfile {
    type Output = f64;

    fn next(&mut self, (price, volume): (f64, f64)) -> Self::Output {
        self.window.push_back((price, volume));
        if self.window.len() > self.period {
            self.window.pop_front();
        }

        if self.window.is_empty() {
            return f64::NAN;
        }

        // Find min and max price in the window
        let mut min_p = f64::MAX;
        let mut max_p = f64::MIN;
        for &(p, _) in self.window.iter() {
            if p < min_p { min_p = p; }
            if p > max_p { max_p = p; }
        }

        if min_p == max_p {
            return min_p;
        }

        // Create histogram
        let mut histogram = vec![0.0; self.bins];
        let bin_size = (max_p - min_p) / self.bins as f64;

        for &(p, v) in self.window.iter() {
            let mut bin_idx = ((p - min_p) / bin_size).floor() as usize;
            if bin_idx >= self.bins {
                bin_idx = self.bins - 1;
            }
            histogram[bin_idx] += v;
        }

        // Find bin with max volume
        let mut max_v = -1.0;
        let mut poc_idx = 0;
        for (i, &v) in histogram.iter().enumerate() {
            if v > max_v {
                max_v = v;
                poc_idx = i;
            }
        }

        // Return the center of the POC bin
        min_p + (poc_idx as f64 + 0.5) * bin_size
    }
}

pub const VOLUME_PROFILE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Volume Profile",
    description: "Calculates the price level with the highest traded volume (Point of Control) over a sliding window.",
    usage: "Use to identify significant support and resistance levels. The POC represents the price where most market activity occurred, often acting as a magnet for price or a strong barrier. Essential for volume spread analysis and auction market theory.",
    keywords: &["volume", "profile", "poc", "support-resistance", "auction-market-theory"],
    ehlers_summary: "Volume Profile is an advanced charting study that displays trading activity over a specified time period at specified price levels. The Point of Control (POC) is the single most important level in the profile, representing the price at which the most volume was traded. It serves as a key benchmark for identifying value areas and potential trend reversals.",
    params: &[
        ParamDef {
            name: "period",
            default: "200",
            description: "Sliding window size",
        },
        ParamDef {
            name: "bins",
            default: "50",
            description: "Number of price bins in the histogram",
        },
    ],
    formula_source: "https://www.tradingview.com/support/solutions/43000502040-volume-profile-visible-range-vpvr/",
    formula_latex: r#"
\[
BinIdx = \lfloor \frac{Price - Price_{min}}{BinSize} \rfloor
\]
\[
POC = Price_{min} + (Idx_{max\_vol} + 0.5) \times BinSize
\]
"#,
    gold_standard_file: "volume_profile.json",
    category: "Volume",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;

    #[test]
    fn test_volume_profile_basic() {
        let mut vp = VolumeProfile::new(10, 5);
        // All volume at 100
        for _ in 0..5 {
            vp.next((100.0, 10.0));
        }
        // Some volume at 110
        let res = vp.next((110.0, 5.0));
        assert!(res >= 100.0 && res <= 105.0); // POC should still be in the 100 bin
        
        // More volume at 110
        vp.next((110.0, 20.0));
        vp.next((110.0, 20.0));
        let res2 = vp.next((110.0, 20.0));
        assert!(res2 >= 105.0); // POC should shift to higher bin
    }
}
