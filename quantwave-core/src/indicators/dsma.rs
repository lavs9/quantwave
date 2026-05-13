use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// Deviation Scaled Moving Average (DSMA)
///
/// Based on John Ehlers' "Deviation Scaled Moving Average" (2018).
/// DSMA is an adaptive moving average that modifies the alpha term of an EMA
/// based on the amplitude of an oscillator scaled in Standard Deviations from the mean.
/// This allows it to adapt rapidly to price variations while maintaining heavy smoothing
/// when variations are small.
#[derive(Debug, Clone)]
pub struct DSMA {
    period: usize,
    c1: f64,
    c2: f64,
    c3: f64,
    price_history: VecDeque<f64>,
    zeros_history: [f64; 2],
    filt_history: [f64; 2],
    filt_window: VecDeque<f64>,
    dsma_prev: f64,
    count: usize,
}

impl DSMA {
    pub fn new(period: usize) -> Self {
        let period_f = period as f64;
        let a1 = (-1.414 * PI / (0.5 * period_f)).exp();
        let c2 = 2.0 * a1 * (1.414 * PI / (0.5 * period_f)).cos();
        let c3 = -a1 * a1;
        let c1 = 1.0 - c2 - c3;

        Self {
            period,
            c1,
            c2,
            c3,
            price_history: VecDeque::from(vec![0.0; 4]),
            zeros_history: [0.0; 2],
            filt_history: [0.0; 2],
            filt_window: VecDeque::from(vec![0.0; period]),
            dsma_prev: 0.0,
            count: 0,
        }
    }
}

impl Next<f64> for DSMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;

        // price_history[0] is Close, [1] is Close[1], [2] is Close[2], [3] is Close[3]
        self.price_history.push_front(input);
        self.price_history.pop_back();

        if self.count == 1 {
            self.dsma_prev = input;
            return input;
        }

        // Zeros = Close - Close[2];
        let zeros = self.price_history[0] - self.price_history[2];

        // Filt = c1*(Zeros + Zeros[1]) / 2 + c2*Filt[1] + c3*Filt[2];
        let filt = self.c1 * (zeros + self.zeros_history[0]) / 2.0
            + self.c2 * self.filt_history[0]
            + self.c3 * self.filt_history[1];

        self.zeros_history[1] = self.zeros_history[0];
        self.zeros_history[0] = zeros;

        self.filt_history[1] = self.filt_history[0];
        self.filt_history[0] = filt;

        self.filt_window.push_front(filt);
        self.filt_window.pop_back();

        // Compute RMS (Standard Deviation from zero mean) over last Period bars
        // The EL code uses a loop: For count = 0 to Period - 1 Begin RMS = RMS + Filt[count]*Filt[count]; End;
        let mut sum_sq = 0.0;
        for &f in &self.filt_window {
            sum_sq += f * f;
        }
        let rms = (sum_sq / self.period as f64).sqrt();

        // Rescale Filt in terms of Standard Deviations
        let scaled_filt = if rms != 0.0 { filt / rms } else { 0.0 };

        // alpha1 = AbsValue(ScaledFilt)*5 / Period;
        let mut alpha1 = scaled_filt.abs() * 5.0 / self.period as f64;
        if alpha1 > 1.0 {
            alpha1 = 1.0;
        }

        // DSMA = alpha1*Close + (1 - alpha1)*DSMA[1];
        let dsma = alpha1 * input + (1.0 - alpha1) * self.dsma_prev;
        self.dsma_prev = dsma;

        dsma
    }
}

pub const DSMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "DSMA",
    description: "Deviation Scaled Moving Average adapts to price variations using standard deviation scaled oscillators.",
    params: &[ParamDef {
        name: "period",
        default: "40",
        description: "Critical period for smoothing and RMS calculation",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/DEVIATION%20SCALED%20MOVING%20AVERAGE.pdf",
    formula_latex: r#"
\[
Zeros = Close - Close_{t-2}
\]
\[
Filt = c_1 \frac{Zeros + Zeros_{t-1}}{2} + c_2 Filt_{t-1} + c_3 Filt_{t-2}
\]
\[
RMS = \sqrt{\frac{1}{P} \sum_{i=0}^{P-1} Filt_{t-i}^2}
\]
\[
\alpha = \min\left(1.0, \left| \frac{Filt}{RMS} \right| \frac{5}{P}\right)
\]
\[
DSMA = \alpha \cdot Close + (1 - \alpha) \cdot DSMA_{t-1}
\]
"#,
    gold_standard_file: "dsma.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_dsma_basic() {
        let mut dsma = DSMA::new(40);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = dsma.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_dsma_parity(
            inputs in prop::collection::vec(1.0..100.0, 100..200),
        ) {
            let period = 40;
            let mut dsma = DSMA::new(period);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| dsma.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let period_f = period as f64;
            let a1 = (-1.414 * PI / (0.5 * period_f)).exp();
            let c2 = 2.0 * a1 * (1.414 * PI / (0.5 * period_f)).cos();
            let c3 = -a1 * a1;
            let c1 = 1.0 - c2 - c3;

            let mut price_hist = vec![0.0; inputs.len() + 4];
            let mut zeros_hist = vec![0.0; inputs.len() + 4];
            let mut filt_hist = vec![0.0; inputs.len() + 4];
            let mut dsma_prev = 0.0;

            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                let idx = i + 2; // Offset for historical access
                price_hist[idx] = input;

                if bar == 1 {
                    dsma_prev = input;
                    batch_results.push(input);
                    continue;
                }

                let zeros = price_hist[idx] - price_hist[idx-2];
                zeros_hist[idx] = zeros;

                let filt = c1 * (zeros + zeros_hist[idx-1]) / 2.0
                    + c2 * filt_hist[idx-1]
                    + c3 * filt_hist[idx-2];
                filt_hist[idx] = filt;

                let mut sum_sq = 0.0;
                for j in 0..period {
                    if idx >= j {
                        let f = filt_hist[idx-j];
                        sum_sq += f * f;
                    }
                }
                let rms = (sum_sq / period_f).sqrt();

                let scaled_filt = if rms != 0.0 { filt / rms } else { 0.0 };
                let mut alpha1 = scaled_filt.abs() * 5.0 / period_f;
                if alpha1 > 1.0 { alpha1 = 1.0; }

                let dsma = alpha1 * input + (1.0 - alpha1) * dsma_prev;
                dsma_prev = dsma;
                batch_results.push(dsma);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
