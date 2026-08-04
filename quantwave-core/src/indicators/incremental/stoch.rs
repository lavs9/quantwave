//! Native O(1) Stochastic family — TA-Lib parity (STOCH, STOCHF, STOCHRSI).

use crate::indicators::incremental::ma_stream::MaStream;
use crate::indicators::incremental::rsi::RSI;
use crate::indicators::ma_type::MaType;
use crate::traits::Next;
use crate::utils::RingBuffer;

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
        for (&h, &l) in self.highs.iter().zip(self.lows.iter()) {
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
        let slowd_out = if slowd_raw.is_nan() {
            f64::NAN
        } else {
            slowd_raw
        };

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
        let fastd_out = if fastd_raw.is_nan() {
            f64::NAN
        } else {
            fastd_raw
        };

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
        let fastd_out = if fastd_raw.is_nan() {
            f64::NAN
        } else {
            fastd_raw
        };

        (fastk_out, fastd_out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// `MaType`s whose lookback is exactly `period - 1`.
    ///
    /// `talib_rs::momentum::stoch` (and `stochf` / `stochrsi`) slice the NaN
    /// prefix off the smoothed %K with the hard-coded length `slowk_period - 1`
    /// rather than the selected MA's real lookback. For SMA/EMA/WMA/TRIMA that
    /// is the same number and the oracle is meaningful. For DEMA/TEMA/KAMA/
    /// MAMA/T3 the batch keeps NaNs inside the "valid" slice and then smooths
    /// them, so the oracle's own output is NaN-poisoned garbage — there is
    /// nothing coherent to assert parity against. Our streaming path uses each
    /// MA's true warmup instead. Tracked for an upstream fix; do not "fix" the
    /// streaming side to reproduce the poisoning.
    const STOCH_PARITY_MATYPES: [MaType; 4] =
        [MaType::Sma, MaType::Ema, MaType::Wma, MaType::Trima];

    fn hlc(
        len: usize,
        highs: &[f64],
        lows: &[f64],
        closes: &[f64],
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut high = Vec::with_capacity(len);
        let mut low = Vec::with_capacity(len);
        let mut close = Vec::with_capacity(len);
        for i in 0..len {
            let (h, l, c): (f64, f64, f64) = (highs[i], lows[i], closes[i]);
            high.push(h.max(l).max(c));
            low.push(h.min(l).min(c));
            close.push(c);
        }
        (high, low, close)
    }

    fn assert_pair_parity(streaming: &[(f64, f64)], b_k: &[f64], b_d: &[f64], label: &str) {
        for (i, &(s_k, s_d)) in streaming.iter().enumerate() {
            for (s, b, name) in [(s_k, b_k[i], "k"), (s_d, b_d[i], "d")] {
                if s.is_nan() {
                    assert!(
                        b.is_nan(),
                        "{label} bar {i} {name}: streaming NaN, batch {b}"
                    );
                } else {
                    assert!(
                        !b.is_nan(),
                        "{label} bar {i} {name}: streaming {s}, batch NaN"
                    );
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_stoch_parity(
            highs in prop::collection::vec(1.0..100.0, 1..100),
            lows in prop::collection::vec(1.0..100.0, 1..100),
            closes in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = highs.len().min(lows.len()).min(closes.len());
            if len < 20 { return Ok(()); }
            let (high, low, close) = hlc(len, &highs, &lows, &closes);

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
                &high, &low, &close, fastk, slowk, slowk_ma.into(), slowd, slowd_ma.into(),
            )
            .unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len]));

            assert_pair_parity(&streaming, &b_k, &b_d, "stoch/sma");
        }

        /// STOCH and STOCHF smooth through `MaStream`, so `slowk_matype` /
        /// `fastd_matype` were silently ignored for everything except EMA.
        #[test]
        fn test_stoch_family_parity_matypes(
            highs in prop::collection::vec(1.0..100.0, 60..120),
            lows in prop::collection::vec(1.0..100.0, 60..120),
            closes in prop::collection::vec(1.0..100.0, 60..120),
            idx in 0usize..4,
        ) {
            let matype = STOCH_PARITY_MATYPES[idx];
            let len = highs.len().min(lows.len()).min(closes.len());
            let (high, low, close) = hlc(len, &highs, &lows, &closes);
            let (fastk, slowk, slowd) = (5usize, 3usize, 3usize);

            let mut stoch = STOCH::new(fastk, slowk, matype, slowd, matype);
            let streaming: Vec<(f64, f64)> = (0..len)
                .map(|i| stoch.next((high[i], low[i], close[i])))
                .collect();
            let (b_k, b_d) = talib_rs::momentum::stoch(
                &high, &low, &close, fastk, slowk, matype.into(), slowd, matype.into(),
            )
            .unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len]));
            assert_pair_parity(&streaming, &b_k, &b_d, &format!("stoch/{matype}"));

            let mut stochf = STOCHF::new(fastk, slowd, matype);
            let streaming: Vec<(f64, f64)> = (0..len)
                .map(|i| stochf.next((high[i], low[i], close[i])))
                .collect();
            let (b_k, b_d) = talib_rs::momentum::stochf(
                &high, &low, &close, fastk, slowd, matype.into(),
            )
            .unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len]));
            assert_pair_parity(&streaming, &b_k, &b_d, &format!("stochf/{matype}"));
        }
    }

    /// The `matype` argument must actually reach the smoothing stage.
    #[test]
    fn stoch_matype_changes_output() {
        let bars: Vec<(f64, f64, f64)> = (0..200)
            .map(|i| {
                let c = 50.0 + 10.0 * (i as f64 * 0.3).sin();
                (c + 1.0, c - 1.0, c)
            })
            .collect();
        let run = |m: MaType| {
            let mut s = STOCH::new(5, 3, m, 3, m);
            bars.iter().map(|&b| s.next(b)).collect::<Vec<_>>()
        };
        let sma = run(MaType::Sma);
        for matype in MaType::ALL {
            if matype == MaType::Sma {
                continue;
            }
            let got = run(matype);
            let identical = got.iter().zip(sma.iter()).all(|(a, b)| {
                ((a.0.is_nan() && b.0.is_nan()) || (a.0 - b.0).abs() < 1e-12)
                    && ((a.1.is_nan() && b.1.is_nan()) || (a.1 - b.1).abs() < 1e-12)
            });
            assert!(!identical, "STOCH with {matype} still produces SMA values");
        }
    }
}
