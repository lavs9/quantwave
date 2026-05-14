use anyhow::{Context, Result};
use reqwest::blocking::get;
use roxmltree::Document;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    println!("Generating documentation...");

    let workspace_root =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()))
            .parent()
            .unwrap()
            .to_path_buf();
    let docs_dir = workspace_root.join("docs/src");
    let indicators_dir = workspace_root.join("quantwave-core/src/indicators");

    println!("Workspace Root: {:?}", workspace_root);
    println!("Docs Dir: {:?}", docs_dir);
    println!("Indicators Dir: {:?}", indicators_dir);

    if !workspace_root.exists() {
        return Err(anyhow::anyhow!("Workspace root does not exist: {:?}", workspace_root));
    }
    if !docs_dir.exists() {
        return Err(anyhow::anyhow!("Docs directory does not exist: {:?}. Make sure you are running from the workspace root.", docs_dir));
    }
    if !indicators_dir.exists() {
        return Err(anyhow::anyhow!("Indicators directory does not exist: {:?}", indicators_dir));
    }

    fs::create_dir_all(docs_dir.join("indicators/native")).context("Failed to create indicators/native directory")?;
    fs::create_dir_all(docs_dir.join("indicators/talib")).context("Failed to create indicators/talib directory")?;

    // We will generate the SUMMARY.md dynamically based on the parsed indicators
    let mut summary = String::new();
    summary.push_str("# Summary\n\n");
    summary.push_str("- [Introduction](README.md)\n");
    summary.push_str("- [Indicators](indicators/README.md)\n");

    let native_docs = generate_native_docs(&docs_dir, &indicators_dir)?;
    if !native_docs.is_empty() {
        let mut categories: std::collections::BTreeMap<String, Vec<(String, String)>> =
            std::collections::BTreeMap::new();
        for (name, filename, category) in native_docs {
            categories
                .entry(category)
                .or_default()
                .push((name, filename));
        }

        summary.push_str("    - [Native Indicators](indicators/native/README.md)\n");
        for (category, indicators) in categories {
            summary.push_str(&format!(
                "        - [{}]()\n",
                if category.is_empty() {
                    "General"
                } else {
                    &category
                }
            ));
            for (name, filename) in indicators {
                summary.push_str(&format!(
                    "            - [{}](indicators/native/{}.md)\n",
                    name, filename
                ));
            }
        }
    }

    let talib_list = generate_talib_docs().context("Failed to generate TA-Lib docs")?;
    summary.push_str("    - [TA-Lib Wrappers](indicators/talib/README.md)\n");

    let main_intro = r#"# QuantWave 🌊

**High-performance, Polars-native Technical Analysis for Rust.**

QuantWave is a modern technical analysis library built from the ground up for the Polars ecosystem. It bridges the gap between high-speed batch backtesting and real-time streaming execution by ensuring bit-identical results across both modes.

Whether you are performing quantitative research over terabytes of historical data or deploying a live trading system on a tick-by-tick stream, QuantWave delivers industry-standard accuracy and extreme performance.

## Design Philosophy
1. **Universal Indicator Pattern:** Every indicator guarantees identical results for batch and streaming.
2. **Zero-Copy Performance:** Native Polars plugins operate directly on Arrow memory buffers.
3. **Rigorous Validation:** Every indicator is tested against industry gold-standard data (TradingView, MetaTrader) to ensure correctness.

Select an indicator from the sidebar to view its mathematical formula, parameters, and documentation.
"#;

    let indicators_intro = r#"# Indicator Suite

The QuantWave indicator suite is divided into two primary categories to give you maximum flexibility and coverage:

- **Native Indicators**: Highly optimized, modern indicators implemented natively in Rust. These include modern DSP suites, order flow tools, and advanced moving averages.
- **TA-Lib Wrappers**: A comprehensive suite of 158 classic indicators wrapping the battle-tested `ta-lib` C library.

Every single indicator, regardless of its category, supports both live streaming (`Next` trait) and batch Polars processing (`.ta()` namespace).
"#;

    let native_intro = r#"# Native Indicators

Native indicators in QuantWave are written entirely in safe, zero-cost Rust.

These algorithms are compiled as native Polars Expressions, allowing them to benefit from vectorized execution, multi-threading, and query optimization without serialization overhead.

Here you will find our implementations of algorithms like `SuperTrend`, `WaveTrend`, `ALMA`, and more.
"#;

    let mut talib_intro = String::from(
        r#"# TA-Lib Wrappers

QuantWave seamlessly integrates with the industry standard TA-Lib via `talib-rs`.

We have wrapped all 158 technical analysis functions provided by TA-Lib so that they adhere to the QuantWave Universal Indicator pattern. This means you can use classic indicators like RSI, MACD, and Bollinger Bands natively within your Polars dataframes.

For more information, visit the [official TA-Lib website](https://ta-lib.org/) or the [talib-rs repository](https://github.com/0xcjun/talib-rs.git).

## Available Indicators

"#,
    );
    talib_intro.push_str(&talib_list);

    fs::write(docs_dir.join("SUMMARY.md"), summary).context("Failed to write SUMMARY.md")?;
    fs::write(docs_dir.join("README.md"), main_intro).context("Failed to write README.md")?;
    fs::write(docs_dir.join("indicators/README.md"), indicators_intro).context("Failed to write indicators/README.md")?;
    fs::write(docs_dir.join("indicators/native/README.md"), native_intro).context("Failed to write indicators/native/README.md")?;
    fs::write(docs_dir.join("indicators/talib/README.md"), talib_intro).context("Failed to write indicators/talib/README.md")?;

    println!("Documentation generation complete.");
    Ok(())
}

fn generate_native_docs(docs_dir: &Path, indicators_dir: &Path) -> Result<Vec<(String, String, String)>> {
    let mut generated = Vec::new();

    for entry in fs::read_dir(indicators_dir).with_context(|| format!("Failed to read indicators directory: {:?}", indicators_dir))? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        if path.extension().unwrap_or_default() == "rs" {
            let content = fs::read_to_string(&path).with_context(|| format!("Failed to read indicator file: {:?}", path))?;
            if let Ok(ast) = syn::parse_file(&content) {
                for item in ast.items {
                    if let syn::Item::Const(item_const) = item {
                        let is_metadata = match &*item_const.ty {
                            syn::Type::Path(type_path) => {
                                type_path.path.segments.last().map(|s| s.ident.to_string())
                                    == Some("IndicatorMetadata".to_string())
                            }
                            _ => false,
                        };

                        if is_metadata && let syn::Expr::Struct(expr_struct) = &*item_const.expr {
                            let mut name = String::new();
                            let mut desc = String::new();
                            let mut usage = String::new();
                            let mut keywords: Vec<String> = Vec::new();
                            let mut ehlers_summary = String::new();
                            let mut latex = String::new();
                            let mut source = String::new();
                            let mut category = String::new();
                            let mut params_str = String::new();

                            for field in &expr_struct.fields {
                                if let syn::Member::Named(ident) = &field.member {
                                    let field_name = ident.to_string();

                                    // Simple &'static str fields
                                    if let syn::Expr::Lit(expr_lit) = &field.expr && let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                        match field_name.as_str() {
                                            "name" => name = lit_str.value(),
                                            "description" => desc = lit_str.value(),
                                            "usage" => usage = lit_str.value(),
                                            "ehlers_summary" => ehlers_summary = lit_str.value(),
                                            "formula_source" => source = lit_str.value(),
                                            "formula_latex" => latex = lit_str.value(),
                                            "category" => category = lit_str.value(),
                                            _ => {}
                                        }
                                    } else if let syn::Expr::Reference(expr_ref) = &field.expr && let syn::Expr::Array(expr_array) = &*expr_ref.expr {
                                        if field_name == "keywords" {
                                            for elem in &expr_array.elems {
                                                if let syn::Expr::Lit(kw_lit) = elem && let syn::Lit::Str(s) = &kw_lit.lit {
                                                    keywords.push(s.value());
                                                }
                                            }
                                        } else if field_name == "params" {
                                            for elem in &expr_array.elems {
                                                if let syn::Expr::Struct(param_struct) = elem {
                                                    let mut p_name = String::new();
                                                    let mut p_def = String::new();
                                                    let mut p_desc = String::new();
                                                    for p_field in &param_struct.fields {
                                                        if let syn::Member::Named(p_ident) = &p_field.member && let syn::Expr::Lit(p_expr_lit) = &p_field.expr && let syn::Lit::Str(p_lit_str) = &p_expr_lit.lit {
                                                            match p_ident.to_string().as_str() {
                                                                "name" => { p_name = p_lit_str.value() }
                                                                "default" => { p_def = p_lit_str.value() }
                                                                "description" => { p_desc = p_lit_str.value() }
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                    params_str.push_str(&format!(
                                                        "- `{}` (default: {}): {}\n",
                                                        p_name, p_def, p_desc
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            let filename = name.to_lowercase()
                                .chars()
                                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                                .collect::<String>()
                                .split('_')
                                .filter(|s| !s.is_empty())
                                .collect::<Vec<_>>()
                                .join("_");

                            let mut md = String::new();
                            md.push_str(&format!("# {}\n\n", name));
                            let cat_label = if category.is_empty() { "General" } else { &category };
                            let mut meta_line = format!(
                                "<div class=\"indicator-meta\"><span class=\"category-badge\">{}</span>",
                                cat_label
                            );
                            for kw in &keywords {
                                meta_line.push_str(&format!(" <span class=\"kw-badge\">{}</span>", kw));
                            }
                            meta_line.push_str("</div>\n\n");
                            md.push_str(&meta_line);
                            md.push_str(&format!("{}\n\n", desc));

                            if !usage.is_empty() {
                                md.push_str("## Usage\n\n");
                                md.push_str(&format!("{}\n\n", usage));
                            }

                            if !ehlers_summary.is_empty() {
                                md.push_str("## Background\n\n");
                                md.push_str(&format!("> {}\n\n", ehlers_summary));
                            }

                            if !params_str.is_empty() {
                                md.push_str("## Parameters\n\n");
                                md.push_str(&params_str);
                                md.push('\n');
                            }

                            md.push_str("## Formula\n\n");
                            md.push_str(&latex);
                            md.push_str("\n\n");

                            if !source.is_empty() {
                                md.push_str(&format!("[Source]({})\n", source));
                            }

                            let out_path = docs_dir.join(format!("indicators/native/{}.md", filename));
                            fs::write(&out_path, md).with_context(|| format!("Failed to write indicator documentation: {:?}", out_path))?;
                            generated.push((name, filename, category));
                        }
                    }
                }
            }
        }
    }

    generated.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(generated)
}

fn generate_talib_docs() -> Result<String> {
    let mut list = String::new();
    println!("Fetching TA-Lib API XML...");

    let xml_url = "https://raw.githubusercontent.com/TA-Lib/ta-lib/master/ta_func_api.xml";
    let xml_data = match get(xml_url) {
        Ok(resp) => resp.text()?,
        Err(e) => {
            println!("Warning: Could not fetch TA-Lib XML: {}", e);
            return Ok(list);
        }
    };

    let doc = match Document::parse(&xml_data) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Warning: Could not parse TA-Lib XML: {}", e);
            return Ok(list);
        }
    };

    let mut indicators = Vec::new();
    for node in doc
        .descendants()
        .filter(|n| n.has_tag_name("FinancialFunction"))
    {
        let abbr = node
            .children()
            .find(|n| n.has_tag_name("Abbreviation"))
            .and_then(|n| n.text())
            .unwrap_or("");
        let name = node
            .children()
            .find(|n| n.has_tag_name("ShortDescription"))
            .and_then(|n| n.text())
            .unwrap_or("");

        if abbr.is_empty() {
            continue;
        }
        indicators.push((abbr.to_string(), name.to_string()));
    }

    indicators.sort_by(|a, b| a.0.cmp(&b.0));

    for (abbr, name) in indicators {
        list.push_str(&format!("- **`{}`**: {}\n", abbr, name));
    }

    Ok(list)
}
