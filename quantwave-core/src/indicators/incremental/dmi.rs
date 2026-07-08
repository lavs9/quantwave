//! Native O(1) DMI family — TA-Lib Wilder smoothing parity (ADX, ADXR, DX, +DI, -DI).

use crate::traits::Next;

/// Shared Wilder-smoothed TR / +DM / -DM state.
#[derive(Debug, Clone)]
struct DmiCore {
    timeperiod: usize,
    period_f: f64,
    prev_high: Option<f64>,
    prev_low: Option<f64>,
    prev_close: Option<f64>,
    bar_index: usize,
    sum_tr: f64,
    sum_pdm: f64,
    sum_mdm: f64,
    seeded: bool,
}

impl DmiCore {
    fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            period_f: timeperiod as f64,
            prev_high: None,
            prev_low: None,
            prev_close: None,
            bar_index: 0,
            sum_tr: 0.0,
            sum_pdm: 0.0,
            sum_mdm: 0.0,
            seeded: false,
        }
    }

    #[inline]
    fn dm_components(&self, high: f64, low: f64) -> (f64, f64, f64) {
        let (ph, pl, pc) = match (self.prev_high, self.prev_low, self.prev_close) {
            (Some(ph), Some(pl), Some(pc)) => (ph, pl, pc),
            _ => return (0.0, 0.0, 0.0),
        };
        let hl = high - low;
        let hc = (high - pc).abs();
        let lc = (low - pc).abs();
        let tr = hl.max(hc).max(lc);
        let up = high - ph;
        let down = pl - low;
        let pdm = if up > down && up > 0.0 { up } else { 0.0 };
        let mdm = if down > up && down > 0.0 { down } else { 0.0 };
        (tr, pdm, mdm)
    }

    /// Advance one bar; returns `Some((pdi, mdi, dx))` once DI values are defined.
    fn step(&mut self, high: f64, low: f64, close: f64) -> Option<(f64, f64, f64)> {
        let period = self.timeperiod;
        if period < 1 {
            return None;
        }

        if self.prev_high.is_none() {
            self.prev_high = Some(high);
            self.prev_low = Some(low);
            self.prev_close = Some(close);
            self.bar_index = 1;
            return None;
        }

        let (tr, pdm, mdm) = self.dm_components(high, low);
        self.prev_high = Some(high);
        self.prev_low = Some(low);
        self.prev_close = Some(close);
        let i = self.bar_index;
        self.bar_index += 1;

        if !self.seeded {
            if i < period {
                self.sum_tr += tr;
                self.sum_pdm += pdm;
                self.sum_mdm += mdm;
                return None;
            }
            self.seeded = true;
        }
        self.sum_tr = self.sum_tr - self.sum_tr / self.period_f + tr;
        self.sum_pdm = self.sum_pdm - self.sum_pdm / self.period_f + pdm;
        self.sum_mdm = self.sum_mdm - self.sum_mdm / self.period_f + mdm;

        if self.sum_tr <= 0.0 {
            return None;
        }
        let pdi = 100.0 * self.sum_pdm / self.sum_tr;
        let mdi = 100.0 * self.sum_mdm / self.sum_tr;
        let sum_di = pdi + mdi;
        let dx = if sum_di > 0.0 {
            100.0 * (pdi - mdi).abs() / sum_di
        } else {
            0.0
        };
        Some((pdi, mdi, dx))
    }
}

/// Plus Directional Indicator (+DI).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct PLUS_DI {
    pub timeperiod: usize,
    core: DmiCore,
}

impl PLUS_DI {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: DmiCore::new(timeperiod),
        }
    }
}

impl Next<(f64, f64, f64)> for PLUS_DI {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        match self.core.step(high, low, close) {
            Some((pdi, _, _)) => pdi,
            None => f64::NAN,
        }
    }
}

/// Minus Directional Indicator (-DI).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MINUS_DI {
    pub timeperiod: usize,
    core: DmiCore,
}

impl MINUS_DI {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: DmiCore::new(timeperiod),
        }
    }
}

impl Next<(f64, f64, f64)> for MINUS_DI {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        match self.core.step(high, low, close) {
            Some((_, mdi, _)) => mdi,
            None => f64::NAN,
        }
    }
}

/// Directional Movement Index (DX).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DX {
    pub timeperiod: usize,
    core: DmiCore,
}

impl DX {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: DmiCore::new(timeperiod),
        }
    }
}

impl Next<(f64, f64, f64)> for DX {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        match self.core.step(high, low, close) {
            Some((_, _, dx)) => dx,
            None => f64::NAN,
        }
    }
}

/// Average Directional Index (ADX).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct ADX {
    pub timeperiod: usize,
    core: DmiCore,
    dx_values: Vec<f64>,
    adx: f64,
    adx_ready: bool,
}

impl ADX {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: DmiCore::new(timeperiod),
            dx_values: Vec::new(),
            adx: 0.0,
            adx_ready: false,
        }
    }
}

impl Next<(f64, f64, f64)> for ADX {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let period = self.timeperiod;
        if period < 2 {
            return f64::NAN;
        }

        let Some((_, _, dx)) = self.core.step(high, low, close) else {
            return f64::NAN;
        };

        let adx_start = 2 * period - 1;
        let bar = self.core.bar_index.saturating_sub(1);

        if bar < period {
            return f64::NAN;
        }

        if bar < adx_start {
            self.dx_values.push(dx);
            return f64::NAN;
        }

        if bar == adx_start {
            self.dx_values.push(dx);
            let seed: f64 = self.dx_values.iter().sum::<f64>() / period as f64;
            self.adx = seed;
            self.adx_ready = true;
            return seed;
        }

        if self.adx_ready {
            self.adx = (self.adx * (period as f64 - 1.0) + dx) / period as f64;
            return self.adx;
        }

        f64::NAN
    }
}

/// Average Directional Movement Index Rating (ADXR).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct ADXR {
    pub timeperiod: usize,
    adx: ADX,
    adx_history: Vec<f64>,
}

impl ADXR {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            adx: ADX::new(timeperiod),
            adx_history: Vec::new(),
        }
    }
}

impl Next<(f64, f64, f64)> for ADXR {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let period = self.timeperiod;
        let adx_val = self.adx.next((high, low, close));
        self.adx_history.push(adx_val);

        let adxr_lookback = 3 * period - 2;
        let bar = self.adx_history.len().saturating_sub(1);
        if bar < adxr_lookback {
            return f64::NAN;
        }
        if adx_val.is_nan() {
            return f64::NAN;
        }
        let past_idx = bar + 1 - period;
        let past = self.adx_history[past_idx];
        if past.is_nan() {
            return f64::NAN;
        }
        (adx_val + past) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn hlc(len: usize, h: &[f64], l: &[f64], c: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut high = Vec::with_capacity(len);
        let mut low = Vec::with_capacity(len);
        let mut close = Vec::with_capacity(len);
        for i in 0..len {
            let val_h = h[i];
            let val_l = l[i];
            let val_c = c[i];
            high.push(val_h.max(val_l).max(val_c));
            low.push(val_h.min(val_l).min(val_c));
            close.push(val_c);
        }
        (high, low, close)
    }

    proptest! {
        #[test]
        fn test_adx_parity(
            h in prop::collection::vec(1.0..100.0, 1..100),
            l in prop::collection::vec(1.0..100.0, 1..100),
            c in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len());
            if len < 30 { return Ok(()); }
            let (high, low, close) = hlc(len, &h, &l, &c);
            let period = 14;
            let mut adx = ADX::new(period);
            let streaming: Vec<f64> = (0..len)
                .map(|i| adx.next((high[i], low[i], close[i])))
                .collect();
            let batch = talib_rs::momentum::adx(&high, &low, &close, period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_dx_parity(
            h in prop::collection::vec(1.0..100.0, 1..100),
            l in prop::collection::vec(1.0..100.0, 1..100),
            c in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len());
            if len < 20 { return Ok(()); }
            let (high, low, close) = hlc(len, &h, &l, &c);
            let period = 14;
            let mut dx = DX::new(period);
            let streaming: Vec<f64> = (0..len)
                .map(|i| dx.next((high[i], low[i], close[i])))
                .collect();
            let batch = talib_rs::momentum::dx(&high, &low, &close, period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_plus_di_parity(
            h in prop::collection::vec(1.0..100.0, 1..100),
            l in prop::collection::vec(1.0..100.0, 1..100),
            c in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len());
            if len < 20 { return Ok(()); }
            let (high, low, close) = hlc(len, &h, &l, &c);
            let period = 14;
            let mut pdi = PLUS_DI::new(period);
            let streaming: Vec<f64> = (0..len)
                .map(|i| pdi.next((high[i], low[i], close[i])))
                .collect();
            let batch = talib_rs::momentum::plus_di(&high, &low, &close, period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}
