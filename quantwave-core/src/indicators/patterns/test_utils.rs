#[cfg(test)]
#[macro_export]
/// Parity harness for a candlestick pattern.
///
/// Three forms:
///
/// ```ignore
/// test_pattern_parity!(name, STRUCT, talib_rs::pattern::func);
/// test_pattern_parity!(name, STRUCT, talib_rs::pattern::func, fixture);
/// test_pattern_parity!(
///     name, STRUCT, talib_rs::pattern::func, fixture,
///     oracle_exempt = "why the oracle is wrong, with evidence",
///     max_mismatches = 12,
///     reference_mismatches = 8
/// );
/// ```
///
/// The `oracle_exempt` form is for patterns where the `talib-rs` oracle is
/// itself defective and the native implementation is the correct one. It does
/// **not** skip testing:
///
/// * the batch (`next_batch`) vs streaming (`next`) consistency check still
///   runs, and is *strengthened* — it compares against the streaming output
///   rather than against the (defective) oracle;
/// * the bounded-memory walk still runs;
/// * the oracle comparison still runs on every proptest-generated series, but
///   as a *bounded divergence* check: at most `max_mismatches` bars per run may
///   differ. Pick the bound from measured data with modest headroom and keep it
///   well under the number of non-zero signals the oracle produces, so a
///   regression that silences the pattern or fires it everywhere still fails;
/// * on the deterministic 200,000-bar reference walk the divergence is pinned
///   *exactly* to `reference_mismatches`. That walk is seeded, so the count is
///   reproducible to the bar, and any behaviour change — including one that
///   moves the implementation *closer* to the defective oracle, which a cap
///   alone cannot catch — breaks the test and forces the number and the stated
///   reason to be re-derived. Adding or changing a `fixture` also changes this
///   count, deliberately: the evidence must be re-measured, not re-fudged;
/// * the exemption surfaces as a test named `oracle_parity_is_exempted`
///   (vs `oracle_parity_is_enforced` for every other pattern), so
///   `cargo nextest list | grep oracle_parity_is_exempted` enumerates every
///   exemption in the workspace and the reason is printed in test output.
macro_rules! test_pattern_parity {
    ($name:ident, $struct_name:ident, $talib_func:path) => {
        $crate::test_pattern_parity!($name, $struct_name, $talib_func, |_, _, _, _| {});
    };
    ($name:ident, $struct_name:ident, $talib_func:path, $fixture:expr) => {
        $crate::test_pattern_parity!(
            @impl $name, $struct_name, $talib_func, $fixture,
            None, 0, 0, oracle_parity_is_enforced
        );
    };
    (
        $name:ident, $struct_name:ident, $talib_func:path, $fixture:expr,
        oracle_exempt = $reason:expr,
        max_mismatches = $max:expr,
        reference_mismatches = $reference:expr
    ) => {
        $crate::test_pattern_parity!(
            @impl $name, $struct_name, $talib_func, $fixture,
            Some($reason), $max, $reference, oracle_parity_is_exempted
        );
    };
    (
        @impl $name:ident, $struct_name:ident, $talib_func:path, $fixture:expr,
        $exemption:expr, $max:expr, $reference:expr, $notice:ident
    ) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            use proptest::prelude::*;

            /// `Some(reason)` when oracle parity is knowingly not expected.
            const ORACLE_EXEMPTION: Option<&str> = $exemption;
            /// Maximum bars per run allowed to differ from the oracle when exempt.
            const MAX_ORACLE_MISMATCHES: usize = $max;
            /// Exact number of diverging bars on the deterministic 200k reference walk.
            const REFERENCE_ORACLE_MISMATCHES: usize = $reference;

            /// Indices where the streaming output disagrees with the oracle.
            fn oracle_mismatches(o: &[f64], h: &[f64], l: &[f64], c: &[f64]) -> Vec<usize> {
                let mut ind = $struct_name::new();
                let streaming: Vec<f64> = (0..o.len())
                    .map(|i| ind.next((o[i], h[i], l[i], c[i])))
                    .collect();
                let batch = $talib_func(o, h, l, c).expect("talib oracle failed");
                streaming
                    .iter()
                    .zip(batch.iter())
                    .enumerate()
                    .filter(|(_, (s, b))| **s as i32 != **b)
                    .map(|(i, _)| i)
                    .collect()
            }

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

            /// Declares, in the test list itself, whether this pattern is held to
            /// exact oracle parity or is a documented exemption.
            #[test]
            fn $notice() {
                match ORACLE_EXEMPTION {
                    Some(reason) => {
                        assert!(
                            !reason.trim().is_empty(),
                            "{} declares an oracle exemption but gives no reason",
                            stringify!($struct_name)
                        );
                        assert!(
                            MAX_ORACLE_MISMATCHES > 0,
                            "{} declares an oracle exemption with max_mismatches = 0; \
                             use the non-exempt form instead",
                            stringify!($struct_name)
                        );
                        assert!(
                            REFERENCE_ORACLE_MISMATCHES <= MAX_ORACLE_MISMATCHES,
                            "{}: reference_mismatches ({}) exceeds max_mismatches ({})",
                            stringify!($struct_name),
                            REFERENCE_ORACLE_MISMATCHES,
                            MAX_ORACLE_MISMATCHES
                        );
                        println!(
                            "ORACLE PARITY EXEMPT: {} — exactly {} diverging bars on the \
                             deterministic 200k reference walk, at most {} per random run; \
                             batch-vs-streaming parity is still exact. Reason: {}",
                            stringify!($struct_name),
                            REFERENCE_ORACLE_MISMATCHES,
                            MAX_ORACLE_MISMATCHES,
                            reason
                        );
                    }
                    None => {
                        println!(
                            "ORACLE PARITY ENFORCED: {} — exact match against the oracle required.",
                            stringify!($struct_name)
                        );
                    }
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

                if let Some(reason) = ORACLE_EXEMPTION {
                    // Bounded-divergence check: the oracle is known-defective for this
                    // pattern, but the disagreement must stay rare. A regression that
                    // shifts, silences or over-fires the pattern blows past the bound.
                    let mismatches: Vec<usize> = streaming
                        .iter()
                        .zip(batch.iter())
                        .enumerate()
                        .filter(|(_, (s, b))| **s as i32 != **b)
                        .map(|(i, _)| i)
                        .collect();
                    if !mismatches.is_empty() {
                        println!(
                            "[oracle-exempt] {}: {} of {} bars diverge from the oracle \
                             (allowed <= {}) at {:?}. Reason: {}",
                            stringify!($struct_name),
                            mismatches.len(),
                            len,
                            MAX_ORACLE_MISMATCHES,
                            &mismatches[..mismatches.len().min(16)],
                            reason
                        );
                    }
                    assert!(
                        mismatches.len() <= MAX_ORACLE_MISMATCHES,
                        "{}: oracle divergence exceeded its documented bound — {} of {} bars \
                         differ, at most {} allowed (first offenders: {:?}). Either the native \
                         implementation regressed or the exemption is no longer accurate. \
                         Exemption reason: {}",
                        stringify!($struct_name),
                        mismatches.len(),
                        len,
                        MAX_ORACLE_MISMATCHES,
                        &mismatches[..mismatches.len().min(16)],
                        reason
                    );
                } else {
                    for (i, (s, b)) in streaming.iter().zip(batch.iter()).enumerate() {
                        assert_eq!(*s as i32, *b, "Mismatch at index {}: streaming {}, batch {}", i, s, b);
                    }
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

                // For an exempt pattern, pin the divergence on this deterministic walk
                // exactly. The cap in `run_parity` only catches divergence getting
                // *worse*; this catches any change at all, including one that drifts
                // toward the defective oracle.
                if let Some(reason) = ORACLE_EXEMPTION {
                    let mismatches = oracle_mismatches(&o, &h, &l, &c);
                    assert_eq!(
                        mismatches.len(),
                        REFERENCE_ORACLE_MISMATCHES,
                        "{}: divergence from the oracle on the deterministic 200k reference walk \
                         changed from the documented {} bars to {} (at {:?}). This walk is seeded, \
                         so the count only moves if the implementation, the fixture, or the oracle \
                         changed. Re-derive the evidence before updating the number. \
                         Exemption reason: {}",
                        stringify!($struct_name),
                        REFERENCE_ORACLE_MISMATCHES,
                        mismatches.len(),
                        &mismatches[..mismatches.len().min(16)],
                        reason
                    );
                }

                assert!(found_signal_in_base, "Zero non-zero signals produced by oracle across 200,000 bars. Provide a positive fixture or fix the random walk.");

                // Bounded memory test & next_batch test
                let mut ind_streaming = $struct_name::new();
                let streaming_out: Vec<f64> = (0..10_000)
                    .map(|i| ind_streaming.next((o[i], h[i], l[i], c[i])))
                    .collect();

                // batch vs next_batch
                let mut ind_batch = $struct_name::new();
                let inputs: Vec<(f64, f64, f64, f64)> = (0..10_000)
                    .map(|i| (o[i], h[i], l[i], c[i]))
                    .collect();
                let batch_out = ind_batch.next_batch(&inputs);

                // `next_batch` must always agree with `next`, exemption or not.
                for (i, (b_out, s_out)) in batch_out.iter().zip(streaming_out.iter()).enumerate() {
                    assert_eq!(*b_out as i32, *s_out as i32, "next_batch vs streaming mismatch at {}", i);
                }

                if ORACLE_EXEMPTION.is_none() {
                    let oracle_out = $talib_func(&o[..10_000], &h[..10_000], &l[..10_000], &c[..10_000]).unwrap();

                    for (i, (b_out, o_out)) in batch_out.iter().zip(oracle_out.iter()).enumerate() {
                        assert_eq!(*b_out as i32, *o_out, "next_batch mismatch at {}", i);
                    }
                }
            }
        }
    };
}
