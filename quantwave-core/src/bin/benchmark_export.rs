//! JSON throughput export for the Python benchmark harness.
//!
//! Run: `cargo run -p quantwave-core --release --bin benchmark_export`

use quantwave_core::Next;
use quantwave_core::indicators::momentum::RSI;
use quantwave_core::indicators::smoothing::SMA;
use quantwave_core::indicators::supertrend::SuperTrend;
use std::env;
use std::time::Instant;

fn rows_from_args() -> usize {
    env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_000_000)
}

fn round_ms(v: f64) -> f64 {
    (v * 10_000.0).round() / 10_000.0
}

fn bench_sma(data: &[f64]) -> f64 {
    let mut sma = SMA::new(20);
    let t0 = Instant::now();
    for &x in data {
        let _ = sma.next(x);
    }
    round_ms(t0.elapsed().as_secs_f64() * 1000.0)
}

fn bench_rsi(data: &[f64]) -> f64 {
    let mut rsi = RSI::new(14);
    let t0 = Instant::now();
    for &x in data {
        let _ = rsi.next(x);
    }
    round_ms(t0.elapsed().as_secs_f64() * 1000.0)
}

fn bench_supertrend(data: &[(f64, f64, f64)]) -> f64 {
    let mut st = SuperTrend::new(10, 3.0);
    let t0 = Instant::now();
    for bar in data {
        let _ = st.next(*bar);
    }
    round_ms(t0.elapsed().as_secs_f64() * 1000.0)
}

fn per_tick_latency_ns<F>(mut f: F, n: usize) -> (f64, f64)
where
    F: FnMut(),
{
    let mut samples = Vec::with_capacity(n);
    for _ in 0..n {
        let t0 = Instant::now();
        f();
        samples.push(t0.elapsed().as_secs_f64() * 1e9);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mean = samples.iter().sum::<f64>() / n as f64;
    let p99_idx = ((n as f64) * 0.99).floor() as usize;
    let p99 = samples[p99_idx.min(n - 1)];
    (mean, p99)
}

fn main() {
    let rows = rows_from_args();
    let close: Vec<f64> = (0..rows).map(|i| 100.0 + (i as f64) * 0.001).collect();
    let ohlc: Vec<(f64, f64, f64)> = (0..rows)
        .map(|i| {
            let c = 100.0 + (i as f64) * 0.001;
            (c + 1.0, c - 1.0, c)
        })
        .collect();

    let (sma_mean_ns, sma_p99_ns) = {
        let mut sma = SMA::new(20);
        let mut i = 0usize;
        per_tick_latency_ns(
            || {
                let v = close[i % close.len()];
                i += 1;
                let _ = sma.next(v);
            },
            10_000,
        )
    };

    let (rsi_mean_ns, rsi_p99_ns) = {
        let mut rsi = RSI::new(14);
        let mut i = 0usize;
        per_tick_latency_ns(
            || {
                let v = close[i % close.len()];
                i += 1;
                let _ = rsi.next(v);
            },
            10_000,
        )
    };

    let out = serde_json::json!({
        "throughput": {
            "sma_20_streaming_ms": bench_sma(&close),
            "rsi_14_streaming_ms": bench_rsi(&close),
            "supertrend_10_3_streaming_ms": bench_supertrend(&ohlc),
            "rows": rows,
            "mode": "rust_streaming",
            "source": "quantwave-core/benchmark_export",
        },
        "latency": {
            "sma_20_mean_ns": sma_mean_ns,
            "sma_20_p99_ns": sma_p99_ns,
            "rsi_14_mean_ns": rsi_mean_ns,
            "rsi_14_p99_ns": rsi_p99_ns,
            "samples": 10_000,
            "source": "per_tick_instrumented",
        },
        "criterion": {
            "note": "Run `cargo bench -p quantwave-core --bench indicator_throughput` for HTML reports",
            "bench_rows": 100_000,
        },
    });
    println!("{}", out);
}
