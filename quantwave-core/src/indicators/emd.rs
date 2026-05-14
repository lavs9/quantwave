use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::SMA;
use crate::traits::Next;
use std::f64::consts::PI;

/// Empirical Mode Decomposition (EMD)
///
/// Based on John Ehlers' "Empirical Mode Decomposition" (2010).
/// EMD decomposes price data into a cycle component (via Bandpass filter)
/// and a trend component (via averaging the bandpass output).
/// It also provides thresholds based on averaged peaks/valleys to identify market modes.
///
/// Returns (Trend, UpperThreshold, LowerThreshold).
#[derive(Debug, Clone)]
pub struct EMD {
    alpha: f64,
    beta: f64,
    fraction: f64,
    price_prev1: f64,
    price_prev2: f64,
    bp_history: [f64; 2],
    mean_sma: SMA,
    peak_sma: SMA,
    valley_sma: SMA,
    peak: f64,
    valley: f64,
    count: usize,
}

impl EMD {
    pub fn new(period: usize, delta: f64, fraction: f64) -> Self {
        // beta = Cosine(360 / Period);
        let beta = (2.0 * PI / period as f64).cos();
        // gamma = 1 / Cosine(720*delta / Period);
        let gamma = 1.0 / (4.0 * PI * delta / period as f64).cos();
        // alpha = gamma - SquareRoot(gamma*gamma - 1);
        let alpha = gamma - (gamma * gamma - 1.0).sqrt();

        Self {
            alpha,
            beta,
            fraction,
            price_prev1: 0.0,
            price_prev2: 0.0,
            bp_history: [0.0; 2],
            mean_sma: SMA::new(2 * period),
            peak_sma: SMA::new(50),
            valley_sma: SMA::new(50),
            peak: 0.0,
            valley: 0.0,
            count: 0,
        }
    }
}

impl Next<f64> for EMD {
    type Output = (f64, f64, f64); // (Trend/Mean, Upper, Lower)

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;

        // BP = .5*(1 - alpha)*(Price - Price[2]) + beta*(1 + alpha)*BP[1] - alpha*BP[2];
        let bp = 0.5 * (1.0 - self.alpha) * (input - self.price_prev2)
            + self.beta * (1.0 + self.alpha) * self.bp_history[0]
            - self.alpha * self.bp_history[1];

        // Mean = Average(BP, 2*Period);
        let mean = self.mean_sma.next(bp);

        // Peak/Valley logic
        // If BP[1] > BP and BP[1] > BP[2] Then Peak = BP[1];
        if self.count > 2 {
            if self.bp_history[0] > bp && self.bp_history[0] > self.bp_history[1] {
                self.peak = self.bp_history[0];
            }
            if self.bp_history[0] < bp && self.bp_history[0] < self.bp_history[1] {
                self.valley = self.bp_history[0];
            }
        }

        // AvgPeak = Average(Peak, 50);
        let avg_peak = self.peak_sma.next(self.peak);
        let avg_valley = self.valley_sma.next(self.valley);

        // Shift history
        self.bp_history[1] = self.bp_history[0];
        self.bp_history[0] = bp;

        self.price_prev2 = self.price_prev1;
        self.price_prev1 = input;

        (mean, self.fraction * avg_peak, self.fraction * avg_valley)
    }
}

pub const EMD_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "EMD",
    description: "Empirical Mode Decomposition separates cycles from trends using bandpass filtering and identifies market modes via adaptive thresholds.",
    usage: "Use to decompose price into Intrinsic Mode Functions to separate cycles of different periods without any a priori period assumption. Useful for multi-timescale analysis.",
    keywords: &["decomposition", "cycle", "spectral", "dsp"],
    ehlers_summary: "Empirical Mode Decomposition is a data-driven method developed by Huang et al. (1998) that decomposes a signal into Intrinsic Mode Functions by iteratively sifting local extrema. Unlike Fourier methods, it requires no predetermined basis functions, making it adaptive to non-stationary market data.",
    params: &[
        ParamDef {
            name: "period",
            default: "20",
            description: "Bandpass center period",
        },
        ParamDef {
            name: "delta",
            default: "0.5",
            description: "Bandwidth half-width",
        },
        ParamDef {
            name: "fraction",
            default: "0.1",
            description: "Threshold multiplier for peaks/valleys",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EmpiricalModeDecomposition.pdf",
    formula_latex: r#"
\[
\beta = \cos\left(\frac{360}{P}\right), \gamma = \frac{1}{\cos\left(\frac{720\delta}{P}\right)}, \alpha = \gamma - \sqrt{\gamma^2 - 1}
\]
\[
BP = 0.5(1 - \alpha)(Price - Price_{t-2}) + \beta(1 + \alpha)BP_{t-1} - \alpha BP_{t-2}
\]
\[
Mean = \text{SMA}(BP, 2P)
\]
\[
Threshold = \text{Fraction} \cdot \text{SMA}(\text{Peak/Valley}, 50)
\]
"#,
    gold_standard_file: "emd.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_emd_basic() {
        let mut emd = EMD::new(20, 0.5, 0.1);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let (m, u, l) = emd.next(input);
            assert!(!m.is_nan());
            assert!(!u.is_nan());
            assert!(!l.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_emd_parity(
            inputs in prop::collection::vec(1.0..100.0, 150..250),
        ) {
            let period = 20;
            let delta = 0.5;
            let fraction = 0.1;
            let mut emd = EMD::new(period, delta, fraction);
            let streaming_results: Vec<(f64, f64, f64)> = inputs.iter().map(|&x| emd.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let beta = (2.0 * PI / period as f64).cos();
            let gamma = 1.0 / (4.0 * PI * delta / period as f64).cos();
            let alpha = gamma - (gamma * gamma - 1.0).sqrt();

            let mut price_hist = vec![0.0; inputs.len() + 4];
            let mut bp_hist = vec![0.0; inputs.len() + 4];
            let mut peak = 0.0;
            let mut valley = 0.0;
            let mut peak_hist = Vec::new();
            let mut valley_hist = Vec::new();

            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                let idx = i + 2;
                price_hist[idx] = input;

                let bp = 0.5 * (1.0 - alpha) * (price_hist[idx] - price_hist[idx-2])
                    + beta * (1.0 + alpha) * bp_hist[idx-1]
                    - alpha * bp_hist[idx-2];
                bp_hist[idx] = bp;

                let mut mean_sum = 0.0;
                let mean_len = (2 * period).min(bar);
                for j in 0..mean_len {
                    mean_sum += bp_hist[idx-j];
                }
                let mean = mean_sum / mean_len as f64;

                if bar > 2 {
                    if bp_hist[idx-1] > bp && bp_hist[idx-1] > bp_hist[idx-2] {
                        peak = bp_hist[idx-1];
                    }
                    if bp_hist[idx-1] < bp && bp_hist[idx-1] < bp_hist[idx-2] {
                        valley = bp_hist[idx-1];
                    }
                }
                peak_hist.push(peak);
                valley_hist.push(valley);

                let mut p_sum = 0.0;
                let p_len = 50.min(bar);
                for j in 0..p_len {
                    p_sum += peak_hist[i-j];
                }
                let avg_p = p_sum / p_len as f64;

                let mut v_sum = 0.0;
                for j in 0..p_len {
                    v_sum += valley_hist[i-j];
                }
                let avg_v = v_sum / p_len as f64;

                batch_results.push((mean, fraction * avg_p, fraction * avg_v));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
                approx::assert_relative_eq!(s.2, b.2, epsilon = 1e-10);
            }
        }
    }
}
