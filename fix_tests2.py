import os

fixes = {
    "quantwave-core/src/indicators/amfm.rs": ("VecDeque::with_capacity(avg_len)", "VecDeque::with_capacity(20)"),
    "quantwave-core/src/indicators/ehlers_ultimate_oscillator.rs": ("VecDeque::with_capacity(period)", "VecDeque::with_capacity(20)"),
    "quantwave-core/src/indicators/mesa_stochastic.rs": ("VecDeque::with_capacity(2 * period)", "VecDeque::with_capacity(40)"),
    "quantwave-core/src/indicators/synthetic_oscillator.rs": ("VecDeque::with_capacity(2 * length)", "VecDeque::with_capacity(40)")
}

for file, (old, new) in fixes.items():
    if os.path.exists(file):
        with open(file, "r") as f:
            content = f.read()
        content = content.replace(old, new)
        with open(file, "w") as f:
            f.write(content)
