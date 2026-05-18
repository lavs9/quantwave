use quantwave_core::indicators::cyber_cycle::CyberCycle;
use quantwave_core::indicators::instantaneous_trendline::InstantaneousTrendline;
use quantwave_core::traits::Next;
use std::time::Instant;

fn main() {
    let num_rows = 1_000_000;
    println!("Generating {} rows of synthetic data...", num_rows);
    let mut data = vec![100.0; num_rows];
    for i in 0..num_rows {
        data[i] += (i as f64).sin() * 5.0;
    }

    // Benchmark CyberCycle
    println!("Benchmarking CyberCycle (14) on {} rows...", num_rows);
    let mut cc = CyberCycle::new(14);
    let mut sum_cc = 0.0;
    let start_cc = Instant::now();
    for i in 0..num_rows {
        let (val, _) = cc.next(data[i]);
        sum_cc += val;
    }
    let duration_cc = start_cc.elapsed();
    println!("CyberCycle: {:?} (Sum: {})", duration_cc, sum_cc);
    println!("Result: {:.2} ms for 1M rows", duration_cc.as_secs_f64() * 1000.0);

    // Benchmark Instantaneous Trendline
    println!("\nBenchmarking Instantaneous Trendline on {} rows...", num_rows);
    let mut it = InstantaneousTrendline::new();
    let mut sum_it = 0.0;
    let start_it = Instant::now();
    for i in 0..num_rows {
        sum_it += it.next(data[i]);
    }
    let duration_it = start_it.elapsed();
    println!("Instantaneous Trendline: {:?} (Sum: {})", duration_it, sum_it);
    println!("Result: {:.2} ms for 1M rows", duration_it.as_secs_f64() * 1000.0);
}
