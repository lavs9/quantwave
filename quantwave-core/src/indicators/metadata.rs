#[derive(Debug, Clone)]
pub struct ParamDef {
    pub name: &'static str,
    pub default: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct IndicatorMetadata {
    pub name: &'static str,
    pub description: &'static str,
    pub params: &'static [ParamDef],
    pub formula_source: &'static str,
    pub formula_latex: &'static str,
    pub gold_standard_file: &'static str,
}
