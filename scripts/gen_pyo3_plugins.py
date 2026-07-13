import os
import re

POLARS_LIB_PATH = "quantwave-polars/src/lib.rs"
PLUGINS_SRC_DIR = "quantwave-py/src/plugins/"

with open(POLARS_LIB_PATH, "r") as f:
    polars_content = f.read()

implemented = set()
for file in os.listdir(PLUGINS_SRC_DIR):
    if not file.endswith(".rs"): continue
    if file == "generated.rs": continue
    with open(os.path.join(PLUGINS_SRC_DIR, file), "r") as f:
        content = f.read()
        matches = re.findall(r'fn\s+([a-zA-Z0-9_]+)\s*\(\s*inputs', content)
        for m in matches:
            implemented.add(m.lower())

methods = re.findall(r'pub fn ([a-zA-Z0-9_]+)\s*\(\s*self[^)]*\)\s*->\s*LazyFrame', polars_content)
all_methods = set(m.lower() for m in methods)
missing = sorted(list(all_methods - implemented))

patterns = {}
for m in missing:
    body_match = re.search(r'pub fn ' + m + r'\s*\([^)]*\)\s*->\s*LazyFrame\s*\{([^}]*)\}', polars_content)
    if body_match:
        body = body_match.group(1).strip()
        pattern_match = re.search(r'self\.([a-zA-Z0-9_]+)::<', body)
        if pattern_match:
            pat = pattern_match.group(1)
            patterns.setdefault(pat, []).append(m)
        else:
            patterns.setdefault("custom", []).append(m)

rust_code = """use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

use quantwave_core::traits::Next;

#[derive(Deserialize)]
pub struct SinglePeriodKwargs {
    pub timeperiod: usize,
}
"""

def get_struct_name(func_name):
    body_match = re.search(r'pub fn ' + func_name + r'\s*\([^)]*\)\s*->\s*LazyFrame\s*\{([^}]*)\}', polars_content)
    if body_match:
        body = body_match.group(1).strip()
        type_match = re.search(r'::<([a-zA-Z0-9_]+)>', body)
        if type_match:
            return type_match.group(1)
    return func_name.upper()

for m in patterns.get("math_transform_1_in_1_out", []):
    struct_name = get_struct_name(m)
    rust_code += f"""
#[polars_expr(output_type=Float64)]
fn {m}(inputs: &[Series]) -> PolarsResult<Series> {{
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::{struct_name}::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {{
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }}).collect();
    Ok(out.into_series())
}}
"""

for m in patterns.get("math_operator_1_in_1_out_period", []):
    struct_name = get_struct_name(m)
    rust_code += f"""
#[polars_expr(output_type=Float64)]
fn {m}(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {{
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::{struct_name}::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {{
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }}).collect();
    Ok(out.into_series())
}}
"""

for m in patterns.get("math_operator_2_in_1_out", []):
    struct_name = get_struct_name(m)
    rust_code += f"""
#[polars_expr(output_type=Float64)]
fn {m}(inputs: &[Series]) -> PolarsResult<Series> {{
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let mut indicator = quantwave_core::{struct_name}::new();
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).map(|(v1, v2)| match (v1, v2) {{
        (Some(a), Some(b)) if !a.is_nan() && !b.is_nan() => Some(indicator.next((a, b))),
        (Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }}).collect();
    Ok(out.into_series())
}}
"""

for m in patterns.get("math_operator_2_in_1_out_period", []):
    struct_name = get_struct_name(m)
    rust_code += f"""
#[polars_expr(output_type=Float64)]
fn {m}(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {{
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let mut indicator = quantwave_core::{struct_name}::new(kwargs.timeperiod);
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).map(|(v1, v2)| match (v1, v2) {{
        (Some(a), Some(b)) if !a.is_nan() && !b.is_nan() => Some(indicator.next((a, b))),
        (Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }}).collect();
    Ok(out.into_series())
}}
"""

for m in patterns.get("ta_3_in_1_out_default", []):
    struct_name = get_struct_name(m)
    rust_code += f"""
#[polars_expr(output_type=Float64)]
fn {m}(inputs: &[Series]) -> PolarsResult<Series> {{
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let in3 = inputs[2].f64()?;
    let mut indicator = quantwave_core::{struct_name}::new();
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).zip(in3.into_iter()).map(|((v1, v2), v3)| match (v1, v2, v3) {{
        (Some(a), Some(b), Some(c)) if !a.is_nan() && !b.is_nan() && !c.is_nan() => Some(indicator.next((a, b, c))),
        (Some(_), Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }}).collect();
    Ok(out.into_series())
}}
"""

for m in patterns.get("ta_3_in_1_out_period", []):
    struct_name = get_struct_name(m)
    rust_code += f"""
#[polars_expr(output_type=Float64)]
fn {m}(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {{
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let in3 = inputs[2].f64()?;
    let mut indicator = quantwave_core::{struct_name}::new(kwargs.timeperiod);
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).zip(in3.into_iter()).map(|((v1, v2), v3)| match (v1, v2, v3) {{
        (Some(a), Some(b), Some(c)) if !a.is_nan() && !b.is_nan() && !c.is_nan() => Some(indicator.next((a, b, c))),
        (Some(_), Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }}).collect();
    Ok(out.into_series())
}}
"""

for m in patterns.get("ta_4_in_1_out_default", []):
    struct_name = get_struct_name(m)
    rust_code += f"""
#[polars_expr(output_type=Float64)]
fn {m}(inputs: &[Series]) -> PolarsResult<Series> {{
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::{struct_name}::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {{
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }}).collect();
    Ok(out.into_series())
}}
"""

with open("quantwave-py/src/plugins/generated.rs", "w") as f:
    f.write(rust_code)
