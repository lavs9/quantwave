talib_4_in_1_out!(AD, talib_rs::volume::ad);
impl Default for AD {
    fn default() -> Self {
        Self::new()
    }
}
talib_4_in_1_out!(ADOSC, talib_rs::volume::adosc, fastperiod: usize, slowperiod: usize);
talib_2_in_1_out!(OBV, talib_rs::volume::obv);
impl Default for OBV {
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
        fn test_ad_parity(
            h in prop::collection::vec(10.0..100.0, 1..100),
            l in prop::collection::vec(10.0..100.0, 1..100),
            c in prop::collection::vec(10.0..100.0, 1..100),
            v in prop::collection::vec(1.0..1000.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len()).min(v.len());
            if len == 0 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            let mut volume = Vec::with_capacity(len);
            for i in 0..len {
                let v_h: f64 = h[i];
                let v_l: f64 = l[i];
                let v_c: f64 = c[i];
                let v_v: f64 = v[i];
                high.push(v_h.max(v_l).max(v_c));
                low.push(v_h.min(v_l).min(v_c));
                close.push(v_c);
                volume.push(v_v);
            }

            let mut ad = AD::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| ad.next((high[i], low[i], close[i], volume[i]))).collect();
            let batch_results = talib_rs::volume::ad(&high, &low, &close, &volume).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_obv_parity(
            c in prop::collection::vec(10.0..100.0, 1..100),
            v in prop::collection::vec(1.0..1000.0, 1..100)
        ) {
            let len = c.len().min(v.len());
            if len == 0 { return Ok(()); }
            let close = c[..len].to_vec();
            let volume = v[..len].to_vec();

            let mut obv = OBV::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| obv.next((close[i], volume[i]))).collect();
            let batch_results = talib_rs::volume::obv(&close, &volume).unwrap_or_else(|_| vec![f64::NAN; len]);

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
