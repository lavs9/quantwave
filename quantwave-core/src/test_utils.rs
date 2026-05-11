use crate::traits::Next;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub input: Vec<f64>,
    pub expected: Vec<f64>,
}

/// Load a gold standard test case from a JSON file.
pub fn load_gold_standard(name: &str) -> TestCase {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = Path::new(&manifest_dir);
    
    // Try current dir, then parent (workspace root)
    let path = manifest_path.join("tests/gold_standard").join(format!("{}.json", name));
    let path = if path.exists() {
        path
    } else {
        manifest_path.parent().unwrap().join("tests/gold_standard").join(format!("{}.json", name))
    };

    let content = fs::read_to_string(&path).expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

/// Verify that a streaming indicator matches the expected output.
pub fn assert_indicator_parity<I>(mut indicator: I, input: &[f64], expected: &[f64])
where
    I: Next<f64, Output = f64>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        approx::assert_relative_eq!(result, expected[i], epsilon = 1e-6);
    }
}

/// Helper for property-based testing of batch vs streaming parity.
/// This is a generic test that can be used by all indicators.
pub fn check_batch_streaming_parity<I, F>(input: Vec<f64>, mut indicator: I, batch_fn: F)
where
    I: Next<f64, Output = f64>,
    F: FnOnce(Vec<f64>) -> Vec<f64>,
{
    let batch_results = batch_fn(input.clone());
    let mut streaming_results = Vec::with_capacity(input.len());

    for val in input {
        streaming_results.push(indicator.next(val));
    }

    for (_i, (&s, &b)) in streaming_results.iter().zip(batch_results.iter()).enumerate() {
        approx::assert_relative_eq!(s, b, epsilon = 1e-6, max_relative = 1e-6);
    }
}
