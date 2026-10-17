//! Prints a JSON parity fixture computed by `quantwave-core` directly (not the
//! wasm shims). `tests-node/parity.mjs` feeds the same bars to the wasm build
//! and asserts bit-identical output. f64s are emitted as hex bit patterns so
//! NaN and every last ulp round-trip exactly.
use quantwave_core::indicators::cyber_cycle::CyberCycle;
use quantwave_core::indicators::incremental::rsi::RSI;
use quantwave_core::traits::Next;
use quantwave_core::{EMA, HarmonicPatternScanner, KagiBuilder, RenkoBuilder, SuperTrend};

fn bits(v: &[f64]) -> String {
    let items: Vec<String> = v
        .iter()
        .map(|x| format!("\"{:016x}\"", x.to_bits()))
        .collect();
    format!("[{}]", items.join(","))
}

/// The reviewer probe's LCG random walk (same as the shim unit tests).
fn random_walk(seed: u64, n: usize) -> Vec<(f64, f64)> {
    let mut state = seed * 7919;
    let mut next = || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((state >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let mut p = 100.0;
    (0..n)
        .map(|_| {
            p *= 1.0 + (next() - 0.5) * 0.03;
            let w = p * 0.004 * (1.0 + next());
            (p + w, p - w)
        })
        .collect()
}

/// Harmonic series: (name, swing_strength, high/low bars). Three random walks
/// that together hit every kind in both directions, plus a monotonic no-match.
/// (name, swing_strength, high/low bars).
type HarmonicSeries = (String, usize, Vec<(f64, f64)>);

fn harmonic_series() -> Vec<HarmonicSeries> {
    let mut out: Vec<HarmonicSeries> = [(6u64, 1usize), (4, 3), (5, 2)]
        .iter()
        .map(|&(seed, ss)| {
            (
                format!("randwalk_seed{seed}_ss{ss}"),
                ss,
                random_walk(seed, 3000),
            )
        })
        .collect();
    let mono = (0..500)
        .map(|i| {
            let p = 100.0 + 0.1 * i as f64;
            (p + 0.5, p - 0.5)
        })
        .collect();
    out.push(("no_match_monotonic".to_string(), 2, mono));
    out
}

/// Each pattern as `{"kind": as_str(), "bull": bool, "f": [14 hex fields]}`,
/// built from the core struct independently of the wasm `encode_pattern`.
fn harmonic_json() -> String {
    let mut series = vec![];
    for (name, ss, bars) in harmonic_series() {
        let mut sc = HarmonicPatternScanner::new(ss);
        let mut pats = vec![];
        for &(h, l) in &bars {
            for p in sc.next((h, l)) {
                let f = [
                    p.x_bar.map_or(f64::NAN, |b| b as f64),
                    p.x_price.unwrap_or(f64::NAN),
                    p.a_bar as f64,
                    p.a_price,
                    p.b_bar as f64,
                    p.b_price,
                    p.c_bar as f64,
                    p.c_price,
                    p.d_bar as f64,
                    p.d_price,
                    p.score,
                    p.prz_low,
                    p.prz_high,
                ];
                pats.push(format!(
                    "{{\"kind\":\"{}\",\"bull\":{},\"f\":{}}}",
                    p.kind.as_str(),
                    p.is_bull,
                    bits(&f)
                ));
            }
        }
        let flat: Vec<f64> = bars.iter().flat_map(|&(h, l)| [h, l]).collect();
        series.push(format!(
            "{{\"name\":\"{name}\",\"ss\":{ss},\"bars\":{},\"patterns\":[{}]}}",
            bits(&flat),
            pats.join(",")
        ));
    }
    format!("[{}]", series.join(","))
}

fn main() {
    // Deterministic OHLC: two sines + drift (pattern-rich, with flat-ish stretches).
    let n = 400;
    let bars: Vec<(f64, f64, f64)> = (0..n)
        .map(|i| {
            let t = i as f64;
            let c = 100.0 + 10.0 * (t / 9.0).sin() + 4.0 * (t / 3.7).sin() + 0.02 * t;
            (c + 0.6 + 0.3 * (t / 2.3).cos().abs(), c - 0.6, c)
        })
        .collect();
    let mut st = SuperTrend::new(10, 3.0);
    let mut rsi = RSI::new(14);
    let mut ema = EMA::new(20);
    let mut cc = CyberCycle::new(14);
    let mut renko = RenkoBuilder::new(1.5);
    let mut kagi = KagiBuilder::new(2.0);
    let mut harm = HarmonicPatternScanner::new(2);
    let (mut o_st, mut o_rsi, mut o_ema, mut o_cc) = (vec![], vec![], vec![], vec![]);
    let (mut o_renko, mut o_kagi, mut o_harm) = (vec![], vec![], vec![]);
    let mut n_patterns = 0usize;
    for &(h, l, c) in &bars {
        let (v, d) = st.next((h, l, c));
        o_st.extend([v, d as f64]);
        o_rsi.push(rsi.next(c));
        o_ema.push(ema.next(c));
        let (a, b) = cc.next(c);
        o_cc.extend([a, b]);
        for b in renko.next(c) {
            o_renko.extend([b.open, b.close, f64::from(b.direction)]);
        }
        if let Some(k) = kagi.next(c) {
            o_kagi.extend([
                k.open,
                k.close,
                f64::from(k.direction),
                f64::from(k.thickness),
            ]);
        }
        for p in harm.next((h, l)) {
            n_patterns += 1;
            o_harm.extend([p.d_bar as f64, p.d_price, p.score]);
        }
    }
    let flat: Vec<f64> = bars.iter().flat_map(|&(h, l, c)| [h, l, c]).collect();
    println!(
        "{{\"bars\":{},\"supertrend\":{},\"rsi\":{},\"ema\":{},\"cyber_cycle\":{},\"renko\":{},\"kagi\":{},\"harmonic_d_bar_price_score\":{},\"harmonic_count\":{},\"harmonic_series\":{}}}",
        bits(&flat),
        bits(&o_st),
        bits(&o_rsi),
        bits(&o_ema),
        bits(&o_cc),
        bits(&o_renko),
        bits(&o_kagi),
        bits(&o_harm),
        n_patterns,
        harmonic_json()
    );
}
