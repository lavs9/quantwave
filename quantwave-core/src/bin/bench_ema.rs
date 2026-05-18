use quantwave_core::indicators::smoothing::EMA;
use quantwave_core::traits::Next;
use std::time::Instant;

fn main() {
    let num_rows = 1_000_000;
    let mut data = vec![100.0; num_rows];
    for i in 0..num_rows {
        data[i] += (i as f64).sin() * 5.0;
    }

    println!("Benchmarking EMA (20) on {} rows...", num_rows);
    let mut ema = EMA::new(20);
    
    let mut sum = 0.0;
    let start = Instant::now();
    for i in 0..num_rows {
        sum += ema.next(data[i]);
    }
    let duration = start.elapsed();
    
    println!("Total time: {:?}, Sum: {}", duration, sum);
    println!("Result: {:.2} ms for 1M rows", duration.as_secs_f64() * 1000.0);
}
