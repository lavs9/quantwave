import os

replacements = {
    "quantwave-core/src/indicators/amfm.rs": [
        ("let mut envelope_hist = VecDeque::new();", "let mut envelope_hist = VecDeque::with_capacity(avg_len);")
    ],
    "quantwave-core/src/indicators/autotune.rs": [
        ("let mut filt_hist = VecDeque::new();", "let mut filt_hist = VecDeque::with_capacity(2 * window);")
    ],
    "quantwave-core/src/indicators/ehlers_ultimate_oscillator.rs": [
        ("let mut win = VecDeque::new();", "let mut win = VecDeque::with_capacity(period);")
    ],
    "quantwave-core/src/indicators/mesa_stochastic.rs": [
        ("let mut filt_hist = VecDeque::new();", "let mut filt_hist = VecDeque::with_capacity(2 * period);")
    ],
    "quantwave-core/src/indicators/synthetic_oscillator.rs": [
        ("let mut lp_win = VecDeque::new();", "let mut lp_win = VecDeque::with_capacity(2 * length);"),
        ("let mut roc_win = VecDeque::new();", "let mut roc_win = VecDeque::with_capacity(2 * length);")
    ],
    "quantwave-core/src/indicators/vfi.rs": [
        ("let mut dir_vol_window = VecDeque::new();", "let mut dir_vol_window = VecDeque::with_capacity(period);")
    ],
    "quantwave-core/src/indicators/voss_predictor.rs": [
        ("let mut v_hist = VecDeque::new();", "let mut v_hist = VecDeque::with_capacity(predict);")
    ]
}

for file, changes in replacements.items():
    with open(file, "r") as f:
        content = f.read()
    for old, new in changes:
        content = content.replace(old, new)
    with open(file, "w") as f:
        f.write(content)

