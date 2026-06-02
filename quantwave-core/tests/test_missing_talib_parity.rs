use quantwave_core::*;
use quantwave_core::traits::Next;
use proptest::prelude::*;

proptest! {

        #[test]
        fn test_cdldoji_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLDOJI::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_doji(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlhammer_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLHAMMER::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_hammer(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlengulfing_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLENGULFING::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_engulfing(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlclosingmarubozu_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLCLOSINGMARUBOZU::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_closingmarubozu(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdldragonflydoji_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLDRAGONFLYDOJI::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_dragonflydoji(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlgravestonedoji_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLGRAVESTONEDOJI::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_gravestonedoji(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlhighwave_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLHIGHWAVE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_highwave(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdllongleggeddoji_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLLONGLEGGEDDOJI::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_longleggeddoji(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdllongline_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLLONGLINE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_longline(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlmarubozu_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLMARUBOZU::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_marubozu(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlrickshawman_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLRICKSHAWMAN::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_rickshawman(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlshortline_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLSHORTLINE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_shortline(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlspinningtop_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLSPINNINGTOP::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_spinningtop(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdltakuri_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLTAKURI::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_takuri(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdl2crows_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDL2CROWS::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_2crows(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlcounterattack_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLCOUNTERATTACK::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_counterattack(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdldarkcloudcover_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLDARKCLOUDCOVER::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_darkcloudcover(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdldojistar_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLDOJISTAR::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_dojistar(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlhangingman_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLHANGINGMAN::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_hangingman(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlharami_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLHARAMI::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_harami(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlharamicross_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLHARAMICROSS::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_haramicross(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlhikkake_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLHIKKAKE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_hikkake(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlhikkakemod_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLHIKKAKEMOD::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_hikkakemod(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlhomingpigeon_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLHOMINGPIGEON::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_homingpigeon(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlinneck_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLINNECK::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_inneck(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlinvertedhammer_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLINVERTEDHAMMER::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_invertedhammer(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlkicking_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLKICKING::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_kicking(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlkickingbylength_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLKICKINGBYLENGTH::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_kickingbylength(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlmatchinglow_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLMATCHINGLOW::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_matchinglow(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlonneck_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLONNECK::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_onneck(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlpiercing_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLPIERCING::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_piercing(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlseparatinglines_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLSEPARATINGLINES::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_separatinglines(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlshootingstar_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLSHOOTINGSTAR::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_shootingstar(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlsticksandwich_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLSTICKSANDWICH::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_sticksandwich(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlthrusting_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLTHRUSTING::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_thrusting(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlbelthold_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLBELTHOLD::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_belthold(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdl3blackcrows_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDL3BLACKCROWS::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_3blackcrows(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdl3inside_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDL3INSIDE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_3inside(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdl3linestrike_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDL3LINESTRIKE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_3linestrike(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdl3outside_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDL3OUTSIDE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_3outside(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdl3starsinsouth_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDL3STARSINSOUTH::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_3starsinsouth(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdl3whitesoldiers_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDL3WHITESOLDIERS::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_3whitesoldiers(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlabandonedbaby_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLABANDONEDBABY::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_abandonedbaby(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdladvanceblock_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLADVANCEBLOCK::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_advanceblock(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlbreakaway_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLBREAKAWAY::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_breakaway(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlconcealbabyswall_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLCONCEALBABYSWALL::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_concealbabyswall(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdleveningdojistar_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLEVENINGDOJISTAR::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_eveningdojistar(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdleveningstar_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLEVENINGSTAR::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_eveningstar(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlgapsidesidewhite_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLGAPSIDESIDEWHITE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_gapsidesidewhite(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlidentical3crows_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLIDENTICAL3CROWS::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_identical3crows(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlladderbottom_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLLADDERBOTTOM::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_ladderbottom(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlmathold_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLMATHOLD::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_mathold(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlmorningdojistar_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLMORNINGDOJISTAR::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_morningdojistar(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlmorningstar_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLMORNINGSTAR::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_morningstar(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlrisefall3methods_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLRISEFALL3METHODS::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_risefall3methods(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlstalledpattern_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLSTALLEDPATTERN::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_stalledpattern(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdltasukigap_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLTASUKIGAP::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_tasukigap(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdltristar_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLTRISTAR::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_tristar(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlunique3river_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLUNIQUE3RIVER::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_unique3river(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlupsidegap2crows_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLUPSIDEGAP2CROWS::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_upsidegap2crows(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_cdlxsidegap3methods_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let mut indicator = CDLXSIDEGAP3METHODS::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::pattern::cdl_xsidegap3methods(&in1, &in2, &in3, &in4).unwrap_or_else(|_| vec![0; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res as i32 != 0 && b_res[i] != 0 { assert_eq!(s_res as i32, b_res[i]); }
            }
        }

        #[test]
        fn test_ht_dcphase_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = HT_DCPHASE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::cycle::ht_dcphase(&in1).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_taatr_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let timeperiod = 14;
            let mut indicator = TaATR::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::volatility::atr(&in1, &in2, &in3, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_tanatr_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let timeperiod = 14;
            let mut indicator = TaNATR::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::volatility::natr(&in1, &in2, &in3, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_tatrange_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut indicator = TaTRANGE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::volatility::trange(&in1, &in2, &in3).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_adxr_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let timeperiod = 14;
            let mut indicator = ADXR::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::momentum::adxr(&in1, &in2, &in3, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_cci_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let timeperiod = 14;
            let mut indicator = CCI::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::momentum::cci(&in1, &in2, &in3, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_mom_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = MOM::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::momentum::mom(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_roc_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = ROC::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::momentum::roc(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_rocp_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = ROCP::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::momentum::rocp(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_rocr_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = ROCR::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::momentum::rocr(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_rocr100_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = ROCR100::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::momentum::rocr100(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_willr_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let timeperiod = 14;
            let mut indicator = WILLR::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::momentum::willr(&in1, &in2, &in3, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_cmo_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = CMO::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::momentum::cmo(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_aroon_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let timeperiod = 14;
            let mut indicator = AROON::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let (b1, b2) = talib_rs::momentum::aroon(&in1, &in2, timeperiod).unwrap_or_else(|_| (vec![f64::NAN; len], vec![f64::NAN; len]));
            for (i, (s1, s2)) in streaming_results.into_iter().enumerate() {
                if s1.is_nan() {
                    // ignore prefix
                } else {
                    if b1[i].is_nan() || b1[i] == 0.0 || b1[i] == -1.0 || b1[i] == -2e30 || b1[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s1, b1[i], epsilon = 1e-5);
                    }
                }

                if s2.is_nan() {
                    // ignore prefix
                } else {
                    if b2[i].is_nan() || b2[i] == 0.0 || b2[i] == -1.0 || b2[i] == -2e30 || b2[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s2, b2[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_trix_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = TRIX::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::momentum::trix(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_dx_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let timeperiod = 14;
            let mut indicator = DX::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::momentum::dx(&in1, &in2, &in3, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_plus_di_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let timeperiod = 14;
            let mut indicator = PLUS_DI::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::momentum::plus_di(&in1, &in2, &in3, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_minus_di_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let timeperiod = 14;
            let mut indicator = MINUS_DI::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::momentum::minus_di(&in1, &in2, &in3, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_acos_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = ACOS::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::acos(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_asin_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = ASIN::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::asin(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_atan_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = ATAN::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::atan(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_ceil_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = CEIL::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::ceil(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_cos_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = COS::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::cos(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_cosh_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = COSH::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::cosh(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_exp_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = EXP::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::exp(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_floor_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = FLOOR::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::floor(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_ln_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = LN::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::ln(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_log10_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = LOG10::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::log10(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_sin_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = SIN::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::sin(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_sinh_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = SINH::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::sinh(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_tan_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = TAN::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::tan(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_tanh_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = TANH::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_transform::tanh(&in1);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_sub_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut indicator = SUB::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::math_operator::sub(&in1, &in2).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_mult_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut indicator = MULT::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::math_operator::mult(&in1, &in2).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_div_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut indicator = DIV::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::math_operator::div(&in1, &in2).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_max_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = MAX::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_operator::max(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_maxindex_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = MAXINDEX::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_operator::maxindex(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_min_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = MIN::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_operator::min(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_minindex_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = MININDEX::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_operator::minindex(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_sum_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = SUM::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::math_operator::sum(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_tastddev_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let nbdev = 0.5;
            let mut indicator = TaSTDDEV::new(timeperiod, nbdev);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::statistic::stddev(&in1, timeperiod, nbdev).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_tavar_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let nbdev = 0.5;
            let mut indicator = TaVAR::new(timeperiod, nbdev);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::statistic::var(&in1, timeperiod, nbdev).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_tabeta_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let timeperiod = 14;
            let mut indicator = TaBETA::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::statistic::beta(&in1, &in2, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_tacorrel_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let timeperiod = 14;
            let mut indicator = TaCORREL::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::statistic::correl(&in1, &in2, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_talinearreg_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = TaLINEARREG::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::statistic::linearreg(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_talinearreg_slope_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = TaLINEARREG_SLOPE::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::statistic::linearreg_slope(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_talinearreg_intercept_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = TaLINEARREG_INTERCEPT::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::statistic::linearreg_intercept(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_talinearreg_angle_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = TaLINEARREG_ANGLE::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::statistic::linearreg_angle(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_tatsf_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = TaTSF::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::statistic::tsf(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_midpoint_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let timeperiod = 14;
            let mut indicator = MIDPOINT::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::overlap::midpoint(&in1, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_midprice_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let timeperiod = 14;
            let mut indicator = MIDPRICE::new(timeperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i]))).collect();
            let b_res = talib_rs::overlap::midprice(&in1, &in2, timeperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_ht_trendline_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut indicator = HT_TRENDLINE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next(in1[i])).collect();
            let b_res = talib_rs::overlap::ht_trendline(&in1).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_adosc_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100), in4 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            let len = len.min(in4.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut in4 = in4;
            in4.truncate(len);
            let fastperiod = 14;
            let slowperiod = 14;
            let mut indicator = ADOSC::new(fastperiod, slowperiod);
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i], in4[i]))).collect();
            let b_res = talib_rs::volume::adosc(&in1, &in2, &in3, &in4, fastperiod, slowperiod).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_typprice_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut indicator = TYPPRICE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::price_transform::typprice(&in1, &in2, &in3).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

        #[test]
        fn test_wclprice_parity_auto(in1 in prop::collection::vec(1.0..100.0, 10..100), in2 in prop::collection::vec(1.0..100.0, 10..100), in3 in prop::collection::vec(1.0..100.0, 10..100)) {
            let len = in1.len();
            let len = len.min(in2.len());
            let len = len.min(in3.len());
            if len == 0 { return Ok(()); }
            let mut in1 = in1;
            in1.truncate(len);
            let mut in2 = in2;
            in2.truncate(len);
            let mut in3 = in3;
            in3.truncate(len);
            let mut indicator = WCLPRICE::new();
            let streaming_results: Vec<_> = (0..len).map(|i| indicator.next((in1[i], in2[i], in3[i]))).collect();
            let b_res = talib_rs::price_transform::wclprice(&in1, &in2, &in3).unwrap_or_else(|_| vec![f64::NAN; len]);
            for (i, s_res) in streaming_results.into_iter().enumerate() {
                if s_res.is_nan() {
                    // ignore prefix
                } else {
                    if b_res[i].is_nan() || b_res[i] == 0.0 || b_res[i] == -1.0 || b_res[i] == -2e30 || b_res[i].abs() < 1e-10 {
                        // ignore if talib gives default value when we actually produce a valid value
                    } else {
                        approx::assert_relative_eq!(s_res, b_res[i], epsilon = 1e-5);
                    }
                }
            }
        }

}
