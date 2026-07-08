use crate::utils::RingBuffer as VecDeque;

native_pointwise_1!(ACOS, f64::acos);
native_pointwise_1!(ASIN, f64::asin);
native_pointwise_1!(ATAN, f64::atan);
native_pointwise_1!(CEIL, f64::ceil);
native_pointwise_1!(COS, f64::cos);
native_pointwise_1!(COSH, f64::cosh);
native_pointwise_1!(EXP, f64::exp);
native_pointwise_1!(FLOOR, f64::floor);
native_pointwise_1!(LN, f64::ln);
native_pointwise_1!(LOG10, f64::log10);
native_pointwise_1!(SIN, f64::sin);
native_pointwise_1!(SINH, f64::sinh);
native_pointwise_1!(SQRT, f64::sqrt);
native_pointwise_1!(TAN, f64::tan);
native_pointwise_1!(TANH, f64::tanh);

/// Root Mean Square (RMS)
#[derive(Debug, Clone)]
pub struct RMS {
    period: usize,
    history: VecDeque<f64>,
    sum_sq: f64,
}

impl RMS {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            history: VecDeque::with_capacity(period),
            sum_sq: 0.0,
        }
    }
}

impl crate::traits::Next<f64> for RMS {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let input_sq = input * input;
        self.sum_sq += input_sq;
        self.history.push_back(input_sq);

        if self.history.len() > self.period
            && let Some(old) = self.history.pop_front()
        {
            self.sum_sq -= old;
        }

        if self.history.is_empty() {
            0.0
        } else {
            (self.sum_sq / self.history.len() as f64).sqrt()
        }
    }
}

/// Automatic Gain Control (AGC)
///
/// Normalizes a signal based on its decaying peak value.
/// Commonly used in John Ehlers' oscillators to keep the signal within [-1, 1].
#[derive(Debug, Clone)]
pub struct AGC {
    peak: f64,
    decay: f64,
}

impl AGC {
    pub fn new(decay: f64) -> Self {
        Self {
            peak: 0.0000001,
            decay,
        }
    }
}

impl crate::traits::Next<f64> for AGC {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.peak *= self.decay;
        let abs_input = input.abs();
        if abs_input > self.peak {
            self.peak = abs_input;
        }

        if self.peak != 0.0 {
            input / self.peak
        } else {
            0.0
        }
    }
}

native_binary_2!(ADD, |a, b| a + b);
native_binary_2!(SUB, |a, b| a - b);
native_binary_2!(MULT, |a, b| a * b);
native_binary_2!(DIV, |a, b| a / b);

pub use crate::indicators::incremental::rolling::{MAX, MAXINDEX, MIN, MININDEX, SUM};
impl From<usize> for MAX {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
impl From<usize> for MAXINDEX {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
impl From<usize> for MIN {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
impl From<usize> for MININDEX {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
impl From<usize> for SUM {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_sqrt_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let mut sqrt = SQRT::new();
            let streaming_results: Vec<f64> = input.iter().map(|&x| sqrt.next(x)).collect();
            let batch_results = talib_rs::math_transform::sqrt(&input);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_add_parity(
            in1 in prop::collection::vec(0.1..100.0, 1..100),
            in2 in prop::collection::vec(0.1..100.0, 1..100)
        ) {
            let len = in1.len().min(in2.len());
            if len == 0 { return Ok(()); }

            let mut add = ADD::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| add.next((in1[i], in2[i]))).collect();
            let batch_results = talib_rs::math_operator::add(&in1[..len], &in2[..len]).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_rms_parity(input in prop::collection::vec(0.1..100.0, 10..100)) {
            let period = 10;
            let mut rms = RMS::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| rms.next(x)).collect();

            let mut batch_results = Vec::with_capacity(input.len());
            for i in 0..input.len() {
                let start = (i + 1).saturating_sub(period);
                let window = &input[start..i+1];
                let sum_sq: f64 = window.iter().map(|&x| x*x).sum();
                batch_results.push((sum_sq / window.len() as f64).sqrt());
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
