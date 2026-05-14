use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// MESA Adaptive Moving Average (MAMA)
/// Adapts to price movement based on the rate change of phase as measured by the Hilbert Transform Discriminator.
/// Returns (MAMA, FAMA).
#[derive(Debug, Clone)]
pub struct MAMA {
    fast_limit: f64,
    slow_limit: f64,
    price_history: VecDeque<f64>,
    smooth_history: VecDeque<f64>,
    detrender_history: VecDeque<f64>,
    i1_history: VecDeque<f64>,
    q1_history: VecDeque<f64>,
    i2_prev: f64,
    q2_prev: f64,
    re_prev: f64,
    im_prev: f64,
    period_prev: f64,
    smooth_period_prev: f64,
    phase_prev: f64,
    mama_prev: f64,
    fama_prev: f64,
    count: usize,
}

impl MAMA {
    pub fn new(fast_limit: f64, slow_limit: f64) -> Self {
        Self {
            fast_limit,
            slow_limit,
            price_history: VecDeque::from(vec![0.0; 4]),
            smooth_history: VecDeque::from(vec![0.0; 7]),
            detrender_history: VecDeque::from(vec![0.0; 7]),
            i1_history: VecDeque::from(vec![0.0; 7]),
            q1_history: VecDeque::from(vec![0.0; 7]),
            i2_prev: 0.0,
            q2_prev: 0.0,
            re_prev: 0.0,
            im_prev: 0.0,
            period_prev: 0.0,
            smooth_period_prev: 0.0,
            phase_prev: 0.0,
            mama_prev: 0.0,
            fama_prev: 0.0,
            count: 0,
        }
    }
}

impl Default for MAMA {
    fn default() -> Self {
        Self::new(0.5, 0.05)
    }
}

impl Next<f64> for MAMA {
    type Output = (f64, f64);

    fn next(&mut self, price: f64) -> Self::Output {
        self.count += 1;

        self.price_history.pop_back();
        self.price_history.push_front(price);

        if self.count < 6 {
            self.mama_prev = price;
            self.fama_prev = price;
            return (price, price);
        }

        // Smooth = (4*Price + 3*Price[1] + 2*Price[2] + Price[3]) / 10;
        let smooth = (4.0 * self.price_history[0]
            + 3.0 * self.price_history[1]
            + 2.0 * self.price_history[2]
            + self.price_history[3])
            / 10.0;

        self.smooth_history.pop_back();
        self.smooth_history.push_front(smooth);

        // Detrender = (.0962*Smooth + .5769*Smooth[2] - .5769*Smooth[4] - .0962*Smooth[6])*(.075*Period[1] + .54);
        let detrender = (0.0962 * self.smooth_history[0] + 0.5769 * self.smooth_history[2]
            - 0.5769 * self.smooth_history[4]
            - 0.0962 * self.smooth_history[6])
            * (0.075 * self.period_prev + 0.54);

        self.detrender_history.pop_back();
        self.detrender_history.push_front(detrender);

        // Q1 = (.0962*Detrender + .5769*Detrender[2] - .5769*Detrender[4] - .0962*Detrender[6])*(.075*Period[1] + .54);
        let q1 = (0.0962 * self.detrender_history[0] + 0.5769 * self.detrender_history[2]
            - 0.5769 * self.detrender_history[4]
            - 0.0962 * self.detrender_history[6])
            * (0.075 * self.period_prev + 0.54);

        // I1 = Detrender[3];
        let i1 = self.detrender_history[3];

        self.i1_history.pop_back();
        self.i1_history.push_front(i1);
        self.q1_history.pop_back();
        self.q1_history.push_front(q1);

        // jI = (.0962*I1 + .5769*I1[2] - .5769*I1[4] - .0962*I1[6])*(.075*Period[1] + .54);
        let ji = (0.0962 * self.i1_history[0] + 0.5769 * self.i1_history[2]
            - 0.5769 * self.i1_history[4]
            - 0.0962 * self.i1_history[6])
            * (0.075 * self.period_prev + 0.54);

        // jQ = (.0962*Q1 + .5769*Q1[2] - .5769*Q1[4] - .0962*Q1[6])*(.075*Period[1] + .54);
        let jq = (0.0962 * self.q1_history[0] + 0.5769 * self.q1_history[2]
            - 0.5769 * self.q1_history[4]
            - 0.0962 * self.q1_history[6])
            * (0.075 * self.period_prev + 0.54);

        // I2 = I1 - jQ;
        // Q2 = Q1 + jI;
        let mut i2 = i1 - jq;
        let mut q2 = q1 + ji;

        // I2 = .2*I2 + .8*I2[1];
        // Q2 = .2*Q2 + .8*Q2[1];
        i2 = 0.2 * i2 + 0.8 * self.i2_prev;
        q2 = 0.2 * q2 + 0.8 * self.q2_prev;
        self.i2_prev = i2;
        self.q2_prev = q2;

        // Homodyne Discriminator
        // Re = I2*I2[1] + Q2*Q2[1];
        // Im = I2*Q2[1] - Q2*I2[1];
        let mut re = i2 * self.i2_prev + q2 * self.q2_prev;
        let mut im = i2 * self.q2_prev - q2 * self.i2_prev;

        // Note: The EL code uses I2[1] and Q2[1] which are the values BEFORE the current i2/q2 update.
        // Wait, in EL, Vars are updated at the end of the bar or immediately.
        // I2 = .2*I2 + .8*I2[1] update i2. Then Re = I2*I2[1].
        // This means I2[1] is indeed the PREVIOUS value.

        re = 0.2 * re + 0.8 * self.re_prev;
        im = 0.2 * im + 0.8 * self.im_prev;
        self.re_prev = re;
        self.im_prev = im;

        let mut period = self.period_prev;
        if im != 0.0 && re != 0.0 {
            period = 360.0 / (im / re).atan().to_degrees();
        }
        if period > 1.5 * self.period_prev {
            period = 1.5 * self.period_prev;
        }
        if period < 0.67 * self.period_prev {
            period = 0.67 * self.period_prev;
        }
        if period < 6.0 {
            period = 6.0;
        }
        if period > 50.0 {
            period = 50.0;
        }
        period = 0.2 * period + 0.8 * self.period_prev;
        self.period_prev = period;

        let _smooth_period = 0.33 * period + 0.67 * self.smooth_period_prev;
        self.smooth_period_prev = _smooth_period;

        let mut phase = 0.0;
        if i1 != 0.0 {
            phase = (q1 / i1).atan().to_degrees();
        }

        let mut delta_phase = self.phase_prev - phase;
        self.phase_prev = phase;

        if delta_phase < 1.0 {
            delta_phase = 1.0;
        }

        let mut alpha = self.fast_limit / delta_phase;
        if alpha < self.slow_limit {
            alpha = self.slow_limit;
        }
        if alpha > self.fast_limit {
            alpha = self.fast_limit;
        }

        let mama = alpha * price + (1.0 - alpha) * self.mama_prev;
        let fama = 0.5 * alpha * mama + (1.0 - 0.5 * alpha) * self.fama_prev;

        self.mama_prev = mama;
        self.fama_prev = fama;

        (mama, fama)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_mama_basic() {
        let mut mama = MAMA::new(0.5, 0.05);
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0];
        for p in prices {
            let (m, f) = mama.next(p);
            assert!(!m.is_nan());
            assert!(!f.is_nan());
        }
    }

    fn mama_batch(data: Vec<f64>, fast: f64, slow: f64) -> Vec<(f64, f64)> {
        let mut mama = MAMA::new(fast, slow);
        data.into_iter().map(|x| mama.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_mama_parity(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let fast = 0.5;
            let slow = 0.05;
            let mut mama = MAMA::new(fast, slow);
            let mut streaming_results = Vec::with_capacity(input.len());
            for &val in &input {
                streaming_results.push(mama.next(val));
            }

            let batch_results = mama_batch(input, fast, slow);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-6);
            }
        }
    }
}

pub const MAMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "MESA Adaptive Moving Average",
    description: "MAMA adapts to price movement in an entirely new and unique way based on the rate change of phase.",
    usage: "Use as an adaptive trend filter that automatically speeds up in fast markets and slows in choppy ones. The FAMA line crossing MAMA provides high-probability trend change signals.",
    keywords: &["moving-average", "adaptive", "ehlers", "dsp", "trend"],
    ehlers_summary: "Presented in Rocket Science for Traders (2001), MAMA adapts its alpha based on the rate of phase change measured by the Hilbert Transform Discriminator. Fast cycles produce large alpha for responsiveness; slow cycles produce small alpha to reduce noise.",
    params: &[
        ParamDef {
            name: "fast_limit",
            default: "0.5",
            description: "Fast limit for alpha",
        },
        ParamDef {
            name: "slow_limit",
            default: "0.05",
            description: "Slow limit for alpha",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/MAMA.pdf",
    formula_latex: r#"
\[
\text{MAMA} = \alpha \cdot \text{Price} + (1 - \alpha) \cdot \text{MAMA}_{1}
\]
\[
\text{FAMA} = 0.5\alpha \cdot \text{MAMA} + (1 - 0.5\alpha) \cdot \text{FAMA}_{1}
\]
"#,
    gold_standard_file: "mama.json",
    category: "Ehlers DSP",
};
