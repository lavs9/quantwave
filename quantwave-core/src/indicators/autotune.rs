use crate::indicators::high_pass::HighPass;
use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// AutoTune Filter
///
/// Based on John Ehlers' "The AutoTune Filter" (2025).
/// This indicator dynamically tunes a BandPass filter by identifying the Dominant Cycle
/// using a rolling autocorrelation of high-pass filtered data.
#[derive(Debug, Clone)]
pub struct AutoTuneFilter {
    window: usize,
    bandwidth: f64,
    highpass: HighPass,
    filt_history: VecDeque<f64>,
    dc_prev: f64,
    price_prev: [f64; 2],
    bp_history: [f64; 2],
    count: usize,
}

impl AutoTuneFilter {
    pub fn new(window: usize, bandwidth: f64) -> Self {
        Self {
            window,
            bandwidth,
            highpass: HighPass::new(window),
            filt_history: VecDeque::with_capacity(2 * window),
            dc_prev: window as f64, // Initial guess
            price_prev: [0.0; 2],
            bp_history: [0.0; 2],
            count: 0,
        }
    }
}

impl Next<f64> for AutoTuneFilter {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        let filt = self.highpass.next(input);
        self.filt_history.push_front(filt);
        if self.filt_history.len() > 2 * self.window {
            self.filt_history.pop_back();
        }

        if self.filt_history.len() < 2 * self.window {
            self.price_prev[1] = self.price_prev[0];
            self.price_prev[0] = input;
            return 0.0;
        }

        let mut dc = self.dc_prev;
        let mut min_corr = 1.0;
        let window_f = self.window as f64;

        // Find minimum correlation and Dominant Cycle
        for lag in 1..=self.window {
            let mut sx = 0.0;
            let mut sy = 0.0;
            let mut sxx = 0.0;
            let mut sxy = 0.0;
            let mut syy = 0.0;

            for j in 0..self.window {
                let x = self.filt_history[j];
                let y = self.filt_history[lag + j];
                sx += x;
                sy += y;
                sxx += x * x;
                sxy += x * y;
                syy += y * y;
            }

            let div1 = window_f * sxx - sx * sx;
            let div2 = window_f * syy - sy * sy;

            if div1 > 0.0 && div2 > 0.0 {
                let corr = (window_f * sxy - sx * sy) / (div1 * div2).sqrt();
                if corr < min_corr {
                    min_corr = corr;
                    dc = 2.0 * lag as f64;
                }
            }
        }

        // Limit the rate of change of the Dominant Cycle
        if dc > self.dc_prev + 2.0 {
            dc = self.dc_prev + 2.0;
        }
        if dc < self.dc_prev - 2.0 {
            dc = self.dc_prev - 2.0;
        }
        if dc < 2.0 {
            dc = 2.0;
        }
        self.dc_prev = dc;

        // Bandpass Filter tuned to the Dominant Cycle
        let l1 = (2.0 * PI / dc).cos();
        let g1 = (2.0 * PI * self.bandwidth / dc).cos();
        // Prevent division by zero if g1 is somehow 0, though cos is 0 only at PI/2 + kPI
        let s1 = if g1.abs() > 1e-10 {
            let gamma_inv = 1.0 / g1;
            gamma_inv - (gamma_inv * gamma_inv - 1.0).max(0.0).sqrt()
        } else {
            1.0
        };

        let bp = 0.5 * (1.0 - s1) * (input - self.price_prev[1])
            + l1 * (1.0 + s1) * self.bp_history[0]
            - s1 * self.bp_history[1];

        self.bp_history[1] = self.bp_history[0];
        self.bp_history[0] = bp;
        self.price_prev[1] = self.price_prev[0];
        self.price_prev[0] = input;

        bp
    }
}

pub const AUTOTUNE_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "AutoTune Filter",
    description: "An adaptive BandPass filter that dynamically tunes itself to the market's dominant cycle.",
    usage: "Use to isolate the cyclical component of price while automatically adapting to changes in cycle length. Zero crossings of the output or its rate of change can be used as trading signals.",
    keywords: &["adaptive", "filter", "cycle", "ehlers", "dsp", "autotune"],
    ehlers_summary: "The AutoTune filter provides a bridge between the time domain and frequency domain by using a rolling autocorrelation function to measure the Dominant Cycle in real time. By dynamically tuning a Bandpass filter to twice the lag at which autocorrelation is minimized, it maintains consistent performance and avoids the destructive phase shifts typical of fixed-tuned filters.",
    params: &[
        ParamDef {
            name: "window",
            default: "20",
            description: "Window length for autocorrelation and HighPass filter",
        },
        ParamDef {
            name: "bandwidth",
            default: "0.25",
            description: "Bandwidth of the tuned BandPass filter",
        },
    ],
    formula_source: "references/Ehlers Papers/The AutoTune Filter.pdf",
    formula_latex: r#"
\[
R(lag) = \frac{n \sum X_i Y_i - \sum X_i \sum Y_i}{\sqrt{(n \sum X_i^2 - (\sum X_i)^2)(n \sum Y_i^2 - (\sum Y_i)^2)}}
\]
\[
DC = 2 \times \text{argmin}_{lag} R(lag)
\]
\[
BP = \text{BandPass}(Price, DC, BW)
\]
"#,
    gold_standard_file: "autotune_filter.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_autotune_basic() {
        let mut at = AutoTuneFilter::new(20, 0.25);
        // Feed some data to warm up the 2*window buffer
        for _ in 0..40 {
            at.next(100.0);
        }
        for i in 0..100 {
            // Sine wave with period 20
            let val = at.next(100.0 + (i as f64 * 2.0 * PI / 20.0).sin());
            assert!(!val.is_nan());
        }
    }

    #[test]
    fn test_autotune_dc_tracking() {
        let window = 20;
        let mut at = AutoTuneFilter::new(window, 0.25);
        
        // Sine wave with period 10
        let period = 10.0;
        for i in 0..100 {
            let _ = at.next(100.0 + (i as f64 * 2.0 * PI / period).sin());
        }
        
        // After warming up, dc_prev should be close to the period (10.0)
        // Note: dc is twice the lag of minimum correlation. 
        // For a sine wave, min correlation is at half-period lag.
        // So lag = 5, dc = 10.
        assert!(at.dc_prev >= 8.0 && at.dc_prev <= 12.0, "DC was {}", at.dc_prev);
    }

    proptest! {
        #[test]
        fn test_autotune_parity(
            inputs in prop::collection::vec(90.0..110.0, 60..100),
        ) {
            let window = 20;
            let bandwidth = 0.25;
            let mut at_obj = AutoTuneFilter::new(window, bandwidth);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| at_obj.next(x)).collect();

            // Batch-like verification of the state machine
            let mut hp = HighPass::new(window);
            let mut filt_hist = VecDeque::new();
            let mut dc_prev = window as f64;
            let mut price_prev = [0.0; 2];
            let mut bp_hist = [0.0; 2];
            let mut expected = Vec::with_capacity(inputs.len());

            for (i, &input) in inputs.iter().enumerate() {
                let filt = hp.next(input);
                filt_hist.push_front(filt);
                if filt_hist.len() > 2 * window {
                    filt_hist.pop_back();
                }

                if filt_hist.len() < 2 * window {
                    price_prev[1] = price_prev[0];
                    price_prev[0] = input;
                    expected.push(0.0);
                    continue;
                }

                let mut dc = dc_prev;
                let mut min_corr = 1.0;
                for lag in 1..=window {
                    let mut sx = 0.0; let mut sy = 0.0;
                    let mut sxx = 0.0; let mut sxy = 0.0; let mut syy = 0.0;
                    for j in 0..window {
                        let x = filt_hist[j];
                        let y = filt_hist[lag+j];
                        sx += x; sy += y;
                        sxx += x*x; sxy += x*y; syy += y*y;
                    }
                    let div1 = (window as f64) * sxx - sx*sx;
                    let div2 = (window as f64) * syy - sy*sy;
                    if div1 > 0.0 && div2 > 0.0 {
                        let corr = ((window as f64) * sxy - sx*sy) / (div1*div2).sqrt();
                        if corr < min_corr {
                            min_corr = corr;
                            dc = 2.0 * lag as f64;
                        }
                    }
                }

                if dc > dc_prev + 2.0 { dc = dc_prev + 2.0; }
                if dc < dc_prev - 2.0 { dc = dc_prev - 2.0; }
                if dc < 2.0 { dc = 2.0; }
                dc_prev = dc;

                let l1 = (2.0 * PI / dc).cos();
                let g1 = (2.0 * PI * bandwidth / dc).cos();
                let s1 = 1.0/g1 - (1.0/(g1*g1) - 1.0).max(0.0).sqrt();

                let bp = 0.5 * (1.0 - s1) * (input - price_prev[1])
                    + l1 * (1.0 + s1) * bp_hist[0]
                    - s1 * bp_hist[1];

                bp_hist[1] = bp_hist[0];
                bp_hist[0] = bp;
                price_prev[1] = price_prev[0];
                price_prev[0] = input;
                expected.push(bp);
            }

            for (s, e) in streaming_results.iter().zip(expected.iter()) {
                approx::assert_relative_eq!(s, e, epsilon = 1e-10);
            }
        }
    }
}
