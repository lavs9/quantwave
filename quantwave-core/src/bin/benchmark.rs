use quantwave_core::*;
use quantwave_core::traits::Next;
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let size = 100_000;
    let data: Vec<f64> = (0..size).map(|i| (i as f64).sin() * 100.0 + 100.0).collect();

    println!("| Indicator | talib-rs (batch) | quantwave (streaming 100k) | quantwave (next_batch 100k) |");
    println!("|-----------|------------------|-----------------------------|-----------------------------|");

    // SMA
    let start = Instant::now();
    let _ = talib_rs::overlap::sma(&data, 14);
    let talib_sma = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind = SMA::new(14);
    for &val in &data { black_box(ind.next(val)); }
    let qw_sma = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind2 = SMA::new(14);
    black_box(ind2.next_batch(&data));
    let qw_batch_sma = start.elapsed().as_micros();

    println!("| SMA (14) | {} µs | {} µs | {} µs |", talib_sma, qw_sma, qw_batch_sma);

    // RSI
    let start = Instant::now();
    let _ = talib_rs::momentum::rsi(&data, 14);
    let talib_rsi = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind = RSI::new(14);
    for &val in &data { black_box(ind.next(val)); }
    let qw_rsi = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind2 = RSI::new(14);
    black_box(ind2.next_batch(&data));
    let qw_batch_rsi = start.elapsed().as_micros();

    println!("| RSI (14) | {} µs | {} µs | {} µs |", talib_rsi, qw_rsi, qw_batch_rsi);

    // MACD
    let start = Instant::now();
    let _ = talib_rs::momentum::macd(&data, 12, 26, 9);
    let talib_macd = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind = MACD::new(12, 26, 9);
    for &val in &data { black_box(ind.next(val)); }
    let qw_macd = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind2 = MACD::new(12, 26, 9);
    black_box(ind2.next_batch(&data));
    let qw_batch_macd = start.elapsed().as_micros();

    println!("| MACD (12,26,9) | {} µs | {} µs | {} µs |", talib_macd, qw_macd, qw_batch_macd);

    // CDLDOJI
    let open = data.clone();
    let high = data.clone();
    let low = data.clone();
    let close = data.clone();

    let start = Instant::now();
    let _ = talib_rs::pattern::cdl_doji(&open, &high, &low, &close);
    let talib_doji = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind = CDLDOJI::new();
    for i in 0..size {
        black_box(ind.next((open[i], high[i], low[i], close[i])));
    }
    let qw_doji = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind2 = CDLDOJI::new();
    let inputs: Vec<(f64, f64, f64, f64)> = (0..size)
        .map(|i| (open[i], high[i], low[i], close[i]))
        .collect();
    black_box(ind2.next_batch(&inputs));
    let qw_batch_doji = start.elapsed().as_micros();

    println!(
        "| CDLDOJI | {} µs | {} µs | {} µs |",
        talib_doji, qw_doji, qw_batch_doji
    );
}
