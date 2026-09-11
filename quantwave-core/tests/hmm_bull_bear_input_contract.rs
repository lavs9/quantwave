//! Input-contract regression tests for `HMM::bull_bear`.
//!
//! `bull_bear()` hardcodes Gaussian emission means/stds to daily-returns scale.
//! Feeding it price-scale input used to underflow both emissions to `0.0` on
//! every bar, which made the online Viterbi decode a `-inf`/`-inf` tie that
//! always resolved to state `0` (Bull) — i.e. a silently constant output. This
//! file checks the detector (`HMM::is_degenerate`) that callers use to turn
//! that into an explicit rejection, and the regression case that a genuine
//! returns-scale series still decodes varying Bull/Bear states.

use quantwave_core::regimes::MarketRegime;
use quantwave_core::regimes::hmm::HMM;
use quantwave_core::traits::Next;

#[test]
fn price_scale_input_is_flagged_degenerate_on_every_bar() {
    let mut hmm = HMM::bull_bear();
    let prices: Vec<f64> = (0..60).map(|i| 100.0 + i as f64 * 0.5).collect();

    let mut degenerate_count = 0;
    for &p in &prices {
        hmm.next(p);
        if hmm.is_degenerate() {
            degenerate_count += 1;
        }
    }

    // Every bar underflows both emissions on price-scale input.
    assert_eq!(degenerate_count, prices.len());
}

#[test]
fn price_scale_input_would_otherwise_silently_decode_constant_bull() {
    // Documents the pre-fix failure mode this test guards against: without the
    // is_degenerate() check, best_state sticks at index 0 (Bull) forever on
    // off-scale input, for uptrend, downtrend, or a realistic price index alike.
    for series in [
        (0..60).map(|i| 100.0 + i as f64 * 0.5).collect::<Vec<_>>(), // uptrend
        (0..60).map(|i| 100.0 - i as f64 * 0.3).collect::<Vec<_>>(), // downtrend
        vec![4500.0, 4510.0, 4495.0, 4520.0, 4530.0, 4512.0, 4540.0, 4551.0]
            .into_iter()
            .cycle()
            .take(60)
            .collect::<Vec<_>>(), // realistic price index
    ] {
        let mut hmm = HMM::bull_bear();
        let regimes: Vec<MarketRegime> = series.iter().map(|&x| hmm.next(x)).collect();
        assert!(
            regimes.iter().all(|r| matches!(r, MarketRegime::Bull)),
            "expected the known degenerate-tie failure mode (constant Bull) on \
             off-scale input, got {regimes:?}"
        );
        // ... and the detector must have caught it on every bar, so a caller can
        // reject instead of emitting the constant column above.
        assert!(hmm.is_degenerate());
    }
}

#[test]
fn returns_scale_input_decodes_varying_bull_bear_without_degeneracy() {
    let returns = [
        0.01, 0.009, 0.011, 0.008, 0.012, 0.01, 0.009, // bull run
        -0.02, -0.018, -0.022, -0.019, -0.025, -0.021, -0.017, // bear run
        0.011, 0.009, 0.012, 0.01, 0.008, 0.011, // bull again
    ];

    let mut hmm = HMM::bull_bear();
    let mut regimes = Vec::with_capacity(returns.len());
    let mut degenerate_count = 0;
    for &r in &returns {
        regimes.push(hmm.next(r));
        if hmm.is_degenerate() {
            degenerate_count += 1;
        }
    }

    assert_eq!(degenerate_count, 0, "a real returns series must never underflow");
    assert!(
        regimes.iter().any(|r| matches!(r, MarketRegime::Bear)),
        "expected at least one Bear bar during the sustained negative-return run, got {regimes:?}"
    );
    assert!(
        regimes.iter().any(|r| matches!(r, MarketRegime::Bull)),
        "expected at least one Bull bar during the positive-return runs, got {regimes:?}"
    );
}
