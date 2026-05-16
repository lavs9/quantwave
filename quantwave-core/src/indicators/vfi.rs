use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::{EMA, SMA};
use crate::indicators::statistics::StandardDeviation;
use crate::traits::Next;
use std::collections::VecDeque;

pub const METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "VFI",
    description: "Volume Flow Indicator - a volume-based indicator that uses price and volume relative to a cutoff to measure money flow.",
    usage: "Used to identify trend direction and potential reversals. Values above 0 are bullish, below 0 are bearish. Extreme readings and divergences are also significant.",
    keywords: &["volume", "vfi", "money-flow", "katsanos", "oscillator"],
    ehlers_summary: "Katsanos' Volume Flow Indicator (VFI) is based on the popular On Balance Volume (OBV) but with three main modifications: it is bounded, it filters out small price changes, and it caps volume extremes. It provides a more balanced view of buying and selling pressure by accounting for price volatility and volume outliers. — TASC June 2004",
    params: &[
        ParamDef {
            name: "period",
            default: "130",
            description: "Lookback period for Vave and Summation",
        },
        ParamDef {
            name: "coef",
            default: "0.2",
            description: "Coefficient for minimal price cut-off",
        },
        ParamDef {
            name: "vcoef",
            default: "2.5",
            description: "Coefficient for volume cut-off",
        },
        ParamDef {
            name: "smoothing_period",
            default: "3",
            description: "EMA period for final smoothing",
        },
    ],
    formula_source: "https://www.traders.com/Documentation/FEEDbk_docs/2022/04/TradersTips.html",
    formula_latex: r#"
\begin{aligned}
TP &= \frac{H+L+C}{3} \\
Inter &= \ln(TP) - \ln(TP_{t-1}) \\
VInter &= StdDev(Inter, 30) \\
Cutoff &= Coef \cdot VInter \cdot Close \\
Vave &= SMA(Volume, Period)_{t-1} \\
Vmax &= Vave \cdot Vcoef \\
VC &= \min(Volume, Vmax) \\
MF &= TP - TP_{t-1} \\
VCP &= \begin{cases} VC, & \text{if } MF > Cutoff \\ -VC, & \text{if } MF < -Cutoff \\ 0, & \text{otherwise} \end{cases} \\
VFI_{raw} &= \frac{\sum_{i=0}^{Period-1} VCP_{t-i}}{Vave} \\
VFI &= EMA(VFI_{raw}, 3)
\end{aligned}
"#,
    gold_standard_file: "vfi.json",
    category: "Volume Indicators",
};

/// Volume Flow Indicator (VFI)
///
/// Based on Markos Katsanos' Volume Flow Indicator.
#[derive(Debug, Clone)]
pub struct Vfi {
    period: usize,
    coef: f64,
    vcoef: f64,
    prev_tp: Option<f64>,
    stddev: StandardDeviation,
    v_sma: SMA,
    prev_v_ave: f64,
    dir_vol_window: VecDeque<f64>,
    dir_vol_sum: f64,
    ema: EMA,
}

impl Vfi {
    pub fn new(period: usize, coef: f64, vcoef: f64, smoothing_period: usize) -> Self {
        Self {
            period,
            coef,
            vcoef,
            prev_tp: None,
            stddev: StandardDeviation::new(30),
            v_sma: SMA::new(period),
            prev_v_ave: 0.0,
            dir_vol_window: VecDeque::with_capacity(period),
            dir_vol_sum: 0.0,
            ema: EMA::new(smoothing_period),
        }
    }
}

impl Next<(f64, f64, f64, f64)> for Vfi {
    type Output = f64;

    fn next(&mut self, (high, low, close, volume): (f64, f64, f64, f64)) -> Self::Output {
        let tp = (high + low + close) / 3.0;

        let inter = match self.prev_tp {
            Some(prev) if tp > 0.0 && prev > 0.0 => tp.ln() - prev.ln(),
            _ => 0.0,
        };

        let v_inter = self.stddev.next(inter);
        let cutoff = self.coef * v_inter * close;

        // VAve = Average(V, Period)[1]
        let v_ave = if self.prev_v_ave == 0.0 {
            // On initialization, if we don't have a previous average, we use current
            // This is a common way to handle the startup of lagged indicators.
            self.v_sma.next(volume)
        } else {
            self.prev_v_ave
        };
        // Update prev_v_ave for next bar
        self.prev_v_ave = self.v_sma.next(volume);

        let v_max = v_ave * self.vcoef;
        let vc = volume.min(v_max);

        let mf = match self.prev_tp {
            Some(prev) => tp - prev,
            None => 0.0,
        };

        let dir_vol = if mf > cutoff {
            vc
        } else if mf < -cutoff {
            -vc
        } else {
            0.0
        };

        // Rolling sum of DirectionalVolume
        self.dir_vol_window.push_back(dir_vol);
        self.dir_vol_sum += dir_vol;
        if self.dir_vol_window.len() > self.period {
            if let Some(oldest) = self.dir_vol_window.pop_front() {
                self.dir_vol_sum -= oldest;
            }
        }

        let vfi_raw = if v_ave != 0.0 {
            self.dir_vol_sum / v_ave
        } else {
            0.0
        };

        self.prev_tp = Some(tp);

        self.ema.next(vfi_raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_vfi_basic() {
        let mut vfi = Vfi::new(130, 0.2, 2.5, 3);
        let inputs = vec![
            (10.0, 10.0, 10.0, 1000.0),
            (11.0, 11.0, 11.0, 1000.0),
            (12.0, 12.0, 12.0, 1000.0),
            (11.0, 11.0, 11.0, 1000.0),
        ];

        for input in inputs {
            let res = vfi.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_vfi_parity(
            inputs in prop::collection::vec((1.0..100.0, 1.0..100.0, 1.0..100.0, 1.0..1000.0), 10..100),
        ) {
            let period = 130;
            let coef = 0.2;
            let vcoef = 2.5;
            let smoothing = 3;
            let mut vfi = Vfi::new(period, coef, vcoef, smoothing);
            
            // Manual calculation for parity check
            let mut prev_tp: Option<f64> = None;
            let mut stddev = StandardDeviation::new(30);
            let mut v_sma = SMA::new(period);
            let mut prev_v_ave = 0.0;
            let mut dir_vol_window = VecDeque::new();
            let mut dir_vol_sum = 0.0;
            let mut ema = EMA::new(smoothing);
            
            for (h, l, c, v) in inputs {
                let h: f64 = h;
                let l: f64 = l;
                let c: f64 = c;
                let high = h.max(l).max(c);
                let low = h.min(l).min(c);
                let tp = (high + low + c) / 3.0;
                
                let inter = match prev_tp {
                    Some(p) if tp > 0.0 && p > 0.0 => tp.ln() - p.ln(),
                    _ => 0.0,
                };
                let v_inter = stddev.next(inter);
                let cutoff = coef * v_inter * c;
                
                let v_ave = if prev_v_ave == 0.0 {
                    v_sma.next(v)
                } else {
                    prev_v_ave
                };
                prev_v_ave = v_sma.next(v);
                
                let v_max = v_ave * vcoef;
                let vc = v.min(v_max);
                
                let mf = match prev_tp {
                    Some(p) => tp - p,
                    None => 0.0,
                };
                
                let dir_vol = if mf > cutoff {
                    vc
                } else if mf < -cutoff {
                    -vc
                } else {
                    0.0
                };
                
                dir_vol_window.push_back(dir_vol);
                dir_vol_sum += dir_vol;
                if dir_vol_window.len() > period {
                    dir_vol_sum -= dir_vol_window.pop_front().unwrap();
                }
                
                let vfi_raw = if v_ave != 0.0 {
                    dir_vol_sum / v_ave
                } else {
                    0.0
                };
                
                let expected_vfi = ema.next(vfi_raw);
                let actual_vfi = vfi.next((high, low, c, v));
                
                approx::assert_relative_eq!(actual_vfi, expected_vfi, epsilon = 1e-10);
                prev_tp = Some(tp);
            }
        }
    }
}
