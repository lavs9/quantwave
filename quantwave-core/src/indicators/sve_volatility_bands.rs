use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::{SMA, WMA};
use crate::traits::Next;

pub const METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "SVE Volatility Bands",
    description: "Volatility bands designed to highlight volatility changes especially when using non-time-related charts like Renko.",
    usage: "Use to identify extreme price excursions and volatility contraction/expansion. The bands adapt to volatility using a smoothed ATR-like calculation.",
    keywords: &["bands", "volatility", "renko", "vervoort"],
    ehlers_summary: "Introduced by Sylvain Vervoort, SVE Volatility Bands use a weighted moving average of price and a smoothed True Range to create dynamic bands. It includes a specific adjustment for the lower band and a midline based on typical price.",
    params: &[
        ParamDef {
            name: "bands_period",
            default: "20",
            description: "Period for the price WMA and the ATR smoothing basis.",
        },
        ParamDef {
            name: "bands_deviation",
            default: "2.4",
            description: "Multiplier for the volatility range.",
        },
        ParamDef {
            name: "low_band_adjust",
            default: "0.9",
            description: "Adjustment factor for the lower band.",
        },
        ParamDef {
            name: "mid_line_length",
            default: "20",
            description: "Period for the midline WMA.",
        },
    ],
    formula_source: "Technical Analysis of Stocks & Commodities, January 2019",
    formula_latex: r#"
\[
ATR\_MA = SMA(TrueRange, bands\_period \times 2 - 1) \\
WtdAvgVal = WMA(Close, bands\_period) \\
Upper = WtdAvgVal \times (1 + (ATR\_MA \times bands\_deviation) / Close) \\
Lower = WtdAvgVal \times (1 - (ATR\_MA \times bands\_deviation \times low\_band\_adjust) / Close) \\
MidLine = WMA(TypicalPrice, mid\_line\_length)
\]
"#,
    gold_standard_file: "sve_volatility_bands_20_2.4_0.9_20.json",
    category: "Classic",
};

/// SVE Volatility Bands
#[derive(Debug, Clone)]
pub struct SVEVolatilityBands {
    price_wma: WMA,
    tr_sma: SMA,
    mid_line_wma: WMA,
    bands_deviation: f64,
    low_band_adjust: f64,
    prev_close: Option<f64>,
}

impl SVEVolatilityBands {
    pub fn new(bands_period: usize, bands_deviation: f64, low_band_adjust: f64, mid_line_length: usize) -> Self {
        Self {
            price_wma: WMA::new(bands_period),
            tr_sma: SMA::new(bands_period * 2 - 1),
            mid_line_wma: WMA::new(mid_line_length),
            bands_deviation,
            low_band_adjust,
            prev_close: None,
        }
    }
}

impl Next<(f64, f64, f64)> for SVEVolatilityBands {
    type Output = (f64, f64, f64); // (Upper, Mid, Lower)

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        // True Range calculation
        let tr = match self.prev_close {
            Some(pc) => {
                let h = high.max(pc);
                let l = low.min(pc);
                h - l
            }
            None => high - low,
        };
        self.prev_close = Some(close);

        let ma_tr = self.tr_sma.next(tr);
        let wtd_avg_val = self.price_wma.next(close);
        
        let typical_price = (high + low + close) / 3.0;
        let mid_line = self.mid_line_wma.next(typical_price);

        let atr_val = ma_tr * self.bands_deviation;
        
        // From TradeStation code: 
        // HighBand = WtdAvgVal + WtdAvgVal * ( AtrVal / Price )
        // LowBand = WtdAvgVal - WtdAvgVal * ( AtrVal * LowBandAdjust / Price )
        
        let upper = wtd_avg_val + wtd_avg_val * (atr_val / close);
        let lower = wtd_avg_val - wtd_avg_val * (atr_val * self.low_band_adjust / close);

        (upper, mid_line, lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_sve_volatility_bands_basic() {
        let mut sve = SVEVolatilityBands::new(20, 2.4, 0.9, 20);
        let (upper, mid, lower) = sve.next((105.0, 95.0, 100.0));
        
        // TR = 10, SMA(39) = 10.0
        // Price WMA(20) = 100.0
        // Typical = (105+95+100)/3 = 100.0. Mid WMA(20) = 100.0
        // ATRVal = 10 * 2.4 = 24.0
        // Upper = 100 + 100 * (24 / 100) = 124.0
        // Lower = 100 - 100 * (24 * 0.9 / 100) = 100 - 21.6 = 78.4
        
        assert_eq!(mid, 100.0);
        assert_eq!(upper, 124.0);
        assert_eq!(lower, 78.4);
    }

    fn sve_volatility_bands_batch(
        data: &[(f64, f64, f64)],
        period: usize,
        dev: f64,
        adj: f64,
        mid_len: usize,
    ) -> Vec<(f64, f64, f64)> {
        let mut sve = SVEVolatilityBands::new(period, dev, adj, mid_len);
        data.iter().map(|&x| sve.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_sve_volatility_bands_parity(input in prop::collection::vec((0.1..100.0, 0.1..100.0, 0.1..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, c) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(l_f).max(c_f);
                let low = l_f.min(h_f).min(c_f);
                adj_input.push((high, low, c_f));
            }

            let period = 20;
            let dev = 2.4;
            let adj = 0.9;
            let mid_len = 20;
            
            let mut sve = SVEVolatilityBands::new(period, dev, adj, mid_len);
            let streaming_results: Vec<(f64, f64, f64)> = adj_input.iter().map(|&x| sve.next(x)).collect();
            let batch_results = sve_volatility_bands_batch(&adj_input, period, dev, adj, mid_len);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-6);
                approx::assert_relative_eq!(s.2, b.2, epsilon = 1e-6);
            }
        }
    }
}
