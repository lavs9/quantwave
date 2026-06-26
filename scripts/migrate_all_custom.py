import re
import os

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

# We'll skip regimes for now if they're too hard, but let's see.

rust_code = """use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use quantwave_core::*;
use quantwave_core::traits::Next;

"""

py_code = ""

for m in missing:
    pattern = r'pub fn ' + m + r'\s*\((.*?)\)\s*->\s*LazyFrame\s*\{(.*?)(?:pub fn |\Z)'
    match = re.search(pattern, content, re.DOTALL)
    if match:
        sig = match.group(1).replace('\n', ' ').strip()
        body = match.group(2)
        print(f"Parsed {m}")
        
        # We won't actually do regex rewrite here, it's too complex to handle all edge cases.
        # Instead, we will print it so we know we got it.

