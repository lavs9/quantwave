//! Criterion comparison of quantwave against the `talib-rs` oracle.
//!
//! Relocated from the former `src/bin/benchmark.rs`. Bin targets cannot use
//! dev-dependencies; benches can — so the talib comparison lives here, which
//! keeps `talib-rs` out of the shipped dependency graph while preserving the
//! benchmark itself.
//!
//! For each indicator three implementations are timed over the same 100k-point
//! series:
//!   * `talib` — talib-rs batch call (the reference implementation)
//!   * `qw_streaming` — quantwave incremental `Next::next` per bar
//!   * `qw_next_batch` — quantwave `Next::next_batch` over the whole slice
//!
//! Run: `cargo bench -p quantwave-core --bench talib_comparison`

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use quantwave_core::indicators::momentum::{MACD, RSI};
use quantwave_core::indicators::pattern::CDLDOJI;
use quantwave_core::indicators::smoothing::SMA;
use quantwave_core::traits::Next;
use std::hint::black_box;

const BENCH_ROWS: usize = 100_000;

/// Same synthetic series the original `src/bin/benchmark.rs` used.
fn series(n: usize) -> Vec<f64> {
    (0..n).map(|i| (i as f64).sin() * 100.0 + 100.0).collect()
}

fn bench_sma(c: &mut Criterion) {
    let data = series(BENCH_ROWS);
    let mut group = c.benchmark_group("talib_comparison/SMA_14");
    group.throughput(Throughput::Elements(BENCH_ROWS as u64));

    group.bench_with_input(BenchmarkId::new("talib", BENCH_ROWS), &data, |b, data| {
        b.iter(|| black_box(talib_rs::overlap::sma(black_box(data), 14)));
    });
    group.bench_with_input(
        BenchmarkId::new("qw_streaming", BENCH_ROWS),
        &data,
        |b, data| {
            b.iter(|| {
                let mut ind = SMA::new(14);
                for &x in data {
                    black_box(ind.next(x));
                }
            });
        },
    );
    group.bench_with_input(
        BenchmarkId::new("qw_next_batch", BENCH_ROWS),
        &data,
        |b, data| {
            b.iter(|| {
                let mut ind = SMA::new(14);
                black_box(ind.next_batch(black_box(data)));
            });
        },
    );
    group.finish();
}

fn bench_rsi(c: &mut Criterion) {
    let data = series(BENCH_ROWS);
    let mut group = c.benchmark_group("talib_comparison/RSI_14");
    group.throughput(Throughput::Elements(BENCH_ROWS as u64));

    group.bench_with_input(BenchmarkId::new("talib", BENCH_ROWS), &data, |b, data| {
        b.iter(|| black_box(talib_rs::momentum::rsi(black_box(data), 14)));
    });
    group.bench_with_input(
        BenchmarkId::new("qw_streaming", BENCH_ROWS),
        &data,
        |b, data| {
            b.iter(|| {
                let mut ind = RSI::new(14);
                for &x in data {
                    black_box(ind.next(x));
                }
            });
        },
    );
    group.bench_with_input(
        BenchmarkId::new("qw_next_batch", BENCH_ROWS),
        &data,
        |b, data| {
            b.iter(|| {
                let mut ind = RSI::new(14);
                black_box(ind.next_batch(black_box(data)));
            });
        },
    );
    group.finish();
}

fn bench_macd(c: &mut Criterion) {
    let data = series(BENCH_ROWS);
    let mut group = c.benchmark_group("talib_comparison/MACD_12_26_9");
    group.throughput(Throughput::Elements(BENCH_ROWS as u64));

    group.bench_with_input(BenchmarkId::new("talib", BENCH_ROWS), &data, |b, data| {
        b.iter(|| black_box(talib_rs::momentum::macd(black_box(data), 12, 26, 9)));
    });
    group.bench_with_input(
        BenchmarkId::new("qw_streaming", BENCH_ROWS),
        &data,
        |b, data| {
            b.iter(|| {
                let mut ind = MACD::new(12, 26, 9);
                for &x in data {
                    black_box(ind.next(x));
                }
            });
        },
    );
    group.bench_with_input(
        BenchmarkId::new("qw_next_batch", BENCH_ROWS),
        &data,
        |b, data| {
            b.iter(|| {
                let mut ind = MACD::new(12, 26, 9);
                black_box(ind.next_batch(black_box(data)));
            });
        },
    );
    group.finish();
}

fn bench_cdl_doji(c: &mut Criterion) {
    let data = series(BENCH_ROWS);
    // The original benchmark fed the same series to all four OHLC legs.
    let ohlc: Vec<(f64, f64, f64, f64)> = data.iter().map(|&x| (x, x, x, x)).collect();
    let mut group = c.benchmark_group("talib_comparison/CDLDOJI");
    group.throughput(Throughput::Elements(BENCH_ROWS as u64));

    group.bench_with_input(BenchmarkId::new("talib", BENCH_ROWS), &data, |b, data| {
        b.iter(|| {
            black_box(talib_rs::pattern::cdl_doji(
                black_box(data),
                black_box(data),
                black_box(data),
                black_box(data),
            ))
        });
    });
    group.bench_with_input(
        BenchmarkId::new("qw_streaming", BENCH_ROWS),
        &ohlc,
        |b, ohlc| {
            b.iter(|| {
                let mut ind = CDLDOJI::new();
                for &bar in ohlc {
                    black_box(ind.next(bar));
                }
            });
        },
    );
    group.bench_with_input(
        BenchmarkId::new("qw_next_batch", BENCH_ROWS),
        &ohlc,
        |b, ohlc| {
            b.iter(|| {
                let mut ind = CDLDOJI::new();
                black_box(ind.next_batch(black_box(ohlc)));
            });
        },
    );
    group.finish();
}

criterion_group!(benches, bench_sma, bench_rsi, bench_macd, bench_cdl_doji);
criterion_main!(benches);
