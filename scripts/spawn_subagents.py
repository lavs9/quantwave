import re
import json
import sys

POLARS_LIB_PATH = "quantwave-polars/src/lib.rs"
with open(POLARS_LIB_PATH, "r") as f:
    content = f.read()

missing = [
    "anchored_vwap", "kinematic_kalman", "vortex_indicator", "sve_volatility_bands", "ttm_squeeze", "gap_momentum",
    "pelt", "zlema", "rodc", "heikin_ashi", "mavp", "mfi", "reverse_ema", "volatility_clusterer",
    "pivot_points", "hma", "obvm", "bill_williams_fractals", "donchian_channels", "gmm",
    "regimes_next_state_prob", "hmm_bull_bear", "alma", "regimes_stability_score", "geometric_patterns",
    "ta_var", "vfi", "wavetrend", "regimes_ensemble", "ta_stddev", "sarext", "keltner_channels",
    "regimes_duration_stats", "market_structure", "regimes_hmm_gas", "regimes_ms_garch",
    "adaptive_ema", "regimes_transition_matrix", "vpn", "regimes_hsmm", "autotune_filter", "tradj_ema",
    "tema", "rsmk", "exp_dev_bands", "sdo", "regimes_tar", "ichimoku_cloud", "mama", "atr_trailing_stop",
    "harrington_adx", "kalman"
]

tasks = []
current_task = []
for m in missing:
    pattern = r'pub fn ' + m + r'\s*\((.*?)\)\s*->\s*LazyFrame\s*\{(.*?)(?:pub fn |\Z)'
    match = re.search(pattern, content, re.DOTALL)
    if match:
        sig = match.group(1).replace('\n', ' ').strip()
        body = match.group(2)
        current_task.append(f"Function: {m}\nSignature: {sig}\nBody: {body}\n")
        if len(current_task) == 5:
            tasks.append("\n---\n".join(current_task))
            current_task = []
if current_task:
    tasks.append("\n---\n".join(current_task))

for i, task in enumerate(tasks):
    with open(f"scripts/task_{i}.txt", "w") as f:
        f.write(task)
    print(f"Created task_{i}.txt with {len(task.split('---'))} functions.")

