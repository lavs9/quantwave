//! Criterion throughput for core streaming indicators.
//!
//! Run: `cargo bench -p quantwave-core --bench indicator_throughput`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use quantwave_core::indicators::momentum::RSI;
use quantwave_core::indicators::smoothing::SMA;
use quantwave_core::indicators::supertrend::SuperTrend;
use quantwave_core::Next;

const BENCH_ROWS: usize = 100_000;

fn close_series(n: usize) -> Vec<f64> {
    (0..n).map(|i| 100.0 + (i as f64) * 0.001).collect()
}

fn ohlc_series(n: usize) -> Vec<(f64, f64, f64)> {
    (0..n)
        .map(|i| {
            let c = 100.0 + (i as f64) * 0.001;
            (c + 1.0, c - 1.0, c)
        })
        .collect()
}

fn bench_sma(c: &mut Criterion) {
    let data = close_series(BENCH_ROWS);
    let mut group = c.benchmark_group("indicator_streaming");
    group.throughput(Throughput::Elements(BENCH_ROWS as u64));
    group.bench_with_input(BenchmarkId::new("SMA", 20), &data, |b, data| {
        b.iter(|| {
            let mut sma = SMA::new(20);
            for &x in data {
                black_box(sma.next(x));
            }
        });
    });
    group.finish();
}

fn bench_rsi(c: &mut Criterion) {
    let data = close_series(BENCH_ROWS);
    let mut group = c.benchmark_group("indicator_streaming");
    group.throughput(Throughput::Elements(BENCH_ROWS as u64));
    group.bench_with_input(BenchmarkId::new("RSI", 14), &data, |b, data| {
        b.iter(|| {
            let mut rsi = RSI::new(14);
            for &x in data {
                black_box(rsi.next(x));
            }
        });
    });
    group.finish();
}

fn bench_supertrend(c: &mut Criterion) {
    let data = ohlc_series(BENCH_ROWS);
    let mut group = c.benchmark_group("indicator_streaming");
    group.throughput(Throughput::Elements(BENCH_ROWS as u64));
    group.bench_with_input(BenchmarkId::new("SuperTrend", "10_3"), &data, |b, data| {
        b.iter(|| {
            let mut st = SuperTrend::new(10, 3.0);
            for bar in data {
                black_box(st.next(*bar));
            }
        });
    });
    group.finish();
}

criterion_group!(benches, bench_sma, bench_rsi, bench_supertrend);
criterion_main!(benches);