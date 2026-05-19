use crate::indicators::metadata::IndicatorMetadata;
#[allow(unused_imports)]
use crate::traits::Next;

talib_1_in_1_out!(HT_DCPERIOD, talib_rs::cycle::ht_dcperiod);
impl Default for HT_DCPERIOD {
    fn default() -> Self {
        Self::new()
    }
}
talib_1_in_2_out!(HT_PHASOR, talib_rs::cycle::ht_phasor);
impl Default for HT_PHASOR {
    fn default() -> Self {
        Self::new()
    }
}
talib_1_in_1_out!(HT_DCPHASE, talib_rs::cycle::ht_dcphase);
impl Default for HT_DCPHASE {
    fn default() -> Self {
        Self::new()
    }
}
talib_1_in_2_out!(HT_SINE, talib_rs::cycle::ht_sine);
impl Default for HT_SINE {
    fn default() -> Self {
        Self::new()
    }
}
talib_1_in_1_out_i32!(HT_TRENDMODE, talib_rs::cycle::ht_trendmode);
impl Default for HT_TRENDMODE {
    fn default() -> Self {
        Self::new()
    }
}

pub const HT_DCPERIOD_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Hilbert Transform - Dominant Cycle Period (HT_DCPERIOD)",
    description: "Identifies the period of the dominant cycle in the price data using the Hilbert Transform.",
    usage: "Use to dynamically adjust the lookback periods of other indicators (e.g., adaptive moving averages). Knowing the current dominant cycle length allows for more accurate smoothing and trend detection.",
    keywords: &["cycle", "hilbert", "adaptive", "dsp"],
    ehlers_summary: "John Ehlers popularized the use of the Hilbert Transform to identify the dominant cycle in financial time series. The DCPERIOD indicator tracks the length of this cycle in bars, providing a crucial parameter for creating market-responsive technical indicators that adapt to changing volatility. — Rocket Science for Traders",
    params: &[],
    formula_source: "https://www.tradingview.com/support/solutions/43000502011-hilbert-transform-dominant-cycle-period-ht-dcperiod/",
    formula_latex: r#"
\[
\text{DCPERIOD}_t = \text{Recalculated Dominant Cycle using Hilbert Transform}
\]
"#,
    gold_standard_file: "ht_dcperiod.json",
    category: "Ehlers DSP",
};

pub const HT_DCPHASE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Hilbert Transform - Dominant Cycle Phase (HT_DCPHASE)",
    description: "Calculates the phase angle (0 to 360 degrees) of the dominant cycle identified by the Hilbert Transform.",
    usage: "Use to identify the current position within a market cycle. It is the core component for generating the Hilbert Sine Wave indicator, which signals trend vs. cycle regimes.",
    keywords: &["cycle", "hilbert", "phase", "dsp"],
    ehlers_summary: "The Dominant Cycle Phase represents the instantaneous position within a detected cycle. By measuring the phase angle, traders can determine if the market is at a peak, trough, or mid-cycle, enabling more precise timing for entry and exit signals. — Rocket Science for Traders",
    params: &[],
    formula_source: "https://www.tradingview.com/support/solutions/43000502010-hilbert-transform-dominant-cycle-phase-ht-dcphase/",
    formula_latex: r#"
\[
Phase = \arctan\left(\frac{\text{Quadrature}}{\text{InPhase}}\right)
\]
"#,
    gold_standard_file: "ht_dcphase.json",
    category: "Ehlers DSP",
};

pub const HT_PHASOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Hilbert Transform - Phasor Components (HT_PHASOR)",
    description: "Outputs the In-Phase and Quadrature components of the signal, which are used to calculate phase and amplitude.",
    usage: "Use as building blocks for custom DSP indicators. The In-Phase component is the signal itself, while the Quadrature component is shifted by 90 degrees.",
    keywords: &["cycle", "hilbert", "phasor", "dsp"],
    ehlers_summary: "The Phasor components (In-Phase and Quadrature) are the fundamental outputs of the Hilbert Transform. They allow for the decomposition of a signal into its vector representation, which is essential for advanced cycle analysis and the creation of lag-free filters. — Rocket Science for Traders",
    params: &[],
    formula_source: "https://www.tradingview.com/support/solutions/43000502012-hilbert-transform-phasor-components-ht-phasor/",
    formula_latex: r#"
\[
\text{Result} = (InPhase, Quadrature)
\]
"#,
    gold_standard_file: "ht_phasor.json",
    category: "Ehlers DSP",
};

pub const HT_SINE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Hilbert Transform - Sine Wave (HT_SINE)",
    description: "An indicator that plots a sine wave and a lead-sine wave (shifted by 45 degrees) to identify cyclical turns.",
    usage: "Use to identify cycle turning points and trend regimes. When the two waves are separated and rhythmic, the market is in a cycle; when they are compressed or crossover erratically, the market is in a trend.",
    keywords: &["cycle", "hilbert", "sine", "dsp"],
    ehlers_summary: "The Hilbert Sine Wave is one of John Ehlers' most famous contributions. It provides a clear visual indication of market cycles. Crossovers of the Sine and Lead-Sine waves provide high-probability entry points in ranging markets while identifying when a strong trend has taken over. — Rocket Science for Traders",
    params: &[],
    formula_source: "https://www.tradingview.com/support/solutions/43000502013-hilbert-transform-sine-wave-ht-sine/",
    formula_latex: r#"
\[
Sine = \sin(Phase) \\ LeadSine = \sin(Phase + 45^\circ)
\]
"#,
    gold_standard_file: "ht_sine.json",
    category: "Ehlers DSP",
};

pub const HT_TRENDMODE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Hilbert Transform - Trend vs. Cycle Mode (HT_TRENDMODE)",
    description: "A binary indicator that determines if the market is currently in a trending state (1) or a cyclical state (0).",
    usage: "Use as a master filter for strategy selection. Deploy trend-following tools when TRENDMODE is 1, and mean-reversion tools when TRENDMODE is 0.",
    keywords: &["cycle", "trend", "hilbert", "regime-detection", "dsp"],
    ehlers_summary: "Determining the current market regime is the 'holy grail' of technical analysis. The HT_TRENDMODE indicator uses the rate of change of the dominant cycle phase to distinguish between trending and ranging price action, allowing traders to avoid 'whipsaws' in non-conducive environments. — Rocket Science for Traders",
    params: &[],
    formula_source: "https://www.tradingview.com/support/solutions/43000502014-hilbert-transform-trend-vs-cycle-mode-ht-trendmode/",
    formula_latex: r#"
\[
\text{TRENDMODE} = \begin{cases} 1 & \text{if trend detected} \\ 0 & \text{if cycle detected} \end{cases}
\]
"#,
    gold_standard_file: "ht_trendmode.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_ht_dcperiod_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let mut ht = HT_DCPERIOD::new();
            let streaming_results: Vec<f64> = input.iter().map(|&x| ht.next(x)).collect();
            let batch_results = talib_rs::cycle::ht_dcperiod(&input).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_ht_phasor_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let mut ht = HT_PHASOR::new();
            let streaming_results: Vec<(f64, f64)> = input.iter().map(|&x| ht.next(x)).collect();
            let (b_inphase, b_quadrature) = talib_rs::cycle::ht_phasor(&input).unwrap_or_else(|_| {
                (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
            });

            for (i, (s_in, s_quad)) in streaming_results.into_iter().enumerate() {
                if s_in.is_nan() {
                    assert!(b_inphase[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_in, b_inphase[i], epsilon = 1e-6);
                }
                if s_quad.is_nan() {
                    assert!(b_quadrature[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_quad, b_quadrature[i], epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_ht_trendmode_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let mut ht = HT_TRENDMODE::new();
            let streaming_results: Vec<f64> = input.iter().map(|&x| ht.next(x)).collect();
            let batch_results = talib_rs::cycle::ht_trendmode(&input).unwrap_or_else(|_| vec![0; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                assert_eq!(*s as i32, *b);
            }
        }
    }
}
