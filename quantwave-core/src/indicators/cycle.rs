talib_1_in_1_out!(HT_DCPERIOD, talib_rs::cycle::ht_dcperiod);
impl Default for HT_DCPERIOD {
    fn default() -> Self {
        Self::new()
    }
}
talib_1_in_2_out!(HT_PHASOR, talib_rs::cycle::ht_phasor);
impl Default for HT_PHASOR {
    fn default() -> Self {
        Self::new()
    }
}
talib_1_in_1_out!(HT_DCPHASE, talib_rs::cycle::ht_dcphase);
impl Default for HT_DCPHASE {
    fn default() -> Self {
        Self::new()
    }
}
talib_1_in_2_out!(HT_SINE, talib_rs::cycle::ht_sine);
impl Default for HT_SINE {
    fn default() -> Self {
        Self::new()
    }
}
talib_1_in_1_out_i32!(HT_TRENDMODE, talib_rs::cycle::ht_trendmode);
impl Default for HT_TRENDMODE {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_ht_dcperiod_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let mut ht = HT_DCPERIOD::new();
            let streaming_results: Vec<f64> = input.iter().map(|&x| ht.next(x)).collect();
            let batch_results = talib_rs::cycle::ht_dcperiod(&input).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_ht_phasor_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let mut ht = HT_PHASOR::new();
            let streaming_results: Vec<(f64, f64)> = input.iter().map(|&x| ht.next(x)).collect();
            let (b_inphase, b_quadrature) = talib_rs::cycle::ht_phasor(&input).unwrap_or_else(|_| {
                (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
            });

            for (i, (s_in, s_quad)) in streaming_results.into_iter().enumerate() {
                if s_in.is_nan() {
                    assert!(b_inphase[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_in, b_inphase[i], epsilon = 1e-6);
                }
                if s_quad.is_nan() {
                    assert!(b_quadrature[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_quad, b_quadrature[i], epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_ht_trendmode_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let mut ht = HT_TRENDMODE::new();
            let streaming_results: Vec<f64> = input.iter().map(|&x| ht.next(x)).collect();
            let batch_results = talib_rs::cycle::ht_trendmode(&input).unwrap_or_else(|_| vec![0; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                assert_eq!(*s as i32, *b);
            }
        }
    }
}
