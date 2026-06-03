//! Bounded-window CDL streaming parity tests (native_cdl macro).

#[cfg(test)]
mod tests {
    use crate::indicators::pattern::{CDLENGULFING, CDLHAMMER};
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_cdl_hammer_window_parity(
            o in prop::collection::vec(10.0..100.0, 1..80),
            h in prop::collection::vec(10.0..100.0, 1..80),
            l in prop::collection::vec(10.0..100.0, 1..80),
            c in prop::collection::vec(10.0..100.0, 1..80)
        ) {
            let len = o.len().min(h.len()).min(l.len()).min(c.len());
            if len < 15 { return Ok(()); }
            let mut hammer = CDLHAMMER::new();
            let streaming: Vec<f64> = (0..len)
                .map(|i| hammer.next((o[i], h[i], l[i], c[i])))
                .collect();
            let batch = talib_rs::pattern::cdl_hammer(&o[..len], &h[..len], &l[..len], &c[..len])
                .unwrap_or_else(|_| vec![0; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                assert_eq!(*s as i32, *b, "CDLHAMMER mismatch");
            }
        }

        #[test]
        fn test_cdl_engulfing_window_parity(
            o in prop::collection::vec(10.0..100.0, 1..80),
            h in prop::collection::vec(10.0..100.0, 1..80),
            l in prop::collection::vec(10.0..100.0, 1..80),
            c in prop::collection::vec(10.0..100.0, 1..80)
        ) {
            let len = o.len().min(h.len()).min(l.len()).min(c.len());
            if len < 15 { return Ok(()); }
            let mut eng = CDLENGULFING::new();
            let streaming: Vec<f64> = (0..len)
                .map(|i| eng.next((o[i], h[i], l[i], c[i])))
                .collect();
            let batch = talib_rs::pattern::cdl_engulfing(&o[..len], &h[..len], &l[..len], &c[..len])
                .unwrap_or_else(|_| vec![0; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                assert_eq!(*s as i32, *b, "CDLENGULFING mismatch");
            }
        }
    }
}