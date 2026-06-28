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
    let docs_dir = workspace_root.join("docs");
    let guides_dir = docs_dir.join("guides");
    let indicators_base = guides_dir.join("indicators");
    let indicators_dir = workspace_root.join("quantwave-core/src/indicators");

    println!("Workspace Root: {:?}", workspace_root);
    println!("Docs Dir: {:?}", docs_dir);
    println!("Indicators Dir: {:?}", indicators_dir);

    if !workspace_root.exists() {
        return Err(anyhow::anyhow!("Workspace root does not exist: {:?}", workspace_root));
    }
    
    // Create docs/guides/indicators if it doesn't exist
    if !indicators_base.exists() {
        println!("Creating missing indicators directory: {:?}", indicators_base);
        fs::create_dir_all(&indicators_base).context("Failed to create indicators directory")?;
    }

    if !indicators_dir.exists() {
        return Err(anyhow::anyhow!("Indicators directory does not exist: {:?}", indicators_dir));
    }

    fs::create_dir_all(indicators_base.join("native")).context("Failed to create indicators/native directory")?;
    fs::create_dir_all(indicators_base.join("talib")).context("Failed to create indicators/talib directory")?;

    // We will generate the SUMMARY.md dynamically based on the parsed indicators
    // This SUMMARY.md will be used by literate-nav for the Indicators section
    let mut summary = String::new();
    summary.push_str("# Indicators\n\n");
    summary.push_str("- [Overview](README.md)\n");

    let native_docs = generate_native_docs(&guides_dir, &indicators_dir)?;
    if !native_docs.is_empty() {
        let mut categories: std::collections::BTreeMap<String, Vec<(String, String)>> =
            std::collections::BTreeMap::new();
        for (name, filename, category) in native_docs {
            categories
                .entry(category)
                .or_default()
                .push((name, filename));
        }

        summary.push_str("- [Native Indicators](native/README.md)\n");
        for (category, indicators) in categories {
            summary.push_str(&format!(
                "    - {}\n",
                if category.is_empty() {
                    "General"
                } else {
                    &category
                }
            ));
            for (name, filename) in indicators {
                summary.push_str(&format!(
                    "        - [{}](native/{}.md)\n",
                    name, filename
                ));
            }
        }
    }

    let talib_list = generate_talib_docs().context("Failed to generate TA-Lib docs")?;
    summary.push_str("- [TA-Lib Wrappers](talib/README.md)\n");

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

    fs::write(indicators_base.join("SUMMARY.md"), summary).context("Failed to write SUMMARY.md")?;
    fs::write(indicators_base.join("README.md"), indicators_intro).context("Failed to write indicators/README.md")?;
    fs::write(indicators_base.join("native/README.md"), native_intro).context("Failed to write indicators/native/README.md")?;
    fs::write(indicators_base.join("talib/README.md"), talib_intro).context("Failed to write indicators/talib/README.md")?;

    println!("Documentation generation complete.");
    Ok(())
}

fn generate_native_docs(_docs_dir: &Path, _indicators_dir: &Path) -> Result<Vec<(String, String, String)>> {
    // Legacy Rust-based parser removed per quantwave-frq0.3
    // Use `python scripts/generate_native_docs.py` for standard docs.
    // We just return an empty vec here so the summary doesn't break,
    // or we could parse the JSON if we wanted to build the summary.
    // For now, we will execute the python script from here to ensure the pipeline runs!
    println!("Delegating native doc generation to python scripts/generate_native_docs.py...");
    let status = std::process::Command::new("python")
        .arg("scripts/generate_native_docs.py")
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("generate_native_docs.py failed"));
    }
    
    // We can fetch the metadata JSON to build the summary
    let output = std::process::Command::new("cargo")
        .args(&["run", "-p", "quantwave-core", "--bin", "export_metadata"])
        .output()?;
    if !output.status.success() {
        return Err(anyhow::anyhow!("export_metadata failed"));
    }
    
    let json_str = String::from_utf8(output.stdout)?;
    let metadata: Vec<serde_json::Value> = serde_json::from_str(&json_str)?;
    
    let mut generated = Vec::new();
    for item in metadata {
        let name = item["name"].as_str().unwrap_or("").to_string();
        let slug = item["slug"].as_str().unwrap_or("").to_string();
        let category = item["category"].as_str().unwrap_or("").to_string();
        generated.push((name, slug, category));
    }
    
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
