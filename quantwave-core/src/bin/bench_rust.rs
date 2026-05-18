use quantwave_core::indicators::supertrend::SuperTrend;
use quantwave_core::traits::Next;
use std::time::Instant;

fn main() {
    let num_rows = 1_000_000;
    println!("Generating {} rows of synthetic data...", num_rows);
    
    // Synthetic OHLC data
    let mut high = vec![110.0; num_rows];
    let mut low = vec![90.0; num_rows];
    let mut close = vec![100.0; num_rows];
    
    // Add some noise
    for i in 0..num_rows {
        high[i] += (i as f64).sin() * 5.0;
        low[i] -= (i as f64).cos() * 5.0;
        close[i] += (i as f64).sin() * 2.0;
    }

    println!("Benchmarking SuperTrend (10, 3.0) on {} rows...", num_rows);
    
    let mut st = SuperTrend::new(10, 3.0);
    
    let mut sum = 0.0;
    let start = Instant::now();
    for i in 0..num_rows {
        let (val, _) = st.next((high[i], low[i], close[i]));
        sum += val;
    }
    let duration = start.elapsed();
    
    println!("Total time: {:?}, Sum: {}", duration, sum);
    println!("Time per iteration: {:?}", duration / num_rows as u32);
    println!("Result: {:.2} ms for 1M rows", duration.as_secs_f64() * 1000.0);
}
