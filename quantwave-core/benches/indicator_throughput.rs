//! Criterion throughput for core streaming indicators.
//!
//! Run: `cargo bench -p quantwave-core --bench indicator_throughput`

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use quantwave_core::Next;
use quantwave_core::indicators::incremental::bbands::BBANDS;
use quantwave_core::indicators::ma_type::MaType;
use quantwave_core::indicators::momentum::RSI;
use quantwave_core::indicators::smoothing::SMA;
use quantwave_core::indicators::statistics::TaSTDDEV;
use quantwave_core::indicators::supertrend::SuperTrend;
use std::hint::black_box;

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

/// Guards the cost of the quantwave-qkft stable variance accumulator: it does
/// one O(period) exact refresh per `period` bars on top of the O(1) update.
fn bench_stddev(c: &mut Criterion) {
    let data = close_series(BENCH_ROWS);
    let mut group = c.benchmark_group("indicator_streaming");
    group.throughput(Throughput::Elements(BENCH_ROWS as u64));
    for period in [14usize, 50, 200] {
        group.bench_with_input(BenchmarkId::new("TaSTDDEV", period), &data, |b, data| {
            b.iter(|| {
                let mut sd = TaSTDDEV::new(period, 1.0);
                for &x in data {
                    black_box(sd.next(x));
                }
            });
        });
    }
    group.finish();
}

fn bench_bbands(c: &mut Criterion) {
    let data = close_series(BENCH_ROWS);
    let mut group = c.benchmark_group("indicator_streaming");
    group.throughput(Throughput::Elements(BENCH_ROWS as u64));
    group.bench_with_input(BenchmarkId::new("BBANDS_SMA", 20), &data, |b, data| {
        b.iter(|| {
            let mut bb = BBANDS::new(20, 2.0, 2.0, MaType::Sma);
            for &x in data {
                black_box(bb.next(x));
            }
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_sma,
    bench_rsi,
    bench_supertrend,
    bench_stddev,
    bench_bbands
);
criterion_main!(benches);
