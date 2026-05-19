use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::SMA;
use crate::indicators::ultimate_smoother::UltimateSmoother;
use crate::indicators::volatility::ATR;
use crate::traits::Next;

/// Volume Positive Negative (VPN) Indicator
///
/// Developed by Markos Katsanos, the VPN indicator attempts to identify high-volume breakouts
/// by comparing volume on "up" days versus "down" days. It is normalized to oscillate
/// between -100 and 100.
///
/// Formula:
/// TypicalPrice = (High + Low + Close) / 3
/// MF = TypicalPrice - TypicalPrice[1]
/// MC = 0.1 * ATR(Period)
/// VMP = If MF > MC then Volume else 0
/// VMN = If MF < -MC then Volume else 0
/// VP = Sum(VMP, Period)
/// VN = Sum(VMN, Period)
/// MAV = Average(Volume, Period)
/// VPN = (VP - VN) / (MAV * Period) * 100
///
/// This implementation uses an UltimateSmoother for the final VPN value to minimize lag,
/// as per QuantWave's preference for Ehlers-style smoothing.
#[derive(Debug, Clone)]
pub struct VPNIndicator {
    _period: usize,
    atr: ATR,
    vp_sma: SMA,
    vn_sma: SMA,
    vol_sma: SMA,
    smoother: UltimateSmoother,
    prev_tp: Option<f64>,
}

impl VPNIndicator {
    pub fn new(period: usize, smooth_period: usize) -> Self {
        Self {
            _period: period,
            atr: ATR::new(period),
            vp_sma: SMA::new(period),
            vn_sma: SMA::new(period),
            vol_sma: SMA::new(period),
            smoother: UltimateSmoother::new(smooth_period),
            prev_tp: None,
        }
    }
}

impl Next<(f64, f64, f64, f64)> for VPNIndicator {
    type Output = f64;

    fn next(&mut self, (high, low, close, volume): (f64, f64, f64, f64)) -> Self::Output {
        let tp = (high + low + close) / 3.0;
        let atr = self.atr.next((high, low, close));
        let mc = 0.1 * atr;

        let (vmp, vmn) = match self.prev_tp {
            Some(ptp) => {
                let mf = tp - ptp;
                if mf > mc {
                    (volume, 0.0)
                } else if mf < -mc {
                    (0.0, volume)
                } else {
                    (0.0, 0.0)
                }
            }
            None => (0.0, 0.0),
        };

        self.prev_tp = Some(tp);

        let vp_avg = self.vp_sma.next(vmp);
        let vn_avg = self.vn_sma.next(vmn);
        let vol_avg = self.vol_sma.next(volume);

        let mav = if vol_avg <= 0.0 { 1.0 } else { vol_avg };

        // VPN = (VP - VN) / (MAV * Period) * 100
        // (vp_avg * period - vn_avg * period) / (mav * period) * 100
        // = (vp_avg - vn_avg) / mav * 100
        let vpn = (vp_avg - vn_avg) / mav * 100.0;

        self.smoother.next(vpn)
    }
}

pub const VPN_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Volume Positive Negative",
    description: "Detects high-volume breakouts by comparing volume on up days vs down days, normalized between -100 and 100.",
    usage: "Use to confirm breakouts. A VPN value crossing above a critical threshold (e.g., 10) signals a high-volume positive breakout.",
    keywords: &["volume", "breakout", "katsanos", "vpn", "momentum"],
    ehlers_summary: "While originally using EMA for smoothing, this implementation employs the UltimateSmoother to further reduce lag in detecting volume-driven trend shifts, aligning with modern DSP standards for technical indicators.",
    params: &[
        ParamDef {
            name: "period",
            default: "30",
            description: "Calculation period for volume sums and ATR",
        },
        ParamDef {
            name: "smooth_period",
            default: "3",
            description: "Smoothing period for the final VPN value",
        },
    ],
    formula_source: "https://www.traders.com/Documentation/FEEDbk_docs/2021/04/TradersTips.html",
    formula_latex: r#"
\[
TP = \frac{High + Low + Close}{3}
\]
\[
MF = TP - TP_{t-1}
\]
\[
MC = 0.1 \times ATR(Period)
\]
\[
VP = \sum_{i=0}^{Period-1} (\text{if } MF_{t-i} > MC_{t-i} \text{ then } Volume_{t-i} \text{ else } 0)
\]
\[
VN = \sum_{i=0}^{Period-1} (\text{if } MF_{t-i} < -MC_{t-i} \text{ then } Volume_{t-i} \text{ else } 0)
\]
\[
MAV = \text{Average}(Volume, Period)
\]
\[
VPN = \frac{VP - VN}{MAV \times Period} \times 100
\]
"#,
    gold_standard_file: "vpn.json",
    category: "Volume",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_vpn_basic() {
        let mut vpn = VPNIndicator::new(30, 3);
        // Provide some increasing price and volume data
        for i in 0..40 {
            let val = 100.0 + i as f64;
            let res = vpn.next((val + 1.0, val - 1.0, val, 1000.0));
            if i > 35 {
                assert!(res > 0.0);
            }
        }
    }

    proptest! {
        #[test]
        fn test_vpn_parity(
            inputs in prop::collection::vec((10.0..20.0, 5.0..10.0, 7.0..15.0, 1000.0..5000.0), 50..100),
        ) {
            let period = 30;
            let smooth_period = 3;
            let mut vpn_ind = VPNIndicator::new(period, smooth_period);

            let mut streaming_results = Vec::with_capacity(inputs.len());
            for &val in &inputs {
                streaming_results.push(vpn_ind.next(val));
            }

            // Reference implementation
            let mut atr = ATR::new(period);
            let mut vp_sma = SMA::new(period);
            let mut vn_sma = SMA::new(period);
            let mut vol_sma = SMA::new(period);
            let mut smoother = UltimateSmoother::new(smooth_period);
            let mut prev_tp = None;
            let mut batch_results = Vec::with_capacity(inputs.len());

            for &(h, l, c, v) in &inputs {
                let tp = (h + l + c) / 3.0;
                let cur_atr = atr.next((h, l, c));
                let mc = 0.1 * cur_atr;

                let (vmp, vmn) = match prev_tp {
                    Some(ptp) => {
                        let mf = tp - ptp;
                        if mf > mc {
                            (v, 0.0)
                        } else if mf < -mc {
                            (0.0, v)
                        } else {
                            (0.0, 0.0)
                        }
                    }
                    None => (0.0, 0.0),
                };
                prev_tp = Some(tp);

                let vp_avg = vp_sma.next(vmp);
                let vn_avg = vn_sma.next(vmn);
                let vol_avg = vol_sma.next(v);

                let mav = if vol_avg <= 0.0 { 1.0 } else { vol_avg };
                let vpn = (vp_avg - vn_avg) / mav * 100.0;
                batch_results.push(smoother.next(vpn));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
