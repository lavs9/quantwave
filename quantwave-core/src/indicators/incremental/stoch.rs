//! Native O(1) Stochastic family — TA-Lib parity (STOCH, STOCHF, STOCHRSI).

use crate::indicators::incremental::ma_stream::MaStream;
use crate::indicators::incremental::rsi::RSI;
use crate::traits::Next;
use crate::utils::RingBuffer;
use talib_rs::MaType;

/// Rolling highest high / lowest low over `period` bars.
#[derive(Debug, Clone)]
struct HlWindow {
    highs: RingBuffer<f64>,
    lows: RingBuffer<f64>,
    period: usize,
}

impl HlWindow {
    fn new(period: usize) -> Self {
        Self {
            highs: RingBuffer::with_capacity(period),
            lows: RingBuffer::with_capacity(period),
            period,
        }
    }

    fn push(&mut self, high: f64, low: f64) -> Option<(f64, f64, f64)> {
        if self.highs.len() >= self.period {
            let _ = self.highs.pop_front();
            let _ = self.lows.pop_front();
        }
        self.highs.push_back(high);
        self.lows.push_back(low);
        if self.highs.len() < self.period {
            return None;
        }
        let mut hh = f64::NEG_INFINITY;
        let mut ll = f64::INFINITY;
        for i in 0..self.highs.len() {
            let h = *self.highs.get(i).unwrap();
            let l = *self.lows.get(i).unwrap();
            if h > hh {
                hh = h;
            }
            if l < ll {
                ll = l;
            }
        }
        let range = hh - ll;
        Some((hh, ll, range))
    }
}

fn fastk_from_hlc(close: f64, ll: f64, range: f64) -> f64 {
    if range > 0.0 {
        100.0 * (close - ll) / range
    } else {
        50.0
    }
}

/// Stochastic Oscillator (STOCH) — default SMA smoothing on %K and %D.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct STOCH {
    pub fastk_period: usize,
    pub slowk_period: usize,
    pub slowk_matype: MaType,
    pub slowd_period: usize,
    pub slowd_matype: MaType,
    hl: HlWindow,
    slowk_ma: MaStream,
    slowd_ma: MaStream,
    slowk_valid: Vec<f64>,
    bar_index: usize,
    out_start: usize,
}

impl STOCH {
    pub fn new(
        fastk_period: usize,
        slowk_period: usize,
        slowk_matype: MaType,
        slowd_period: usize,
        slowd_matype: MaType,
    ) -> Self {
        Self {
            fastk_period,
            slowk_period,
            slowk_matype,
            slowd_period,
            slowd_matype,
            hl: HlWindow::new(fastk_period),
            slowk_ma: MaStream::new(slowk_period, slowk_matype),
            slowd_ma: MaStream::new(slowd_period, slowd_matype),
            slowk_valid: Vec::new(),
            bar_index: 0,
            out_start: fastk_period - 1 + slowk_period - 1 + slowd_period - 1,
        }
    }
}

impl Next<(f64, f64, f64)> for STOCH {
    type Output = (f64, f64);

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let i = self.bar_index;
        self.bar_index += 1;

        let Some((_, ll, range)) = self.hl.push(high, low) else {
            return (f64::NAN, f64::NAN);
        };
        let fastk = fastk_from_hlc(close, ll, range);
        let slowk_raw = self.slowk_ma.next(fastk);
        if !slowk_raw.is_nan() {
            self.slowk_valid.push(slowk_raw);
        }
        let slowd_raw = if slowk_raw.is_nan() {
            f64::NAN
        } else {
            self.slowd_ma.next(slowk_raw)
        };

        if i < self.out_start {
            return (f64::NAN, f64::NAN);
        }

        let k_skip = self.slowd_period - 1;
        let j = i - self.out_start;
        let idx = k_skip + j;
        let slowk_out = self.slowk_valid.get(idx).copied().unwrap_or(f64::NAN);
        let slowd_out = if slowd_raw.is_nan() { f64::NAN } else { slowd_raw };

        (slowk_out, slowd_out)
    }
}

/// Fast Stochastic (STOCHF).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct STOCHF {
    pub fastk_period: usize,
    pub fastd_period: usize,
    pub fastd_matype: MaType,
    hl: HlWindow,
    fastd_ma: MaStream,
    fastk_values: Vec<f64>,
    bar_index: usize,
    out_start: usize,
}

impl STOCHF {
    pub fn new(fastk_period: usize, fastd_period: usize, fastd_matype: MaType) -> Self {
        Self {
            fastk_period,
            fastd_period,
            fastd_matype,
            hl: HlWindow::new(fastk_period),
            fastd_ma: MaStream::new(fastd_period, fastd_matype),
            fastk_values: Vec::new(),
            bar_index: 0,
            out_start: fastk_period - 1 + fastd_period - 1,
        }
    }
}

impl Next<(f64, f64, f64)> for STOCHF {
    type Output = (f64, f64);

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let i = self.bar_index;
        self.bar_index += 1;

        let Some((_, ll, range)) = self.hl.push(high, low) else {
            return (f64::NAN, f64::NAN);
        };
        let fastk = fastk_from_hlc(close, ll, range);
        self.fastk_values.push(fastk);

        let fastd_raw = self.fastd_ma.next(fastk);

        if i < self.out_start {
            return (f64::NAN, f64::NAN);
        }

        let k_skip = self.fastd_period - 1;
        let j = i - self.out_start;
        let idx = k_skip + j;
        let fastk_out = self.fastk_values.get(idx).copied().unwrap_or(f64::NAN);
        let fastd_out = if fastd_raw.is_nan() { f64::NAN } else { fastd_raw };

        (fastk_out, fastd_out)
    }
}

/// Stochastic RSI (STOCHRSI).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct STOCHRSI {
    pub timeperiod: usize,
    pub fastk_period: usize,
    pub fastd_period: usize,
    pub fastd_matype: MaType,
    rsi: RSI,
    rsi_valid: Vec<f64>,
    fastd_ma: MaStream,
    fastk_values: Vec<f64>,
    bar_index: usize,
    d_start: usize,
}

impl STOCHRSI {
    pub fn new(
        timeperiod: usize,
        fastk_period: usize,
        fastd_period: usize,
        fastd_matype: MaType,
    ) -> Self {
        let d_start = timeperiod + fastk_period - 1 + fastd_period - 1;
        Self {
            timeperiod,
            fastk_period,
            fastd_period,
            fastd_matype,
            rsi: RSI::new(timeperiod),
            rsi_valid: Vec::new(),
            fastd_ma: MaStream::new(fastd_period, fastd_matype),
            fastk_values: Vec::new(),
            bar_index: 0,
            d_start,
        }
    }
}

impl Next<f64> for STOCHRSI {
    type Output = (f64, f64);

    fn next(&mut self, input: f64) -> Self::Output {
        let i = self.bar_index;
        self.bar_index += 1;

        let rsi_v = self.rsi.next(input);
        if !rsi_v.is_nan() {
            self.rsi_valid.push(rsi_v);
        }

        if self.rsi_valid.len() < self.fastk_period {
            return (f64::NAN, f64::NAN);
        }

        let idx = self.rsi_valid.len() - 1;
        let start = idx + 1 - self.fastk_period;
        let mut hh = f64::NEG_INFINITY;
        let mut ll = f64::INFINITY;
        for j in start..=idx {
            let v = self.rsi_valid[j];
            if v > hh {
                hh = v;
            }
            if v < ll {
                ll = v;
            }
        }
        let range = hh - ll;
        let fastk = if range > 0.0 {
            100.0 * (self.rsi_valid[idx] - ll) / range
        } else {
            50.0
        };
        self.fastk_values.push(fastk);

        let fastd_raw = self.fastd_ma.next(fastk);

        if i < self.d_start {
            return (f64::NAN, f64::NAN);
        }

        let k_skip = self.fastd_period - 1;
        let j = i - self.d_start;
        let idx = k_skip + j;
        let fastk_out = self.fastk_values.get(idx).copied().unwrap_or(f64::NAN);
        let fastd_out = if fastd_raw.is_nan() { f64::NAN } else { fastd_raw };

        (fastk_out, fastd_out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_stoch_parity(
            highs in prop::collection::vec(1.0..100.0, 1..100),
            lows in prop::collection::vec(1.0..100.0, 1..100),
            closes in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = highs.len().min(lows.len()).min(closes.len());
            if len < 20 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            for i in 0..len {
                let val_h: f64 = highs[i];
                let val_l: f64 = lows[i];
                let val_c: f64 = closes[i];
                high.push(val_h.max(val_l).max(val_c));
                low.push(val_h.min(val_l).min(val_c));
                close.push(val_c);
            }

            let fastk = 5;
            let slowk = 3;
            let slowk_ma = MaType::Sma;
            let slowd = 3;
            let slowd_ma = MaType::Sma;

            let mut stoch = STOCH::new(fastk, slowk, slowk_ma, slowd, slowd_ma);
            let streaming: Vec<(f64, f64)> = (0..len)
                .map(|i| stoch.next((high[i], low[i], close[i])))
                .collect();
            let (b_k, b_d) = talib_rs::momentum::stoch(
                &high, &low, &close, fastk, slowk, slowk_ma, slowd, slowd_ma,
            )
            .unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len]));

            for (i, (s_k, s_d)) in streaming.into_iter().enumerate() {
                if s_k.is_nan() { assert!(b_k[i].is_nan()); }
                else { approx::assert_relative_eq!(s_k, b_k[i], epsilon = 1e-6); }
                if s_d.is_nan() { assert!(b_d[i].is_nan()); }
                else { approx::assert_relative_eq!(s_d, b_d[i], epsilon = 1e-6); }
            }
        }
    }
}