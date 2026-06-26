//! Export all IndicatorMetadata to JSON (stdout) for Python codegen.
//!
//! Run: `cargo run -p quantwave-core --bin export_metadata > metadata_export.json`

use quantwave_core::indicators::metadata::IndicatorMetadata;
use quantwave_core::indicators::metadata_registry::ALL_REGISTERED;
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
    gold_standard_file: String,
}

fn export_one(slug: &str, meta: &IndicatorMetadata) -> ExportedMetadata {
    ExportedMetadata {
        slug: slug.to_string(),
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
        gold_standard_file: meta.gold_standard_file.to_string(),
    }
}

fn main() -> io::Result<()> {
    let mut out: Vec<ExportedMetadata> = ALL_REGISTERED
        .iter()
        .map(|r| export_one(r.slug, r.meta))
        .collect();

    out.sort_by(|a, b| a.slug.cmp(&b.slug));

    let json = serde_json::to_string_pretty(&out).expect("serialize metadata");
    io::stdout().write_all(json.as_bytes())?;
    io::stdout().write_all(b"\n")?;
    Ok(())
}