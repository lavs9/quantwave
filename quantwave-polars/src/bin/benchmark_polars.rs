use quantwave_polars::QuantWaveExt;
use polars::prelude::*;
use std::time::Instant;

fn main() -> PolarsResult<()> {
    let size = 100_000;
    
    // Create random-ish data
    let data: Vec<f64> = (0..size).map(|i| (i as f64).sin() * 100.0 + 100.0).collect();
    
    let s = Series::new("close".into(), &data);
    let s_open = Series::new("open".into(), &data);
    let s_high = Series::new("high".into(), &data);
    let s_low = Series::new("low".into(), &data);
    
    let df = DataFrame::new(vec![s_open.into(), s_high.into(), s_low.into(), s.into()])?;
    
    println!("Benchmarking Polars plugins for size 100k");
    
    // SMA
    let lf = df.clone().lazy();
    let start = Instant::now();
    let _ = lf.ta().sma("close", 14).collect()?;
    let polars_sma = start.elapsed().as_micros();
    println!("SMA(14): {} µs", polars_sma);
    
    // RSI
    let lf = df.clone().lazy();
    let start = Instant::now();
    let _ = lf.ta().rsi("close", 14).collect()?;
    let polars_rsi = start.elapsed().as_micros();
    println!("RSI(14): {} µs", polars_rsi);
    
    // MACD
    let lf = df.clone().lazy();
    let start = Instant::now();
    let _ = lf.ta().macd("close", 12, 26, 9).collect()?;
    let polars_macd = start.elapsed().as_micros();
    println!("MACD(12,26,9): {} µs", polars_macd);
    
    // CDLDOJI
    let lf = df.clone().lazy();
    let start = Instant::now();
    let _ = lf.ta().cdl_doji("open", "high", "low", "close").collect()?;
    let polars_doji = start.elapsed().as_micros();
    println!("CDLDOJI: {} µs", polars_doji);
    
    Ok(())
}
