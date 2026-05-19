use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::SMA;
use crate::traits::Next;

/// Harrington ADX Oscillator
///
/// Based on Neil Jon Harrington's article "Revisualizing The ADX Oscillator" (TASC December 2024).
/// This indicator revisualizes the standard ADX by giving it a sign based on the DMI direction.
/// It uses a histogram-like display where the sign depends on (Smoothed DMI+ - Smoothed DMI-).
#[derive(Debug, Clone)]
pub struct HarringtonADXOscillator {
    _adx_length: usize,
    _adx_smooth_length: usize,
    sf: f64,
    tr_ema: f64,
    pdm_ema: f64,
    mdm_ema: f64,
    adx_ema: f64,
    p_sma: SMA,
    m_sma: SMA,
    prev_h: f64,
    prev_l: f64,
    prev_c: f64,
    count: usize,
}

impl HarringtonADXOscillator {
    pub fn new(adx_length: usize, adx_smooth_length: usize) -> Self {
        Self {
            _adx_length: adx_length,
            _adx_smooth_length: adx_smooth_length,
            sf: 1.0 / (adx_length as f64),
            tr_ema: 0.0,
            pdm_ema: 0.0,
            mdm_ema: 0.0,
            adx_ema: 0.0,
            p_sma: SMA::new(adx_smooth_length),
            m_sma: SMA::new(adx_smooth_length),
            prev_h: 0.0,
            prev_l: 0.0,
            prev_c: 0.0,
            count: 0,
        }
    }
}

impl Next<(f64, f64, f64)> for HarringtonADXOscillator {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        self.count += 1;

        if self.count == 1 {
            self.prev_h = high;
            self.prev_l = low;
            self.prev_c = close;
            self.tr_ema = high - low;
            return 0.0;
        }

        let tr = (high - low)
            .max((high - self.prev_c).abs())
            .max((low - self.prev_c).abs());
        let up_move = high - self.prev_h;
        let down_move = self.prev_l - low;

        let (pdm, mdm) = if up_move > down_move && up_move > 0.0 {
            (up_move, 0.0)
        } else if down_move > up_move && down_move > 0.0 {
            (0.0, down_move)
        } else {
            (0.0, 0.0)
        };

        // Wilder's Smoothing (EMA with alpha = 1/length)
        self.tr_ema = self.sf * tr + (1.0 - self.sf) * self.tr_ema;
        self.pdm_ema = self.sf * pdm + (1.0 - self.sf) * self.pdm_ema;
        self.mdm_ema = self.sf * mdm + (1.0 - self.sf) * self.mdm_ema;

        let (pdi, mdi) = if self.tr_ema > 0.0 {
            (
                100.0 * self.pdm_ema / self.tr_ema,
                100.0 * self.mdm_ema / self.tr_ema,
            )
        } else {
            (0.0, 0.0)
        };

        let dx = if (pdi + mdi) > 0.0 {
            100.0 * (pdi - mdi).abs() / (pdi + mdi)
        } else {
            0.0
        };

        self.adx_ema = self.sf * dx + (1.0 - self.sf) * self.adx_ema;

        let smoothed_plus = self.p_sma.next(pdi);
        let smoothed_minus = self.m_sma.next(mdi);
        let net_dmi = smoothed_plus - smoothed_minus;

        self.prev_h = high;
        self.prev_l = low;
        self.prev_c = close;

        if net_dmi >= 0.0 {
            self.adx_ema
        } else {
            -self.adx_ema
        }
    }
}

pub const HARRINGTON_ADX_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Harrington ADX Oscillator",
    description: "An oscillator variant of the ADX where the sign reflects trend direction determined by DMI+ and DMI-.",
    usage: "The oscillator is positive when DMI+ > DMI- and negative when DMI- > DMI+. The magnitude represents trend strength (ADX). Thresholds at 15 and 40 are often used to identify trend initiation and overextended states.",
    keywords: &["adx", "dmi", "oscillator", "wilder", "momentum"],
    ehlers_summary: "While originally created by Wilder, this revisualization by Harrington transforms the unipolar ADX into a bipolar oscillator. This allows for simultaneous identification of trend strength and direction in a single histogram display, simplifying the interpretation of complex directional movement data.",
    params: &[
        ParamDef {
            name: "adx_length",
            default: "10",
            description: "Wilder's ADX period",
        },
        ParamDef {
            name: "adx_smooth_length",
            default: "1",
            description: "SMA period for DMI components smoothing",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20DECEMBER%202024.html",
    formula_latex: r#"
\[
TR = \max(H-L, |H-C_{t-1}|, |L-C_{t-1}|)
\]
\[
+DM = (H-H_{t-1} > L_{t-1}-L) \text{ and } (H-H_{t-1} > 0) ? H-H_{t-1} : 0
\]
\[
-DM = (L_{t-1}-L > H-H_{t-1}) \text{ and } (L_{t-1}-L > 0) ? L_{t-1}-L : 0
\]
\[
+DI = 100 \cdot \frac{EMA(+DM, 1/L)}{EMA(TR, 1/L)}
\]
\[
-DI = 100 \cdot \frac{EMA(-DM, 1/L)}{EMA(TR, 1/L)}
\]
\[
DX = 100 \cdot \frac{|+DI - -DI|}{+DI + -DI}
\]
\[
ADX = EMA(DX, 1/L)
\]
\[
Result = (SMA(+DI, S) \ge SMA(-DI, S)) ? ADX : -ADX
\]
"#,
    gold_standard_file: "harrington_adx.json",
    category: "Wilder",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_harrington_adx_basic() {
        let mut hadx = HarringtonADXOscillator::new(10, 1);
        let inputs = vec![
            (10.0, 9.0, 9.5),
            (11.0, 10.0, 10.5),
            (12.0, 11.0, 11.5),
            (11.0, 10.0, 10.5),
            (10.0, 9.0, 9.5),
        ];
        for input in inputs {
            let res = hadx.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_harrington_adx_parity(
            highs in prop::collection::vec(100.0..110.0, 50..100),
            lows in prop::collection::vec(90.0..100.0, 50..100),
            closes in prop::collection::vec(90.0..110.0, 50..100),
        ) {
            let len = highs.len().min(lows.len()).min(closes.len());
            let mut inputs = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = highs[i];
                let l: f64 = lows[i];
                let c_val: f64 = closes[i];
                let c: f64 = c_val.max(l).min(h);
                inputs.push((h, l, c));
            }

            let adx_len = 10;
            let smooth_len = 1;
            let mut hadx = HarringtonADXOscillator::new(adx_len, smooth_len);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| hadx.next(x)).collect();

            // Reference implementation
            let mut tr_ema = 0.0;
            let mut pdm_ema = 0.0;
            let mut mdm_ema = 0.0;
            let mut adx_ema = 0.0;
            let mut p_sma = SMA::new(smooth_len);
            let mut m_sma = SMA::new(smooth_len);
            let mut prev_h = 0.0;
            let mut prev_l = 0.0;
            let mut prev_c = 0.0;
            let sf = 1.0 / adx_len as f64;
            let mut batch_results = Vec::with_capacity(len);

            for i in 0..len {
                let (h, l, c) = inputs[i];
                if i == 0 {
                    prev_h = h;
                    prev_l = l;
                    prev_c = c;
                    tr_ema = h - l;
                    batch_results.push(0.0);
                    continue;
                }

                let tr = (h - l).max((h - prev_c).abs()).max((l - prev_c).abs());
                let up = h - prev_h;
                let down = prev_l - l;
                let (pdm, mdm) = if up > down && up > 0.0 { (up, 0.0) } else if down > up && down > 0.0 { (0.0, down) } else { (0.0, 0.0) };

                tr_ema = sf * tr + (1.0 - sf) * tr_ema;
                pdm_ema = sf * pdm + (1.0 - sf) * pdm_ema;
                mdm_ema = sf * mdm + (1.0 - sf) * mdm_ema;

                let (pdi, mdi) = if tr_ema > 0.0 { (100.0 * pdm_ema / tr_ema, 100.0 * mdm_ema / tr_ema) } else { (0.0, 0.0) };
                let dx = if (pdi + mdi) > 0.0 { 100.0 * (pdi - mdi).abs() / (pdi + mdi) } else { 0.0 };
                adx_ema = sf * dx + (1.0 - sf) * adx_ema;

                let sp = p_sma.next(pdi);
                let sm = m_sma.next(mdi);
                let res = if sp >= sm { adx_ema } else { -adx_ema };
                batch_results.push(res);

                prev_h = h;
                prev_l = l;
                prev_c = c;
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
