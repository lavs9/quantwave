import re

POLARS_LIB_PATH = "quantwave-polars/src/lib.rs"
with open(POLARS_LIB_PATH, "r") as f:
    content = f.read()

missing = [
    "anchored_vwap", "filter_by_regime", "apply_regime_strategy", "regimes_conditioned_metrics",
    "kinematic_kalman", "vortex_indicator", "sve_volatility_bands", "ttm_squeeze", "gap_momentum",
    "pelt", "zlema", "rodc", "heikin_ashi", "mavp", "mfi", "reverse_ema", "volatility_clusterer",
    "pivot_points", "hma", "obvm", "bill_williams_fractals", "donchian_channels", "gmm",
    "regimes_next_state_prob", "hmm_bull_bear", "alma", "regimes_stability_score", "geometric_patterns",
    "ta_var", "vfi", "wavetrend", "regimes_ensemble", "ta_stddev", "sarext", "keltner_channels",
    "regimes_duration_stats", "market_structure", "regimes_hmm_gas", "regimes_ms_garch",
    "adaptive_ema", "regimes_transition_matrix", "vpn", "regimes_hsmm", "autotune_filter", "tradj_ema",
    "tema", "rsmk", "exp_dev_bands", "sdo", "regimes_tar", "ichimoku_cloud", "mama", "atr_trailing_stop",
    "harrington_adx", "kalman"
]

results = []
for m in missing:
    pattern = r'pub fn ' + m + r'\s*\((.*?)\)\s*->\s*LazyFrame\s*\{'
    match = re.search(pattern, content, re.DOTALL)
    if match:
        sig = match.group(1).replace('\n', ' ').strip()
        sig = re.sub(r'\s+', ' ', sig)
        
        # Find struct name
        body_pattern = r'pub fn ' + m + r'\s*\([^)]*\)\s*->\s*LazyFrame\s*\{(.*?)(?:pub fn |\Z)'
        body_match = re.search(body_pattern, content, re.DOTALL)
        struct_name = "UNKNOWN"
        if body_match:
            body = body_match.group(1)
            struct_match = re.search(r'quantwave_core::(\w+)', body)
            if struct_match:
                struct_name = struct_match.group(1)
            else:
                struct_match = re.search(r'(\w+)::new\(', body)
                if struct_match:
                    struct_name = struct_match.group(1)
        
        results.append(f"{m} | {struct_name} | {sig}")

for r in results:
    print(r)
