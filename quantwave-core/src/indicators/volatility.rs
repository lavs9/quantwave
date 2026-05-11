use crate::traits::Next;
use crate::indicators::smoothing::EMA;

talib_3_in_1_out!(TaATR, talib_rs::volatility::atr, timeperiod: usize);
talib_3_in_1_out!(TaNATR, talib_rs::volatility::natr, timeperiod: usize);
talib_3_in_1_out!(TaTRANGE, talib_rs::volatility::trange);

/// True Range (TR)
#[derive(Debug, Clone, Default)]
pub struct TrueRange {
    prev_close: Option<f64>,
}

impl Next<(f64, f64, f64)> for TrueRange {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let tr = match self.prev_close {
            Some(pc) => {
                let h_l = high - low;
                let h_pc = (high - pc).abs();
                let l_pc = (low - pc).abs();
                h_l.max(h_pc).max(l_pc)
            }
            None => high - low,
        };
        self.prev_close = Some(close);
        tr
    }
}

/// Average True Range (ATR)
#[derive(Debug, Clone)]
pub struct ATR {
    tr: TrueRange,
    smoothing: EMA,
}

impl ATR {
    pub fn new(period: usize) -> Self {
        Self {
            tr: TrueRange::default(),
            smoothing: EMA::new(period),
        }
    }
}

impl Next<(f64, f64, f64)> for ATR {
    type Output = f64;

    fn next(&mut self, input: (f64, f64, f64)) -> Self::Output {
        let tr = self.tr.next(input);
        self.smoothing.next(tr)
    }
}
