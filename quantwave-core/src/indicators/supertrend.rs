use crate::traits::Next;
use crate::indicators::volatility::ATR;

/// SuperTrend Indicator
#[derive(Debug, Clone)]
pub struct SuperTrend {
    atr: ATR,
    multiplier: f64,
    prev_close: Option<f64>,
    prev_upper_band: Option<f64>,
    prev_lower_band: Option<f64>,
    direction: i8, // 1 for up, -1 for down
}

impl SuperTrend {
    pub fn new(period: usize, multiplier: f64) -> Self {
        Self {
            atr: ATR::new(period),
            multiplier,
            prev_close: None,
            prev_upper_band: None,
            prev_lower_band: None,
            direction: 1,
        }
    }
}

impl Next<(f64, f64, f64)> for SuperTrend {
    type Output = (f64, i8);

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let atr = self.atr.next((high, low, close));
        let mid = (high + low) / 2.0;
        
        let basic_upper = mid + self.multiplier * atr;
        let basic_lower = mid - self.multiplier * atr;

        let upper_band = match self.prev_upper_band {
            Some(prev_upper) => {
                if basic_upper < prev_upper || self.prev_close.unwrap_or(0.0) > prev_upper {
                    basic_upper
                } else {
                    prev_upper
                }
            }
            None => basic_upper,
        };

        let lower_band = match self.prev_lower_band {
            Some(prev_lower) => {
                if basic_lower > prev_lower || self.prev_close.unwrap_or(0.0) < prev_lower {
                    basic_lower
                } else {
                    prev_lower
                }
            }
            None => basic_lower,
        };

        if self.direction == -1 && close > upper_band {
            self.direction = 1;
        } else if self.direction == 1 && close < lower_band {
            self.direction = -1;
        }

        let supertrend = if self.direction == 1 {
            lower_band
        } else {
            upper_band
        };

        self.prev_close = Some(close);
        self.prev_upper_band = Some(upper_band);
        self.prev_lower_band = Some(lower_band);

        (supertrend, self.direction)
    }
}
