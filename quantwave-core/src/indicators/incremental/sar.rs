//! Native O(1) Parabolic SAR and SAREXT — TA-Lib parity.

use crate::traits::Next;

/// Parabolic SAR — matches `talib_rs::overlap::sar`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SAR {
    pub acceleration: f64,
    pub maximum: f64,
    bar: usize,
    is_long: bool,
    sar_val: f64,
    ep: f64,
    af: f64,
    prev_low: f64,
    prev_high: f64,
    low0: f64,
    high0: f64,
    high1: f64,
    low1: f64,
}

impl SAR {
    pub fn new(acceleration: f64, maximum: f64) -> Self {
        Self {
            acceleration,
            maximum,
            bar: 0,
            is_long: false,
            sar_val: 0.0,
            ep: 0.0,
            af: acceleration,
            prev_low: 0.0,
            prev_high: 0.0,
            low0: 0.0,
            high0: 0.0,
            high1: 0.0,
            low1: 0.0,
        }
    }

    fn init_direction(&self) -> bool {
        let diff_m = self.low0 - self.low1;
        let diff_p = self.high1 - self.high0;
        !(diff_m > 0.0 && diff_m > diff_p)
    }
}

impl Next<(f64, f64)> for SAR {
    type Output = f64;

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        let i = self.bar;
        self.bar += 1;

        if i == 0 {
            self.high0 = high;
            self.low0 = low;
            return f64::NAN;
        }

        if i == 1 {
            self.high1 = high;
            self.low1 = low;
            self.is_long = self.init_direction();
            self.af = self.acceleration;
            if self.is_long {
                self.ep = high;
                self.sar_val = self.low0;
            } else {
                self.ep = low;
                self.sar_val = self.high0;
            }
            let out = self.step_bar1(high, low);
            self.prev_low = low;
            self.prev_high = high;
            return out;
        }

        let p_low = self.prev_low;
        let p_high = self.prev_high;
        self.prev_low = low;
        self.prev_high = high;
        self.step_main(high, low, p_low, p_high)
    }
}

impl SAR {
    fn step_bar1(&mut self, new_high: f64, new_low: f64) -> f64 {
        let p_low = new_low;
        let p_high = new_high;

        if self.is_long {
            if new_low <= self.sar_val {
                self.is_long = false;
                self.sar_val = self.ep;
                self.sar_val = self.sar_val.max(p_high).max(new_high);
                let out = self.sar_val;
                self.af = self.acceleration;
                self.ep = new_low;
                self.sar_val += self.af * (self.ep - self.sar_val);
                self.sar_val = self.sar_val.max(p_high).max(new_high);
                out
            } else {
                let out = self.sar_val;
                if new_high > self.ep {
                    self.ep = new_high;
                    self.af = (self.af + self.acceleration).min(self.maximum);
                }
                self.sar_val += self.af * (self.ep - self.sar_val);
                self.sar_val = self.sar_val.min(p_low).min(new_low);
                out
            }
        } else if new_high >= self.sar_val {
            self.is_long = true;
            self.sar_val = self.ep;
            self.sar_val = self.sar_val.min(p_low).min(new_low);
            let out = self.sar_val;
            self.af = self.acceleration;
            self.ep = new_high;
            self.sar_val += self.af * (self.ep - self.sar_val);
            self.sar_val = self.sar_val.min(p_low).min(new_low);
            out
        } else {
            let out = self.sar_val;
            if new_low < self.ep {
                self.ep = new_low;
                self.af = (self.af + self.acceleration).min(self.maximum);
            }
            self.sar_val += self.af * (self.ep - self.sar_val);
            self.sar_val = self.sar_val.max(p_high).max(new_high);
            out
        }
    }

    fn step_main(&mut self, new_high: f64, new_low: f64, p_low: f64, p_high: f64) -> f64 {
        if self.is_long {
            if new_low <= self.sar_val {
                self.is_long = false;
                self.sar_val = self.ep;
                self.sar_val = self.sar_val.max(p_high).max(new_high);
                let out = self.sar_val;
                self.af = self.acceleration;
                self.ep = new_low;
                self.sar_val += self.af * (self.ep - self.sar_val);
                self.sar_val = self.sar_val.max(p_high).max(new_high);
                out
            } else {
                let out = self.sar_val;
                if new_high > self.ep {
                    self.ep = new_high;
                    self.af = (self.af + self.acceleration).min(self.maximum);
                }
                self.sar_val += self.af * (self.ep - self.sar_val);
                self.sar_val = self.sar_val.min(p_low).min(new_low);
                out
            }
        } else if new_high >= self.sar_val {
            self.is_long = true;
            self.sar_val = self.ep;
            self.sar_val = self.sar_val.min(p_low).min(new_low);
            let out = self.sar_val;
            self.af = self.acceleration;
            self.ep = new_high;
            self.sar_val += self.af * (self.ep - self.sar_val);
            self.sar_val = self.sar_val.min(p_low).min(new_low);
            out
        } else {
            let out = self.sar_val;
            if new_low < self.ep {
                self.ep = new_low;
                self.af = (self.af + self.acceleration).min(self.maximum);
            }
            self.sar_val += self.af * (self.ep - self.sar_val);
            self.sar_val = self.sar_val.max(p_high).max(new_high);
            out
        }
    }
}

/// Parabolic SAR Extended — matches `talib_rs::overlap::sar_ext`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SAREXT {
    pub startvalue: f64,
    pub offsetonreverse: f64,
    pub accelerationinitlong: f64,
    pub accelerationlong: f64,
    pub accelerationmaxlong: f64,
    pub accelerationinitshort: f64,
    pub accelerationshort: f64,
    pub accelerationmaxshort: f64,
    bar: usize,
    is_long: bool,
    sar_val: f64,
    ep: f64,
    af_long: f64,
    af_short: f64,
    prev_low: f64,
    prev_high: f64,
    low0: f64,
    high0: f64,
    high1: f64,
    low1: f64,
}

impl SAREXT {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        startvalue: f64,
        offsetonreverse: f64,
        accelerationinitlong: f64,
        accelerationlong: f64,
        accelerationmaxlong: f64,
        accelerationinitshort: f64,
        accelerationshort: f64,
        accelerationmaxshort: f64,
    ) -> Self {
        Self {
            startvalue,
            offsetonreverse,
            accelerationinitlong,
            accelerationlong,
            accelerationmaxlong,
            accelerationinitshort,
            accelerationshort,
            accelerationmaxshort,
            bar: 0,
            is_long: false,
            sar_val: 0.0,
            ep: 0.0,
            af_long: accelerationinitlong,
            af_short: accelerationinitshort,
            prev_low: 0.0,
            prev_high: 0.0,
            low0: 0.0,
            high0: 0.0,
            high1: 0.0,
            low1: 0.0,
        }
    }

    fn init_direction(&self) -> bool {
        let diff_m = self.low0 - self.low1;
        let diff_p = self.high1 - self.high0;
        !(diff_m > 0.0 && diff_m > diff_p)
    }

    fn init_state(&mut self) {
        if self.startvalue == 0.0 {
            self.is_long = self.init_direction();
            if self.is_long {
                self.ep = self.high1;
                self.sar_val = self.low0;
            } else {
                self.ep = self.low1;
                self.sar_val = self.high0;
            }
        } else if self.startvalue > 0.0 {
            self.is_long = true;
            self.ep = self.high1;
            self.sar_val = self.startvalue;
        } else {
            self.is_long = false;
            self.ep = self.low1;
            self.sar_val = self.startvalue.abs();
        }
        self.af_long = self.accelerationinitlong;
        self.af_short = self.accelerationinitshort;
    }
}

impl Next<(f64, f64)> for SAREXT {
    type Output = f64;

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        let i = self.bar;
        self.bar += 1;

        if i == 0 {
            self.high0 = high;
            self.low0 = low;
            return f64::NAN;
        }

        if i == 1 {
            self.high1 = high;
            self.low1 = low;
            self.init_state();
            let out = self.step_bar1(high, low);
            self.prev_low = low;
            self.prev_high = high;
            return out;
        }

        let p_low = self.prev_low;
        let p_high = self.prev_high;
        self.prev_low = low;
        self.prev_high = high;
        self.step_main(high, low, p_low, p_high)
    }
}

impl SAREXT {
    fn step_bar1(&mut self, new_high: f64, new_low: f64) -> f64 {
        let p_low = new_low;
        let p_high = new_high;

        if self.is_long {
            if new_low <= self.sar_val {
                self.is_long = false;
                self.sar_val = self.ep;
                self.sar_val = self.sar_val.max(p_high).max(new_high);
                if self.offsetonreverse != 0.0 {
                    self.sar_val += self.sar_val * self.offsetonreverse;
                }
                let out = -self.sar_val;
                self.af_short = self.accelerationinitshort;
                self.ep = new_low;
                self.sar_val += self.af_short * (self.ep - self.sar_val);
                self.sar_val = self.sar_val.max(p_high).max(new_high);
                out
            } else {
                let out = self.sar_val;
                if new_high > self.ep {
                    self.ep = new_high;
                    self.af_long =
                        (self.af_long + self.accelerationlong).min(self.accelerationmaxlong);
                }
                self.sar_val += self.af_long * (self.ep - self.sar_val);
                self.sar_val = self.sar_val.min(p_low).min(new_low);
                out
            }
        } else if new_high >= self.sar_val {
            self.is_long = true;
            self.sar_val = self.ep;
            self.sar_val = self.sar_val.min(p_low).min(new_low);
            if self.offsetonreverse != 0.0 {
                self.sar_val -= self.sar_val * self.offsetonreverse;
            }
            let out = self.sar_val;
            self.af_long = self.accelerationinitlong;
            self.ep = new_high;
            self.sar_val += self.af_long * (self.ep - self.sar_val);
            self.sar_val = self.sar_val.min(p_low).min(new_low);
            out
        } else {
            let out = -self.sar_val;
            if new_low < self.ep {
                self.ep = new_low;
                self.af_short =
                    (self.af_short + self.accelerationshort).min(self.accelerationmaxshort);
            }
            self.sar_val += self.af_short * (self.ep - self.sar_val);
            self.sar_val = self.sar_val.max(p_high).max(new_high);
            out
        }
    }

    fn step_main(&mut self, new_high: f64, new_low: f64, p_low: f64, p_high: f64) -> f64 {
        if self.is_long {
            if new_low <= self.sar_val {
                self.is_long = false;
                self.sar_val = self.ep;
                self.sar_val = self.sar_val.max(p_high).max(new_high);
                if self.offsetonreverse != 0.0 {
                    self.sar_val += self.sar_val * self.offsetonreverse;
                }
                let out = -self.sar_val;
                self.af_short = self.accelerationinitshort;
                self.ep = new_low;
                self.sar_val += self.af_short * (self.ep - self.sar_val);
                self.sar_val = self.sar_val.max(p_high).max(new_high);
                out
            } else {
                let out = self.sar_val;
                if new_high > self.ep {
                    self.ep = new_high;
                    self.af_long =
                        (self.af_long + self.accelerationlong).min(self.accelerationmaxlong);
                }
                self.sar_val += self.af_long * (self.ep - self.sar_val);
                self.sar_val = self.sar_val.min(p_low).min(new_low);
                out
            }
        } else if new_high >= self.sar_val {
            self.is_long = true;
            self.sar_val = self.ep;
            self.sar_val = self.sar_val.min(p_low).min(new_low);
            if self.offsetonreverse != 0.0 {
                self.sar_val -= self.sar_val * self.offsetonreverse;
            }
            let out = self.sar_val;
            self.af_long = self.accelerationinitlong;
            self.ep = new_high;
            self.sar_val += self.af_long * (self.ep - self.sar_val);
            self.sar_val = self.sar_val.min(p_low).min(new_low);
            out
        } else {
            let out = -self.sar_val;
            if new_low < self.ep {
                self.ep = new_low;
                self.af_short =
                    (self.af_short + self.accelerationshort).min(self.accelerationmaxshort);
            }
            self.sar_val += self.af_short * (self.ep - self.sar_val);
            self.sar_val = self.sar_val.max(p_high).max(new_high);
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_sar_parity(
            h in prop::collection::vec(10.0..100.0, 2..100),
            l in prop::collection::vec(10.0..100.0, 2..100)
        ) {
            let len = h.len().min(l.len());
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            for i in 0..len {
                let hi: f64 = h[i];
                let lo: f64 = l[i];
                high.push(hi.max(lo));
                low.push(hi.min(lo));
            }
            let accel = 0.02;
            let max = 0.2;
            let mut sar = SAR::new(accel, max);
            let streaming: Vec<f64> = (0..len).map(|i| sar.next((high[i], low[i]))).collect();
            let batch = talib_rs::overlap::sar(&high, &low, accel, max)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}
