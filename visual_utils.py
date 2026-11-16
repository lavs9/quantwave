"""Shared matplotlib utilities for QuantWave documentation preview generation."""

from __future__ import annotations

import re
from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.patches as patches
import numpy as np

SCRIPT_DIR = Path(__file__).resolve().parent
INDICATOR_OUT = SCRIPT_DIR / "assets" / "indicator-previews"
CANDLE_OUT = SCRIPT_DIR / "assets" / "candlestick-previews"

EHLERS_COLOR = "#6366f1"
CLASSIC_COLOR = "#0ea5e9"
OSC_COLOR = "#f59e0b"
BAND_COLOR = "#8b5cf6"
VOL_COLOR = "#64748b"
BULL = "#16a34a"
BEAR = "#dc2626"
NEUTRAL = "#854d0e"

plt.rcParams.update({
    "figure.facecolor": "white",
    "axes.facecolor": "white",
    "savefig.facecolor": "white",
    "font.size": 8,
    "lines.linewidth": 1.35,
    "axes.edgecolor": "#334155",
    "axes.grid": False,
})


def slugify(name: str) -> str:
    raw = "".join(c.lower() if c.isalnum() else "_" for c in name)
    return "_".join(part for part in raw.split("_") if part)


def synthetic_price(n: int = 140, seed: int = 42, regime: str = "mixed") -> np.ndarray:
    rng = np.random.default_rng(seed)
    t = np.linspace(0, 8 * np.pi, n)
    if regime == "trend":
        base = np.linspace(100, 112, n)
        return base + rng.normal(0, 0.45, n)
    if regime == "cycle":
        return 100 + 3.2 * np.sin(t) + 1.1 * np.sin(t * 2.3) + rng.normal(0, 0.35, n)
    trend = np.linspace(100, 108, n)
    cycle = 2.0 * np.sin(t)
    return trend + cycle + np.cumsum(rng.normal(0, 0.12, n))


def sma(x: np.ndarray, period: int) -> np.ndarray:
    out = np.full_like(x, np.nan, dtype=float)
    for i in range(period - 1, len(x)):
        out[i] = x[i - period + 1 : i + 1].mean()
    return np.nan_to_num(out, nan=x[0])


def ema(x: np.ndarray, period: int) -> np.ndarray:
    alpha = 2.0 / (period + 1)
    out = np.zeros_like(x, dtype=float)
    out[0] = x[0]
    for i in range(1, len(x)):
        out[i] = alpha * x[i] + (1 - alpha) * out[i - 1]
    return out


def rsi_series(x: np.ndarray, period: int = 14) -> np.ndarray:
    delta = np.diff(x, prepend=x[0])
    gain = np.where(delta > 0, delta, 0.0)
    loss = np.where(delta < 0, -delta, 0.0)
    avg_gain = ema(gain, period)
    avg_loss = ema(loss, period)
    rs = np.divide(avg_gain, avg_loss, out=np.zeros_like(avg_gain), where=avg_loss > 1e-12)
    return 100 - (100 / (1 + rs))


def super_smoother(x: np.ndarray, period: int) -> np.ndarray:
    a1 = np.exp(-1.414 * np.pi / period)
    c2 = 2.0 * a1 * np.cos(1.414 * np.pi / period)
    c3 = -a1 * a1
    c1 = (1.0 + c2 - c3) / 4.0
    out = np.zeros_like(x)
    for i in range(len(x)):
        if i < 2:
            out[i] = x[i]
        else:
            out[i] = (
                (1 - c1) * x[i]
                + (2 * c1 - c2) * x[i - 1]
                - (c1 + c3) * x[i - 2]
                + c2 * out[i - 1]
                + c3 * out[i - 2]
            )
    return out


def viz_type(category: str, keywords: list[str], slug: str, is_pattern: bool, is_ehlers: bool) -> str:
    if is_pattern:
        return "candle"
    kws = {k.lower() for k in keywords}
    cat = (category or "").lower()
    if "oscillator" in kws or "overbought" in kws or "stochastic" in kws:
        return "oscillator"
    if "macd" in slug or "ppo" in slug or "apo" in slug:
        return "dual_line"
    if "histogram" in kws or slug.endswith("_hist"):
        return "histogram"
    if "volume" in kws or cat == "volume":
        return "volume"
    if "bands" in kws or "channel" in kws or "squeeze" in slug or "keltner" in slug or "donchian" in slug:
        return "bands"
    if (
        is_ehlers
        or "ehlers" in kws
        or "dsp" in kws
        or "filter" in kws
        or "laguerre" in kws
        or "hilbert" in kws
        or cat == "ehlers dsp"
    ):
        return "ehlers_dual"
    if "moving-average" in kws or "smoothing" in kws or "ema" in kws or "sma" in slug:
        return "smoother"
    if "trend" in kws or "supertrend" in slug or "parabolic" in slug:
        return "trend"
    return "sparkline"


def _clean_axes(ax: plt.Axes) -> None:
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.set_xticks([])
    ax.set_yticks([])


def save_indicator_figure(fig: plt.Figure, slug: str) -> Path:
    INDICATOR_OUT.mkdir(parents=True, exist_ok=True)
    path = INDICATOR_OUT / f"{slug}.png"
    fig.savefig(path, dpi=160, bbox_inches="tight", pad_inches=0.1, facecolor="white")
    plt.close(fig)
    return path


def save_candle_figure(fig: plt.Figure, slug: str) -> Path:
    CANDLE_OUT.mkdir(parents=True, exist_ok=True)
    path = CANDLE_OUT / f"{slug}.png"
    fig.savefig(path, dpi=160, bbox_inches="tight", pad_inches=0.08, facecolor="white")
    plt.close(fig)
    return path


def draw_candle(ax, x, o, h, l, c, width=0.55, color=None):
    color = color or (BULL if c >= o else BEAR)
    ax.plot([x, x], [l, h], color=color, linewidth=1.15, solid_capstyle="round")
    body_bottom = min(o, c)
    body_height = abs(c - o)
    rect = patches.Rectangle(
        (x - width / 2, body_bottom),
        width,
        body_height,
        facecolor=color,
        edgecolor="#111",
        linewidth=0.25,
    )
    ax.add_patch(rect)


def generate_indicator_preview(
    slug: str,
    title: str,
    category: str,
    keywords: list[str],
    *,
    is_pattern: bool = False,
    is_ehlers: bool = False,
    struct_name: str = "",
) -> Path:
    vtype = viz_type(category, keywords, slug, is_pattern, is_ehlers)
    seed = abs(hash(slug)) % (2**31)
    price = synthetic_price(seed=seed, regime="cycle" if is_ehlers else "mixed")
    n = len(price)
    x = np.arange(n)

    if vtype == "oscillator":
        ind = rsi_series(price) if "rsi" in slug else 50 + 30 * np.sin(np.linspace(0, 5 * np.pi, n))
        ind = np.clip(ind, 0, 100)
        fig, ax = plt.subplots(figsize=(5.8, 1.9))
        ax.plot(x, ind, color=OSC_COLOR, alpha=0.96)
        ax.axhline(70, color="#fca5a5", linewidth=0.8, linestyle="--", alpha=0.7)
        ax.axhline(30, color="#86efac", linewidth=0.8, linestyle="--", alpha=0.7)
        color = OSC_COLOR
    elif vtype == "smoother":
        smooth = ema(price, 14)
        fig, ax = plt.subplots(figsize=(5.8, 2.1))
        ax.plot(x, price, color="#94a3b8", alpha=0.65, linewidth=1.0, label="Price")
        ax.plot(x, smooth, color=CLASSIC_COLOR, linewidth=1.5, label=title)
        ax.legend(loc="upper left", frameon=False, fontsize=7)
        color = CLASSIC_COLOR
    elif vtype == "bands":
        mid = sma(price, 20)
        std = np.array([price[max(0, i - 19) : i + 1].std() if i >= 19 else 0.5 for i in range(n)])
        upper, lower = mid + 2 * std, mid - 2 * std
        fig, ax = plt.subplots(figsize=(5.8, 2.1))
        ax.fill_between(x, lower, upper, color=BAND_COLOR, alpha=0.12)
        ax.plot(x, price, color="#94a3b8", alpha=0.7, linewidth=1.0)
        ax.plot(x, mid, color=BAND_COLOR, linewidth=1.3)
        color = BAND_COLOR
    elif vtype == "volume":
        vol = np.abs(np.diff(price, prepend=price[0])) * 800 + np.random.default_rng(seed).uniform(200, 600, n)
        fig, ax = plt.subplots(figsize=(5.8, 1.9))
        ax.bar(x, vol, color=VOL_COLOR, alpha=0.75, width=0.85)
        color = VOL_COLOR
    elif vtype == "ehlers_dual":
        filt = super_smoother(price, 20)
        fig, ax = plt.subplots(figsize=(5.8, 2.1))
        ax.plot(x, price, color="#94a3b8", alpha=0.65, linewidth=1.0, label="Synthetic Input")
        ax.plot(x, filt, color=EHLERS_COLOR, linewidth=1.5, label=title)
        ax.legend(loc="upper left", frameon=False, fontsize=7)
        color = EHLERS_COLOR
    elif vtype == "trend":
        trend_line = ema(price, 10)
        fig, ax = plt.subplots(figsize=(5.8, 2.1))
        ax.plot(x, price, color="#94a3b8", alpha=0.65, linewidth=1.0)
        ax.plot(x, trend_line, color=CLASSIC_COLOR, linewidth=1.6, linestyle="-")
        color = CLASSIC_COLOR
    elif vtype == "dual_line":
        fast, slow = ema(price, 12), ema(price, 26)
        signal = ema(fast - slow, 9)
        fig, ax = plt.subplots(figsize=(5.8, 2.1))
        ax.plot(x, fast - slow, color=CLASSIC_COLOR, label="Line")
        ax.plot(x, signal, color=OSC_COLOR, label="Signal")
        ax.legend(loc="upper left", frameon=False, fontsize=7)
        color = CLASSIC_COLOR
    elif vtype == "histogram":
        hist = np.diff(ema(price, 12), prepend=ema(price, 12)[0])
        colors = [BULL if v >= 0 else BEAR for v in hist]
        fig, ax = plt.subplots(figsize=(5.8, 1.9))
        ax.bar(x, hist, color=colors, alpha=0.8, width=0.85)
        color = CLASSIC_COLOR
    else:
        wave = price - price.mean()
        fig, ax = plt.subplots(figsize=(5.8, 1.9))
        ax.plot(x, wave + price.mean(), color=CLASSIC_COLOR, alpha=0.96)
        color = CLASSIC_COLOR

    _clean_axes(ax)
    subtitle = struct_name or slug
    ax.set_title(f"{title}\n{subtitle}", fontsize=9, fontweight="bold", pad=6, loc="left", color="#1e293b")
    plt.tight_layout(pad=0.25)
    return save_indicator_figure(fig, slug)


def pattern_candle_count(slug: str) -> int:
    if any(k in slug for k in ("three", "3_", "tristar", "stars", "soldiers", "crows", "river", "methods", "strike", "outside", "inside")):
        return 3
    if any(
        k in slug
        for k in (
            "two",
            "2_",
            "engulf",
            "harami",
            "piercing",
            "dark_cloud",
            "counter",
            "matching",
            "separating",
            "hikkake",
            "gap",
            "neck",
            "thrust",
            "kicking",
            "tasuki",
            "stick",
            "homing",
            "advance",
            "break",
            "belt",
            "upside",
        )
    ):
        return 2
    return 1


def pattern_bias(slug: str, keywords: list[str]) -> str:
    s = slug.lower()
    if any(k in s for k in ("black", "bear", "evening", "gravestone", "hanging", "shooting", "dark_cloud", "crows", "upside_gap")):
        return "bear"
    if any(k in s for k in ("white", "bull", "morning", "hammer", "piercing", "soldiers", "dragonfly", "inverted_hammer", "takuri", "abandoned", "ladder")):
        return "bull"
    if "reversal" in keywords:
        return "bull"
    return "neutral"


def generate_candle_preview(slug: str, title: str, keywords: list[str], struct_name: str = "") -> Path:
    count = pattern_candle_count(slug)
    bias = pattern_bias(slug, keywords)
    fig, ax = plt.subplots(figsize=(5.4 if count == 1 else 5.8, 3.0))
    ax.set_xlim(-0.6, count + 1.2)
    ax.set_ylim(92, 112)

    if count == 1:
        if "doji" in slug or "rickshaw" in slug or "spinning" in slug:
            draw_candle(ax, 1, 102.0, 105.0, 98.5, 102.05, color=NEUTRAL)
        elif bias == "bull" or "hammer" in slug or "takuri" in slug:
            draw_candle(ax, 1, 102.0, 103.0, 96.0, 102.3, color=BULL)
        elif bias == "bear" or "shooting" in slug or "hanging" in slug:
            draw_candle(ax, 1, 102.0, 110.0, 101.5, 102.2, color=BEAR)
        else:
            draw_candle(ax, 1, 101.0, 104.0, 99.5, 103.0, color=BULL if bias == "bull" else BEAR)
    elif count == 2:
        if bias == "bull":
            draw_candle(ax, 1, 105.0, 106.0, 100.0, 100.5, color=BEAR)
            draw_candle(ax, 2, 100.0, 107.0, 99.5, 106.5, color=BULL)
        else:
            draw_candle(ax, 1, 95.0, 100.0, 94.5, 99.5, color=BULL)
            draw_candle(ax, 2, 99.0, 100.0, 92.0, 93.0, color=BEAR)
    else:
        if bias == "bull":
            draw_candle(ax, 1, 108.0, 109.0, 100.0, 100.5, color=BEAR)
            draw_candle(ax, 2, 99.0, 99.8, 98.5, 99.2, color=NEUTRAL)
            draw_candle(ax, 3, 99.5, 106.0, 99.0, 105.5, color=BULL)
        else:
            draw_candle(ax, 1, 95.0, 98.0, 94.5, 97.8, color=BULL)
            draw_candle(ax, 2, 98.0, 98.8, 97.5, 98.2, color=NEUTRAL)
            draw_candle(ax, 3, 98.5, 99.0, 92.0, 92.5, color=BEAR)

    label_color = BULL if bias == "bull" else BEAR if bias == "bear" else NEUTRAL
    ax.text(
        count / 2 + 0.5,
        92.5,
        title,
        ha="center",
        fontsize=9,
        fontweight="bold",
        color=label_color,
    )
    cdl = struct_name or slug.upper()
    ax.set_title(f"{cdl} — TA-Lib candlestick pattern (synthetic ideal)", fontsize=10, loc="left", pad=3)
    ax.axis("off")
    return save_candle_figure(fig, slug)