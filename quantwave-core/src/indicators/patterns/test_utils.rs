#[cfg(test)]
#[macro_export]
macro_rules! test_pattern_parity {
    ($name:ident, $struct_name:ident, $talib_func:path) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            use proptest::prelude::*;

            fn arb_ohlc(len_range: std::ops::Range<usize>) -> impl Strategy<Value = (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
                // Generate normal realistic-ish bars, plus degenerate cases (flat bars, zero range)
                let bar = prop_oneof![
                    4 => (10.0f64..100.0f64, -5.0f64..5.0f64, 0.0f64..10.0f64).prop_map(|(o, drift, spread): (f64, f64, f64)| {
                        let c = o + drift;
                        let h = o.max(c) + spread;
                        let l = o.min(c) - spread;
                        (o, h, l, c)
                    }),
                    1 => (10.0f64..100.0f64).prop_map(|p: f64| (p, p, p, p)) // Flat bars
                ];
                prop::collection::vec(bar, len_range).prop_map(|bars| {
                    let mut o = Vec::with_capacity(bars.len());
                    let mut h = Vec::with_capacity(bars.len());
                    let mut l = Vec::with_capacity(bars.len());
                    let mut c = Vec::with_capacity(bars.len());
                    for (open, high, low, close) in bars {
                        o.push(open);
                        h.push(high);
                        l.push(low);
                        c.push(close);
                    }
                    (o, h, l, c)
                })
            }

            proptest! {
                #[test]
                fn test_parity_long(
                    (o, h, l, c) in arb_ohlc(1..200)
                ) {
                    run_parity(&o, &h, &l, &c);
                }

                #[test]
                fn test_parity_short(
                    (o, h, l, c) in arb_ohlc(1..30)
                ) {
                    run_parity(&o, &h, &l, &c);
                }
            }

            fn run_parity(o: &[f64], h: &[f64], l: &[f64], c: &[f64]) {
                let len = o.len();
                if len == 0 { return; }

                let mut ind = $struct_name::new();
                let streaming: Vec<f64> = (0..len)
                    .map(|i| ind.next((o[i], h[i], l[i], c[i])))
                    .collect();
                
                let batch = $talib_func(o, h, l, c)
                    .unwrap_or_else(|_| vec![0; len]);

                for (i, (s, b)) in streaming.iter().zip(batch.iter()).enumerate() {
                    assert_eq!(*s as i32, *b, "Mismatch at index {}: streaming {}, batch {}", i, s, b);
                }
            }
        }
    };
}
