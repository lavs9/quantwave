use crate::traits::Next;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub input: Vec<f64>,
    pub expected: Vec<Option<f64>>,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseVec {
    pub input: Vec<f64>,
    pub expected: Vec<Vec<Option<f64>>>,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseLoops {
    pub input: Vec<(f64, f64)>,
    pub expected: Vec<(Option<f64>, Option<f64>)>,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseOC {
    pub input: Vec<(f64, f64)>,
    pub expected: Vec<Option<f64>>,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseTuple {
    pub input: Vec<f64>,
    pub expected: Vec<(Option<f64>, Option<f64>)>,
}

/// Load a gold standard test case from a JSON file.
pub fn load_gold_standard(name: &str) -> TestCase {
    let path = get_gold_standard_path(name);
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

pub fn load_gold_standard_vec(name: &str) -> TestCaseVec {
    let path = get_gold_standard_path(name);
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

pub fn load_gold_standard_loops(name: &str) -> TestCaseLoops {
    let path = get_gold_standard_path(name);
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

fn get_gold_standard_path(name: &str) -> std::path::PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = Path::new(&manifest_dir);

    let path = manifest_path
        .join("tests/gold_standard")
        .join(format!("{}.json", name));
    if path.exists() {
        path
    } else {
        manifest_path
            .parent()
            .unwrap()
            .join("tests/gold_standard")
            .join(format!("{}.json", name))
    }
}

/// Verify that a streaming indicator matches the expected output.
pub fn assert_indicator_parity<I>(mut indicator: I, input: &[f64], expected: &[Option<f64>])
where
    I: Next<f64, Output = f64>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        match expected[i] {
            Some(exp) => approx::assert_relative_eq!(result, exp, epsilon = 1e-6),
            None => assert!(result.is_nan(), "Expected NaN at index {}", i),
        }
    }
}

pub fn assert_indicator_parity_vec<I>(mut indicator: I, input: &[f64], expected: &[Vec<Option<f64>>])
where
    I: Next<f64, Output = Vec<f64>>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        for (j, &v) in result.iter().enumerate() {
            match expected[i][j] {
                Some(exp) => approx::assert_relative_eq!(v, exp, epsilon = 1e-6),
                None => assert!(v.is_nan(), "Expected NaN at index {},{}", i, j),
            }
        }
    }
}

pub fn assert_indicator_parity_loops<I>(mut indicator: I, input: &[(f64, f64)], expected: &[(Option<f64>, Option<f64>)])
where
    I: Next<(f64, f64), Output = (f64, f64)>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        match expected[i].0 {
            Some(exp) => approx::assert_relative_eq!(result.0, exp, epsilon = 1e-6),
            None => assert!(result.0.is_nan(), "Expected NaN at index {}.0", i),
        }
        match expected[i].1 {
            Some(exp) => approx::assert_relative_eq!(result.1, exp, epsilon = 1e-6),
            None => assert!(result.1.is_nan(), "Expected NaN at index {}.1", i),
        }
    }
}

pub fn load_gold_standard_oc(name: &str) -> TestCaseOC {
    let path = get_gold_standard_path(name);
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

pub fn load_gold_standard_tuple(name: &str) -> TestCaseTuple {
    let path = get_gold_standard_path(name);
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read gold standard file at {:?}", path));
    serde_json::from_str(&content).expect("Failed to parse gold standard JSON")
}

pub fn assert_indicator_parity_oc<I>(mut indicator: I, input: &[(f64, f64)], expected: &[Option<f64>])
where
    I: Next<(f64, f64), Output = f64>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        match expected[i] {
            Some(exp) => approx::assert_relative_eq!(result, exp, epsilon = 1e-6),
            None => assert!(result.is_nan(), "Expected NaN at index {}", i),
        }
    }
}

pub fn assert_indicator_parity_tuple<I>(mut indicator: I, input: &[f64], expected: &[(Option<f64>, Option<f64>)])
where
    I: Next<f64, Output = (f64, f64)>,
{
    for (i, &val) in input.iter().enumerate() {
        let result = indicator.next(val);
        match expected[i].0 {
            Some(exp) => approx::assert_relative_eq!(result.0, exp, epsilon = 1e-6),
            None => assert!(result.0.is_nan(), "Expected NaN at index {}.0", i),
        }
        match expected[i].1 {
            Some(exp) => approx::assert_relative_eq!(result.1, exp, epsilon = 1e-6),
            None => assert!(result.1.is_nan(), "Expected NaN at index {}.1", i),
        }
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

    for (_i, (&s, &b)) in streaming_results
        .iter()
        .zip(batch_results.iter())
        .enumerate()
    {
        approx::assert_relative_eq!(s, b, epsilon = 1e-6, max_relative = 1e-6);
    }
}
