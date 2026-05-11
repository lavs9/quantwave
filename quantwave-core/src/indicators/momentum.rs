talib_1_in_1_out!(RSI, talib_rs::momentum::rsi, timeperiod: usize);
talib_1_in_3_out!(MACD, talib_rs::momentum::macd, fastperiod: usize, slowperiod: usize, signalperiod: usize);
talib_1_in_3_out!(MACDEXT, talib_rs::momentum::macd_ext, fastperiod: usize, fastmatype: talib_rs::MaType, slowperiod: usize, slowmatype: talib_rs::MaType, signalperiod: usize, signalmatype: talib_rs::MaType);
talib_1_in_3_out!(MACDFIX, talib_rs::momentum::macd_fix, signalperiod: usize);

talib_3_in_2_out!(STOCH, talib_rs::momentum::stoch, fastk_period: usize, slowk_period: usize, slowk_matype: talib_rs::MaType, slowd_period: usize, slowd_matype: talib_rs::MaType);
talib_3_in_2_out!(STOCHF, talib_rs::momentum::stochf, fastk_period: usize, fastd_period: usize, fastd_matype: talib_rs::MaType);
talib_1_in_2_out!(STOCHRSI, talib_rs::momentum::stochrsi, timeperiod: usize, fastk_period: usize, fastd_period: usize, fastd_matype: talib_rs::MaType);

talib_3_in_1_out!(ADX, talib_rs::momentum::adx, timeperiod: usize);
talib_3_in_1_out!(ADXR, talib_rs::momentum::adxr, timeperiod: usize);
talib_3_in_1_out!(CCI, talib_rs::momentum::cci, timeperiod: usize);
talib_1_in_1_out!(MOM, talib_rs::momentum::mom, timeperiod: usize);
talib_1_in_1_out!(ROC, talib_rs::momentum::roc, timeperiod: usize);
talib_1_in_1_out!(ROCP, talib_rs::momentum::rocp, timeperiod: usize);
talib_1_in_1_out!(ROCR, talib_rs::momentum::rocr, timeperiod: usize);
talib_1_in_1_out!(ROCR100, talib_rs::momentum::rocr100, timeperiod: usize);
talib_3_in_1_out!(WILLR, talib_rs::momentum::willr, timeperiod: usize);
talib_1_in_1_out!(APO, talib_rs::momentum::apo, fastperiod: usize, slowperiod: usize, matype: talib_rs::MaType);
talib_1_in_1_out!(PPO, talib_rs::momentum::ppo, fastperiod: usize, slowperiod: usize, matype: talib_rs::MaType);
talib_4_in_1_out!(BOP, talib_rs::momentum::bop);
talib_1_in_1_out!(CMO, talib_rs::momentum::cmo, timeperiod: usize);
talib_2_in_2_out!(AROON, talib_rs::momentum::aroon, timeperiod: usize);
talib_2_in_1_out!(AROONOSC, talib_rs::momentum::aroon_osc, timeperiod: usize);
talib_4_in_1_out!(MFI, talib_rs::momentum::mfi, timeperiod: usize);
talib_1_in_1_out!(TRIX, talib_rs::momentum::trix, timeperiod: usize);
talib_3_in_1_out!(ULTOSC, talib_rs::momentum::ultosc, timeperiod1: usize, timeperiod2: usize, timeperiod3: usize);
talib_3_in_1_out!(DX, talib_rs::momentum::dx, timeperiod: usize);
talib_3_in_1_out!(PLUS_DI, talib_rs::momentum::plus_di, timeperiod: usize);
talib_3_in_1_out!(MINUS_DI, talib_rs::momentum::minus_di, timeperiod: usize);
talib_2_in_1_out!(PLUS_DM, talib_rs::momentum::plus_dm, timeperiod: usize);
talib_2_in_1_out!(MINUS_DM, talib_rs::momentum::minus_dm, timeperiod: usize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_rsi_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 14;
            let mut rsi = RSI::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| rsi.next(x)).collect();
            let batch_results = talib_rs::momentum::rsi(&input, period).unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_macd_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let fast = 12;
            let slow = 26;
            let signal = 9;
            let mut macd = MACD::new(fast, slow, signal);
            let streaming_results: Vec<(f64, f64, f64)> = input.iter().map(|&x| macd.next(x)).collect();
            let (b_macd, b_signal, b_hist) = talib_rs::momentum::macd(&input, fast, slow, signal).unwrap_or_else(|_| {
                (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
            });
            
            for (i, (s_macd, s_signal, s_hist)) in streaming_results.into_iter().enumerate() {
                if s_macd.is_nan() {
                    assert!(b_macd[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_macd, b_macd[i], epsilon = 1e-6);
                }
                if s_signal.is_nan() {
                    assert!(b_signal[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_signal, b_signal[i], epsilon = 1e-6);
                }
                if s_hist.is_nan() {
                    assert!(b_hist[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_hist, b_hist[i], epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_stoch_parity(
            h in prop::collection::vec(1.0..100.0, 1..100),
            l in prop::collection::vec(1.0..100.0, 1..100),
            c in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len());
            if len == 0 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            for i in 0..len {
                let val_h: f64 = h[i];
                let val_l: f64 = l[i];
                let val_c: f64 = c[i];
                let max: f64 = val_h.max(val_l).max(val_c);
                let min: f64 = val_h.min(val_l).min(val_c);
                high.push(max);
                low.push(min);
                close.push(val_c);
            }

            let fastk = 5;
            let slowk = 3;
            let slowk_ma = talib_rs::MaType::Sma;
            let slowd = 3;
            let slowd_ma = talib_rs::MaType::Sma;
            
            let mut stoch = STOCH::new(fastk, slowk, slowk_ma, slowd, slowd_ma);
            let streaming_results: Vec<(f64, f64)> = (0..len).map(|i| stoch.next((high[i], low[i], close[i]))).collect();
            let (b_k, b_d) = talib_rs::momentum::stoch(&high, &low, &close, fastk, slowk, slowk_ma, slowd, slowd_ma).unwrap_or_else(|_| {
                (vec![f64::NAN; len], vec![f64::NAN; len])
            });

            for (i, (s_k, s_d)) in streaming_results.into_iter().enumerate() {
                if s_k.is_nan() {
                    assert!(b_k[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_k, b_k[i], epsilon = 1e-6);
                }
                if s_d.is_nan() {
                    assert!(b_d[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_d, b_d[i], epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_adx_parity(
            h in prop::collection::vec(1.0..100.0, 1..100),
            l in prop::collection::vec(1.0..100.0, 1..100),
            c in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len());
            if len == 0 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            for i in 0..len {
                let val_h: f64 = h[i];
                let val_l: f64 = l[i];
                let val_c: f64 = c[i];
                let max: f64 = val_h.max(val_l).max(val_c);
                let min: f64 = val_h.min(val_l).min(val_c);
                high.push(max);
                low.push(min);
                close.push(val_c);
            }

            let period = 14;
            let mut adx = ADX::new(period);
            let streaming_results: Vec<f64> = (0..len).map(|i| adx.next((high[i], low[i], close[i]))).collect();
            let batch_results = talib_rs::momentum::adx(&high, &low, &close, period).unwrap_or_else(|_| vec![f64::NAN; len]);

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
