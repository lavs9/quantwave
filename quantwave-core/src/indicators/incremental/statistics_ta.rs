//! Native O(1) TA-Lib statistics: STDDEV, VAR, LINEARREG family, BETA, CORREL, TSF.

use crate::traits::Next;
use crate::utils::RingBuffer;

#[inline]
fn linreg_coeffs(ys: &[f64]) -> (f64, f64) {
    let n = ys.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xy = 0.0;
    for (i, &y) in ys.iter().enumerate() {
        let x = i as f64;
        sum_x += x;
        sum_x2 += x * x;
        sum_y += y;
        sum_xy += x * y;
    }
    let denom = n.mul_add(sum_x2, -sum_x * sum_x);
    if denom == 0.0 {
        return (f64::NAN, f64::NAN);
    }
    let slope = n.mul_add(sum_xy, -sum_x * sum_y) / denom;
    let intercept = (sum_y - slope * sum_x) / n;
    (slope, intercept)
}

#[derive(Debug, Clone)]
struct RollingWindow {
    period: usize,
    buf: RingBuffer<f64>,
    sum: f64,
    sum_sq: f64,
}

impl RollingWindow {
    fn new(period: usize) -> Self {
        Self {
            period,
            buf: RingBuffer::with_capacity(period),
            sum: 0.0,
            sum_sq: 0.0,
        }
    }

    fn push(&mut self, v: f64) -> bool {
        if self.buf.len() >= self.period
            && let Some(old) = self.buf.pop_front()
        {
            self.sum -= old;
            self.sum_sq -= old * old;
        }
        self.buf.push_back(v);
        self.sum += v;
        self.sum_sq += v * v;
        self.buf.len() >= self.period
    }

    fn values(&self) -> Vec<f64> {
        self.buf.iter().cloned().collect()
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaSTDDEV {
    pub timeperiod: usize,
    pub nbdev: f64,
    inner: RollingWindow,
}

impl TaSTDDEV {
    pub fn new(timeperiod: usize, nbdev: f64) -> Self {
        Self {
            timeperiod,
            nbdev,
            inner: RollingWindow::new(timeperiod),
        }
    }
}

impl Next<f64> for TaSTDDEV {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if !self.inner.push(input) {
            return f64::NAN;
        }
        let n = self.timeperiod as f64;
        let mean = self.inner.sum / n;
        let var = (self.inner.sum_sq / n - mean * mean).max(0.0);
        var.sqrt() * self.nbdev
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaVAR {
    pub timeperiod: usize,
    pub nbdev: f64,
    inner: RollingWindow,
}

impl TaVAR {
    pub fn new(timeperiod: usize, nbdev: f64) -> Self {
        Self {
            timeperiod,
            nbdev,
            inner: RollingWindow::new(timeperiod),
        }
    }
}

impl Next<f64> for TaVAR {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if !self.inner.push(input) {
            return f64::NAN;
        }
        let n = self.timeperiod as f64;
        let mean = self.inner.sum / n;
        (self.inner.sum_sq / n - mean * mean).max(0.0)
    }
}

#[derive(Debug, Clone)]
struct LinRegCore {
    _period: usize,
    inner: RollingWindow,
}

impl LinRegCore {
    fn new(period: usize) -> Self {
        Self {
            _period: period,
            inner: RollingWindow::new(period),
        }
    }

    fn push(&mut self, v: f64) -> Option<(f64, f64)> {
        if !self.inner.push(v) {
            return None;
        }
        Some(linreg_coeffs(&self.inner.values()))
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaLINEARREG {
    pub timeperiod: usize,
    core: LinRegCore,
}

impl TaLINEARREG {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: LinRegCore::new(timeperiod),
        }
    }
}

impl Next<f64> for TaLINEARREG {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let Some((slope, intercept)) = self.core.push(input) else {
            return f64::NAN;
        };
        let x = (self.timeperiod - 1) as f64;
        intercept + slope * x
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaLINEARREG_SLOPE {
    pub timeperiod: usize,
    core: LinRegCore,
}

impl TaLINEARREG_SLOPE {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: LinRegCore::new(timeperiod),
        }
    }
}

impl Next<f64> for TaLINEARREG_SLOPE {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        match self.core.push(input) {
            Some((slope, _)) => slope,
            None => f64::NAN,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaLINEARREG_INTERCEPT {
    pub timeperiod: usize,
    core: LinRegCore,
}

impl TaLINEARREG_INTERCEPT {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: LinRegCore::new(timeperiod),
        }
    }
}

impl Next<f64> for TaLINEARREG_INTERCEPT {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        match self.core.push(input) {
            Some((_, intercept)) => intercept,
            None => f64::NAN,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaLINEARREG_ANGLE {
    pub timeperiod: usize,
    core: LinRegCore,
}

impl TaLINEARREG_ANGLE {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: LinRegCore::new(timeperiod),
        }
    }
}

impl Next<f64> for TaLINEARREG_ANGLE {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        match self.core.push(input) {
            Some((slope, _)) => slope.atan().to_degrees(),
            None => f64::NAN,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaTSF {
    pub timeperiod: usize,
    core: LinRegCore,
}

impl TaTSF {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            core: LinRegCore::new(timeperiod),
        }
    }
}

impl Next<f64> for TaTSF {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let Some((slope, intercept)) = self.core.push(input) else {
            return f64::NAN;
        };
        intercept + slope * self.timeperiod as f64
    }
}

#[derive(Debug, Clone)]
struct DualWindow {
    period: usize,
    xs: RingBuffer<f64>,
    ys: RingBuffer<f64>,
}

impl DualWindow {
    fn new(period: usize) -> Self {
        Self {
            period,
            xs: RingBuffer::with_capacity(period),
            ys: RingBuffer::with_capacity(period),
        }
    }

    fn push(&mut self, x: f64, y: f64) -> bool {
        if self.xs.len() >= self.period {
            let _ = self.xs.pop_front();
            let _ = self.ys.pop_front();
        }
        self.xs.push_back(x);
        self.ys.push_back(y);
        self.xs.len() >= self.period
    }

    fn xs_values(&self) -> Vec<f64> {
        self.xs.iter().cloned().collect()
    }

    fn ys_values(&self) -> Vec<f64> {
        self.ys.iter().cloned().collect()
    }
}

#[inline]
fn correl_beta(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_y2 = 0.0;
    let mut sum_xy = 0.0;
    for i in 0..xs.len() {
        let x = xs[i];
        let y = ys[i];
        sum_x += x;
        sum_y += y;
        sum_x2 += x * x;
        sum_y2 += y * y;
        sum_xy += x * y;
    }
    let cov = n.mul_add(sum_xy, -sum_x * sum_y);
    let var_x = n.mul_add(sum_x2, -sum_x * sum_x);
    let var_y = n.mul_add(sum_y2, -sum_y * sum_y);
    if var_x == 0.0 || var_y == 0.0 {
        return (f64::NAN, f64::NAN);
    }
    let correl = cov / (var_x.sqrt() * var_y.sqrt());
    // Linear regression Y = alpha + beta * X  =>  beta = cov(X,Y) / var(X).
    let beta = cov / var_x;
    (correl, beta)
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaCORREL {
    pub timeperiod: usize,
    inner: DualWindow,
}

impl TaCORREL {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            inner: DualWindow::new(timeperiod),
        }
    }
}

impl Next<(f64, f64)> for TaCORREL {
    type Output = f64;

    fn next(&mut self, (x, y): (f64, f64)) -> Self::Output {
        if !self.inner.push(x, y) {
            return f64::NAN;
        }
        correl_beta(&self.inner.xs_values(), &self.inner.ys_values()).0
    }
}

/// Beta on percentage returns of two series (TA-Lib `statistic::beta`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaBETA {
    pub timeperiod: usize,
    prev0: Option<f64>,
    prev1: Option<f64>,
    rx: RingBuffer<f64>,
    ry: RingBuffer<f64>,
    sx: f64,
    sy: f64,
    sxx: f64,
    sxy: f64,
}

impl TaBETA {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            prev0: None,
            prev1: None,
            rx: RingBuffer::with_capacity(timeperiod),
            ry: RingBuffer::with_capacity(timeperiod),
            sx: 0.0,
            sy: 0.0,
            sxx: 0.0,
            sxy: 0.0,
        }
    }

    fn push_return(&mut self, x: f64, y: f64) -> Option<f64> {
        let p = self.timeperiod;
        if self.rx.len() >= p
            && let (Some(ox), Some(oy)) = (self.rx.pop_front(), self.ry.pop_front())
        {
            self.sx -= ox;
            self.sy -= oy;
            self.sxx -= ox * ox;
            self.sxy -= ox * oy;
        }
        self.rx.push_back(x);
        self.ry.push_back(y);
        self.sx += x;
        self.sy += y;
        self.sxx += x * x;
        self.sxy += x * y;
        if self.rx.len() < p {
            return None;
        }
        let n = p as f64;
        let denom = n * self.sxx - self.sx * self.sx;
        Some(if denom > 0.0 {
            (n * self.sxy - self.sx * self.sy) / denom
        } else {
            0.0
        })
    }
}

impl Next<(f64, f64)> for TaBETA {
    type Output = f64;

    fn next(&mut self, (v0, v1): (f64, f64)) -> Self::Output {
        let (Some(p0), Some(p1)) = (self.prev0, self.prev1) else {
            self.prev0 = Some(v0);
            self.prev1 = Some(v1);
            return f64::NAN;
        };
        self.prev0 = Some(v0);
        self.prev1 = Some(v1);
        if p0 == 0.0 || p1 == 0.0 {
            return f64::NAN;
        }
        let rx = (v0 - p0) / p0;
        let ry = (v1 - p1) / p1;
        self.push_return(rx, ry).unwrap_or(f64::NAN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_ta_beta_parity(
            x in prop::collection::vec(1.0..100.0, 15..100),
            y in prop::collection::vec(1.0..100.0, 15..100)
        ) {
            let len = x.len().min(y.len());
            let period = 14;
            let mut ind = TaBETA::new(period);
            let streaming: Vec<f64> = (0..len).map(|i| ind.next((x[i], y[i]))).collect();
            let batch = talib_rs::statistic::beta(&x[..len], &y[..len], period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else if !b.is_nan() { approx::assert_relative_eq!(s, b, epsilon = 1e-5); }
            }
        }
        #[test]
        fn test_ta_stddev_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let nbdev = 1.0;
            let mut s = TaSTDDEV::new(period, nbdev);
            let streaming: Vec<f64> = input.iter().map(|&x| s.next(x)).collect();
            let batch = talib_rs::statistic::stddev(&input, period, nbdev)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (a, b) in streaming.iter().zip(batch.iter()) {
                if a.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(a, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_ta_linearreg_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut s = TaLINEARREG::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| s.next(x)).collect();
            let batch = talib_rs::statistic::linearreg(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (a, b) in streaming.iter().zip(batch.iter()) {
                if a.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(a, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_ta_correl_parity(
            x in prop::collection::vec(0.1..100.0, 1..100),
            y in prop::collection::vec(0.1..100.0, 1..100)
        ) {
            let len = x.len().min(y.len());
            let period = 10;
            let mut s = TaCORREL::new(period);
            let streaming: Vec<f64> = (0..len).map(|i| s.next((x[i], y[i]))).collect();
            let batch = talib_rs::statistic::correl(&x[..len], &y[..len], period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (a, b) in streaming.iter().zip(batch.iter()) {
                if a.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(a, b, epsilon = 1e-6); }
            }
        }
    }
}
