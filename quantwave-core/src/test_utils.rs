//! Gold-standard test helpers (cfg(test) only).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::expect_fun_call,
    clippy::unused_enumerate_index,
    clippy::collapsible_if
)]

use crate::indicators::market_structure::Bias;
use crate::traits::Next;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub input: Vec<f64>,
    pub expected: Vec<Option<f64>>,
}

// =============================================================================
// NUMERICAL-PRECISION HELPERS (quantwave-qkft)
// =============================================================================

/// Neumaier-compensated summation — accurate to ~1 ulp regardless of ordering.
pub fn compensated_sum(xs: impl Iterator<Item = f64>) -> f64 {
    let mut s = 0.0f64;
    let mut c = 0.0f64;
    for x in xs {
        let t = s + x;
        if s.abs() >= x.abs() {
            c += (s - t) + x;
        } else {
            c += (x - t) + s;
        }
        s = t;
    }
    s + c
}

/// High-precision two-pass population standard deviation. Used as the oracle
/// for stddev-derived indicators; far tighter than either the old one-pass
/// accumulator or the new shifted-data one, so it can arbitrate between them.
pub fn reference_stddev(window: &[f64]) -> f64 {
    let n = window.len() as f64;
    let mean = compensated_sum(window.iter().copied()) / n;
    let var = compensated_sum(window.iter().map(|&x| (x - mean) * (x - mean))) / n;
    var.max(0.0).sqrt()
}

/// High-precision population mean of a window.
pub fn reference_mean(window: &[f64]) -> f64 {
    compensated_sum(window.iter().copied()) / window.len() as f64
}

/// Deterministic log-price series: `ln(P)` in the 1..9 band, slowly varying,
/// with per-bar moves ~1e-4. This is the input shape that exposed the one-pass
/// variance cancellation in quantwave-qkft. No RNG, so tests never flake.
pub fn log_price_series(n: usize) -> Vec<f64> {
    let mut out = Vec::with_capacity(n);
    let mut p: f64 = 2980.0;
    for i in 0..n {
        let i_f = i as f64;
        let bump = (i_f * 0.7).sin() * 0.35 + (i_f * 0.13).cos() * 0.2;
        p += bump;
        out.push(p.ln());
    }
    out
}

/// Assert that driving an indicator over the whole series in one go (the
/// "batch" path, which is literally what the Polars plugins do) is
/// **bit-identical** to driving a resumed instance chunk by chunk (the
/// "streaming" path). Any data-dependent or non-deterministic internal state —
/// e.g. a variance accumulator that refreshed on an error heuristic instead of
/// on bar index — would break this.
pub fn assert_bitwise_batch_streaming_parity<I, F>(input: &[f64], make: F)
where
    I: Next<f64, Output = f64>,
    F: Fn() -> I,
{
    let mut batch = make();
    let batch_out: Vec<f64> = input.iter().map(|&x| batch.next(x)).collect();

    for chunk in [1usize, 7, 13, 64] {
        let mut streaming = make();
        let mut streaming_out = Vec::with_capacity(input.len());
        for part in input.chunks(chunk) {
            for &x in part {
                streaming_out.push(streaming.next(x));
            }
        }
        for (i, (b, s)) in batch_out.iter().zip(streaming_out.iter()).enumerate() {
            assert_eq!(
                b.to_bits(),
                s.to_bits(),
                "batch/streaming divergence at bar {i} (chunk {chunk}): {b} vs {s}"
            );
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TestCaseVec {
    pub input: Vec<f64>,
    pub expected: Vec<Vec<Option<f64>>>,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseLoops {
    pub input: Vec<(f64, f64)>,
    pub expected: Vec<(Option<f64>, Option<f64>)>,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseOC {
    pub input: Vec<(f64, f64)>,
    pub expected: Vec<Option<f64>>,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseTuple {
    pub input: Vec<f64>,
    pub expected: Vec<(Option<f64>, Option<f64>)>,
}

/// Load a gold standard test case from a JSON file.
pub fn load_gold_standard(name: &str) -> TestCase {
    let path = get_gold_standard_path(name);
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

pub fn load_gold_standard_vec(name: &str) -> TestCaseVec {
    let path = get_gold_standard_path(name);
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

pub fn load_gold_standard_loops(name: &str) -> TestCaseLoops {
    let path = get_gold_standard_path(name);
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

fn get_gold_standard_path(name: &str) -> std::path::PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = Path::new(&manifest_dir);

    let path = manifest_path
        .join("tests/gold_standard")
        .join(format!("{}.json", name));
    if path.exists() {
        path
    } else {
        manifest_path
            .parent()
            .unwrap()
            .join("tests/gold_standard")
            .join(format!("{}.json", name))
    }
}

/// Verify that a streaming indicator matches the expected output.
pub fn assert_indicator_parity<I>(mut indicator: I, input: &[f64], expected: &[Option<f64>])
where
    I: Next<f64, Output = f64>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        match expected[i] {
            Some(exp) => approx::assert_relative_eq!(result, exp, epsilon = 1e-6),
            None => assert!(result.is_nan(), "Expected NaN at index {}", i),
        }
    }
}

pub fn assert_indicator_parity_vec<I>(
    mut indicator: I,
    input: &[f64],
    expected: &[Vec<Option<f64>>],
) where
    I: Next<f64, Output = Vec<f64>>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        for (j, &v) in result.iter().enumerate() {
            match expected[i][j] {
                Some(exp) => approx::assert_relative_eq!(v, exp, epsilon = 1e-6),
                None => assert!(v.is_nan(), "Expected NaN at index {},{}", i, j),
            }
        }
    }
}

pub fn assert_indicator_parity_loops<I>(
    mut indicator: I,
    input: &[(f64, f64)],
    expected: &[(Option<f64>, Option<f64>)],
) where
    I: Next<(f64, f64), Output = (f64, f64)>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        match expected[i].0 {
            Some(exp) => approx::assert_relative_eq!(result.0, exp, epsilon = 1e-6),
            None => assert!(result.0.is_nan(), "Expected NaN at index {}.0", i),
        }
        match expected[i].1 {
            Some(exp) => approx::assert_relative_eq!(result.1, exp, epsilon = 1e-6),
            None => assert!(result.1.is_nan(), "Expected NaN at index {}.1", i),
        }
    }
}

pub fn load_gold_standard_oc(name: &str) -> TestCaseOC {
    let path = get_gold_standard_path(name);
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

pub fn load_gold_standard_tuple(name: &str) -> TestCaseTuple {
    let path = get_gold_standard_path(name);
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

pub fn assert_indicator_parity_oc<I>(
    mut indicator: I,
    input: &[(f64, f64)],
    expected: &[Option<f64>],
) where
    I: Next<(f64, f64), Output = f64>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        match expected[i] {
            Some(exp) => approx::assert_relative_eq!(result, exp, epsilon = 1e-6),
            None => assert!(result.is_nan(), "Expected NaN at index {}", i),
        }
    }
}

pub fn assert_indicator_parity_tuple<I>(
    mut indicator: I,
    input: &[f64],
    expected: &[(Option<f64>, Option<f64>)],
) where
    I: Next<f64, Output = (f64, f64)>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        match expected[i].0 {
            Some(exp) => approx::assert_relative_eq!(result.0, exp, epsilon = 1e-6),
            None => assert!(result.0.is_nan(), "Expected NaN at index {}.0", i),
        }
        match expected[i].1 {
            Some(exp) => approx::assert_relative_eq!(result.1, exp, epsilon = 1e-6),
            None => assert!(result.1.is_nan(), "Expected NaN at index {}.1", i),
        }
    }
}

/// Helper for property-based testing of batch vs streaming parity.
/// This is a generic test that can be used by all indicators.
pub fn check_batch_streaming_parity<I, F>(input: Vec<f64>, mut indicator: I, batch_fn: F)
where
    I: Next<f64, Output = f64>,
    F: FnOnce(Vec<f64>) -> Vec<f64>,
{
    let batch_results = batch_fn(input.clone());
    let mut streaming_results = Vec::with_capacity(input.len());

    for val in input {
        streaming_results.push(indicator.next(val));
    }

    for (_i, (&s, &b)) in streaming_results
        .iter()
        .zip(batch_results.iter())
        .enumerate()
    {
        approx::assert_relative_eq!(s, b, epsilon = 1e-6, max_relative = 1e-6);
    }
}

// =============================================================================
// PA VALIDATION HARNESS: Synthetic Generators + Ground Truth (for 5mfc)
// Sources: MQL5 Part 21 (art 17891), Part 66 (art 22194 + bfg design),
//          Part 69 (art 22503 + r46a design), current market_structure + geometric_patterns.
// All generators produce (high,low) streams + labeled expected events for invariants.
// Used by proptests in indicator modules + future tests/test_pa_validation.rs .
// =============================================================================

/// Labeled ground-truth for a synthetic market structure test case.
/// Encodes "confirmed only after bias" rule from Part 21.
#[derive(Debug, Clone)]
pub struct SyntheticStructureCase {
    pub data: Vec<(f64, f64)>,
    pub expected_flips: Vec<(usize, bool)>, // (bar, is_bearish)
    pub final_bias: Bias,
    pub description: &'static str,
}

/// Generate a clean bullish structure sequence (HL, HH, HL) ending in confirmed bearish flip (LH after bias).
/// Uses fixed swing window; adds optional gaussian-ish noise.
/// Matches the "bias >=2 before flip" + min_swing_distance logic in market_structure.
pub fn generate_bullish_structure_confirmed_flip(
    _swing_strength: usize,
    noise_std: f64,
    seed: u64,
) -> SyntheticStructureCase {
    // Deterministic construction (seed for reproducibility in proptests)
    let mut rng_state = seed;
    let mut next_rand = || {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((rng_state >> 32) as f64 / u32::MAX as f64 - 0.5) * noise_std
    };

    let base: Vec<f64> = (0..40).map(|i| 100.0 + (i as f64 * 0.8)).collect();
    let mut highs: Vec<f64> = base.iter().map(|&p| p + 0.5 + next_rand()).collect();
    let lows: Vec<f64> = base.iter().map(|&p| p - 0.5 + next_rand()).collect();

    // Force clear structure points at known bars (adjusted for lag ~ strength)
    // Simplified: create sequence that produces HH/HL then LH after bias established.
    // (Real validation uses the exact is_swing_* window logic; this produces observable flips in practice.)
    let flip_bar = 25usize;
    if flip_bar < highs.len() {
        highs[flip_bar] = highs[flip_bar - 3] - 1.5; // force LH
    }

    let data: Vec<(f64, f64)> = highs
        .into_iter()
        .zip(lows)
        .map(|(h, l)| (h.max(l), l.min(h)))
        .collect();

    SyntheticStructureCase {
        data,
        expected_flips: vec![(flip_bar, true)], // approximate; exact depends on lag but test asserts presence + bias rule
        final_bias: Bias::Bearish,
        description: "bullish HL/HH sequence + confirmed LH flip after bias>=2 (Part 21 rule)",
    }
}

/// Generate a clean bearish structure with confirmed bullish flip (HL after bear bias).
pub fn generate_bearish_structure_confirmed_flip(
    _swing_strength: usize,
    noise_std: f64,
    seed: u64,
) -> SyntheticStructureCase {
    let mut rng_state = seed;
    let mut next_rand = || {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((rng_state >> 32) as f64 / u32::MAX as f64 - 0.5) * noise_std
    };

    let base: Vec<f64> = (0..40).map(|i| 120.0 - (i as f64 * 0.7)).collect();
    let highs: Vec<f64> = base.iter().map(|&p| p + 0.4 + next_rand()).collect();
    let mut lows: Vec<f64> = base.iter().map(|&p| p - 0.4 + next_rand()).collect();

    let flip_bar = 28usize;
    if flip_bar < lows.len() {
        lows[flip_bar] = lows[flip_bar - 3] + 1.2; // force HL
    }

    let data: Vec<(f64, f64)> = highs
        .into_iter()
        .zip(lows)
        .map(|(h, l)| (h.max(l), l.min(h)))
        .collect();

    SyntheticStructureCase {
        data,
        expected_flips: vec![(flip_bar, false)],
        final_bias: Bias::Bullish,
        description: "bearish LL/LH sequence + confirmed HL flip after bias>=2 (Part 21)",
    }
}

/// Ground truth for geometric pattern synthetic (Flags + H&S).
#[derive(Debug, Clone)]
pub struct SyntheticGeometricCase {
    pub data: Vec<(f64, f64)>,
    pub expected_flags: Vec<FlagPatternGroundTruth>,
    pub expected_hs: Vec<HsPatternGroundTruth>,
    pub description: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlagPatternGroundTruth {
    pub is_bull: bool,
    pub pole_length_atr_min: f64,  // must satisfy > MinPoleATR
    pub max_retrace_observed: f64, // <= 61.8
    pub pullbacks_gt_pushes: bool,
    pub breakout_bar_approx: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HsPatternGroundTruth {
    pub is_bearish: bool,
    pub head_dominance: bool,
    pub shoulder_symmetry: f64, // |ls-rs|/head <= 0.02
    pub score_min: f64,         // >=60
    pub height_atr_min: f64,    // >=1.5
}

/// Generate clean bullish flag: strong 3-bar pole (body sum >> ATR), then consolidation with pullbacks>pushes, retrace<61.8%, clean breakout.
/// Directly encodes r46a / Part69 rules (MinPoleATR~1.0, MaxRetrace=61.8, MinFlagBars=4, pullback>push).
pub fn generate_clean_bull_flag(_swing_strength: usize, atr_proxy: f64) -> SyntheticGeometricCase {
    // Construct ~30 bars: strong up impulse (3 bars large bodies), then 8 bars consolidation (higher lows = pullbacks for bull), then breakout.
    let mut highs = vec![100.0; 30];
    let mut lows = vec![99.0; 30];

    // Pole bars 5-7: strong directional bodies (close-open sum proxy via hi/lo range)
    for i in 5..8 {
        highs[i] = 100.0 + (i as f64 - 4.0) * 1.8 * atr_proxy;
        lows[i] = 100.0 + (i as f64 - 5.0) * 0.9 * atr_proxy;
    }
    let pole_end = 7;
    let pole_high = highs[pole_end];
    let pole_low = lows[5];
    let pole_len = pole_high - pole_low;
    let max_extreme = pole_high - pole_len * 0.55; // keep retrace under Part 69 61.8% cap
    // Consolidation: lower highs (pullbacks) + shallow higher lows (pushes) per Part 69.
    for i in 8..15 {
        let t = (i - 8) as f64;
        highs[i] = pole_high - t * 0.04 * atr_proxy;
        lows[i] = max_extreme + t * 0.01 * atr_proxy;
    }
    // Breakout bar (index 15 = bar 16) — immediately after min_flag_bars consolidation
    highs[15] = pole_high + 0.5 * atr_proxy;
    lows[15] = highs[15] - 0.2 * atr_proxy;
    for i in 16..30 {
        highs[i] = highs[15];
        lows[i] = lows[15];
    }

    let data: Vec<_> = highs
        .into_iter()
        .zip(lows)
        .map(|(h, l)| (h.max(l), l.min(h)))
        .collect();

    SyntheticGeometricCase {
        data,
        expected_flags: vec![FlagPatternGroundTruth {
            is_bull: true,
            pole_length_atr_min: 1.2,
            max_retrace_observed: 0.55,
            pullbacks_gt_pushes: true,
            breakout_bar_approx: 16,
        }],
        expected_hs: vec![],
        description: "clean bull flag per Part69: 3-bar impulse pole, pullbacks>pushes, retrace<61.8, breakout at pole high (r46a invariants)",
    }
}

/// Generate a perfect symmetric flat-neck H&S (bearish) that must score high and be detected.
/// Encodes bfg/Part66: 5-swing H L H L H, head dominance, shoulder tol 0.02, height >=1.5 ATR, slope~0, time sym high, score>=60 via ComputePatternScore weights.
pub fn generate_perfect_bear_hs(_atr_proxy: f64) -> SyntheticGeometricCase {
    // 5 key swings around bars 10,15,20,25,30 : H L H L H with head highest, shoulders ~equal, neck flat.
    let mut highs = vec![100.0; 45];
    let mut lows = vec![99.0; 45];

    let ls_bar = 10;
    let neck1 = 12;
    let head = 20;
    let neck2 = 28;
    let rs_bar = 35;
    highs[ls_bar] = 102.0;
    lows[neck1] = 100.0;
    highs[head] = 108.0;
    lows[neck2] = 100.2;
    highs[rs_bar] = 102.1;

    // Fill some bars for swing detection window (strength ~3)
    for i in (ls_bar - 3)..(rs_bar + 3) {
        if i < highs.len() {
            if highs[i] < 101.0 {
                highs[i] = 101.0 + (i as f64 % 3.0) * 0.1;
            }
        }
    }

    let data: Vec<_> = highs
        .into_iter()
        .zip(lows)
        .map(|(h, l)| (h.max(l), l.min(h)))
        .collect();

    SyntheticGeometricCase {
        data,
        expected_flags: vec![],
        expected_hs: vec![HsPatternGroundTruth {
            is_bearish: true,
            head_dominance: true,
            shoulder_symmetry: 0.001, // very close
            score_min: 85.0,
            height_atr_min: 1.6,
        }],
        description: "perfect flat-neck bear H&S (Part66/bfg): head>shoulders, sym<=0.02, height>1.5ATR, slope~0, high time/price score via ComputePatternScore",
    }
}

/// Generate a violation case (retrace too deep for flag) — harness must ensure detector rejects.
pub fn generate_flag_violation_retrace_too_deep() -> SyntheticGeometricCase {
    // Same as clean but force deep retrace >61.8%
    let mut case = generate_clean_bull_flag(2, 1.0);
    // Deep dip during consolidation invalidates the flag (Part 69 MaxRetracePercent).
    if case.data.len() > 14 {
        let (_, pole_low) = case.data[5];
        case.data[12].1 = pole_low - 2.0;
    }
    case.description = "flag violation: retrace >61.8% of pole — must be rejected per r46a ValidateFlagConsolidation";
    case.expected_flags.clear(); // ground truth: no valid flag
    case
}
