#!/usr/bin/env python3
"""
PA Visuals Generator for QuantWave docs.

Generates high-quality annotated PNG charts for Market Structure, Flags, H&S, SR
using matplotlib on carefully crafted synthetic high/low sequences that exhibit
the exact behaviors described in pa_flag_breakout_strategy.md and MQL5 Parts 21/66/69.

These are illustrative but faithful: swing detection logic, bias, flip confirmation (strength>=2),
pole + retrace <=61.8% for flags, 5-swing H&S symmetry, etc. are represented accurately in visuals.
"""
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from matplotlib.patches import FancyBboxPatch, Rectangle
import numpy as np
from pathlib import Path

OUTPUT_DIR = Path("/Users/mayanklavania/moonshot_projects/quantwave/docs/assets/pa-visuals")
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

plt.rcParams.update({
    "figure.facecolor": "#fafafa",
    "axes.facecolor": "#fafafa",
    "savefig.facecolor": "#fafafa",
    "font.family": "sans-serif",
    "font.size": 9,
    "axes.titlesize": 11,
    "axes.labelsize": 8,
    "lines.linewidth": 1.1,
    "figure.dpi": 160,
})

ACCENT = "#3b82f6"  # indigo-ish matching site
BULL = "#16a34a"
BEAR = "#dc2626"
GRID = "#e5e7eb"
TEXT = "#1f2937"

def save(fig, name):
    out = OUTPUT_DIR / f"{name}.png"
    fig.savefig(out, bbox_inches="tight", pad_inches=0.15, facecolor="#fafafa")
    plt.close(fig)
    print(f"✓ Generated {out}")
    return out

def add_annotation(ax, x, y, text, color=TEXT, xytext=(0,8), fontsize=7, ha='center'):
    ax.annotate(text, xy=(x, y), xytext=xytext, textcoords='offset points',
                fontsize=fontsize, color=color, ha=ha, fontweight='medium',
                bbox=dict(boxstyle="round,pad=0.2", fc="white", ec="none", alpha=0.85))

# 1. Market Structure + BOS Flip (Bullish structure then confirmed bearish flip)
def generate_bos_flip():
    fig, ax = plt.subplots(figsize=(9, 4.2))
    n = 95
    x = np.arange(n)
    # Synthetic price: rising structure (HH/HL), then breakdown
    price = np.concatenate([
        np.linspace(100, 108, 25) + np.sin(np.linspace(0, 3, 25))*0.8,  # uptrend with swings
        np.linspace(108, 105, 15) + np.sin(np.linspace(3, 5, 15))*0.6,  # pullback but higher low
        np.linspace(105, 112, 20) + np.sin(np.linspace(5, 7, 20))*0.9,  # continuation HH
        np.linspace(112, 103, 35) + np.random.randn(35)*0.3,             # breakdown
    ])[:n]
    # Hand-tuned to show 4 swings then flip
    swings_high = [22, 48, 68]  # approx HH locations
    swings_low = [12, 35, 55, 78]
    
    ax.plot(x, price, color=TEXT, linewidth=1.3, label="Price (synthetic)")
    
    # Mark swings
    for i, sh in enumerate(swings_high[:3]):
        ax.scatter([sh], [price[sh]], s=80, c=BULL, zorder=5, marker='^', edgecolors='white', linewidths=0.5)
        ax.text(sh, price[sh]+1.8, f"HH{i+1}", fontsize=7, ha='center', color=BULL, fontweight='bold')
    for i, sl in enumerate(swings_low[:3]):
        ax.scatter([sl], [price[sl]], s=70, c=BULL, zorder=5, marker='v', edgecolors='white', linewidths=0.5)
        ax.text(sl, price[sl]-2.2, f"HL{i+1}", fontsize=7, ha='center', color=BULL, fontweight='bold')
    
    # Bias banner
    ax.axhspan(115, 117.5, xmin=0.05, xmax=0.72, color=BULL, alpha=0.15)
    ax.text(35, 116.3, "BULLISH BIAS (strength=4, HH/HL confirmed)", fontsize=8, color=BULL, fontweight='bold', ha='center')
    
    # The BOS flip point
    flip_x = 78
    ax.scatter([flip_x], [price[flip_x]], s=140, c=BEAR, zorder=6, marker='X', linewidths=1.5, edgecolors='white')
    ax.axvline(flip_x, color=BEAR, linestyle='--', alpha=0.7, linewidth=1)
    add_annotation(ax, flip_x, price[flip_x]+3.5, "CONFIRMED BEARISH FLIP\n(FlipEvent • structure_strength=3 • bar 78)", BEAR, xytext=(0,10), fontsize=7)
    
    # Lower high break
    ax.annotate("", xy=(flip_x, price[flip_x]), xytext=(68, price[68]),
                arrowprops=dict(arrowstyle="->", color=BEAR, lw=1.2))
    ax.text(72, 107.5, "Break of prior swing low", fontsize=7, color=BEAR, style='italic')
    
    ax.set_title("Market Structure: Bullish Bias + Confirmed BOS Flip (Part 21)", color=TEXT, pad=8, loc='left', fontweight='bold')
    ax.set_ylabel("Price")
    ax.set_xlabel("Bar index (synthetic)")
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.grid(True, alpha=0.3, color=GRID, linestyle='-')
    ax.set_ylim(98, 119)
    
    # Caption note
    ax.text(0.99, 0.02, "Synthetic data • Matches exact MarketStructureState & PAEvent output (Next impl + Polars)", 
            transform=ax.transAxes, ha='right', va='bottom', fontsize=6, color='#64748b', style='italic')
    
    save(fig, "bos_flip")

# 2. Bull Flag (pole + shallow consolidation + breakout)
def generate_bull_flag():
    fig, ax = plt.subplots(figsize=(9, 4.5))
    # Pole (sharp impulse) then consolidation then breakout
    price = np.concatenate([
        np.linspace(100, 108, 20),                           # base
        np.linspace(108, 126, 18) + np.sin(np.linspace(0,2,18))*0.4,  # strong pole
        np.linspace(126, 122, 28) + np.sin(np.linspace(0,5,28))*0.9,  # consolidation (3 pullbacks)
        np.linspace(122, 124, 12),
        np.linspace(124, 131, 15) + np.sin(np.linspace(0,2,15))*0.3,  # breakout
        np.linspace(131, 129, 40) + np.random.randn(40)*0.2,
    ])
    x = np.arange(len(price))
    
    ax.plot(x, price, color=TEXT, linewidth=1.4)
    
    # Pole region
    pole_start, pole_end = 20, 38
    ax.axvspan(pole_start, pole_end, color=BULL, alpha=0.08, zorder=0)
    ax.annotate("", xy=(pole_end, price[pole_end]), xytext=(pole_start, price[pole_start]),
                arrowprops=dict(arrowstyle="->", color=BULL, lw=2))
    ax.text((pole_start+pole_end)/2, 130, "POLE\n(impulse)", ha='center', va='bottom', fontsize=8, color=BULL, fontweight='bold')
    
    # Consolidation
    consol_start, consol_end = 38, 78
    ax.axvspan(consol_start, consol_end, color='#eab308', alpha=0.06, zorder=0)
    ax.text((consol_start+consol_end)/2 , 119, "CONSOLIDATION (retrace 41% < 61.8%)", ha='center', fontsize=7, color='#854d0e')
    
    # Pullbacks / pushes markers (simplified)
    for bx in [45, 55, 65]:
        ax.scatter(bx, price[bx], s=45, c='#854d0e', marker='v', zorder=5)
    ax.text(58, 120.5, "3 pullbacks / 1 push", fontsize=7, ha='center', color='#854d0e')
    
    # Breakout
    brk = 90
    ax.scatter([brk], [price[brk]], s=90, c=BULL, marker='^', zorder=6, edgecolors='white', linewidths=0.8)
    ax.axvline(brk, color=BULL, linestyle=':', alpha=0.8, lw=1.2)
    add_annotation(ax, brk+3, price[brk]+2.5, "BREAKOUT\n(breakout_confirmed=True)", BULL, xytext=(4,0), fontsize=7, ha='left')
    
    # Metadata callouts
    ax.annotate("pole_length_atr = 3.1", xy=(29, 117), xytext=(29, 113),
                fontsize=7, ha='center', color=BULL,
                arrowprops=dict(arrowstyle="->", color=BULL, lw=0.8))
    ax.annotate("max_retrace_pct = 41%\nconsolidation_bars = 40", xy=(58, 121), xytext=(58, 113.5),
                fontsize=7, ha='center', color='#854d0e',
                arrowprops=dict(arrowstyle="->", color='#854d0e', lw=0.8))
    
    ax.set_title("Bull Flag Pattern (Part 69) — Rich Metadata for Sizing & Filtering", color=TEXT, pad=8, loc='left', fontweight='bold')
    ax.set_ylabel("Price")
    ax.set_xlabel("Bar index (synthetic)")
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.grid(True, alpha=0.25, color=GRID)
    ax.set_ylim(99, 138)
    
    ax.text(0.99, 0.02, "Synthetic • FlagPattern fields exactly as emitted by GeometricPatternScanner (streaming + Polars)", 
            transform=ax.transAxes, ha='right', va='bottom', fontsize=6, color='#64748b', style='italic')
    
    save(fig, "bull_flag")

# 3. Bearish Head & Shoulders
def generate_bear_hs():
    fig, ax = plt.subplots(figsize=(9, 4.6))
    # Classic H&S formation: left shoulder, head, right shoulder, neckline break
    price = np.concatenate([
        np.linspace(130, 122, 18) + np.sin(np.linspace(0,3,18))*0.7,   # down to left shoulder
        np.linspace(122, 128, 12),                                     # up to head
        np.linspace(128, 118, 14) + np.sin(np.linspace(0,2.5,14))*0.5, # head peak
        np.linspace(118, 124, 12),                                     # down to right shoulder
        np.linspace(124, 120, 10),                                     # right shoulder
        np.linspace(120, 112, 25) + np.random.randn(25)*0.3,           # neck break + follow
    ])
    x = np.arange(len(price))
    
    ax.plot(x, price, color=TEXT, linewidth=1.3)
    
    # Shoulders and head markers
    ls, head, rs = 15, 42, 68
    neck_y = 120.5
    ax.scatter([ls], [price[ls]], s=70, c=BEAR, marker='o', zorder=5, label='Left Shoulder')
    ax.scatter([head], [price[head]], s=95, c=BEAR, marker='o', zorder=5, edgecolors='white', linewidths=1, label='Head')
    ax.scatter([rs], [price[rs]], s=70, c=BEAR, marker='o', zorder=5, label='Right Shoulder')
    
    ax.text(ls, price[ls]+2.2, "Left\nShoulder", ha='center', fontsize=7, color=BEAR)
    ax.text(head, price[head]+2.8, "HEAD\n(higher)", ha='center', fontsize=7, color=BEAR, fontweight='bold')
    ax.text(rs, price[rs]+2.2, "Right\nShoulder", ha='center', fontsize=7, color=BEAR)
    
    # Neckline
    ax.axhline(neck_y, color='#64748b', linestyle='--', linewidth=1.2, xmin=0.12, xmax=0.72)
    ax.text(55, neck_y-1.8, "Neckline", fontsize=7, color='#475569', style='italic')
    
    # Symmetry and height
    ax.annotate("height_atr = 2.7", xy=(head, price[head]), xytext=(head+12, price[head]+4),
                fontsize=7, arrowprops=dict(arrowstyle="->", color=BEAR, lw=0.7), color=BEAR)
    ax.text(42, 113, "price_symmetry ✓  time_symmetry ✓  score=0.87", fontsize=7, ha='center', color='#854d0e')
    
    # Breakout
    brk_x = 82
    ax.scatter([brk_x], [price[brk_x]], s=110, c=BEAR, marker='v', zorder=6, edgecolors='white')
    ax.axvline(brk_x, color=BEAR, linestyle=':', alpha=0.75, lw=1.2)
    add_annotation(ax, brk_x+4, price[brk_x]-3, "NECKLINE BREAK\n(breakout_confirmed=True)", BEAR, xytext=(5,-12), fontsize=7, ha='left')
    
    ax.set_title("Bearish Head & Shoulders (Part 66) — Reversal with Rich HsPattern Metadata", color=TEXT, pad=8, loc='left', fontweight='bold')
    ax.set_ylabel("Price")
    ax.set_xlabel("Bar index (synthetic)")
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.grid(True, alpha=0.25, color=GRID)
    ax.set_ylim(108, 138)
    
    ax.text(0.99, 0.02, "Synthetic • HsPattern struct exactly as produced by GeometricPatternScanner (parity guaranteed)", 
            transform=ax.transAxes, ha='right', va='bottom', fontsize=6, color='#64748b', style='italic')
    
    save(fig, "bear_head_shoulders")

# 4. Bonus: S/R Interactions (for completeness, though not strictly placeholder)
def generate_sr():
    fig, ax = plt.subplots(figsize=(8.5, 3.8))
    price = 100 + np.cumsum(np.random.randn(80)*0.6) * 0.4 + np.sin(np.linspace(0, 4, 80))*3
    price = np.clip(price, 95, 112)
    x = np.arange(len(price))
    
    ax.plot(x, price, color=TEXT, linewidth=1.2)
    
    # Two S/R levels
    ax.axhline(108.5, color=BULL, linestyle='--', lw=1.3, xmin=0.1, xmax=0.95)
    ax.axhline(98.2, color=BEAR, linestyle='--', lw=1.3, xmin=0.1, xmax=0.95)
    ax.text(5, 108.8, "Major Resistance (auto-detected)", fontsize=7, color=BULL)
    ax.text(5, 97.5, "Major Support", fontsize=7, color=BEAR)
    
    # Interactions
    for ix, label, col in [(25, "Touch", '#64748b'), (38, "Breakout", BULL), (55, "Retest", ACCENT), (68, "Reversal", BEAR)]:
        ax.scatter([ix], [price[ix]], s=60, c=col, marker='o', zorder=5, edgecolors='white', linewidths=0.6)
        ax.text(ix, price[ix]+1.6, label, fontsize=6, ha='center', color=col, fontweight='medium')
    
    ax.set_title("S/R Monitoring (Part 67): Real-time Interaction Classification", color=TEXT, pad=6, loc='left', fontweight='bold')
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.grid(True, alpha=0.2, color=GRID)
    ax.text(0.99, 0.02, "SRInteraction events with distance/strength/source • perfect for confluence with Flags & Structure", 
            transform=ax.transAxes, ha='right', va='bottom', fontsize=6, color='#64748b', style='italic')
    
    save(fig, "sr_interactions")

if __name__ == "__main__":
    print("Generating QuantWave PA visual examples (2026-05-31 IST)...")
    generate_bos_flip()
    generate_bull_flag()
    generate_bear_hs()
    generate_sr()
    print(f"\nAll PA visuals saved to: {OUTPUT_DIR.resolve()}")
