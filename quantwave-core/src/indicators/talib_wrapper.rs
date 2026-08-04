//! Macros for the macro-generated, TA-Lib-compatible streaming indicators.
//! Struct names intentionally mirror TA-Lib identifiers (`CDLDOJI`, `HT_SINE`, …).
//!
//! Everything here is native. The `native_cdl!` macro that wrapped
//! `talib_rs::pattern::*` at runtime, and the eleven never-invoked `talib_*`
//! macros beside it, were removed once all 60 candlestick patterns were ported
//! to native streaming implementations under `indicators::patterns`.
#![allow(non_camel_case_types)]

/// O(1) unary pointwise transform — no history buffer.
#[macro_export]
macro_rules! native_pointwise_1 {
    ($name:ident, $op:expr) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name;

        impl $name {
            pub fn new() -> Self {
                Self
            }
        }

        impl $crate::traits::Next<f64> for $name {
            type Output = f64;

            fn next(&mut self, input: f64) -> Self::Output {
                ($op)(input)
            }

            fn next_batch(&mut self, inputs: &[f64]) -> Vec<Self::Output>
            where
                f64: Copy,
            {
                inputs.iter().map(|&x| ($op)(x)).collect()
            }
        }
    };
}

/// O(1) binary element-wise operator.
#[macro_export]
macro_rules! native_binary_2 {
    ($name:ident, $op:expr) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name;

        impl $name {
            pub fn new() -> Self {
                Self
            }
        }

        impl $crate::traits::Next<(f64, f64)> for $name {
            type Output = f64;

            fn next(&mut self, (a, b): (f64, f64)) -> Self::Output {
                ($op)(a, b)
            }

            fn next_batch(&mut self, inputs: &[(f64, f64)]) -> Vec<Self::Output>
            where
                (f64, f64): Copy,
            {
                inputs.iter().map(|&(a, b)| ($op)(a, b)).collect()
            }
        }
    };
}
