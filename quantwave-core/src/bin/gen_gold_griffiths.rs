//! Regenerate griffiths_spectrum gold fixture (quantwave-5ipk.8).
//!
//! Run: `cargo run -p quantwave-core --bin gen_gold_griffiths`

use quantwave_core::indicators::griffiths_spectrum::GriffithsSpectrum;
use quantwave_core::traits::Next;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
struct Provenance {
    source: &'static str,
    generator: &'static str,
    date: &'static str,
    params: &'static str,
}

#[derive(Serialize)]
struct Fixture {
    provenance: Provenance,
    input: Vec<f64>,
    expected: Vec<Vec<f64>>,
}

fn synthetic_input(n: usize) -> Vec<f64> {
    // Dominant ~20-bar cycle for qualitative spectrum peak validation.
    (0..n)
        .map(|i| {
            let t = i as f64;
            100.0
                + 2.0 * (2.0 * std::f64::consts::PI * t / 20.0).sin()
                + 0.5 * (2.0 * std::f64::consts::PI * t / 7.0).sin()
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = synthetic_input(120);
    let mut gs = GriffithsSpectrum::new(18, 40, 40);
    let expected: Vec<Vec<f64>> = input.iter().map(|&v| gs.next(v)).collect();

    let fixture = Fixture {
        provenance: Provenance {
            source: "Ehlers Griffiths spectrum (TASC Jan 2025); synthetic 20-bar dominant cycle",
            generator: "quantwave-core/src/bin/gen_gold_griffiths.rs",
            date: "2026-07-08",
            params: "lower_bound=18, upper_bound=40, length=40",
        },
        input,
        expected,
    };

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out = manifest_dir.join("tests/gold_standard/griffiths_spectrum.json");
    let json = serde_json::to_string_pretty(&fixture)?;
    fs::write(&out, format!("{json}\n"))?;
    eprintln!("wrote {}", out.display());
    Ok(())
}
