//! Indicator metadata definition.
//!
//! IMPORTANT PROCESS RULE (see AGENTS.md + task quantwave-i9dn):
//! When adding a new indicator or making significant changes to an existing one,
//! you MUST also create/update its corresponding `XXX_METADATA` constant in the
//! same session, before landing the plane / doing the final git push.
//!
//! This metadata is the single source of truth used by:
//! - Documentation generation (mkdocs)
//! - Python package DX features
//!
//! Long-term this will be auto-generated (see quantwave-iqq7).

#[derive(Debug, Clone)]
pub struct ParamDef {
    pub name: &'static str,
    pub default: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct IndicatorMetadata {
    pub name: &'static str,
    /// One-sentence plain-English description of what the indicator computes.
    pub description: &'static str,
    /// Practical usage: when and why a trader would apply this indicator.
    pub usage: &'static str,
    /// Searchable topic tags (e.g. "momentum", "oscillator", "ehlers", "dsp").
    pub keywords: &'static [&'static str],
    /// 3-4 line authoritative summary from Ehlers' papers/books or StockCharts.
    pub ehlers_summary: &'static str,
    pub params: &'static [ParamDef],
    pub formula_source: &'static str,
    pub formula_latex: &'static str,
    pub gold_standard_file: &'static str,
    pub category: &'static str,
}
