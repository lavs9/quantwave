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
        "{{\"bars\":{},\"supertrend\":{},\"rsi\":{},\"ema\":{},\"cyber_cycle\":{},\"renko\":{},\"kagi\":{},\"harmonic_d_bar_price_score\":{},\"harmonic_count\":{}}}",
        bits(&flat),
        bits(&o_st),
        bits(&o_rsi),
        bits(&o_ema),
        bits(&o_cc),
        bits(&o_renko),
        bits(&o_kagi),
        bits(&o_harm),
        n_patterns
    );
}
