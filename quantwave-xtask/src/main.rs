use anyhow::{Context, Result};
use reqwest::blocking::get;
use roxmltree::Document;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    println!("Generating documentation...");
    
    let workspace_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string())).parent().unwrap().to_path_buf();
    let docs_dir = workspace_root.join("docs/src");
    fs::create_dir_all(&docs_dir.join("indicators/native"))?;
    fs::create_dir_all(&docs_dir.join("indicators/talib"))?;
    
    // We will generate the SUMMARY.md dynamically based on the parsed indicators
    let mut summary = String::new();
    summary.push_str("# Summary\n\n");
    summary.push_str("- [Introduction](README.md)\n");
    summary.push_str("- [Indicators](indicators/README.md)\n");

    let native_docs = generate_native_docs(&docs_dir)?;
    if !native_docs.is_empty() {
        summary.push_str("    - [Native Indicators](indicators/native/README.md)\n");
        for doc in native_docs {
            summary.push_str(&format!("        - [{}](indicators/native/{}.md)\n", doc.0, doc.1));
        }
    }

    let talib_docs = generate_talib_docs(&docs_dir)?;
    if !talib_docs.is_empty() {
        summary.push_str("    - [TA-Lib Wrappers](indicators/talib/README.md)\n");
        for doc in talib_docs {
            summary.push_str(&format!("        - [{}](indicators/talib/{}.md)\n", doc.0, doc.1));
        }
    }

    fs::write(docs_dir.join("SUMMARY.md"), summary)?;
    fs::write(docs_dir.join("README.md"), "# QuantWave Documentation\n\nWelcome to QuantWave.")?;
    fs::write(docs_dir.join("indicators/README.md"), "# Indicators\n\nOverview of all indicators.")?;
    fs::write(docs_dir.join("indicators/native/README.md"), "# Native Indicators\n\nHigh-performance native implementations.")?;
    fs::write(docs_dir.join("indicators/talib/README.md"), "# TA-Lib Wrappers\n\nWrappers around standard TA-Lib functions.")?;

    println!("Documentation generation complete.");
    Ok(())
}

fn generate_native_docs(_docs_dir: &Path) -> Result<Vec<(String, String)>> {
    // Basic syn parsing implementation will go here.
    // For now, we'll just mock the supertrend to ensure the pipeline works
    let mut generated = Vec::new();
    let supertrend_md = r#"# SuperTrend

Trend-following indicator that combines ATR for volatility bands to identify the primary market direction.

## Parameters
- `period` (default: 10): ATR length
- `multiplier` (default: 3.0): ATR multiplier

## Formula
$$
\text{SuperTrend} = \begin{cases}
\text{LowerBand} & \text{if trend is up} \\
\text{UpperBand} & \text{if trend is down}
\end{cases}
$$

[Source](https://www.tradingview.com/script/7zF0a4f8-SuperTrend-by-Mobius/)
"#;
    fs::write(_docs_dir.join("indicators/native/supertrend.md"), supertrend_md)?;
    generated.push(("SuperTrend".to_string(), "supertrend".to_string()));
    Ok(generated)
}

fn generate_talib_docs(docs_dir: &Path) -> Result<Vec<(String, String)>> {
    let mut generated = Vec::new();
    println!("Fetching TA-Lib API XML...");
    
    let xml_url = "https://raw.githubusercontent.com/TA-Lib/ta-lib/master/ta_func_api.xml";
    let xml_data = match get(xml_url) {
        Ok(resp) => resp.text()?,
        Err(e) => {
            println!("Warning: Could not fetch TA-Lib XML: {}", e);
            return Ok(generated);
        }
    };

    let doc = match Document::parse(&xml_data) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Warning: Could not parse TA-Lib XML: {}", e);
            return Ok(generated);
        }
    };

    for node in doc.descendants().filter(|n| n.has_tag_name("FinancialFunction")) {
        let abbr = node.children().find(|n| n.has_tag_name("Abbreviation")).and_then(|n| n.text()).unwrap_or("");
        let name = node.children().find(|n| n.has_tag_name("ShortDescription")).and_then(|n| n.text()).unwrap_or("");
        
        if abbr.is_empty() { continue; }
        
        let filename = abbr.to_lowercase();
        let mut md = String::new();
        md.push_str(&format!("# {} ({})\n\n", name, abbr));
        md.push_str(&format!("TA-Lib `{}` indicator.\n\n", abbr));
        
        // This can be expanded to parse parameters and inputs from the XML
        
        fs::write(docs_dir.join(format!("indicators/talib/{}.md", filename)), md)?;
        generated.push((abbr.to_string(), filename));
    }
    
    Ok(generated)
}
