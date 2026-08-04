#[cfg(test)]
#[macro_export]
macro_rules! test_pattern_parity {
    ($name:ident, $struct_name:ident, $talib_func:path) => {
        crate::test_pattern_parity!($name, $struct_name, $talib_func, |_, _, _, _| {});
    };
    ($name:ident, $struct_name:ident, $talib_func:path, $fixture:expr) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            use proptest::prelude::*;

            fn arb_random_walk(len_range: std::ops::Range<usize>) -> impl Strategy<Value = (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
                // Generate a starting price and a vector of (drift, spread)
                (
                    10.0f64..100.0f64,
                    prop::collection::vec(
                        (-2.0f64..2.0f64, 0.0f64..5.0f64),
                        len_range
                    )
                ).prop_map(|(start_price, steps)| {
                    let mut o = Vec::with_capacity(steps.len());
                    let mut h = Vec::with_capacity(steps.len());
                    let mut l = Vec::with_capacity(steps.len());
                    let mut c = Vec::with_capacity(steps.len());
                    
                    let mut current_price = start_price;
                    for (drift, spread) in steps {
                        let open = current_price;
                        let close = open + drift;
                        let high = open.max(close) + spread;
                        let low = open.min(close) - spread;
                        
                        o.push(open);
                        h.push(high);
                        l.push(low);
                        c.push(close);
                        
                        current_price = close; // next open is this close (or close to it)
                    }
                    (o, h, l, c)
                })
            }

            proptest! {
                #[test]
                fn test_parity_long(
                    (mut o, mut h, mut l, mut c) in arb_random_walk(1..200)
                ) {
                    let f: fn(&mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>) = $fixture;
                    f(&mut o, &mut h, &mut l, &mut c);
                    run_parity(&o, &h, &l, &c);
                }

                #[test]
                fn test_parity_short(
                    (mut o, mut h, mut l, mut c) in arb_random_walk(1..30)
                ) {
                    let f: fn(&mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>) = $fixture;
                    f(&mut o, &mut h, &mut l, &mut c);
                    run_parity(&o, &h, &l, &c);
                }
            }

            fn run_parity(o: &[f64], h: &[f64], l: &[f64], c: &[f64]) -> bool {
                let len = o.len();
                if len == 0 { return false; }

                let mut ind = $struct_name::new();
                let streaming: Vec<f64> = (0..len)
                    .map(|i| ind.next((o[i], h[i], l[i], c[i])))
                    .collect();
                
                // 1. Drop the error-swallowing fallback
                let batch = $talib_func(o, h, l, c).expect("talib oracle failed");

                for (i, (s, b)) in streaming.iter().zip(batch.iter()).enumerate() {
                    assert_eq!(*s as i32, *b, "Mismatch at index {}: streaming {}, batch {}", i, s, b);
                }
                
                // Return true if we found at least one non-zero signal in this run
                batch.iter().any(|b| *b != 0)
            }
            
            #[test]
            fn test_coverage_and_bounded_memory() {
                // Generate a long random walk base
                let mut o = Vec::with_capacity(200_000);
                let mut h = Vec::with_capacity(200_000);
                let mut l = Vec::with_capacity(200_000);
                let mut c = Vec::with_capacity(200_000);
                
                let mut rng = 1337_u32;
                let mut current_price = 50.0;
                for _ in 0..200_000 {
                    rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                    let drift = ((rng >> 16) as f64 / 65536.0) * 4.0 - 2.0;
                    rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                    let spread = ((rng >> 16) as f64 / 65536.0) * 5.0;
                    
                    let open = current_price;
                    let close = open + drift;
                    let high = open.max(close) + spread;
                    let low = open.min(close) - spread;
                    
                    o.push(open);
                    h.push(high);
                    l.push(low);
                    c.push(close);
                    
                    current_price = close;
                }
                
                let f: fn(&mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>) = $fixture;
                f(&mut o, &mut h, &mut l, &mut c);
                
                let found_signal_in_base = run_parity(&o, &h, &l, &c);
                
                assert!(found_signal_in_base, "Zero non-zero signals produced by oracle across 200,000 bars. Provide a positive fixture or fix the random walk.");
                
                // Bounded memory test & next_batch test
                let mut ind_streaming = $struct_name::new();
                for i in 0..10_000 {
                    ind_streaming.next((o[i], h[i], l[i], c[i]));
                }
                
                // batch vs next_batch
                let mut ind_batch = $struct_name::new();
                let inputs: Vec<(f64, f64, f64, f64)> = (0..10_000)
                    .map(|i| (o[i], h[i], l[i], c[i]))
                    .collect();
                let batch_out = ind_batch.next_batch(&inputs);
                let oracle_out = $talib_func(&o[..10_000], &h[..10_000], &l[..10_000], &c[..10_000]).unwrap();
                
                for (i, (b_out, o_out)) in batch_out.iter().zip(oracle_out.iter()).enumerate() {
                    assert_eq!(*b_out as i32, *o_out, "next_batch mismatch at {}", i);
                }
            }
        }
    };
}
