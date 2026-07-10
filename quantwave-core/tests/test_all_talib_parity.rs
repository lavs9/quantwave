use proptest::prelude::*;
use quantwave_core::traits::Next;
use quantwave_core::*;

proptest! {

        #[test]
        fn test_macdext_parity_auto(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = input.len();
            if len == 0 { return Ok(()); }
            let fastperiod = 12;
            let fastmatype = talib_rs::MaType::Sma;
            let slowperiod = 26;
            let slowmatype = talib_rs::MaType::Sma;
            let signalperiod = 9;
            let signalmatype = talib_rs::MaType::Sma;
            let mut indicator = MACDEXT::new(fastperiod, fastmatype, slowperiod, slowmatype, signalperiod, signalmatype);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(input[i])).collect();
            let (b1, b2, b3) = talib_rs::momentum::macd_ext(&input, fastperiod, fastmatype, slowperiod, slowmatype, signalperiod, signalmatype).unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len], vec![f64::NAN; len]));

            for (i, (s1, s2, s3)) in streaming_results.into_iter().enumerate() {
                if s1.is_nan() {
                    assert!(b1[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s1, b1[i], epsilon = 1e-6);
                }
                if s2.is_nan() {
                    assert!(b2[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s2, b2[i], epsilon = 1e-6);
                }
                if s3.is_nan() {
                    assert!(b3[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s3, b3[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_macdfix_parity_auto(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = input.len();
            if len == 0 { return Ok(()); }
            let signalperiod = 9;
            let mut indicator = MACDFIX::new(signalperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(input[i])).collect();
            let (b1, b2, b3) = talib_rs::momentum::macd_fix(&input, signalperiod).unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len], vec![f64::NAN; len]));

            for (i, (s1, s2, s3)) in streaming_results.into_iter().enumerate() {
                if s1.is_nan() {
                    assert!(b1[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s1, b1[i], epsilon = 1e-6);
                }
                if s2.is_nan() {
                    assert!(b2[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s2, b2[i], epsilon = 1e-6);
                }
                if s3.is_nan() {
                    assert!(b3[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s3, b3[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_stochf_parity_auto(h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100), c_in in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = h_in.len().min(l_in.len()).min(c_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            let mut in3 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                let c: f64 = c_in[i];
                in1.push(h.max(l).max(c));
                in2.push(h.min(l).min(c));
                in3.push(c);
            }
            if len == 0 { return Ok(()); }
            let fastk_period = 5;
            let fastd_period = 3;
            let fastd_matype = talib_rs::MaType::Sma;
            let mut indicator = STOCHF::new(fastk_period, fastd_period, fastd_matype);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let (b1, b2) = talib_rs::momentum::stochf(&in1, &in2, &in3, fastk_period, fastd_period, fastd_matype).unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len]));

            for (i, (s1, s2)) in streaming_results.into_iter().enumerate() {
                if s1.is_nan() {
                    assert!(b1[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s1, b1[i], epsilon = 1e-6);
                }
                if s2.is_nan() {
                    assert!(b2[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s2, b2[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_stochrsi_parity_auto(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = input.len();
            if len == 0 { return Ok(()); }
            let timeperiod = 14;
            let fastk_period = 5;
            let fastd_period = 3;
            let fastd_matype = talib_rs::MaType::Sma;
            let mut indicator = STOCHRSI::new(timeperiod, fastk_period, fastd_period, fastd_matype);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(input[i])).collect();
            let (b1, b2) = talib_rs::momentum::stochrsi(&input, timeperiod, fastk_period, fastd_period, fastd_matype).unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len]));

            for (i, (s1, s2)) in streaming_results.into_iter().enumerate() {
                if s1.is_nan() {
                    assert!(b1[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s1, b1[i], epsilon = 1e-6);
                }
                if s2.is_nan() {
                    assert!(b2[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s2, b2[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_apo_parity_auto(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = input.len();
            if len == 0 { return Ok(()); }
            let fastperiod = 12;
            let slowperiod = 26;
            let matype = talib_rs::MaType::Sma;
            let mut indicator = APO::new(fastperiod, slowperiod, matype);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(input[i])).collect();
            let b_res = talib_rs::momentum::apo(&input, fastperiod, slowperiod, matype).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_ppo_parity_auto(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = input.len();
            if len == 0 { return Ok(()); }
            let fastperiod = 12;
            let slowperiod = 26;
            let matype = talib_rs::MaType::Sma;
            let mut indicator = PPO::new(fastperiod, slowperiod, matype);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(input[i])).collect();
            let b_res = talib_rs::momentum::ppo(&input, fastperiod, slowperiod, matype).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_bop_parity_auto(o_in in prop::collection::vec(1.0..100.0, 10..100), h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100), c_in in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = o_in.len().min(h_in.len()).min(l_in.len()).min(c_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            let mut in3 = Vec::with_capacity(len);
            let mut in4 = Vec::with_capacity(len);
            for i in 0..len {
                let o: f64 = o_in[i];
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                let c: f64 = c_in[i];
                in1.push(o);
                in2.push(o.max(h).max(l).max(c));
                in3.push(o.min(h).min(l).min(c));
                in4.push(c);
            }
            if len == 0 { return Ok(()); }

            let mut indicator = BOP::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::momentum::bop(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_aroonosc_parity_auto(h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = h_in.len().min(l_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                in1.push(h.max(l));
                in2.push(h.min(l));
            }
            if len == 0 { return Ok(()); }
            let timeperiod = 14;
            let mut indicator = AROONOSC::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::momentum::aroon_osc(&in1, &in2, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_mfi_parity_auto(o_in in prop::collection::vec(1.0..100.0, 10..100), h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100), c_in in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = o_in.len().min(h_in.len()).min(l_in.len()).min(c_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            let mut in3 = Vec::with_capacity(len);
            let mut in4 = Vec::with_capacity(len);
            for i in 0..len {
                let o: f64 = o_in[i];
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                let c: f64 = c_in[i];
                in1.push(o);
                in2.push(o.max(h).max(l).max(c));
                in3.push(o.min(h).min(l).min(c));
                in4.push(c);
            }
            if len == 0 { return Ok(()); }
            let timeperiod = 14;
            let mut indicator = MFI::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::momentum::mfi(&in1, &in2, &in3, &in4, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_ultosc_parity_auto(h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100), c_in in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = h_in.len().min(l_in.len()).min(c_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            let mut in3 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                let c: f64 = c_in[i];
                in1.push(h.max(l).max(c));
                in2.push(h.min(l).min(c));
                in3.push(c);
            }
            if len == 0 { return Ok(()); }
            let timeperiod1 = 7;
            let timeperiod2 = 14;
            let timeperiod3 = 28;
            let mut indicator = ULTOSC::new(timeperiod1, timeperiod2, timeperiod3);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::momentum::ultosc(&in1, &in2, &in3, timeperiod1, timeperiod2, timeperiod3).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_t3_parity_auto(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = input.len();
            if len == 0 { return Ok(()); }
            let timeperiod = 5;
            let v_factor = 0.7;
            let mut indicator = T3::new(timeperiod, v_factor);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(input[i])).collect();
            let b_res = talib_rs::overlap::t3(&input, timeperiod, v_factor).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_sar_parity_auto(h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = h_in.len().min(l_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                in1.push(h.max(l));
                in2.push(h.min(l));
            }
            if len == 0 { return Ok(()); }
            let acceleration = 0.02;
            let maximum = 0.2;
            let mut indicator = SAR::new(acceleration, maximum);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::overlap::sar(&in1, &in2, acceleration, maximum).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_sarext_parity_auto(h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = h_in.len().min(l_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                in1.push(h.max(l));
                in2.push(h.min(l));
            }
            if len == 0 { return Ok(()); }
            let startvalue = 0.0;
            let offsetonreverse = 0.0;
            let accelerationinitlong = 0.02;
            let accelerationlong = 0.02;
            let accelerationmaxlong = 0.2;
            let accelerationinitshort = 0.02;
            let accelerationshort = 0.02;
            let accelerationmaxshort = 0.2;
            let mut indicator = SAREXT::new(startvalue, offsetonreverse, accelerationinitlong, accelerationlong, accelerationmaxlong, accelerationinitshort, accelerationshort, accelerationmaxshort);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::overlap::sar_ext(&in1, &in2, startvalue, offsetonreverse, accelerationinitlong, accelerationlong, accelerationmaxlong, accelerationinitshort, accelerationshort, accelerationmaxshort).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_mavp_parity_auto(h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = h_in.len().min(l_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                in1.push(h.max(l));
                in2.push(h.min(l));
            }
            if len == 0 { return Ok(()); }
            let minperiod = 2;
            let maxperiod = 30;
            let matype = talib_rs::MaType::Sma;
            let mut indicator = MAVP::new(minperiod, maxperiod, matype);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::overlap::mavp(&in1, &in2, minperiod, maxperiod, matype).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_ht_phasor_parity_auto(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = input.len();
            if len == 0 { return Ok(()); }

            let mut indicator = HT_PHASOR::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(input[i])).collect();
            let (b1, b2) = talib_rs::cycle::ht_phasor(&input).unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len]));

            for (i, (s1, s2)) in streaming_results.into_iter().enumerate() {
                if s1.is_nan() {
                    assert!(b1[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s1, b1[i], epsilon = 1e-6);
                }
                if s2.is_nan() {
                    assert!(b2[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s2, b2[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_ht_sine_parity_auto(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = input.len();
            if len == 0 { return Ok(()); }

            let mut indicator = HT_SINE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(input[i])).collect();
            let (b1, b2) = talib_rs::cycle::ht_sine(&input).unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len]));

            for (i, (s1, s2)) in streaming_results.into_iter().enumerate() {
                if s1.is_nan() {
                    assert!(b1[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s1, b1[i], epsilon = 1e-6);
                }
                if s2.is_nan() {
                    assert!(b2[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s2, b2[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_plus_dm_parity_auto(h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = h_in.len().min(l_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                in1.push(h.max(l));
                in2.push(h.min(l));
            }
            if len == 0 { return Ok(()); }
            let timeperiod = 14;
            let mut indicator = PLUS_DM::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::momentum::plus_dm(&in1, &in2, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }

        #[test]
        fn test_minus_dm_parity_auto(h_in in prop::collection::vec(1.0..100.0, 10..100), l_in in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = h_in.len().min(l_in.len());
            let mut in1 = Vec::with_capacity(len);
            let mut in2 = Vec::with_capacity(len);
            for i in 0..len {
                let h: f64 = h_in[i];
                let l: f64 = l_in[i];
                in1.push(h.max(l));
                in2.push(h.min(l));
            }
            if len == 0 { return Ok(()); }
            let timeperiod = 14;
            let mut indicator = MINUS_DM::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::momentum::minus_dm(&in1, &in2, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    assert!(b_res[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-6);
                }
            }

        }
}
