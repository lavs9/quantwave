use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;

/// Swiss Army Knife Mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwissArmyKnifeMode {
    EMA,
    SMA,
    Gauss,
    Butterworth,
    Smooth,
    HighPass,
    TwoPoleHighPass,
    BandPass,
    BandStop,
}

/// Swiss Army Knife Indicator
///
/// Based on John Ehlers' "Swiss Army Knife Indicator".
/// A versatile general-purpose filter that can be configured as various
/// low-pass, high-pass, band-pass, and band-stop filters.
#[derive(Debug, Clone)]
pub struct SwissArmyKnife {
    mode: SwissArmyKnifeMode,
    period: usize,
    delta: f64,
    c0: f64,
    c1: f64,
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    x: [f64; 3],         // x[t], x[t-1], x[t-2]
    f: [f64; 2],         // f[t-1], f[t-2]
    history_x: Vec<f64>, // For SMA mode only (c1 * Price[N])
    count: usize,
}

impl SwissArmyKnife {
    pub fn new(mode: SwissArmyKnifeMode, period: usize, delta: f64) -> Self {
        let mut sak = Self {
            mode,
            period,
            delta,
            c0: 1.0,
            c1: 0.0,
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x: [0.0; 3],
            f: [0.0; 2],
            history_x: Vec::new(),
            count: 0,
        };
        sak.initialize();
        sak
    }

    fn initialize(&mut self) {
        let period_f = self.period as f64;
        let angle = 2.0 * std::f64::consts::PI / period_f;

        match self.mode {
            SwissArmyKnifeMode::EMA => {
                let alpha = (angle.cos() + angle.sin() - 1.0) / angle.cos();
                self.b0 = alpha;
                self.a1 = 1.0 - alpha;
            }
            SwissArmyKnifeMode::SMA => {
                self.c1 = 1.0 / period_f;
                self.b0 = 1.0 / period_f;
                self.a1 = 1.0;
            }
            SwissArmyKnifeMode::Gauss => {
                let beta = 2.415 * (1.0 - angle.cos());
                let alpha = -beta + (beta * beta + 2.0 * beta).sqrt();
                self.c0 = alpha * alpha;
                self.a1 = 2.0 * (1.0 - alpha);
                self.a2 = -(1.0 - alpha) * (1.0 - alpha);
            }
            SwissArmyKnifeMode::Butterworth => {
                let beta = 2.415 * (1.0 - angle.cos());
                let alpha = -beta + (beta * beta + 2.0 * beta).sqrt();
                self.c0 = alpha * alpha / 4.0;
                self.b1 = 2.0;
                self.b2 = 1.0;
                self.a1 = 2.0 * (1.0 - alpha);
                self.a2 = -(1.0 - alpha) * (1.0 - alpha);
            }
            SwissArmyKnifeMode::Smooth => {
                self.c0 = 0.25;
                self.b1 = 2.0;
                self.b2 = 1.0;
            }
            SwissArmyKnifeMode::HighPass => {
                let alpha = (angle.cos() + angle.sin() - 1.0) / angle.cos();
                self.c0 = 1.0 - alpha / 2.0;
                self.b1 = -1.0;
                self.a1 = 1.0 - alpha;
            }
            SwissArmyKnifeMode::TwoPoleHighPass => {
                let beta = 2.415 * (1.0 - angle.cos());
                let alpha = -beta + (beta * beta + 2.0 * beta).sqrt();
                self.c0 = (1.0 - alpha / 2.0) * (1.0 - alpha / 2.0);
                self.b1 = -2.0;
                self.b2 = 1.0;
                self.a1 = 2.0 * (1.0 - alpha);
                self.a2 = -(1.0 - alpha) * (1.0 - alpha);
            }
            SwissArmyKnifeMode::BandPass => {
                let beta = angle.cos();
                let gamma = 1.0 / (4.0 * std::f64::consts::PI * self.delta / period_f).cos();
                let alpha = gamma - (gamma * gamma - 1.0).sqrt();
                self.c0 = (1.0 - alpha) / 2.0;
                self.b2 = -1.0;
                self.a1 = beta * (1.0 + alpha);
                self.a2 = -alpha;
            }
            SwissArmyKnifeMode::BandStop => {
                let beta = angle.cos();
                let gamma = 1.0 / (4.0 * std::f64::consts::PI * self.delta / period_f).cos();
                let alpha = gamma - (gamma * gamma - 1.0).sqrt();
                self.c0 = (1.0 + alpha) / 2.0;
                self.b1 = -2.0 * beta;
                self.b2 = 1.0;
                self.a1 = beta * (1.0 + alpha);
                self.a2 = -alpha;
            }
        }
    }
}

impl Next<f64> for SwissArmyKnife {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        self.x[2] = self.x[1];
        self.x[1] = self.x[0];
        self.x[0] = input;

        if self.mode == SwissArmyKnifeMode::SMA {
            self.history_x.push(input);
        }

        let filt = if self.count <= self.period {
            match self.mode {
                SwissArmyKnifeMode::HighPass | SwissArmyKnifeMode::TwoPoleHighPass => 0.0,
                _ => input,
            }
        } else {
            let x_n = if self.mode == SwissArmyKnifeMode::SMA {
                self.history_x[self.count - 1 - self.period]
            } else {
                0.0
            };

            self.c0 * (self.b0 * self.x[0] + self.b1 * self.x[1] + self.b2 * self.x[2])
                + self.a1 * self.f[0]
                + self.a2 * self.f[1]
                - self.c1 * x_n
        };

        self.f[1] = self.f[0];
        self.f[0] = filt;

        filt
    }
}

pub const SWISS_ARMY_KNIFE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Swiss Army Knife Indicator",
    description: "A versatile indicator that can be configured as EMA, SMA, Gaussian, Butterworth, High Pass, Band Pass, or Band Stop filter.",
    params: &[
        ParamDef {
            name: "mode",
            default: "BandPass",
            description: "Filter mode (EMA, SMA, Gauss, Butter, Smooth, HP, 2PHP, BP, BS)",
        },
        ParamDef {
            name: "period",
            default: "20",
            description: "Filter period",
        },
        ParamDef {
            name: "delta",
            default: "0.1",
            description: "Bandwidth parameter for BP and BS modes",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/SwissArmyKnifeIndicator.pdf",
    formula_latex: r#"
\[
Filt = c_0(b_0 x_t + b_1 x_{t-1} + b_2 x_{t-2}) + a_1 Filt_{t-1} + a_2 Filt_{t-2} - c_1 x_{t-N}
\]
"#,
    gold_standard_file: "swiss_army_knife.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_sak_ema_basic() {
        let mut sak = SwissArmyKnife::new(SwissArmyKnifeMode::EMA, 20, 0.1);
        let inputs = vec![10.0, 11.0, 12.0, 11.0, 10.0];
        for input in inputs {
            let val = sak.next(input);
            assert!(!val.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_sak_parity(
            inputs in prop::collection::vec(1.0..100.0, 30..100),
        ) {
            let period = 20;
            let delta = 0.1;
            let mode = SwissArmyKnifeMode::Gauss;
            let mut sak = SwissArmyKnife::new(mode, period, delta);

            let streaming_results: Vec<f64> = inputs.iter().map(|&x| sak.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut x = [0.0; 3];
            let mut f = [0.0; 2];

            // Re-calculate coefficients for verification
            let angle = 2.0 * std::f64::consts::PI / (period as f64);
            let beta = 2.415 * (1.0 - angle.cos());
            let alpha = -beta + (beta * beta + 2.0 * beta).sqrt();
            let c0 = alpha * alpha;
            let a1 = 2.0 * (1.0 - alpha);
            let a2 = -(1.0 - alpha) * (1.0 - alpha);

            for (i, &input) in inputs.iter().enumerate() {
                x[2] = x[1];
                x[1] = x[0];
                x[0] = input;

                let filt = if i + 1 <= period {
                    input
                } else {
                    c0 * x[0] + a1 * f[0] + a2 * f[1]
                };

                f[1] = f[0];
                f[0] = filt;
                batch_results.push(filt);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
