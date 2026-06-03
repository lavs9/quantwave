//! Native O(1) rolling MAX / MIN / SUM / MAXINDEX / MININDEX (fixed period).

use crate::traits::Next;
use crate::utils::RingBuffer;

#[derive(Debug, Clone)]
pub struct RollingMax {
    pub timeperiod: usize,
    window: RingBuffer<f64>,
}

impl RollingMax {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            window: RingBuffer::with_capacity(timeperiod),
        }
    }

    fn push(&mut self, v: f64) -> f64 {
        let p = self.timeperiod;
        if self.window.len() >= p {
            let _ = self.window.pop_front();
        }
        self.window.push_back(v);
        if self.window.len() < p {
            return f64::NAN;
        }
        let mut m = f64::NEG_INFINITY;
        for i in 0..self.window.len() {
            let x = *self.window.get(i).unwrap();
            if x > m {
                m = x;
            }
        }
        m
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MAX {
    pub timeperiod: usize,
    inner: RollingMax,
}

impl MAX {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            inner: RollingMax::new(timeperiod),
        }
    }
}

impl Next<f64> for MAX {
    type Output = f64;
    fn next(&mut self, input: f64) -> Self::Output {
        self.inner.push(input)
    }
}

#[derive(Debug, Clone)]
pub struct RollingMin {
    pub timeperiod: usize,
    window: RingBuffer<f64>,
}

impl RollingMin {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            window: RingBuffer::with_capacity(timeperiod),
        }
    }

    fn push(&mut self, v: f64) -> f64 {
        let p = self.timeperiod;
        if self.window.len() >= p {
            let _ = self.window.pop_front();
        }
        self.window.push_back(v);
        if self.window.len() < p {
            return f64::NAN;
        }
        let mut m = f64::INFINITY;
        for i in 0..self.window.len() {
            let x = *self.window.get(i).unwrap();
            if x < m {
                m = x;
            }
        }
        m
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MIN {
    pub timeperiod: usize,
    inner: RollingMin,
}

impl MIN {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            inner: RollingMin::new(timeperiod),
        }
    }
}

impl Next<f64> for MIN {
    type Output = f64;
    fn next(&mut self, input: f64) -> Self::Output {
        self.inner.push(input)
    }
}

#[derive(Debug, Clone)]
pub struct RollingSum {
    pub timeperiod: usize,
    window: RingBuffer<f64>,
    sum: f64,
}

impl RollingSum {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            window: RingBuffer::with_capacity(timeperiod),
            sum: 0.0,
        }
    }

    fn push(&mut self, v: f64) -> f64 {
        let p = self.timeperiod;
        if self.window.len() >= p {
            if let Some(old) = self.window.pop_front() {
                self.sum -= old;
            }
        }
        self.window.push_back(v);
        self.sum += v;
        if self.window.len() < p {
            return f64::NAN;
        }
        self.sum
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SUM {
    pub timeperiod: usize,
    inner: RollingSum,
}

impl SUM {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            inner: RollingSum::new(timeperiod),
        }
    }
}

impl Next<f64> for SUM {
    type Output = f64;
    fn next(&mut self, input: f64) -> Self::Output {
        self.inner.push(input)
    }
}

#[derive(Debug, Clone)]
pub struct RollingMaxIndex {
    pub timeperiod: usize,
    window: RingBuffer<f64>,
    bar_index: usize,
}

impl RollingMaxIndex {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            window: RingBuffer::with_capacity(timeperiod),
            bar_index: 0,
        }
    }

    fn push(&mut self, v: f64) -> f64 {
        let p = self.timeperiod;
        if self.window.len() >= p {
            let _ = self.window.pop_front();
        }
        self.window.push_back(v);
        self.bar_index += 1;
        if self.window.len() < p {
            return f64::NAN;
        }
        let mut best_idx = 0usize;
        let mut best_val = f64::NEG_INFINITY;
        for i in 0..self.window.len() {
            let x = *self.window.get(i).unwrap();
            if x > best_val {
                best_val = x;
                best_idx = i;
            }
        }
        (self.bar_index - p + best_idx) as f64
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MAXINDEX {
    pub timeperiod: usize,
    inner: RollingMaxIndex,
}

impl MAXINDEX {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            inner: RollingMaxIndex::new(timeperiod),
        }
    }
}

impl Next<f64> for MAXINDEX {
    type Output = f64;
    fn next(&mut self, input: f64) -> Self::Output {
        self.inner.push(input)
    }
}

#[derive(Debug, Clone)]
pub struct RollingMinIndex {
    pub timeperiod: usize,
    window: RingBuffer<f64>,
    bar_index: usize,
}

impl RollingMinIndex {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            window: RingBuffer::with_capacity(timeperiod),
            bar_index: 0,
        }
    }

    fn push(&mut self, v: f64) -> f64 {
        let p = self.timeperiod;
        if self.window.len() >= p {
            let _ = self.window.pop_front();
        }
        self.window.push_back(v);
        self.bar_index += 1;
        if self.window.len() < p {
            return f64::NAN;
        }
        let mut best_idx = 0usize;
        let mut best_val = f64::INFINITY;
        for i in 0..self.window.len() {
            let x = *self.window.get(i).unwrap();
            if x < best_val {
                best_val = x;
                best_idx = i;
            }
        }
        (self.bar_index - p + best_idx) as f64
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MININDEX {
    pub timeperiod: usize,
    inner: RollingMinIndex,
}

impl MININDEX {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            inner: RollingMinIndex::new(timeperiod),
        }
    }
}

impl Next<f64> for MININDEX {
    type Output = f64;
    fn next(&mut self, input: f64) -> Self::Output {
        self.inner.push(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_max_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut max = MAX::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| max.next(x)).collect();
            let batch = talib_rs::math_operator::max(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}