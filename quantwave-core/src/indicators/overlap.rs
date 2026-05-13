talib_1_in_1_out!(DEMA, talib_rs::overlap::dema, timeperiod: usize);
impl From<usize> for DEMA {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(TRIMA, talib_rs::overlap::trima, timeperiod: usize);
impl From<usize> for TRIMA {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(KAMA, talib_rs::overlap::kama, timeperiod: usize);
impl From<usize> for KAMA {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(T3, talib_rs::overlap::t3, timeperiod: usize, v_factor: f64);
talib_1_in_2_out!(MAMA, talib_rs::overlap::mama, fastlimit: f64, slowlimit: f64);
talib_1_in_3_out!(BBANDS, talib_rs::overlap::bbands, timeperiod: usize, nbdevup: f64, nbdevdn: f64, matype: talib_rs::MaType);
talib_2_in_1_out!(SAR, talib_rs::overlap::sar, acceleration: f64, maximum: f64);
talib_2_in_1_out!(SAREXT, talib_rs::overlap::sar_ext, startvalue: f64, offsetonreverse: f64, accelerationinitlong: f64, accelerationlong: f64, accelerationmaxlong: f64, accelerationinitshort: f64, accelerationshort: f64, accelerationmaxshort: f64);
talib_1_in_1_out!(MIDPOINT, talib_rs::overlap::midpoint, timeperiod: usize);
impl From<usize> for MIDPOINT {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_2_in_1_out!(MIDPRICE, talib_rs::overlap::midprice, timeperiod: usize);
impl From<usize> for MIDPRICE {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_2_in_1_out!(MAVP, talib_rs::overlap::mavp, minperiod: usize, maxperiod: usize, matype: talib_rs::MaType);
talib_1_in_1_out!(HT_TRENDLINE, talib_rs::overlap::ht_trendline);
impl Default for HT_TRENDLINE {
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
        fn test_dema_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut dema = DEMA::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| dema.next(x)).collect();
            let batch_results = talib_rs::overlap::dema(&input, period).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_trima_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut trima = TRIMA::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| trima.next(x)).collect();
            let batch_results = talib_rs::overlap::trima(&input, period).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_kama_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut kama = KAMA::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| kama.next(x)).collect();
            let batch_results = talib_rs::overlap::kama(&input, period).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_t3_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let v_factor = 0.7;
            let mut t3 = T3::new(period, v_factor);
            let streaming_results: Vec<f64> = input.iter().map(|&x| t3.next(x)).collect();
            let batch_results = talib_rs::overlap::t3(&input, period, v_factor).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_bbands_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let nbdevup = 2.0;
            let nbdevdn = 2.0;
            let matype = talib_rs::MaType::Sma;
            let mut bbands = BBANDS::new(period, nbdevup, nbdevdn, matype);
            let streaming_results: Vec<(f64, f64, f64)> = input.iter().map(|&x| bbands.next(x)).collect();
            let (b_upper, b_middle, b_lower) = talib_rs::overlap::bbands(&input, period, nbdevup, nbdevdn, matype).unwrap_or_else(|_| {
                (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
            });

            for (i, (s_upper, s_middle, s_lower)) in streaming_results.into_iter().enumerate() {
                if s_upper.is_nan() {
                    assert!(b_upper[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_upper, b_upper[i], epsilon = 1e-6);
                }
                if s_middle.is_nan() {
                    assert!(b_middle[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_middle, b_middle[i], epsilon = 1e-6);
                }
                if s_lower.is_nan() {
                    assert!(b_lower[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_lower, b_lower[i], epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_sar_parity(
            h in prop::collection::vec(10.0..100.0, 1..100),
            l in prop::collection::vec(10.0..100.0, 1..100)
        ) {
            let len = h.len().min(l.len());
            if len == 0 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            for i in 0..len {
                let v_h: f64 = h[i];
                let v_l: f64 = l[i];
                high.push(v_h.max(v_l));
                low.push(v_h.min(v_l));
            }

            let accel = 0.02;
            let max = 0.2;
            let mut sar = SAR::new(accel, max);
            let streaming_results: Vec<f64> = (0..len).map(|i| sar.next((high[i], low[i]))).collect();
            let batch_results = talib_rs::overlap::sar(&high, &low, accel, max).unwrap_or_else(|_| vec![f64::NAN; len]);

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
