talib_4_in_1_out!(AVGPRICE, talib_rs::price_transform::avgprice);
impl Default for AVGPRICE {
    fn default() -> Self {
        Self::new()
    }
}
talib_2_in_1_out!(MEDPRICE, talib_rs::price_transform::medprice);
impl Default for MEDPRICE {
    fn default() -> Self {
        Self::new()
    }
}
talib_3_in_1_out!(TYPPRICE, talib_rs::price_transform::typprice);
impl Default for TYPPRICE {
    fn default() -> Self {
        Self::new()
    }
}
talib_3_in_1_out!(WCLPRICE, talib_rs::price_transform::wclprice);
impl Default for WCLPRICE {
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
        fn test_avgprice_parity(
            o in prop::collection::vec(0.1..100.0, 1..100),
            h in prop::collection::vec(0.1..100.0, 1..100),
            l in prop::collection::vec(0.1..100.0, 1..100),
            c in prop::collection::vec(0.1..100.0, 1..100)
        ) {
            let len = o.len().min(h.len()).min(l.len()).min(c.len());
            if len == 0 { return Ok(()); }

            let mut avgprice = AVGPRICE::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| avgprice.next((o[i], h[i], l[i], c[i]))).collect();
            let batch_results = talib_rs::price_transform::avgprice(&o[..len], &h[..len], &l[..len], &c[..len]).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_medprice_parity(
            h in prop::collection::vec(0.1..100.0, 1..100),
            l in prop::collection::vec(0.1..100.0, 1..100)
        ) {
            let len = h.len().min(l.len());
            if len == 0 { return Ok(()); }

            let mut medprice = MEDPRICE::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| medprice.next((h[i], l[i]))).collect();
            let batch_results = talib_rs::price_transform::medprice(&h[..len], &l[..len]).unwrap_or_else(|_| vec![f64::NAN; len]);

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
