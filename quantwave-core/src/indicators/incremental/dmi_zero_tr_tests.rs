//! Zero-true-range regression tests (quantwave-ygg6).
//!
//! Spec: when the Wilder-smoothed true range is zero, +DI = -DI = 0; when
//! +DI + -DI = 0, DX = 0; ADX keeps Wilder-smoothing those zeros (no hold),
//! and the ADX seed is the mean of the first `n` DX values (divide by `n`).

use super::*;

/// Fixture builders: flat bars at 10/10/10, then moving bars.
pub(crate) fn moving_bar(i: usize) -> (f64, f64, f64) {
    let c = 10.0 + (i as f64 * 0.7).sin() * 3.0 + i as f64 * 0.1;
    (c + 1.0, c - 1.0, c)
}

/// `flat` leading zero-range bars, then `moving` bars.
pub(crate) fn flat_then_moving(flat: usize, moving: usize) -> Vec<(f64, f64, f64)> {
    (0..flat)
        .map(|_| (10.0, 10.0, 10.0))
        .chain((0..moving).map(moving_bar))
        .collect()
}

/// Independent per-bar reference written from the Wilder formula and the
/// ygg6 spec. Returns (plus_di, minus_di, dx, adx, adxr).
#[allow(clippy::type_complexity)]
pub(crate) fn reference(
    bars: &[(f64, f64, f64)],
    n: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let len = bars.len();
    let nf = n as f64;
    let mut pdi = vec![f64::NAN; len];
    let mut mdi = vec![f64::NAN; len];
    let mut dx = vec![f64::NAN; len];
    let mut adx = vec![f64::NAN; len];
    let mut adxr = vec![f64::NAN; len];
    let (mut s_tr, mut s_p, mut s_m) = (0.0, 0.0, 0.0);
    for i in 1..len {
        let (h, l, _) = bars[i];
        let (ph, pl, pc) = bars[i - 1];
        let tr = (h - l).max((h - pc).abs()).max((l - pc).abs());
        let up = h - ph;
        let down = pl - l;
        let p = if up > down && up > 0.0 { up } else { 0.0 };
        let m = if down > up && down > 0.0 { down } else { 0.0 };
        if i < n {
            s_tr += tr;
            s_p += p;
            s_m += m;
            continue;
        }
        s_tr = s_tr - s_tr / nf + tr;
        s_p = s_p - s_p / nf + p;
        s_m = s_m - s_m / nf + m;
        let (a, b) = if s_tr == 0.0 {
            (0.0, 0.0)
        } else {
            (100.0 * s_p / s_tr, 100.0 * s_m / s_tr)
        };
        pdi[i] = a;
        mdi[i] = b;
        dx[i] = if a + b == 0.0 {
            0.0
        } else {
            100.0 * (a - b).abs() / (a + b)
        };
    }
    let seed_bar = 2 * n - 1;
    if len > seed_bar {
        adx[seed_bar] = dx[n..=seed_bar].iter().sum::<f64>() / nf;
        for i in seed_bar + 1..len {
            adx[i] = (adx[i - 1] * (nf - 1.0) + dx[i]) / nf;
        }
    }
    for i in (3 * n - 2)..len {
        adxr[i] = (adx[i] + adx[i + 1 - n]) / 2.0;
    }
    (pdi, mdi, dx, adx, adxr)
}

#[allow(clippy::type_complexity)]
pub(crate) fn streaming(
    bars: &[(f64, f64, f64)],
    n: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut p = PLUS_DI::new(n);
    let mut m = MINUS_DI::new(n);
    let mut d = DX::new(n);
    let mut a = ADX::new(n);
    let mut r = ADXR::new(n);
    let mut out = (vec![], vec![], vec![], vec![], vec![]);
    for &b in bars {
        out.0.push(p.next(b));
        out.1.push(m.next(b));
        out.2.push(d.next(b));
        out.3.push(a.next(b));
        out.4.push(r.next(b));
    }
    out
}

fn assert_series_eq(name: &str, got: &[f64], want: &[f64]) {
    assert_eq!(got.len(), want.len());
    for (i, (g, w)) in got.iter().zip(want).enumerate() {
        if w.is_nan() {
            assert!(g.is_nan(), "{name}[{i}]: expected NaN, got {g}");
        } else {
            assert!(
                (g - w).abs() <= 1e-9 * w.abs().max(1.0),
                "{name}[{i}]: expected {w}, got {g}"
            );
        }
    }
}

const N: usize = 14;

/// Zero stretch over the ADX seed bar (2n-1) and beyond into post-seed bars.
fn fixture_over_seed_and_post_seed() -> Vec<(f64, f64, f64)> {
    flat_then_moving(60, 80)
}

/// Zero stretch covering the TR seed and ending mid-way through the DX seed window.
fn fixture_mid_seed() -> Vec<(f64, f64, f64)> {
    flat_then_moving(N + 5, 80)
}

#[test]
fn regression_adx_reseeds_after_zero_tr_stretch() {
    let bars = fixture_over_seed_and_post_seed();
    let (_, _, _, adx, adxr) = streaming(&bars, N);
    assert_eq!(adx[2 * N - 1], 0.0, "ADX must seed (to 0) on the seed bar");
    let last_adx = adx[adx.len() - 1];
    assert!(last_adx.is_finite() && last_adx > 0.0);
    assert!(adxr[adxr.len() - 1].is_finite());
}

#[test]
fn zero_tr_bars_give_zero_di_and_dx_not_nan() {
    let bars = fixture_over_seed_and_post_seed();
    let (pdi, mdi, dx, adx, _) = streaming(&bars, N);
    for i in N..60 {
        assert_eq!(pdi[i], 0.0, "+DI[{i}]");
        assert_eq!(mdi[i], 0.0, "-DI[{i}]");
        assert_eq!(dx[i], 0.0, "DX[{i}]");
    }
    for v in &adx[2 * N - 1..60] {
        assert_eq!(*v, 0.0);
    }
}

#[test]
fn zero_tr_streaming_matches_reference_formula() {
    for (label, bars) in [
        ("over_seed_and_post_seed", fixture_over_seed_and_post_seed()),
        ("mid_seed", fixture_mid_seed()),
        ("no_flat", flat_then_moving(0, 80)),
    ] {
        let s = streaming(&bars, N);
        let r = reference(&bars, N);
        assert_series_eq(&format!("{label}/plus_di"), &s.0, &r.0);
        assert_series_eq(&format!("{label}/minus_di"), &s.1, &r.1);
        assert_series_eq(&format!("{label}/dx"), &s.2, &r.2);
        assert_series_eq(&format!("{label}/adx"), &s.3, &r.3);
        assert_series_eq(&format!("{label}/adxr"), &s.4, &r.4);
    }
}

#[test]
fn mid_seed_zero_dx_counts_toward_seed_divided_by_n() {
    let bars = fixture_mid_seed();
    let (_, _, dx, adx, _) = streaming(&bars, N);
    // Flat bars 0..N+5 -> DX is 0 for bars N..N+5, then real values.
    assert!(dx[N..N + 5].iter().all(|v| *v == 0.0));
    let expected_seed = dx[N..2 * N].iter().sum::<f64>() / N as f64;
    assert!((adx[2 * N - 1] - expected_seed).abs() < 1e-12);
}
