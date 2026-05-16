use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// True Range Adjusted Exponential Moving Average (TRAdj EMA)
/// TASC January 2023, by Vitali Apirine
#[derive(Debug, Clone)]
pub struct TRAdjEMA {
    _period: usize,
    pds: usize,
    mltp: f64,
    mltp1: f64,
    prev_close: Option<f64>,
    trs: VecDeque<f64>,
    prev_ema: Option<f64>,
}

impl TRAdjEMA {
    pub fn new(period: usize, pds: usize, mltp: f64) -> Self {
        Self {
            _period: period,
            pds,
            mltp,
            mltp1: 2.0 / (period as f64 + 1.0),
            prev_close: None,
            trs: VecDeque::with_capacity(pds),
            prev_ema: None,
        }
    }
}

impl Next<(f64, f64, f64)> for TRAdjEMA {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let th = match self.prev_close {
            Some(pc) => if pc > high { pc } else { high },
            None => high,
        };
        let tl = match self.prev_close {
            Some(pc) => if pc < low { pc } else { low },
            None => low,
        };
        self.prev_close = Some(close);

        let tr = (th - tl).abs();
        self.trs.push_back(tr);

        if self.trs.len() > self.pds {
            self.trs.pop_front();
        }

        let mut max_tr = f64::MIN;
        let mut min_tr = f64::MAX;
        for &t in self.trs.iter() {
            if t > max_tr {
                max_tr = t;
            }
            if t < min_tr {
                min_tr = t;
            }
        }

        let tradj = if max_tr - min_tr == 0.0 {
            0.0
        } else {
            (tr - min_tr) / (max_tr - min_tr)
        };

        let mltp2 = tradj * self.mltp;
        let rate = self.mltp1 * (1.0 + mltp2);

        let ema = match self.prev_ema {
            Some(prev) => prev + rate * (close - prev),
            None => close, // Initial value is close according to TradeStation reference
        };

        self.prev_ema = Some(ema);
        ema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn tradj_ema_batch(data: Vec<(f64, f64, f64)>, period: usize, pds: usize, mltp: f64) -> Vec<f64> {
        let mut ema = TRAdjEMA::new(period, pds, mltp);
        data.into_iter().map(|x| ema.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_tradj_ema_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, c) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(l_f).max(c_f);
                let low = l_f.min(h_f).min(c_f);
                adj_input.push((high, low, c_f));
            }

            let period = 40;
            let pds = 40;
            let mltp = 10.0;
            let mut ema = TRAdjEMA::new(period, pds, mltp);
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(ema.next(val));
            }

            let batch_results = tradj_ema_batch(adj_input, period, pds, mltp);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_tradj_ema_basic() {
        let mut ema = TRAdjEMA::new(10, 10, 5.0);
        let val1 = ema.next((10.0, 8.0, 9.0));
        assert_eq!(val1, 9.0); // Starts with close

        let val2 = ema.next((12.0, 7.0, 11.0));
        assert!(val2 > 9.0); // Should move up
    }
}

pub const TRADJ_EMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "True Range Adjusted Exponential Moving Average",
    description: "An exponential moving average that incorporates true range to measure volatility and adapt to price movements.",
    usage: "Use to identify trend turning points and filter price movements. Comparing TRAdj EMA with a standard EMA of the same length provides insights into the overall trend.",
    keywords: &["moving-average", "adaptive", "true-range", "volatility"],
    ehlers_summary: "Introduced by Vitali Apirine in TASC January 2023, TRAdj EMA modifies the standard exponential moving average by adjusting the smoothing factor using the True Range. The normalized true range modifies the rate, making the indicator more responsive during volatile periods while filtering out noise when volatility drops.",
    params: &[
        ParamDef {
            name: "period",
            default: "40",
            description: "Smoothing period",
        },
        ParamDef {
            name: "pds",
            default: "40",
            description: "Lookback period for True Range",
        },
        ParamDef {
            name: "mltp",
            default: "10.0",
            description: "Multiplier",
        },
    ],
    formula_source: "Technical Analysis of Stocks & Commodities, January 2023",
    formula_latex: r#"
\[
TRAdj = \frac{TR - TR_{min}}{TR_{max} - TR_{min}} \\ Rate = \frac{2}{P+1} \times (1 + TRAdj \times Multiplier)
\]
"#,
    gold_standard_file: "",
    category: "Moving Averages",
};
