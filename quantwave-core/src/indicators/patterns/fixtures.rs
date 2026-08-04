//! Hand-built candlestick fixtures spliced into the parity random walk.
//!
//! Rare multi-bar patterns never fire on an uncorrelated random walk, so a
//! parity test that only sees the walk compares all-zeros to all-zeros and
//! proves nothing. `test_pattern_parity!` therefore asserts the oracle emitted
//! at least one non-zero signal; the fixtures here supply that signal by
//! overwriting a slice of the series with bars that satisfy the pattern's
//! preconditions.
//!
//! Every fixture writes [`PRIME_LEN`] identical [`PRIME`] bars first. TA-Lib's
//! candle settings are rolling averages over the *preceding* bars
//! (BODY_*/SHADOW_SHORT/SHADOW_VERY_SHORT over 10, NEAR/FAR/EQUAL over 5), so
//! the priming run pins those averages to known values before the pattern bars
//! land:
//!
//! | setting            | value with `PRIME` priming |
//! |--------------------|----------------------------|
//! | BODY_LONG / SHORT  | 1.00                       |
//! | BODY_DOJI          | 0.18                       |
//! | SHADOW_SHORT       | 0.40                       |
//! | SHADOW_VERY_SHORT  | 0.18                       |
//! | NEAR               | 0.36                       |
//! | FAR                | 1.08                       |
//! | EQUAL              | 0.09                       |
//!
//! Averages drift slightly as the pattern's own bars enter the window; each
//! fixture leaves headroom for that. SHADOW_LONG/SHADOW_VERY_LONG use
//! `avg_period == 0` and are measured against the current bar's own real body.

/// `(open, high, low, close)`.
pub type Bar = (f64, f64, f64, f64);

/// Index the fixture is spliced at — well past every pattern's warmup.
const SPLICE: usize = 1000;

/// Number of priming bars written before the pattern bars.
const PRIME_LEN: usize = 12;

/// Priming bar: body 1.0, high-low range 1.8, both shadows 0.4.
const PRIME: Bar = (100.0, 101.4, 99.6, 101.0);

/// [`PRIME`] shifted *down* 0.4 — same body, range and shadows, lower level.
///
/// Alternating the two keeps every candle average identical while making
/// consecutive priming bars fail both "opens are equal" and "each open is above
/// the previous open" tests, so the priming run itself cannot trip a pattern.
const PRIME_SHIFTED: Bar = (99.6, 101.0, 99.2, 100.6);

#[inline]
fn put(o: &mut [f64], h: &mut [f64], l: &mut [f64], c: &mut [f64], i: usize, bar: Bar) {
    o[i] = bar.0;
    h[i] = bar.1;
    l[i] = bar.2;
    c[i] = bar.3;
}

/// Overwrite `PRIME_LEN` priming bars followed by `bars`, starting at [`SPLICE`].
///
/// No-op when the series is shorter than the splice needs — the macro also
/// applies fixtures to the short proptest walks, where coverage is not asserted.
fn splice_primed(
    o: &mut [f64],
    h: &mut [f64],
    l: &mut [f64],
    c: &mut [f64],
    prime_cycle: &[Bar],
    bars: &[Bar],
) {
    if o.len() < SPLICE + PRIME_LEN + bars.len() {
        return;
    }
    for i in 0..PRIME_LEN {
        put(o, h, l, c, SPLICE + i, prime_cycle[i % prime_cycle.len()]);
    }
    for (j, bar) in bars.iter().enumerate() {
        put(o, h, l, c, SPLICE + PRIME_LEN + j, *bar);
    }
}

/// Generates a fixture function with the signature the macro expects.
macro_rules! fixture {
    ($(#[$meta:meta])* $name:ident, [$($bar:expr),* $(,)?]) => {
        fixture!($(#[$meta])* $name, prime: [PRIME], [$($bar),*]);
    };
    ($(#[$meta:meta])* $name:ident, prime: [$($p:expr),* $(,)?], [$($bar:expr),* $(,)?]) => {
        $(#[$meta])*
        // `test_pattern_parity!` coerces the fixture to
        // `fn(&mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>)`, so the
        // signature has to be `&mut Vec` even though a slice would do here.
        #[allow(clippy::ptr_arg)]
        pub fn $name(
            o: &mut Vec<f64>,
            h: &mut Vec<f64>,
            l: &mut Vec<f64>,
            c: &mut Vec<f64>,
        ) {
            splice_primed(o, h, l, c, &[$($p),*], &[$($bar),*]);
        }
    };
}

// =============================================================================
// Two-bar patterns
// =============================================================================

fixture!(
    /// Long black then long white closing at the same level (`EQUAL` = 0.09).
    counterattack,
    [
        (101.0, 101.2, 96.8, 97.0), // long black, body 4
        (93.0, 97.2, 92.8, 97.0),   // long white, identical close
    ]
);

fixture!(
    /// Long white, then black opening above its high and closing past the midpoint.
    darkcloudcover,
    [
        (97.0, 101.2, 96.8, 101.0),   // long white, body 4
        (101.5, 101.7, 98.3, 98.5),   // opens > 101.2, closes < 101 - 2
    ]
);

fixture!(
    /// Long white, then a doji gapping up over its body.
    dojistar,
    [
        (97.0, 101.2, 96.8, 101.0),   // long white
        (102.0, 102.2, 101.8, 102.0), // doji, real-body gap up
    ]
);

fixture!(
    /// Long white, then a short body entirely inside it.
    harami,
    [
        (97.0, 101.2, 96.8, 101.0),
        (99.5, 99.9, 99.3, 99.7), // body 0.2, inside 97..101
    ]
);

fixture!(
    /// Long white, then a doji entirely inside its body.
    haramicross,
    [
        (97.0, 101.2, 96.8, 101.0),
        (99.6, 99.8, 99.4, 99.6), // doji, inside 97..101
    ]
);

fixture!(
    /// Long black, then a short black engulfed by the prior body.
    homingpigeon,
    [
        (101.0, 101.2, 96.8, 97.0),
        (99.5, 99.7, 99.1, 99.3), // opens lower, closes higher
    ]
);

fixture!(
    /// Long black, then white opening below its low and closing at its close.
    inneck,
    [
        (101.0, 101.2, 96.8, 97.0),
        (96.5, 97.2, 96.3, 97.02), // close within EQUAL of 97
    ]
);

fixture!(
    /// Small body gapping down, with a long upper and a very short lower shadow.
    invertedhammer,
    [
        (98.5, 99.5, 98.4, 98.7), // body 0.2, upper 0.8, lower 0.1
    ]
);

fixture!(
    /// Long black, then white opening below its low and closing at that low.
    onneck,
    [
        (101.0, 101.2, 96.8, 97.0),
        (96.5, 97.0, 96.4, 96.8), // close == prior low
    ]
);

fixture!(
    /// Two opposite marubozu bodies separated by a gap.
    kicking,
    [
        (101.0, 101.0, 98.0, 98.0),   // black marubozu, body 3
        (102.0, 105.0, 102.0, 105.0), // white marubozu, opens above
    ]
);

fixture!(
    /// Same bars as [`kicking`]; equal bodies make the later candle's colour win.
    kickingbylength,
    [
        (101.0, 101.0, 98.0, 98.0),
        (102.0, 105.0, 102.0, 105.0),
    ]
);

fixture!(
    /// Long black, then a long white piercing more than halfway back.
    piercing,
    [
        (101.0, 101.2, 96.8, 97.0),
        (96.5, 99.7, 96.3, 99.5), // opens < low, closes > 97 + 2
    ]
);

fixture!(
    /// Small body gapping up, long upper shadow, very short lower shadow.
    shootingstar,
    [
        (101.5, 102.5, 101.45, 101.7), // gaps over the priming body top (101)
    ]
);

fixture!(
    /// Long black, then white opening below the low but stalling below midpoint.
    thrusting,
    [
        (101.0, 101.2, 96.8, 97.0),
        (96.5, 98.7, 96.3, 98.5), // 97.09 < close <= 99
    ]
);

// =============================================================================
// Three-bar patterns
// =============================================================================

fixture!(
    /// Long white, then two black candles inside its range.
    two_crows,
    [
        (97.0, 101.2, 96.8, 101.0),   // long white
        (103.0, 103.2, 101.8, 102.0), // black gapping up
        (102.5, 102.7, 98.8, 99.0),   // black closing inside the white body
    ]
);

fixture!(
    /// Three descending black candles, each opening inside the prior body,
    /// all with zero lower shadow.
    three_black_crows,
    [
        (101.0, 101.2, 99.0, 99.0),
        (100.0, 100.2, 97.5, 97.5),
        (99.0, 99.2, 96.0, 96.0),
    ]
);

fixture!(
    /// Long white, a short body inside it, then black closing below its open.
    three_inside,
    [
        (97.0, 101.2, 96.8, 101.0),
        (99.0, 99.5, 98.8, 99.3), // harami
        (99.0, 99.2, 95.8, 96.0), // closes below 97
    ]
);

fixture!(
    /// Long black with a long lower shadow, then two shrinking black bodies,
    /// the last with no lower shadow.
    three_stars_in_south,
    [
        (101.0, 101.1, 96.0, 99.0),   // body 2, lower shadow 3
        (100.5, 100.6, 95.5, 99.5),   // inside the body, lower low
        (100.2, 100.3, 99.8, 99.8),   // inside, lower shadow 0
    ]
);

fixture!(
    /// Long black, an island doji, then a long white closing well back up.
    abandonedbaby,
    [
        (101.0, 101.2, 96.8, 97.0),
        (96.0, 96.1, 95.9, 96.0),  // doji, high < prior low
        (96.6, 99.2, 96.5, 99.0),  // low > doji high, close > 97 + 1.2
    ]
);

fixture!(
    /// Long white, a doji gapping above it, then black closing 30% back in.
    eveningdojistar,
    [
        (97.0, 101.2, 96.8, 101.0),
        (102.0, 102.2, 101.8, 102.0),
        (101.5, 101.7, 98.8, 99.0),
    ]
);

fixture!(
    /// Long white, a short body gapping above it, then black closing 30% back in.
    eveningstar,
    [
        (97.0, 101.2, 96.8, 101.0),
        (102.0, 102.5, 101.8, 102.3),
        (101.5, 101.7, 98.8, 99.0),
    ]
);

fixture!(
    /// Long black, a doji gapping below it, then white closing 30% back up.
    morningdojistar,
    [
        (101.0, 101.2, 96.8, 97.0),
        (96.0, 96.2, 95.8, 96.0),
        (96.5, 99.2, 96.3, 99.0),
    ]
);

fixture!(
    /// Long black, a short body gapping below it, then white closing 30% back up.
    morningstar,
    [
        (101.0, 101.2, 96.8, 97.0),
        (96.0, 96.2, 95.5, 95.7),
        (96.5, 99.2, 96.3, 99.0),
    ]
);

fixture!(
    /// Two side-by-side white bodies of near-equal size and near-equal open,
    /// gapping up over the (white) priming bar.
    ///
    /// The alternating priming run matters here: with a single repeated priming
    /// bar, two consecutive identical white candles already satisfy the oracle's
    /// "equal opens, equal bodies" test and it fires on the priming run itself.
    gapsidesidewhite,
    prime: [PRIME, PRIME_SHIFTED],
    [
        (102.0, 103.2, 101.8, 103.0),   // gaps up over the priming body, body 1.00
        (102.05, 103.3, 101.95, 103.1), // opens within EQUAL, body 1.05
    ]
);

fixture!(
    /// White, a gapped-up white, then black filling part of the gap.
    tasukigap,
    [
        PRIME,
        (102.0, 103.2, 101.8, 103.0),
        (102.7, 102.9, 101.6, 101.8),
    ]
);

fixture!(
    /// Three doji, the second gapping up and the third not.
    tristar,
    [
        (100.0, 100.2, 99.8, 100.0),
        (101.0, 101.2, 100.8, 101.0),
        (101.0, 101.2, 100.8, 101.0),
    ]
);

fixture!(
    /// Long black, a harami black with a lower low, then a small white.
    unique3river,
    [
        (101.0, 101.2, 96.8, 97.0),
        (100.0, 100.2, 96.5, 98.0),
        (97.5, 98.0, 97.3, 97.8),
    ]
);

fixture!(
    /// Long white, a small black gapping up, then black engulfing it.
    upsidegap2crows,
    [
        (97.0, 101.2, 96.8, 101.0),
        (102.5, 102.7, 101.8, 102.0),
        (103.0, 103.2, 101.3, 101.5),
    ]
);

fixture!(
    /// Two white candles across a gap, then black closing back inside the first.
    xsidegap3methods,
    [
        (99.0, 101.2, 98.8, 101.0),
        (102.0, 103.2, 101.8, 103.0),
        (102.5, 102.7, 99.8, 100.0),
    ]
);

fixture!(
    /// Black, white above its close, then black closing at the first close.
    sticksandwich,
    [
        (101.0, 101.2, 98.8, 99.0),
        (99.5, 100.7, 99.3, 100.5),
        (100.0, 100.2, 98.8, 99.0),
    ]
);

// =============================================================================
// Four- and five-bar patterns
// =============================================================================

fixture!(
    /// Two marubozu blacks across a gap, a black with a long lower shadow, then
    /// a black engulfing it.
    ///
    /// Both the 2nd->1st and the 3rd->2nd real bodies gap down: the native
    /// implementation tests the former and the oracle the latter, so the fixture
    /// satisfies both.
    concealbabyswall,
    [
        (101.0, 101.0, 100.0, 100.0), // black marubozu
        (99.5, 99.5, 99.0, 99.0),     // black marubozu, gaps down
        (98.9, 99.2, 98.4, 98.8),     // gaps down again, lower shadow 0.4, high > 99.0
        (99.3, 99.4, 97.9, 98.0),     // opens >= its high, closes <= its low
    ]
);

fixture!(
    /// Long black, a gapped-down black, two more lower blacks, then a long
    /// white closing inside the gap.
    breakaway,
    [
        (101.0, 101.2, 96.8, 97.0),
        (96.0, 96.2, 95.3, 95.5),
        (95.4, 95.5, 94.8, 95.0),
        (94.9, 95.0, 94.3, 94.5),
        (94.6, 96.7, 94.4, 96.5), // 96 < close < 97
    ]
);

fixture!(
    /// Four descending blacks, the last with a long upper shadow, then a white
    /// closing above it.
    ladderbottom,
    [
        (101.0, 101.2, 99.8, 100.0),
        (100.0, 100.2, 98.8, 99.0),
        (99.0, 99.2, 97.8, 98.0),
        (98.0, 99.0, 97.3, 97.5), // upper shadow 1.0
        (98.5, 100.2, 98.3, 100.0),
    ]
);

fixture!(
    /// Long white, a gapped-up black, two more small bodies holding above the
    /// halfway mark, then a white breaking to new highs.
    mathold,
    [
        (97.0, 101.2, 96.8, 101.0),
        (102.0, 102.2, 101.3, 101.5),
        (101.0, 101.2, 100.3, 100.5),
        (100.5, 100.7, 99.8, 100.0),
        (100.2, 103.2, 100.0, 103.0),
    ]
);

fixture!(
    /// Long white, three small blacks inside its range, then a white closing
    /// above the first close.
    risefall3methods,
    [
        (97.0, 101.5, 96.5, 101.0),
        (100.8, 101.0, 100.3, 100.5),
        (100.5, 100.7, 100.0, 100.2),
        (100.2, 100.4, 99.7, 99.9),
        (100.0, 102.7, 99.8, 102.5),
    ]
);

fixture!(
    /// Two nested inside bars closing near the low, a downside break, then a
    /// close back above the inside bar's high (the confirmation).
    hikkakemod,
    [
        (100.0, 105.0, 95.0, 100.5), // outer bar
        (102.0, 103.0, 97.0, 97.2),  // inside, closes near its low
        (99.0, 102.0, 98.0, 101.0),  // inside again
        (100.0, 101.0, 97.5, 99.0),  // lower high and lower low -> setup
        (102.0, 103.2, 101.8, 103.0), // close > setup reference high -> confirm
    ]
);

#[cfg(test)]
mod self_check {
    use super::*;

    type OracleFn = fn(&[f64], &[f64], &[f64], &[f64]) -> Result<Vec<i32>, talib_rs::TaError>;
    type FixtureFn = fn(&mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>, &mut Vec<f64>);

    fn walk(n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let (mut o, mut h, mut l, mut c) = (vec![], vec![], vec![], vec![]);
        let mut rng = 1337_u32;
        let mut price = 50.0;
        for _ in 0..n {
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let drift = ((rng >> 16) as f64 / 65536.0) * 4.0 - 2.0;
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            let spread = ((rng >> 16) as f64 / 65536.0) * 5.0;
            let (open, close) = (price, price + drift);
            o.push(open);
            h.push(open.max(close) + spread);
            l.push(open.min(close) - spread);
            c.push(close);
            price = close;
        }
        (o, h, l, c)
    }

    #[test]
    fn every_fixture_makes_the_oracle_fire() {
        let cases: &[(&str, FixtureFn, OracleFn)] = &[
            (
                "counterattack",
                counterattack,
                talib_rs::pattern::cdl_counterattack,
            ),
            (
                "darkcloudcover",
                darkcloudcover,
                talib_rs::pattern::cdl_darkcloudcover,
            ),
            ("dojistar", dojistar, talib_rs::pattern::cdl_dojistar),
            ("harami", harami, talib_rs::pattern::cdl_harami),
            (
                "haramicross",
                haramicross,
                talib_rs::pattern::cdl_haramicross,
            ),
            (
                "homingpigeon",
                homingpigeon,
                talib_rs::pattern::cdl_homingpigeon,
            ),
            ("inneck", inneck, talib_rs::pattern::cdl_inneck),
            (
                "invertedhammer",
                invertedhammer,
                talib_rs::pattern::cdl_invertedhammer,
            ),
            ("onneck", onneck, talib_rs::pattern::cdl_onneck),
            ("kicking", kicking, talib_rs::pattern::cdl_kicking),
            (
                "kickingbylength",
                kickingbylength,
                talib_rs::pattern::cdl_kickingbylength,
            ),
            ("piercing", piercing, talib_rs::pattern::cdl_piercing),
            (
                "shootingstar",
                shootingstar,
                talib_rs::pattern::cdl_shootingstar,
            ),
            ("thrusting", thrusting, talib_rs::pattern::cdl_thrusting),
            ("two_crows", two_crows, talib_rs::pattern::cdl_2crows),
            (
                "three_black_crows",
                three_black_crows,
                talib_rs::pattern::cdl_3blackcrows,
            ),
            ("three_inside", three_inside, talib_rs::pattern::cdl_3inside),
            (
                "three_stars_in_south",
                three_stars_in_south,
                talib_rs::pattern::cdl_3starsinsouth,
            ),
            (
                "abandonedbaby",
                abandonedbaby,
                talib_rs::pattern::cdl_abandonedbaby,
            ),
            (
                "eveningdojistar",
                eveningdojistar,
                talib_rs::pattern::cdl_eveningdojistar,
            ),
            (
                "eveningstar",
                eveningstar,
                talib_rs::pattern::cdl_eveningstar,
            ),
            (
                "morningdojistar",
                morningdojistar,
                talib_rs::pattern::cdl_morningdojistar,
            ),
            (
                "morningstar",
                morningstar,
                talib_rs::pattern::cdl_morningstar,
            ),
            (
                "gapsidesidewhite",
                gapsidesidewhite,
                talib_rs::pattern::cdl_gapsidesidewhite,
            ),
            ("tasukigap", tasukigap, talib_rs::pattern::cdl_tasukigap),
            ("tristar", tristar, talib_rs::pattern::cdl_tristar),
            (
                "unique3river",
                unique3river,
                talib_rs::pattern::cdl_unique3river,
            ),
            (
                "upsidegap2crows",
                upsidegap2crows,
                talib_rs::pattern::cdl_upsidegap2crows,
            ),
            (
                "xsidegap3methods",
                xsidegap3methods,
                talib_rs::pattern::cdl_xsidegap3methods,
            ),
            (
                "sticksandwich",
                sticksandwich,
                talib_rs::pattern::cdl_sticksandwich,
            ),
            (
                "concealbabyswall",
                concealbabyswall,
                talib_rs::pattern::cdl_concealbabyswall,
            ),
            ("breakaway", breakaway, talib_rs::pattern::cdl_breakaway),
            (
                "ladderbottom",
                ladderbottom,
                talib_rs::pattern::cdl_ladderbottom,
            ),
            ("mathold", mathold, talib_rs::pattern::cdl_mathold),
            (
                "risefall3methods",
                risefall3methods,
                talib_rs::pattern::cdl_risefall3methods,
            ),
            ("hikkakemod", hikkakemod, talib_rs::pattern::cdl_hikkakemod),
        ];

        let mut dead = vec![];
        for (name, fixture, oracle) in cases {
            let (mut o, mut h, mut l, mut c) = walk(1200);
            fixture(&mut o, &mut h, &mut l, &mut c);
            let out = oracle(&o, &h, &l, &c).expect("oracle failed");
            let hits: Vec<(usize, i32)> = out
                .iter()
                .enumerate()
                .filter(|(i, v)| **v != 0 && (995..1035).contains(i))
                .map(|(i, v)| (i, *v))
                .collect();
            if hits.is_empty() {
                dead.push(*name);
            } else {
                println!("{name}: {hits:?}");
            }
        }
        assert!(dead.is_empty(), "fixtures that never fired: {dead:?}");
    }
}
