use quantwave_core::*;
use quantwave_core::traits::Next;
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
    for &val in &data { ind.next(val); }
    let qw_sma = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind2 = SMA::new(14);
    let _ = ind2.next_batch(&data);
    let qw_batch_sma = start.elapsed().as_micros();

    println!("| SMA (14) | {} µs | {} µs | {} µs |", talib_sma, qw_sma, qw_batch_sma);

    // RSI
    let start = Instant::now();
    let _ = talib_rs::momentum::rsi(&data, 14);
    let talib_rsi = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind = RSI::new(14);
    for &val in &data { ind.next(val); }
    let qw_rsi = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind2 = RSI::new(14);
    let _ = ind2.next_batch(&data);
    let qw_batch_rsi = start.elapsed().as_micros();

    println!("| RSI (14) | {} µs | {} µs | {} µs |", talib_rsi, qw_rsi, qw_batch_rsi);

    // MACD
    let start = Instant::now();
    let _ = talib_rs::momentum::macd(&data, 12, 26, 9);
    let talib_macd = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind = MACD::new(12, 26, 9);
    for &val in &data { ind.next(val); }
    let qw_macd = start.elapsed().as_micros();

    let start = Instant::now();
    let mut ind2 = MACD::new(12, 26, 9);
    let _ = ind2.next_batch(&data);
    let qw_batch_macd = start.elapsed().as_micros();

    println!("| MACD (12,26,9) | {} µs | {} µs | {} µs |", talib_macd, qw_macd, qw_batch_macd);
}
