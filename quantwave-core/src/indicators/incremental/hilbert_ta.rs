//! Native O(1)-per-bar Hilbert Transform indicators — TA-Lib parity.
//!
//! Faithful incremental port of `talib_rs::cycle` and `talib_rs::overlap::ht_trendline`.

use crate::traits::Next;

const RAD2DEG: f64 = 180.0 / std::f64::consts::PI;
const DEG2RAD: f64 = std::f64::consts::PI / 180.0;
const CONST_DEG2RAD_BY360: f64 = 2.0 * std::f64::consts::PI;
const A: f64 = 0.0962;
const B: f64 = 0.5769;
const SMOOTH_PRICE_SIZE: usize = 50;

#[derive(Debug, Clone, Default)]
struct HilbertVars {
    odd: [f64; 3],
    even: [f64; 3],
    prev_odd: f64,
    prev_even: f64,
    prev_input_odd: f64,
    prev_input_even: f64,
}

#[inline(always)]
fn do_hilbert_even(vars: &mut HilbertVars, input: f64, hilbert_idx: usize, adj: f64) -> f64 {
    let t = A * input;
    let mut result = -vars.even[hilbert_idx];
    vars.even[hilbert_idx] = t;
    result += t;
    result -= vars.prev_even;
    vars.prev_even = B * vars.prev_input_even;
    result += vars.prev_even;
    vars.prev_input_even = input;
    result * adj
}

#[inline(always)]
fn do_hilbert_odd(vars: &mut HilbertVars, input: f64, hilbert_idx: usize, adj: f64) -> f64 {
    let t = A * input;
    let mut result = -vars.odd[hilbert_idx];
    vars.odd[hilbert_idx] = t;
    result += t;
    result -= vars.prev_odd;
    vars.prev_odd = B * vars.prev_input_odd;
    result += vars.prev_odd;
    vars.prev_input_odd = input;
    result * adj
}

#[derive(Debug, Clone)]
struct HtWma {
    period_wma_sub: f64,
    period_wma_sum: f64,
    trailing_wma_value: f64,
    prices: Vec<f64>,
    trailing_idx: usize,
}

impl HtWma {
    fn from_first_three(p0: f64, p1: f64, p2: f64) -> Self {
        Self {
            period_wma_sub: p0 + p1 + p2,
            period_wma_sum: p0 + p1 * 2.0 + p2 * 3.0,
            trailing_wma_value: 0.0,
            prices: vec![p0, p1, p2],
            trailing_idx: 0,
        }
    }

    fn next(&mut self, new_price: f64) -> f64 {
        self.prices.push(new_price);
        self.period_wma_sub += new_price;
        self.period_wma_sub -= self.trailing_wma_value;
        self.period_wma_sum += new_price * 4.0;
        self.trailing_wma_value = self.prices[self.trailing_idx];
        self.trailing_idx += 1;
        let smoothed = self.period_wma_sum * 0.1;
        self.period_wma_sum -= self.period_wma_sub;
        smoothed
    }
}

#[derive(Debug, Clone)]
struct HilbertPeriodState {
    hilbert_idx: usize,
    detrender_vars: HilbertVars,
    q1_vars: HilbertVars,
    ji_vars: HilbertVars,
    jq_vars: HilbertVars,
    period: f64,
    smooth_period: f64,
    prev_i2: f64,
    prev_q2: f64,
    re: f64,
    im: f64,
    i1_for_odd_prev2: f64,
    i1_for_odd_prev3: f64,
    i1_for_even_prev2: f64,
    i1_for_even_prev3: f64,
}

impl Default for HilbertPeriodState {
    fn default() -> Self {
        Self {
            hilbert_idx: 0,
            detrender_vars: HilbertVars::default(),
            q1_vars: HilbertVars::default(),
            ji_vars: HilbertVars::default(),
            jq_vars: HilbertVars::default(),
            period: 0.0,
            smooth_period: 0.0,
            prev_i2: 0.0,
            prev_q2: 0.0,
            re: 0.0,
            im: 0.0,
            i1_for_odd_prev2: 0.0,
            i1_for_odd_prev3: 0.0,
            i1_for_even_prev2: 0.0,
            i1_for_even_prev3: 0.0,
        }
    }
}

impl HilbertPeriodState {
    fn adjust_period(&mut self) {
        let temp_real = self.period;
        if self.im != 0.0 && self.re != 0.0 {
            self.period = 360.0 / ((self.im / self.re).atan() * RAD2DEG);
        }
        let mut temp_real2 = 1.5 * temp_real;
        if self.period > temp_real2 {
            self.period = temp_real2;
        }
        temp_real2 = 0.67 * temp_real;
        if self.period < temp_real2 {
            self.period = temp_real2;
        }
        self.period = self.period.clamp(6.0, 50.0);
        self.period = 0.2 * self.period + 0.8 * temp_real;
        self.smooth_period = 0.33 * self.period + 0.67 * self.smooth_period;
    }

    fn step_hilbert(&mut self, today: usize, smoothed: f64, adj: f64) -> (f64, f64, f64, f64) {
        let (detrender, q1, i2, q2);
        if today.is_multiple_of(2) {
            detrender = do_hilbert_even(&mut self.detrender_vars, smoothed, self.hilbert_idx, adj);
            q1 = do_hilbert_even(&mut self.q1_vars, detrender, self.hilbert_idx, adj);
            let ji = do_hilbert_even(
                &mut self.ji_vars,
                self.i1_for_even_prev3,
                self.hilbert_idx,
                adj,
            );
            let jq = do_hilbert_even(&mut self.jq_vars, q1, self.hilbert_idx, adj);
            self.hilbert_idx += 1;
            if self.hilbert_idx == 3 {
                self.hilbert_idx = 0;
            }
            q2 = 0.2 * (q1 + ji) + 0.8 * self.prev_q2;
            i2 = 0.2 * (self.i1_for_even_prev3 - jq) + 0.8 * self.prev_i2;
            self.i1_for_odd_prev3 = self.i1_for_odd_prev2;
            self.i1_for_odd_prev2 = detrender;
        } else {
            detrender = do_hilbert_odd(&mut self.detrender_vars, smoothed, self.hilbert_idx, adj);
            q1 = do_hilbert_odd(&mut self.q1_vars, detrender, self.hilbert_idx, adj);
            let ji = do_hilbert_odd(
                &mut self.ji_vars,
                self.i1_for_odd_prev3,
                self.hilbert_idx,
                adj,
            );
            let jq = do_hilbert_odd(&mut self.jq_vars, q1, self.hilbert_idx, adj);
            q2 = 0.2 * (q1 + ji) + 0.8 * self.prev_q2;
            i2 = 0.2 * (self.i1_for_odd_prev3 - jq) + 0.8 * self.prev_i2;
            self.i1_for_even_prev3 = self.i1_for_even_prev2;
            self.i1_for_even_prev2 = detrender;
        }
        self.re = 0.2 * (i2 * self.prev_i2 + q2 * self.prev_q2) + 0.8 * self.re;
        self.im = 0.2 * (i2 * self.prev_q2 - q2 * self.prev_i2) + 0.8 * self.im;
        self.prev_q2 = q2;
        self.prev_i2 = i2;
        (detrender, q1, i2, q2)
    }
}

#[derive(Debug, Clone)]
struct HtEngine32 {
    prices: Vec<f64>,
    wma: Option<HtWma>,
    warmup_left: u8,
    hs: HilbertPeriodState,
}

impl HtEngine32 {
    const LOOKBACK: usize = 32;
    const WARMUP: u8 = 9;

    fn new() -> Self {
        Self {
            prices: Vec::new(),
            wma: None,
            warmup_left: Self::WARMUP,
            hs: HilbertPeriodState::default(),
        }
    }

    fn push(&mut self, price: f64) {
        self.prices.push(price);
    }

    fn today(&self) -> usize {
        self.prices.len().saturating_sub(1)
    }

    fn step_wma(&mut self) -> Option<f64> {
        let n = self.prices.len();
        if n < 3 {
            return None;
        }
        if self.wma.is_none() {
            self.wma = Some(HtWma::from_first_three(
                self.prices[0],
                self.prices[1],
                self.prices[2],
            ));
            return None;
        }
        let wma = self.wma.as_mut()?;
        let price = *self.prices.last().unwrap_or(&0.0);
        if self.warmup_left > 0 {
            self.warmup_left -= 1;
            let _ = wma.next(price);
            return None;
        }
        Some(wma.next(price))
    }
}

/// HT_DCPERIOD — lookback 32
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct HT_DCPERIOD {
    eng: HtEngine32,
}

impl Default for HT_DCPERIOD {
    fn default() -> Self {
        Self::new()
    }
}

impl HT_DCPERIOD {
    pub fn new() -> Self {
        Self {
            eng: HtEngine32::new(),
        }
    }
}

impl Next<f64> for HT_DCPERIOD {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.eng.push(input);
        let Some(smoothed) = self.eng.step_wma() else {
            return f64::NAN;
        };
        let today = self.eng.today();
        let adj = 0.075 * self.eng.hs.period + 0.54;
        self.eng.hs.step_hilbert(today, smoothed, adj);
        self.eng.hs.adjust_period();
        if today >= HtEngine32::LOOKBACK {
            self.eng.hs.smooth_period
        } else {
            f64::NAN
        }
    }
}

/// HT_PHASOR — lookback 32
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct HT_PHASOR {
    eng: HtEngine32,
}

impl Default for HT_PHASOR {
    fn default() -> Self {
        Self::new()
    }
}

impl HT_PHASOR {
    pub fn new() -> Self {
        Self {
            eng: HtEngine32::new(),
        }
    }
}

impl Next<f64> for HT_PHASOR {
    type Output = (f64, f64);

    fn next(&mut self, input: f64) -> Self::Output {
        self.eng.push(input);
        let Some(smoothed) = self.eng.step_wma() else {
            return (f64::NAN, f64::NAN);
        };
        let today = self.eng.today();
        let adj = 0.075 * self.eng.hs.period + 0.54;
        let inphase = if today.is_multiple_of(2) {
            self.eng.hs.i1_for_even_prev3
        } else {
            self.eng.hs.i1_for_odd_prev3
        };
        let (_, q1, _, _) = self.eng.hs.step_hilbert(today, smoothed, adj);
        self.eng.hs.adjust_period();
        if today >= HtEngine32::LOOKBACK {
            (inphase, q1)
        } else {
            (f64::NAN, f64::NAN)
        }
    }
}

#[derive(Debug, Clone)]
struct HtEngine63 {
    prices: Vec<f64>,
    wma: Option<HtWma>,
    warmup_left: u8,
    hs: HilbertPeriodState,
    smooth_price: [f64; SMOOTH_PRICE_SIZE],
    smooth_price_idx: usize,
    dc_phase: f64,
    prev_dc_phase: f64,
    i_trend1: f64,
    i_trend2: f64,
    i_trend3: f64,
    days_in_trend: i32,
    prev_sine: f64,
    prev_lead_sine: f64,
    sine: f64,
    lead_sine: f64,
    last_smoothed: f64,
    last_trendline: f64,
    last_trend: f64,
}

impl HtEngine63 {
    const LOOKBACK: usize = 63;
    const WARMUP: u8 = 34;

    fn new() -> Self {
        Self {
            prices: Vec::new(),
            wma: None,
            warmup_left: Self::WARMUP,
            hs: HilbertPeriodState::default(),
            smooth_price: [0.0; SMOOTH_PRICE_SIZE],
            smooth_price_idx: 0,
            dc_phase: 0.0,
            prev_dc_phase: 0.0,
            i_trend1: 0.0,
            i_trend2: 0.0,
            i_trend3: 0.0,
            days_in_trend: 0,
            prev_sine: 0.0,
            prev_lead_sine: 0.0,
            sine: 0.0,
            lead_sine: 0.0,
            last_smoothed: 0.0,
            last_trendline: 0.0,
            last_trend: 0.0,
        }
    }

    fn push(&mut self, price: f64) {
        self.prices.push(price);
    }

    fn today(&self) -> usize {
        self.prices.len().saturating_sub(1)
    }

    fn step_wma(&mut self) -> Option<f64> {
        let n = self.prices.len();
        if n < 3 {
            return None;
        }
        if self.wma.is_none() {
            self.wma = Some(HtWma::from_first_three(
                self.prices[0],
                self.prices[1],
                self.prices[2],
            ));
            return None;
        }
        let wma = self.wma.as_mut()?;
        let price = *self.prices.last().unwrap_or(&0.0);
        if self.warmup_left > 0 {
            self.warmup_left -= 1;
            let _ = wma.next(price);
            return None;
        }
        Some(wma.next(price))
    }

    fn compute_dc_phase(&mut self) {
        self.prev_dc_phase = self.dc_phase;
        let dc_period = self.hs.smooth_period + 0.5;
        let dc_period_int = dc_period as i32;
        let mut real_part = 0.0_f64;
        let mut imag_part = 0.0_f64;
        let mut idx = self.smooth_price_idx;
        for i in 0..dc_period_int {
            let angle = (i as f64 * CONST_DEG2RAD_BY360) / dc_period_int as f64;
            let price = self.smooth_price[idx];
            real_part += angle.sin() * price;
            imag_part += angle.cos() * price;
            if idx == 0 {
                idx = SMOOTH_PRICE_SIZE - 1;
            } else {
                idx -= 1;
            }
        }
        let abs_imag = imag_part.abs();
        if abs_imag > 0.0 {
            self.dc_phase = (real_part / imag_part).atan() * RAD2DEG;
        } else if abs_imag <= 0.01 {
            if real_part < 0.0 {
                self.dc_phase -= 90.0;
            } else if real_part > 0.0 {
                self.dc_phase += 90.0;
            }
        }
        self.dc_phase += 90.0;
        self.dc_phase += 360.0 / self.hs.smooth_period;
        if imag_part < 0.0 {
            self.dc_phase += 180.0;
        }
        if self.dc_phase > 315.0 {
            self.dc_phase -= 360.0;
        }
    }

    fn sum_prices_back(&self, count: i32) -> f64 {
        let mut temp = 0.0_f64;
        let mut price_idx = self.today();
        for _ in 0..count {
            temp += self.prices[price_idx];
            if price_idx == 0 {
                break;
            }
            price_idx -= 1;
        }
        if count > 0 { temp / count as f64 } else { temp }
    }

    fn step_core(&mut self) -> bool {
        let Some(smoothed) = self.step_wma() else {
            return false;
        };
        let today = self.today();
        let adj = 0.075 * self.hs.period + 0.54;
        self.last_smoothed = smoothed;
        self.smooth_price[self.smooth_price_idx] = smoothed;
        self.hs.step_hilbert(today, smoothed, adj);
        self.hs.adjust_period();
        self.compute_dc_phase();
        self.prev_sine = self.sine;
        self.prev_lead_sine = self.lead_sine;
        self.sine = (self.dc_phase * DEG2RAD).sin();
        self.lead_sine = ((self.dc_phase + 45.0) * DEG2RAD).sin();
        self.smooth_price_idx += 1;
        if self.smooth_price_idx >= SMOOTH_PRICE_SIZE {
            self.smooth_price_idx = 0;
        }
        self.update_trend_mode();
        true
    }

    fn trendline(&mut self) -> f64 {
        let dc_period_int = (self.hs.smooth_period + 0.5) as i32;
        let temp = self.sum_prices_back(dc_period_int);
        let trendline =
            (4.0 * temp + 3.0 * self.i_trend1 + 2.0 * self.i_trend2 + self.i_trend3) / 10.0;
        self.i_trend3 = self.i_trend2;
        self.i_trend2 = self.i_trend1;
        self.i_trend1 = temp;
        self.last_trendline = trendline;
        trendline
    }

    /// TA-Lib trend-mode decision — must run every bar after WMA warmup, not only at output index.
    fn update_trend_mode(&mut self) {
        let trendline = self.trendline();
        let mut trend = 1.0_f64;
        if (self.sine > self.lead_sine && self.prev_sine <= self.prev_lead_sine)
            || (self.sine < self.lead_sine && self.prev_sine >= self.prev_lead_sine)
        {
            self.days_in_trend = 0;
            trend = 0.0;
        }
        self.days_in_trend += 1;
        if (self.days_in_trend as f64) < 0.5 * self.hs.smooth_period {
            trend = 0.0;
        }
        let phase_change = self.dc_phase - self.prev_dc_phase;
        if self.hs.smooth_period != 0.0
            && phase_change > 0.67 * 360.0 / self.hs.smooth_period
            && phase_change < 1.5 * 360.0 / self.hs.smooth_period
        {
            trend = 0.0;
        }
        if trendline != 0.0 && ((self.last_smoothed - trendline) / trendline).abs() >= 0.015 {
            trend = 1.0;
        }
        self.last_trend = trend;
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct HT_DCPHASE {
    eng: HtEngine63,
}

impl Default for HT_DCPHASE {
    fn default() -> Self {
        Self::new()
    }
}

impl HT_DCPHASE {
    pub fn new() -> Self {
        Self {
            eng: HtEngine63::new(),
        }
    }
}

impl Next<f64> for HT_DCPHASE {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.eng.push(input);
        if !self.eng.step_core() {
            return f64::NAN;
        }
        if self.eng.today() >= HtEngine63::LOOKBACK {
            self.eng.dc_phase
        } else {
            f64::NAN
        }
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct HT_SINE {
    eng: HtEngine63,
}

impl Default for HT_SINE {
    fn default() -> Self {
        Self::new()
    }
}

impl HT_SINE {
    pub fn new() -> Self {
        Self {
            eng: HtEngine63::new(),
        }
    }
}

impl Next<f64> for HT_SINE {
    type Output = (f64, f64);

    fn next(&mut self, input: f64) -> Self::Output {
        self.eng.push(input);
        if !self.eng.step_core() {
            return (f64::NAN, f64::NAN);
        }
        if self.eng.today() >= HtEngine63::LOOKBACK {
            (self.eng.sine, self.eng.lead_sine)
        } else {
            (f64::NAN, f64::NAN)
        }
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct HT_TRENDMODE {
    eng: HtEngine63,
}

impl Default for HT_TRENDMODE {
    fn default() -> Self {
        Self::new()
    }
}

impl HT_TRENDMODE {
    pub fn new() -> Self {
        Self {
            eng: HtEngine63::new(),
        }
    }
}

impl Next<f64> for HT_TRENDMODE {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.eng.push(input);
        if !self.eng.step_core() {
            return f64::NAN;
        }
        if self.eng.today() >= HtEngine63::LOOKBACK {
            self.eng.last_trend
        } else {
            f64::NAN
        }
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct HT_TRENDLINE {
    eng: HtEngine63,
}

impl Default for HT_TRENDLINE {
    fn default() -> Self {
        Self::new()
    }
}

impl HT_TRENDLINE {
    pub fn new() -> Self {
        Self {
            eng: HtEngine63::new(),
        }
    }
}

impl Next<f64> for HT_TRENDLINE {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.eng.push(input);
        if !self.eng.step_core() {
            return f64::NAN;
        }
        if self.eng.today() >= HtEngine63::LOOKBACK {
            self.eng.last_trendline
        } else {
            f64::NAN
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_ht_dcperiod_parity(input in prop::collection::vec(0.1..100.0, 33..100)) {
            let mut ht = HT_DCPERIOD::new();
            let streaming: Vec<f64> = input.iter().map(|&x| ht.next(x)).collect();
            let batch = talib_rs::cycle::ht_dcperiod(&input).unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_ht_phasor_parity(input in prop::collection::vec(0.1..100.0, 33..100)) {
            let mut ht = HT_PHASOR::new();
            let streaming: Vec<_> = input.iter().map(|&x| ht.next(x)).collect();
            let (bi, bq) = talib_rs::cycle::ht_phasor(&input).unwrap_or_else(|_| {
                (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
            });
            for (i, &(s_i, s_q)) in streaming.iter().enumerate() {
                if s_i.is_nan() { assert!(bi[i].is_nan()); }
                else { approx::assert_relative_eq!(s_i, bi[i], epsilon = 1e-6); }
                if s_q.is_nan() { assert!(bq[i].is_nan()); }
                else { approx::assert_relative_eq!(s_q, bq[i], epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_ht_dcphase_parity(input in prop::collection::vec(0.1..100.0, 64..100)) {
            let mut ht = HT_DCPHASE::new();
            let streaming: Vec<f64> = input.iter().map(|&x| ht.next(x)).collect();
            let batch = talib_rs::cycle::ht_dcphase(&input).unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_ht_sine_parity(input in prop::collection::vec(0.1..100.0, 64..100)) {
            let mut ht = HT_SINE::new();
            let streaming: Vec<_> = input.iter().map(|&x| ht.next(x)).collect();
            let (bs, bl) = talib_rs::cycle::ht_sine(&input).unwrap_or_else(|_| {
                (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
            });
            for (i, &(s_s, s_l)) in streaming.iter().enumerate() {
                if s_s.is_nan() { assert!(bs[i].is_nan()); }
                else { approx::assert_relative_eq!(s_s, bs[i], epsilon = 1e-6); }
                if s_l.is_nan() { assert!(bl[i].is_nan()); }
                else { approx::assert_relative_eq!(s_l, bl[i], epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_ht_trendmode_parity(input in prop::collection::vec(0.1..100.0, 64..100)) {
            let mut ht = HT_TRENDMODE::new();
            let streaming: Vec<f64> = input.iter().map(|&x| ht.next(x)).collect();
            let batch = talib_rs::cycle::ht_trendmode(&input).unwrap_or_else(|_| vec![0; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                assert_eq!(*s as i32, *b);
            }
        }

        #[test]
        fn test_ht_trendline_parity(input in prop::collection::vec(0.1..100.0, 64..100)) {
            let mut ht = HT_TRENDLINE::new();
            let streaming: Vec<f64> = input.iter().map(|&x| ht.next(x)).collect();
            let batch = talib_rs::overlap::ht_trendline(&input).unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}
