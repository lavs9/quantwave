use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::SMA;
use crate::traits::Next;
use std::collections::VecDeque;

pub const METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Rate of Directional Change",
    description: "Measures the frequency of directional changes (zigzag flips) within a moving window to identify whipsaw market conditions.",
    usage: "Use to filter out false signals in trend-following strategies. High RODC values indicate a whipsaw environment, while low values suggest a trending market.",
    keywords: &["zigzag", "whipsaw", "momentum", "volatility", "directional change"],
    ehlers_summary: "RODC tracks the number of alternating up and down zigzag segments within a fixed window. By normalizing this count and smoothing it, the indicator provides a measure of how 'noisy' the price action is. It declines in trending environments and increases during whipsaws. — Richard Poster, TASC March 2024",
    params: &[
        ParamDef {
            name: "window_size",
            default: "30",
            description: "Lookback window for zigzag calculation",
        },
        ParamDef {
            name: "threshold",
            default: "0.0015",
            description: "Zigzag reversal threshold (absolute price change)",
        },
        ParamDef {
            name: "smooth_period",
            default: "3",
            description: "Smoothing period for the resulting rate",
        },
    ],
    formula_source: "TASC March 2024",
    formula_latex: r#"
\[
RODC = SMA(100 \times \frac{NumUD}{WindowSize}, SmoothPeriod)
\]
"#,
    gold_standard_file: "rodc_30_15_3.json",
    category: "Volatility",
};

/// Rate of Directional Change (RODC)
///
/// Measures the frequency of directional changes within a moving window.
#[derive(Debug, Clone)]
pub struct RODC {
    window_size: usize,
    threshold: f64,
    sma: SMA,
    price_window: VecDeque<f64>,
}

impl RODC {
    pub fn new(window_size: usize, threshold: f64, smooth_period: usize) -> Self {
        Self {
            window_size,
            threshold,
            sma: SMA::new(smooth_period),
            price_window: VecDeque::with_capacity(window_size + 1),
        }
    }
}

impl Next<f64> for RODC {
    type Output = f64;

    fn next(&mut self, price: f64) -> Self::Output {
        self.price_window.push_back(price);

        if self.price_window.len() <= self.window_size {
            // Need at least window_size + 1 points to have window_size gaps/segments
            // However, the original code starts from Close[BkData].
            // If we have less data, we can either return 0 or calculate on what we have.
            // TradeStation code is usually executed on a chart where history is available.
            // For streaming, we'll return 0 until we have enough data.
            return 0.0;
        }

        if self.price_window.len() > self.window_size + 1 {
            self.price_window.pop_front();
        }

        // Calculate zigzag flips within the current window
        let mut n_ud = 1;
        let mut mode_up = true;
        
        // Start from the oldest price in the window
        let mut x_ext = *self.price_window.front().unwrap();
        
        // Iterate forward through the window (excluding the very first point which is x_ext)
        for i in 1..self.price_window.len() {
            let x_cls = self.price_window[i];
            
            if !mode_up {
                if x_ext > x_cls {
                    // Still mode down, update extreme low
                    x_ext = x_cls;
                } else if x_cls - x_ext >= self.threshold {
                    // Reversal to mode up
                    mode_up = true;
                    n_ud += 1;
                    x_ext = x_cls;
                }
            } else {
                if x_ext < x_cls {
                    // Still mode up, update extreme high
                    x_ext = x_cls;
                } else if x_ext - x_cls >= self.threshold {
                    // Reversal to mode down
                    mode_up = false;
                    n_ud += 1;
                    x_ext = x_cls;
                }
            }
        }

        let raw_rodc = 100.0 * n_ud as f64 / self.window_size as f64;
        self.sma.next(raw_rodc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rodc_basic() {
        // window_size = 4, threshold = 2.0, smooth = 1 (no smoothing)
        let mut rodc = RODC::new(4, 2.0, 1);
        
        // Initializing... need 5 points
        assert_eq!(rodc.next(10.0), 0.0);
        assert_eq!(rodc.next(11.0), 0.0);
        assert_eq!(rodc.next(12.0), 0.0);
        assert_eq!(rodc.next(13.0), 0.0);
        
        // Bar 5: [10, 11, 12, 13, 14]
        // Window starts at 10. n_ud = 1. mode_up = true. x_ext = 10.
        // 11 > 10, x_ext = 11.
        // 12 > 11, x_ext = 12.
        // 13 > 12, x_ext = 13.
        // 14 > 13, x_ext = 14.
        // Result: 100 * 1 / 4 = 25.0
        assert_eq!(rodc.next(14.0), 25.0);
        
        // Bar 6: [11, 12, 13, 14, 12]
        // x_ext = 11. mode_up = true.
        // 12 > 11, x_ext = 12.
        // 13 > 12, x_ext = 13.
        // 14 > 13, x_ext = 14.
        // 12: 14 - 12 = 2.0 >= threshold. Flip! mode_up = false. n_ud = 2. x_ext = 12.
        // Result: 100 * 2 / 4 = 50.0
        assert_eq!(rodc.next(12.0), 50.0);

        // Bar 7: [12, 13, 14, 12, 15]
        // x_ext = 12. mode_up = true.
        // 13 > 12, x_ext = 13.
        // 14 > 13, x_ext = 14.
        // 12: flip. n_ud = 2. x_ext = 12. mode_up = false.
        // 15: 15 - 12 = 3.0 >= threshold. Flip! n_ud = 3. x_ext = 15. mode_up = true.
        // Result: 100 * 3 / 4 = 75.0
        assert_eq!(rodc.next(15.0), 75.0);
    }
}
