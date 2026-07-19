//! PyO3 bindings for harmonic pattern detection.
//!
//! Like the bar transforms, pattern detection is not a 1:1 `.ta` expression
//! (it returns a variable number of detected-pattern rows), so it is exposed as
//! a free function returning a fresh Polars DataFrame — one row per pattern.
//!
//! Attribution: the harmonic patterns are the work of Scott M. Carney
//! (HarmonicTrader.com); see `quantwave_core::indicators::harmonic`.

use polars::prelude::*;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use quantwave_core::{HarmonicConfig, HarmonicPattern, harmonic_patterns_batch};

fn patterns_to_frame(pats: &[HarmonicPattern]) -> PolarsResult<DataFrame> {
    let u = |v: Option<usize>| v.map(|x| x as u32);
    df![
        "id" => pats.iter().map(|p| p.id).collect::<Vec<u32>>(),
        "kind" => pats.iter().map(|p| p.kind.as_str()).collect::<Vec<&str>>(),
        "is_bull" => pats.iter().map(|p| p.is_bull).collect::<Vec<bool>>(),
        "x_bar" => pats.iter().map(|p| u(p.x_bar)).collect::<Vec<Option<u32>>>(),
        "a_bar" => pats.iter().map(|p| p.a_bar as u32).collect::<Vec<u32>>(),
        "b_bar" => pats.iter().map(|p| p.b_bar as u32).collect::<Vec<u32>>(),
        "c_bar" => pats.iter().map(|p| p.c_bar as u32).collect::<Vec<u32>>(),
        "d_bar" => pats.iter().map(|p| p.d_bar as u32).collect::<Vec<u32>>(),
        "x_price" => pats.iter().map(|p| p.x_price).collect::<Vec<Option<f64>>>(),
        "a_price" => pats.iter().map(|p| p.a_price).collect::<Vec<f64>>(),
        "b_price" => pats.iter().map(|p| p.b_price).collect::<Vec<f64>>(),
        "c_price" => pats.iter().map(|p| p.c_price).collect::<Vec<f64>>(),
        "d_price" => pats.iter().map(|p| p.d_price).collect::<Vec<f64>>(),
        "score" => pats.iter().map(|p| p.score).collect::<Vec<f64>>(),
        "xa_ext" => pats.iter().map(|p| p.xa_ext).collect::<Vec<Option<f64>>>(),
        "bc_ab" => pats.iter().map(|p| p.bc_ab).collect::<Vec<f64>>(),
        "cd_ab" => pats.iter().map(|p| p.cd_ab).collect::<Vec<f64>>(),
        "cd_bc" => pats.iter().map(|p| p.cd_bc).collect::<Vec<f64>>(),
        "prz_low" => pats.iter().map(|p| p.prz_low).collect::<Vec<f64>>(),
        "prz_high" => pats.iter().map(|p| p.prz_high).collect::<Vec<f64>>(),
        "size_atr" => pats.iter().map(|p| p.size_atr).collect::<Vec<f64>>(),
    ]
}

/// Detect harmonic patterns (AB=CD, Alternate AB=CD, 5-0) over `(highs, lows)`.
/// Returns one DataFrame row per detected pattern in completion order.
#[pyfunction]
#[pyo3(signature = (
    highs,
    lows,
    swing_strength = 5,
    ratio_tolerance = 0.10,
    min_score = 0.5,
    min_size_atr = 0.0,
    atr_period = 14,
    detect_abcd = true,
    detect_alternate_abcd = true,
    detect_5_0 = true,
))]
#[allow(clippy::too_many_arguments)]
fn harmonic(
    highs: Vec<f64>,
    lows: Vec<f64>,
    swing_strength: usize,
    ratio_tolerance: f64,
    min_score: f64,
    min_size_atr: f64,
    atr_period: usize,
    detect_abcd: bool,
    detect_alternate_abcd: bool,
    detect_5_0: bool,
) -> PyResult<PyDataFrame> {
    if highs.len() != lows.len() {
        return Err(PyValueError::new_err(
            "highs and lows must have equal length",
        ));
    }
    if swing_strength < 1 {
        return Err(PyValueError::new_err("swing_strength must be >= 1"));
    }
    if !(ratio_tolerance > 0.0) {
        return Err(PyValueError::new_err("ratio_tolerance must be > 0"));
    }
    let config = HarmonicConfig {
        ratio_tolerance,
        min_score,
        min_size_atr,
        atr_period,
        detect_abcd,
        detect_alternate_abcd,
        detect_5_0,
    };
    let bars: Vec<(f64, f64)> = highs.into_iter().zip(lows).collect();
    let pats = harmonic_patterns_batch(&bars, swing_strength, config);
    let df = patterns_to_frame(&pats).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(PyDataFrame(df))
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(harmonic, m)?)?;
    Ok(())
}
