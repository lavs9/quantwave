//! Export all IndicatorMetadata to JSON (stdout) for Python codegen.
//!
//! Run: `cargo run -p quantwave-core --bin export_metadata > metadata_export.json`

use quantwave_core::indicators::metadata_registry::{ALL_REGISTERED, RegisteredMetadata};
use serde::Serialize;
use std::io::{self, Write};

#[derive(Serialize)]
struct ExportedParam {
    name: String,
    default: String,
    description: String,
}

#[derive(Serialize)]
struct ExportedMetadata {
    slug: String,
    name: String,
    description: String,
    usage: String,
    category: String,
    keywords: Vec<String>,
    params: Vec<ExportedParam>,
    formula_source: String,
    formula_latex: String,
    gold_standard_file: String,
    ehlers_summary: String,
    struct_name: String,
    source_file: String,
    boundary_kind: String,
}

fn infer_boundary_kind(r: &RegisteredMetadata) -> &'static str {
    let cat = r.meta.category.to_lowercase();
    let name = r.slug;

    if cat.contains("price action")
        || name == "sr_monitor"
        || name == "geometric_patterns"
        || name == "market_structure"
    {
        return "event";
    }
    if cat.contains("pattern") || name.starts_with("cdl") {
        return "pattern";
    }
    if name == "obv"
        || name == "ad"
        || name == "nvi"
        || name == "pvi"
        || name == "vwap"
        || name == "anchored_vwap"
    {
        return "cumulative";
    }
    "scalar"
}

fn export_one(r: &RegisteredMetadata) -> ExportedMetadata {
    let meta = r.meta;
    ExportedMetadata {
        slug: r.slug.to_string(),
        name: meta.name.to_string(),
        description: meta.description.to_string(),
        usage: meta.usage.to_string(),
        category: meta.category.to_string(),
        keywords: meta.keywords.iter().map(|s| s.to_string()).collect(),
        params: meta
            .params
            .iter()
            .map(|p| ExportedParam {
                name: p.name.to_string(),
                default: p.default.to_string(),
                description: p.description.to_string(),
            })
            .collect(),
        formula_source: meta.formula_source.to_string(),
        formula_latex: meta.formula_latex.to_string(),
        gold_standard_file: meta.gold_standard_file.to_string(),
        ehlers_summary: meta.ehlers_summary.to_string(),
        struct_name: r.struct_name.to_string(),
        source_file: r.source_file.to_string(),
        boundary_kind: infer_boundary_kind(r).to_string(),
    }
}

fn main() -> io::Result<()> {
    let mut out: Vec<ExportedMetadata> = ALL_REGISTERED.iter().map(export_one).collect();

    out.sort_by(|a, b| a.slug.cmp(&b.slug));

    let json = serde_json::to_string_pretty(&out)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    io::stdout().write_all(json.as_bytes())?;
    io::stdout().write_all(b"\n")?;
    Ok(())
}
