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

fn generate_native_docs(docs_dir: &Path) -> Result<Vec<(String, String)>> {
    let mut generated = Vec::new();
    let indicators_dir = docs_dir.parent().unwrap().parent().unwrap().join("quantwave-core/src/indicators");
    
    for entry in fs::read_dir(indicators_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().unwrap_or_default() == "rs" {
            let content = fs::read_to_string(&path)?;
            if let Ok(ast) = syn::parse_file(&content) {
                for item in ast.items {
                    if let syn::Item::Const(item_const) = item {
                        let is_metadata = match &*item_const.ty {
                            syn::Type::Path(type_path) => {
                                type_path.path.segments.last().map(|s| s.ident.to_string()) == Some("IndicatorMetadata".to_string())
                            },
                            _ => false,
                        };
                        
                        if is_metadata {
                            if let syn::Expr::Struct(expr_struct) = &*item_const.expr {
                                let mut name = String::new();
                                let mut desc = String::new();
                                let mut latex = String::new();
                                let mut source = String::new();
                                let mut params_str = String::new();
                                
                                for field in &expr_struct.fields {
                                    if let syn::Member::Named(ident) = &field.member {
                                        let field_name = ident.to_string();
                                        if let syn::Expr::Lit(expr_lit) = &field.expr {
                                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                                match field_name.as_str() {
                                                    "name" => name = lit_str.value(),
                                                    "description" => desc = lit_str.value(),
                                                    "formula_source" => source = lit_str.value(),
                                                    "formula_latex" => latex = lit_str.value(),
                                                    _ => {}
                                                }
                                            }
                                        } else if field_name == "params" {
                                            if let syn::Expr::Reference(expr_ref) = &field.expr {
                                                if let syn::Expr::Array(expr_array) = &*expr_ref.expr {
                                                    for elem in &expr_array.elems {
                                                        if let syn::Expr::Struct(param_struct) = elem {
                                                            let mut p_name = String::new();
                                                            let mut p_def = String::new();
                                                            let mut p_desc = String::new();
                                                            for p_field in &param_struct.fields {
                                                                if let syn::Member::Named(p_ident) = &p_field.member {
                                                                    if let syn::Expr::Lit(p_expr_lit) = &p_field.expr {
                                                                        if let syn::Lit::Str(p_lit_str) = &p_expr_lit.lit {
                                                                            match p_ident.to_string().as_str() {
                                                                                "name" => p_name = p_lit_str.value(),
                                                                                "default" => p_def = p_lit_str.value(),
                                                                                "description" => p_desc = p_lit_str.value(),
                                                                                _ => {}
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            params_str.push_str(&format!("- `{}` (default: {}): {}\n", p_name, p_def, p_desc));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                let filename = name.to_lowercase().replace(" ", "_").replace("-", "_");
                                
                                let mut md = String::new();
                                md.push_str(&format!("# {}\n\n", name));
                                md.push_str(&format!("{}\n\n", desc));
                                
                                if !params_str.is_empty() {
                                    md.push_str("## Parameters\n\n");
                                    md.push_str(&params_str);
                                    md.push_str("\n");
                                }
                                
                                md.push_str("## Formula\n\n");
                                // latex string already contains \\[ \\] formatting from python script injection
                                md.push_str(&latex);
                                md.push_str("\n\n");
                                
                                if !source.is_empty() {
                                    md.push_str(&format!("[Source]({})\n", source));
                                }
                                
                                let out_path = docs_dir.join(format!("indicators/native/{}.md", filename));
                                fs::write(&out_path, md)?;
                                generated.push((name, filename));
                            }
                        }
                    }
                }
            }
        }
    }
    
    generated.sort_by(|a, b| a.0.cmp(&b.0));
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
