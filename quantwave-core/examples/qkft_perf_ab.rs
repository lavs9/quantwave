//! TEMPORARY A/B micro-benchmark for quantwave-qkft. Interleaves the old
//! one-pass accumulator with the new stable one in a single process and takes
//! the minimum over many reps, which is far more noise-robust than two separate
//! criterion runs on a loaded machine.

use quantwave_core::Next;
use quantwave_core::indicators::statistics::TaSTDDEV;
use std::hint::black_box;
use std::time::Instant;

/// The pre-fix accumulator, verbatim.
struct OldStddev {
    period: usize,
    buf: std::collections::VecDeque<f64>,
    sum: f64,
    sum_sq: f64,
}

impl OldStddev {
    fn new(period: usize) -> Self {
        Self {
            period,
            buf: std::collections::VecDeque::with_capacity(period),
            sum: 0.0,
            sum_sq: 0.0,
        }
    }
    fn next(&mut self, v: f64) -> f64 {
        if self.buf.len() >= self.period {
            if let Some(old) = self.buf.pop_front() {
                self.sum -= old;
                self.sum_sq -= old * old;
            }
        }
        self.buf.push_back(v);
        self.sum += v;
        self.sum_sq += v * v;
        if self.buf.len() < self.period {
            return f64::NAN;
        }
        let n = self.period as f64;
        let mean = self.sum / n;
        (self.sum_sq / n - mean * mean).max(0.0).sqrt()
    }
}

fn main() {
    const N: usize = 1_000_000;
    let data: Vec<f64> = (0..N).map(|i| 100.0 + (i as f64) * 0.001).collect();
    const REPS: usize = 25;

    for period in [14usize, 20, 50, 200] {
        let mut old_best = f64::MAX;
        let mut new_best = f64::MAX;
        for _ in 0..REPS {
            let t = Instant::now();
            let mut a = OldStddev::new(period);
            for &x in &data {
                black_box(a.next(x));
            }
            old_best = old_best.min(t.elapsed().as_secs_f64());

            let t = Instant::now();
            let mut b = TaSTDDEV::new(period, 1.0);
            for &x in &data {
                black_box(b.next(x));
            }
            new_best = new_best.min(t.elapsed().as_secs_f64());
        }
        let old_ns = old_best * 1e9 / N as f64;
        let new_ns = new_best * 1e9 / N as f64;
        println!(
            "period {period:>3}: old {old_ns:6.3} ns/bar  new {new_ns:6.3} ns/bar  delta {:+.1}%",
            (new_ns / old_ns - 1.0) * 100.0
        );
    }
}
