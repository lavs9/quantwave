use crate::traits::Next;
use crate::indicators::smoothing::EMA;
use crate::indicators::volatility::ATR;

#[derive(Debug, Clone)]
pub struct KeltnerChannels {
    ema: EMA,
    atr: ATR,
    multiplier: f64,
}

impl KeltnerChannels {
    pub fn new(ema_period: usize, atr_period: usize, multiplier: f64) -> Self {
        Self {
            ema: EMA::new(ema_period),
            atr: ATR::new(atr_period),
            multiplier,
        }
    }
}

impl Next<(f64, f64, f64)> for KeltnerChannels {
    type Output = (f64, f64, f64);

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let typical_price = (high + low + close) / 3.0;
        let middle = self.ema.next(typical_price);
        let atr = self.atr.next((high, low, close));
        
        let upper = middle + self.multiplier * atr;
        let lower = middle - self.multiplier * atr;

        (upper, middle, lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keltner_basic() {
        let mut kc = KeltnerChannels::new(3, 3, 2.0);
        // Typical price = (H+L+C)/3
        // bar 1: H=12, L=8, C=10 -> TP=10. ATR=4 (since TR=4). EMA=10.
        // Upper = 10 + 2*4 = 18. Lower = 10 - 2*4 = 2.
        
        let (upper, middle, lower) = kc.next((12.0, 8.0, 10.0));
        approx::assert_relative_eq!(middle, 10.0);
        approx::assert_relative_eq!(upper, 18.0);
        approx::assert_relative_eq!(lower, 2.0);
    }
}
