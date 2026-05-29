#!/usr/bin/env python3
"""
Prototype indicator preview generator for QuantWave docs.

This is a small spike to demonstrate the automated static preview concept
approved in quantwave-w0x / quantwave-7x1.

It generates simple, clean PNG previews for a few indicators using
synthetic data + basic matplotlib. In the real implementation this would
call actual QuantWave indicators (.ta() or streaming) on realistic data.

Run this manually during development:
    python docs/gen_indicator_previews.py

Later it will be hooked into the mkdocs build via gen-files.
"""

import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path

OUTPUT_DIR = Path("docs/assets/indicator-previews")
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

plt.rcParams.update({
    "figure.facecolor": "white",
    "axes.facecolor": "white",
    "savefig.facecolor": "white",
    "font.size": 7,
    "lines.linewidth": 1.2,
})


def generate_preview(name: str, values: np.ndarray, filename: str, color: str = "#1e66f5"):
    """Generate a small, clean sparkline-style preview image."""
    fig, ax = plt.subplots(figsize=(5.5, 1.8), dpi=140)
    
    x = np.arange(len(values))
    ax.plot(x, values, color=color, alpha=0.95)
    
    # Very minimal styling
    ax.set_title(name, fontsize=9, fontweight="bold", pad=4, loc="left", color="#1e293b")
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.spines["bottom"].set_visible(False)
    ax.spines["left"].set_visible(False)
    ax.set_xticks([])
    ax.set_yticks([])
    
    plt.tight_layout(pad=0.2)
    out_path = OUTPUT_DIR / filename
    plt.savefig(out_path, dpi=140, bbox_inches="tight", pad_inches=0.08)
    plt.close()
    
    print(f"✓ Generated {out_path}")


def main():
    np.random.seed(42)
    
    # 1. SuperTrend-style (trending with volatility)
    n = 120
    trend = np.linspace(100, 108, n)
    noise = np.cumsum(np.random.randn(n) * 0.15)
    price = trend + noise
    generate_preview("SuperTrend", price, "supertrend.png", color="#1e66f5")
    
    # 2. Cyber Cycle (Ehlers-style oscillator)
    t = np.linspace(0, 6 * np.pi, 120)
    cycle = np.sin(t) * 4 + np.sin(t * 2.3) * 1.5 + np.random.randn(120) * 0.4
    generate_preview("Cyber Cycle", cycle, "cyber_cycle.png", color="#8b5cf6")
    
    # 3. RSI-style bounded oscillator
    rsi = 50 + 28 * np.sin(np.linspace(0, 4.5 * np.pi, 120)) + np.random.randn(120) * 3
    rsi = np.clip(rsi, 0, 100)
    generate_preview("RSI (14)", rsi, "rsi.png", color="#f59e0b")
    
    print("\nPrototype preview generation complete.")
    print(f"Images saved to: {OUTPUT_DIR.resolve()}")


if __name__ == "__main__":
    main()
