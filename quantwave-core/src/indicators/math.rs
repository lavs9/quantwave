// Math Transform
talib_1_in_1_out_no_result!(ACOS, talib_rs::math_transform::acos);
impl Default for ACOS { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(ASIN, talib_rs::math_transform::asin);
impl Default for ASIN { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(ATAN, talib_rs::math_transform::atan);
impl Default for ATAN { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(CEIL, talib_rs::math_transform::ceil);
impl Default for CEIL { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(COS, talib_rs::math_transform::cos);
impl Default for COS { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(COSH, talib_rs::math_transform::cosh);
impl Default for COSH { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(EXP, talib_rs::math_transform::exp);
impl Default for EXP { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(FLOOR, talib_rs::math_transform::floor);
impl Default for FLOOR { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(LN, talib_rs::math_transform::ln);
impl Default for LN { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(LOG10, talib_rs::math_transform::log10);
impl Default for LOG10 { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(SIN, talib_rs::math_transform::sin);
impl Default for SIN { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(SINH, talib_rs::math_transform::sinh);
impl Default for SINH { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(SQRT, talib_rs::math_transform::sqrt);
impl Default for SQRT { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(TAN, talib_rs::math_transform::tan);
impl Default for TAN { fn default() -> Self { Self::new() } }
talib_1_in_1_out_no_result!(TANH, talib_rs::math_transform::tanh);
impl Default for TANH { fn default() -> Self { Self::new() } }

// Math Operators
talib_2_in_1_out!(ADD, talib_rs::math_operator::add);
impl Default for ADD { fn default() -> Self { Self::new() } }
talib_2_in_1_out!(SUB, talib_rs::math_operator::sub);
impl Default for SUB { fn default() -> Self { Self::new() } }
talib_2_in_1_out!(MULT, talib_rs::math_operator::mult);
impl Default for MULT { fn default() -> Self { Self::new() } }
talib_2_in_1_out!(DIV, talib_rs::math_operator::div);
impl Default for DIV { fn default() -> Self { Self::new() } }
talib_1_in_1_out!(MAX, talib_rs::math_operator::max, timeperiod: usize);
impl From<usize> for MAX { fn from(p: usize) -> Self { Self::new(p) } }

talib_1_in_1_out!(MAXINDEX, talib_rs::math_operator::maxindex, timeperiod: usize);
impl From<usize> for MAXINDEX { fn from(p: usize) -> Self { Self::new(p) } }

talib_1_in_1_out!(MIN, talib_rs::math_operator::min, timeperiod: usize);
impl From<usize> for MIN { fn from(p: usize) -> Self { Self::new(p) } }

talib_1_in_1_out!(MININDEX, talib_rs::math_operator::minindex, timeperiod: usize);
impl From<usize> for MININDEX { fn from(p: usize) -> Self { Self::new(p) } }

talib_1_in_1_out!(SUM, talib_rs::math_operator::sum, timeperiod: usize);
impl From<usize> for SUM { fn from(p: usize) -> Self { Self::new(p) } }

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
    }
}
