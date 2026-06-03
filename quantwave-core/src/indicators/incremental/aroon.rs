//! Native streaming Aroon — TA-Lib parity (`talib_rs::momentum::aroon`, `aroon_osc`).

use crate::traits::Next;

#[derive(Debug, Clone)]
struct AroonMaxTracker {
    timeperiod: usize,
    inv_period: f64,
    data: Vec<f64>,
    highest: f64,
    highest_idx: usize,
    trailing_idx: usize,
}

impl AroonMaxTracker {
    fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            inv_period: 100.0 / timeperiod as f64,
            data: Vec::new(),
            highest: f64::NEG_INFINITY,
            highest_idx: 0,
            trailing_idx: 0,
        }
    }

    fn next(&mut self, value: f64) -> f64 {
        let timeperiod = self.timeperiod;
        if timeperiod < 2 {
            return f64::NAN;
        }
        self.data.push(value);
        let today = self.data.len() - 1;

        if today < timeperiod {
            if today == 0 {
                self.highest = value;
                self.highest_idx = 0;
            } else if value >= self.highest {
                self.highest = value;
                self.highest_idx = today;
            }
            return f64::NAN;
        }

        if today == timeperiod {
            if value >= self.highest {
                self.highest = value;
                self.highest_idx = today;
            }
            self.trailing_idx = 1;
            return (timeperiod - (timeperiod - self.highest_idx)) as f64 * self.inv_period;
        }

        let data = &self.data;
        if self.highest_idx < self.trailing_idx {
            self.highest_idx = self.trailing_idx;
            self.highest = data[self.trailing_idx];
            for (j, &val) in data[self.trailing_idx + 1..=today].iter().enumerate() {
                if val >= self.highest {
                    self.highest = val;
                    self.highest_idx = self.trailing_idx + 1 + j;
                }
            }
        } else if value >= self.highest {
            self.highest_idx = today;
            self.highest = value;
        }

        let out = (timeperiod - (today - self.highest_idx)) as f64 * self.inv_period;
        self.trailing_idx += 1;
        out
    }
}

#[derive(Debug, Clone)]
struct AroonMinTracker {
    timeperiod: usize,
    inv_period: f64,
    data: Vec<f64>,
    lowest: f64,
    lowest_idx: usize,
    trailing_idx: usize,
}

impl AroonMinTracker {
    fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            inv_period: 100.0 / timeperiod as f64,
            data: Vec::new(),
            lowest: f64::INFINITY,
            lowest_idx: 0,
            trailing_idx: 0,
        }
    }

    fn next(&mut self, value: f64) -> f64 {
        let timeperiod = self.timeperiod;
        if timeperiod < 2 {
            return f64::NAN;
        }
        self.data.push(value);
        let today = self.data.len() - 1;

        if today < timeperiod {
            if today == 0 {
                self.lowest = value;
                self.lowest_idx = 0;
            } else if value <= self.lowest {
                self.lowest = value;
                self.lowest_idx = today;
            }
            return f64::NAN;
        }

        if today == timeperiod {
            if value <= self.lowest {
                self.lowest = value;
                self.lowest_idx = today;
            }
            self.trailing_idx = 1;
            return (timeperiod - (timeperiod - self.lowest_idx)) as f64 * self.inv_period;
        }

        let data = &self.data;
        if self.lowest_idx < self.trailing_idx {
            self.lowest_idx = self.trailing_idx;
            self.lowest = data[self.trailing_idx];
            for (j, &val) in data[self.trailing_idx + 1..=today].iter().enumerate() {
                if val <= self.lowest {
                    self.lowest = val;
                    self.lowest_idx = self.trailing_idx + 1 + j;
                }
            }
        } else if value <= self.lowest {
            self.lowest_idx = today;
            self.lowest = value;
        }

        let out = (timeperiod - (today - self.lowest_idx)) as f64 * self.inv_period;
        self.trailing_idx += 1;
        out
    }
}

/// Aroon — returns `(aroon_down, aroon_up)` for `(high, low)`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct AROON {
    pub timeperiod: usize,
    up: AroonMaxTracker,
    down: AroonMinTracker,
}

impl AROON {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            up: AroonMaxTracker::new(timeperiod),
            down: AroonMinTracker::new(timeperiod),
        }
    }
}

impl Next<(f64, f64)> for AROON {
    type Output = (f64, f64);

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        let down = self.down.next(low);
        let up = self.up.next(high);
        (down, up)
    }
}

/// Aroon Oscillator — `aroon_up - aroon_down`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct AROONOSC {
    pub timeperiod: usize,
    high: Vec<f64>,
    low: Vec<f64>,
    highest: f64,
    highest_idx: usize,
    lowest: f64,
    lowest_idx: usize,
    trailing_idx: usize,
    inv_period: f64,
}

impl AROONOSC {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            high: Vec::new(),
            low: Vec::new(),
            highest: f64::NEG_INFINITY,
            highest_idx: 0,
            lowest: f64::INFINITY,
            lowest_idx: 0,
            trailing_idx: 0,
            inv_period: 100.0 / timeperiod as f64,
        }
    }
}

impl Next<(f64, f64)> for AROONOSC {
    type Output = f64;

    fn next(&mut self, (h, l): (f64, f64)) -> Self::Output {
        let timeperiod = self.timeperiod;
        if timeperiod < 2 {
            return f64::NAN;
        }
        self.high.push(h);
        self.low.push(l);
        let today = self.high.len() - 1;

        if today < timeperiod {
            if today == 0 {
                self.highest = h;
                self.highest_idx = 0;
                self.lowest = l;
                self.lowest_idx = 0;
            } else {
                if h >= self.highest {
                    self.highest = h;
                    self.highest_idx = today;
                }
                if l <= self.lowest {
                    self.lowest = l;
                    self.lowest_idx = today;
                }
            }
            return f64::NAN;
        }

        if today == timeperiod {
            if h >= self.highest {
                self.highest = h;
                self.highest_idx = today;
            }
            if l <= self.lowest {
                self.lowest = l;
                self.lowest_idx = today;
            }
            let up = (timeperiod - (timeperiod - self.highest_idx)) as f64 * self.inv_period;
            let down = (timeperiod - (timeperiod - self.lowest_idx)) as f64 * self.inv_period;
            self.trailing_idx = 1;
            return up - down;
        }

        let high = &self.high;
        let low = &self.low;

        if self.highest_idx < self.trailing_idx {
            self.highest_idx = self.trailing_idx;
            self.highest = high[self.trailing_idx];
            for (j, &val) in high[self.trailing_idx + 1..=today].iter().enumerate() {
                if val >= self.highest {
                    self.highest = val;
                    self.highest_idx = self.trailing_idx + 1 + j;
                }
            }
        } else if h >= self.highest {
            self.highest_idx = today;
            self.highest = h;
        }

        if self.lowest_idx < self.trailing_idx {
            self.lowest_idx = self.trailing_idx;
            self.lowest = low[self.trailing_idx];
            for (j, &val) in low[self.trailing_idx + 1..=today].iter().enumerate() {
                if val <= self.lowest {
                    self.lowest = val;
                    self.lowest_idx = self.trailing_idx + 1 + j;
                }
            }
        } else if l <= self.lowest {
            self.lowest_idx = today;
            self.lowest = l;
        }

        let up = (timeperiod - (today - self.highest_idx)) as f64 * self.inv_period;
        let down = (timeperiod - (today - self.lowest_idx)) as f64 * self.inv_period;
        self.trailing_idx += 1;
        up - down
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_aroon_parity(
            h in prop::collection::vec(1.0..100.0, 10..100),
            l in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let len = h.len().min(l.len());
            let period = 14;
            let mut aroon = AROON::new(period);
            let streaming: Vec<_> =
                (0..len).map(|i| aroon.next((h[i], l[i]))).collect();
            let (b_down, b_up) = talib_rs::momentum::aroon(&h[..len], &l[..len], period)
                .unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len]));
            for (i, (s_down, s_up)) in streaming.iter().enumerate() {
                if !s_down.is_nan() && !b_down[i].is_nan() {
                    approx::assert_relative_eq!(*s_down, b_down[i], epsilon = 1e-6);
                }
                if !s_up.is_nan() && !b_up[i].is_nan() {
                    approx::assert_relative_eq!(*s_up, b_up[i], epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_aroonosc_parity(
            h_in in prop::collection::vec(1.0..100.0, 10..100),
            l_in in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let len = h_in.len().min(l_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                in1.push(h.max(l));
                in2.push(h.min(l));
            }
            let period = 14;
            let mut osc = AROONOSC::new(period);
            let streaming: Vec<f64> =
                (0..len).map(|i| osc.next((in1[i], in2[i]))).collect();
            let batch = talib_rs::momentum::aroon_osc(&in1, &in2, period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else if !b.is_nan() {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }
}