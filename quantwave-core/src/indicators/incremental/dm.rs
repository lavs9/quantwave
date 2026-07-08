//! Native O(1) +DM / -DM — TA-Lib Wilder smoothing parity.

use crate::traits::Next;

#[derive(Debug, Clone)]
struct PlusDmCore {
    timeperiod: usize,
    period_f: f64,
    prev_high: Option<f64>,
    prev_low: Option<f64>,
    bar_index: usize,
    sum: f64,
    seeded: bool,
}

impl PlusDmCore {
    fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            period_f: timeperiod as f64,
            prev_high: None,
            prev_low: None,
            bar_index: 0,
            sum: 0.0,
            seeded: false,
        }
    }

    fn step(&mut self, high: f64, low: f64) -> Option<f64> {
        let period = self.timeperiod;
        if period < 1 {
            return None;
        }
        if self.prev_high.is_none() {
            self.prev_high = Some(high);
            self.prev_low = Some(low);
            self.bar_index = 1;
            return None;
        }
        let (ph, pl) = match (self.prev_high, self.prev_low) {
            (Some(ph), Some(pl)) => (ph, pl),
            _ => {
                self.prev_high = Some(high);
                self.prev_low = Some(low);
                self.bar_index = 1;
                return None;
            }
        };
        self.prev_high = Some(high);
        self.prev_low = Some(low);
        let i = self.bar_index;
        self.bar_index += 1;

        let up = high - ph;
        let down = pl - low;
        let pdm = if up > down && up > 0.0 { up } else { 0.0 };

        if !self.seeded {
            if i < period - 1 {
                self.sum += pdm;
                return None;
            }
            if i == period - 1 {
                self.sum += pdm;
                return Some(self.sum);
            }
            self.seeded = true;
        }
        self.sum = self.sum - self.sum / self.period_f + pdm;
        Some(self.sum)
    }
}

#[derive(Debug, Clone)]
struct MinusDmCore {
    timeperiod: usize,
    period_f: f64,
    prev_high: Option<f64>,
    prev_low: Option<f64>,
    bar_index: usize,
    sum: f64,
    seeded: bool,
}

impl MinusDmCore {
    fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            period_f: timeperiod as f64,
            prev_high: None,
            prev_low: None,
            bar_index: 0,
            sum: 0.0,
            seeded: false,
        }
    }

    fn step(&mut self, high: f64, low: f64) -> Option<f64> {
        let period = self.timeperiod;
        if period < 1 {
            return None;
        }
        if self.prev_high.is_none() {
            self.prev_high = Some(high);
            self.prev_low = Some(low);
            self.bar_index = 1;
            return None;
        }
        let (ph, pl) = match (self.prev_high, self.prev_low) {
            (Some(ph), Some(pl)) => (ph, pl),
            _ => {
                self.prev_high = Some(high);
                self.prev_low = Some(low);
                self.bar_index = 1;
                return None;
            }
        };
        self.prev_high = Some(high);
        self.prev_low = Some(low);
        let i = self.bar_index;
        self.bar_index += 1;

        let up = high - ph;
        let down = pl - low;
        let mdm = if down > up && down > 0.0 { down } else { 0.0 };

        if !self.seeded {
            if i < period - 1 {
                self.sum += mdm;
                return None;
            }
            if i == period - 1 {
                self.sum += mdm;
                return Some(self.sum);
            }
            self.seeded = true;
        }
        self.sum = self.sum - self.sum / self.period_f + mdm;
        Some(self.sum)
    }
}

/// Plus Directional Movement (+DM).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct PLUS_DM {
    pub timeperiod: usize,
    core: PlusDmCore,
}

impl PLUS_DM {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: PlusDmCore::new(timeperiod),
        }
    }
}

impl Next<(f64, f64)> for PLUS_DM {
    type Output = f64;

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        self.core.step(high, low).unwrap_or(f64::NAN)
    }
}

/// Minus Directional Movement (-DM).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MINUS_DM {
    pub timeperiod: usize,
    core: MinusDmCore,
}

impl MINUS_DM {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: MinusDmCore::new(timeperiod),
        }
    }
}

impl Next<(f64, f64)> for MINUS_DM {
    type Output = f64;

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        self.core.step(high, low).unwrap_or(f64::NAN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_plus_dm_parity(
            highs in prop::collection::vec(1.0..100.0, 1..100),
            lows in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = highs.len().min(lows.len());
            if len < 20 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            for i in 0..len {
                let hi: f64 = highs[i];
                let lo: f64 = lows[i];
                high.push(hi.max(lo));
                low.push(hi.min(lo));
            }
            let period = 14;
            let mut pdm = PLUS_DM::new(period);
            let streaming: Vec<f64> = (0..len).map(|i| pdm.next((high[i], low[i]))).collect();
            let batch = talib_rs::momentum::plus_dm(&high, &low, period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}
