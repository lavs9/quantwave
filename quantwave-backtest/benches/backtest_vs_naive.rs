//! Criterion benchmarks: `quantwave-backtest` vs naive row-loop baseline (cr6v.13).
//!
//! Run: `cargo bench -p quantwave-backtest`
#![allow(clippy::unwrap_used, clippy::expect_used)]

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use polars::prelude::*;
use quantwave_backtest::{BacktestConfig, BacktestEngine, CostModel};
use rand::SeedableRng;
use rand::prelude::*;
use rand::rngs::StdRng;

const BENCH_SEED: u64 = 0xC26E_0013;

fn zero_cost_config() -> BacktestConfig {
    BacktestConfig {
        cost_model: CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        },
        ..Default::default()
    }
}

/// Deterministic alternating-block exposure (ensures periodic trades).
fn alternating_signals(n: usize, block: usize) -> Vec<f64> {
    (0..n)
        .map(|i| {
            if (i / block).is_multiple_of(2) {
                1.0
            } else {
                0.0
            }
        })
        .collect()
}

fn synthetic_single_symbol(n_rows: usize) -> DataFrame {
    let mut rng = StdRng::seed_from_u64(BENCH_SEED);
    let mut price = 100.0_f64;
    let closes: Vec<f64> = (0..n_rows)
        .map(|_| {
            price += rng.gen_range(-0.5..0.5);
            price
        })
        .collect();
    let timestamps: Vec<i64> = (0..n_rows as i64)
        .map(|i| 1_700_000_000 + i * 3600)
        .collect();
    let signals = alternating_signals(n_rows, 50);

    DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .expect("single-symbol bench df")
}

fn synthetic_multi_symbol(n_symbols: usize, bars_per_symbol: usize) -> DataFrame {
    let mut rng = StdRng::seed_from_u64(BENCH_SEED);
    let n_rows = n_symbols * bars_per_symbol;
    let mut timestamps = Vec::with_capacity(n_rows);
    let mut symbols = Vec::with_capacity(n_rows);
    let mut closes = Vec::with_capacity(n_rows);
    let mut signals = Vec::with_capacity(n_rows);

    let mut prices = vec![0.0_f64; n_symbols];
    for s in 0..n_symbols {
        prices[s] = 50.0 + s as f64;
    }

    // Long-format: sorted by timestamp, then symbol (matches engine contract).
    for b in 0..bars_per_symbol {
        let ts = 1_700_000_000 + b as i64 * 3600;
        for s in 0..n_symbols {
            prices[s] += rng.gen_range(-0.25..0.25);
            timestamps.push(ts);
            symbols.push(format!("SYM{s:03}"));
            closes.push(prices[s]);
            signals.push(if (b / 40) % 2 == 0 { 1.0 } else { 0.0 });
        }
    }

    DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("symbol".into(), symbols),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .expect("multi-symbol bench df")
}

/// Naive row-by-row flip simulator (zero costs, same-bar fills).
/// Baseline for "Python/Polars loop" style backtests without engine optimizations.
fn naive_row_loop_backtest(closes: &[f64], signals: &[f64], initial_cash: f64) -> f64 {
    let mut cash = initial_cash;
    let mut position = 0.0_f64;
    let mut entry_price = 0.0_f64;

    for i in 0..closes.len() {
        let sig = signals[i];
        let close = closes[i];
        if sig > 0.0 && position == 0.0 {
            position = sig;
            entry_price = close;
        } else if sig <= 0.0 && position > 0.0 {
            cash += position * (close - entry_price);
            position = 0.0;
        }
    }

    if position > 0.0 {
        let last = closes[closes.len() - 1];
        cash += position * (last - entry_price);
    }

    cash
}

fn extract_f64_col(df: &DataFrame, name: &str) -> Vec<f64> {
    df.column(name)
        .expect("column")
        .f64()
        .expect("f64 dtype")
        .into_iter()
        .map(|v| v.unwrap_or(0.0))
        .collect()
}

fn bench_quantwave(df: &DataFrame) {
    let engine = BacktestEngine::new(zero_cost_config());
    let _ = engine.run(df.clone().lazy()).expect("quantwave backtest");
}

fn bench_quantwave_metrics_only(df: &DataFrame) {
    let engine = BacktestEngine::new(zero_cost_config());
    let _ = engine
        .run_metrics_only(df.clone().lazy())
        .expect("quantwave backtest metrics only");
}

fn bench_naive(df: &DataFrame) {
    let closes = extract_f64_col(df, "close");
    let signals = extract_f64_col(df, "signal");
    let _ = naive_row_loop_backtest(&closes, &signals, 100_000.0);
}

fn bench_single_symbol(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_symbol_flip");
    for n_rows in [10_000usize, 100_000, 1_000_000] {
        let df = synthetic_single_symbol(n_rows);
        group.throughput(Throughput::Elements(n_rows as u64));

        group.bench_with_input(
            BenchmarkId::new("quantwave_backtest", n_rows),
            &df,
            |b, data| b.iter(|| bench_quantwave(black_box(data))),
        );
        group.bench_with_input(
            BenchmarkId::new("quantwave_metrics_only", n_rows),
            &df,
            |b, data| b.iter(|| bench_quantwave_metrics_only(black_box(data))),
        );
        group.bench_with_input(
            BenchmarkId::new("naive_row_loop", n_rows),
            &df,
            |b, data| b.iter(|| bench_naive(black_box(data))),
        );
    }
    group.finish();
}

fn bench_multi_symbol(c: &mut Criterion) {
    let n_symbols = 100usize;
    let bars_per_symbol = 5_000usize;
    let n_rows = n_symbols * bars_per_symbol;
    let df = synthetic_multi_symbol(n_symbols, bars_per_symbol);

    let mut group = c.benchmark_group("multi_symbol_long");
    group.throughput(Throughput::Elements(n_rows as u64));

    let mut config = zero_cost_config();
    config.symbol_col = Some("symbol".into());

    group.bench_function("quantwave_backtest", |b| {
        b.iter(|| {
            let engine = BacktestEngine::new(config.clone());
            let _ = engine
                .run(black_box(df.clone().lazy()))
                .expect("multi-symbol quantwave");
        });
    });

    group.bench_function("quantwave_metrics_only", |b| {
        b.iter(|| {
            let engine = BacktestEngine::new(config.clone());
            let _ = engine
                .run_metrics_only(black_box(df.clone().lazy()))
                .expect("multi-symbol quantwave metrics");
        });
    });

    group.bench_function("naive_row_loop_per_symbol", |b| {
        b.iter(|| {
            let symbols = df
                .column("symbol")
                .expect("symbol")
                .str()
                .expect("str")
                .into_iter()
                .map(|s| s.unwrap().to_string())
                .collect::<Vec<_>>();
            let closes = extract_f64_col(&df, "close");
            let signals = extract_f64_col(&df, "signal");

            let mut by_symbol: std::collections::HashMap<String, (Vec<f64>, Vec<f64>)> =
                std::collections::HashMap::new();
            for i in 0..df.height() {
                let entry = by_symbol
                    .entry(symbols[i].clone())
                    .or_insert_with(|| (Vec::new(), Vec::new()));
                entry.0.push(closes[i]);
                entry.1.push(signals[i]);
            }

            let mut total = 0.0_f64;
            for (_, (c, s)) in by_symbol {
                total += naive_row_loop_backtest(&c, &s, 100_000.0);
            }
            black_box(total);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_single_symbol, bench_multi_symbol);
criterion_main!(benches);
