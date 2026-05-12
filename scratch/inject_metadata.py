import os

INDICATORS = {
    "heikin_ashi.rs": [
        {
            "name": "HEIKIN_ASHI",
            "title": "Heikin-Ashi",
            "desc": "Heikin-Ashi candles filter market noise to reveal the underlying trend.",
            "params": [],
            "source": "https://www.investopedia.com/trading/heikin-ashi-better-candlestick/",
            "latex": r"HA_{Close} = \frac{O + H + L + C}{4} \\ HA_{Open} = \frac{HA_{Open, t-1} + HA_{Close, t-1}}{2}",
            "gold": "heikin_ashi.json"
        }
    ],
    "smoothing.rs": [
        {
            "name": "SMA",
            "title": "Simple Moving Average",
            "desc": "The Simple Moving Average calculates the unweighted mean of the previous N data points.",
            "params": [("period", "14", "Smoothing period")],
            "source": "https://www.investopedia.com/terms/s/sma.asp",
            "latex": r"SMA = \frac{1}{n} \sum_{i=1}^{n} P_i",
            "gold": "sma.json"
        },
        {
            "name": "EMA",
            "title": "Exponential Moving Average",
            "desc": "The Exponential Moving Average gives more weight to recent prices.",
            "params": [("period", "14", "Smoothing period")],
            "source": "https://www.investopedia.com/terms/e/ema.asp",
            "latex": r"EMA = P_t \times \alpha + EMA_{t-1} \times (1 - \alpha)",
            "gold": "ema.json"
        },
        {
            "name": "WMA",
            "title": "Weighted Moving Average",
            "desc": "The Weighted Moving Average assigns linearly decreasing weights.",
            "params": [("period", "14", "Smoothing period")],
            "source": "https://www.investopedia.com/articles/technical/060401.asp",
            "latex": r"WMA = \frac{P_1 \times n + P_2 \times (n-1) + \dots}{n + (n-1) + \dots + 1}",
            "gold": "wma.json"
        }
    ],
    "ichimoku.rs": [
        {
            "name": "ICHIMOKU",
            "title": "Ichimoku Cloud",
            "desc": "Ichimoku Kinko Hyo is a comprehensive indicator that defines support and resistance, identifies trend direction, gauges momentum and provides trading signals.",
            "params": [
                ("tenkan_period", "9", "Tenkan-sen period"),
                ("kijun_period", "26", "Kijun-sen period"),
                ("senkou_span_b_period", "52", "Senkou Span B period")
            ],
            "source": "https://www.investopedia.com/terms/i/ichimoku-cloud.asp",
            "latex": r"\text{Tenkan-sen} = \frac{\text{Highest High} + \text{Lowest Low}}{2} \text{ for past 9 periods}",
            "gold": "ichimoku.json"
        }
    ],
    "donchian.rs": [
        {
            "name": "DONCHIAN",
            "title": "Donchian Channels",
            "desc": "Donchian Channels are volatility indicators formed by taking the highest high and the lowest low of the last N periods.",
            "params": [("period", "20", "Channel period")],
            "source": "https://www.investopedia.com/terms/d/donchianchannels.asp",
            "latex": r"UC = \max(H_{t-n \dots t}) \\ LC = \min(L_{t-n \dots t})",
            "gold": "donchian.json"
        }
    ],
    "fractals.rs": [
        {
            "name": "FRACTALS",
            "title": "Bill Williams Fractals",
            "desc": "Fractals are indicators on candlestick charts that identify reversal points in the market.",
            "params": [],
            "source": "https://www.investopedia.com/terms/f/fractal.asp",
            "latex": r"\text{Up Fractal} = \text{High} > \text{High}_{t-1, t-2, t+1, t+2}",
            "gold": "fractals.json"
        }
    ],
    "hma.rs": [
        {
            "name": "HMA",
            "title": "Hull Moving Average",
            "desc": "The Hull Moving Average (HMA) aims to reduce lag while maintaining smoothness.",
            "params": [("period", "14", "Smoothing period")],
            "source": "https://alanhull.com/hull-moving-average",
            "latex": r"HMA = WMA(2 \times WMA(\frac{n}{2}) - WMA(n), \sqrt{n})",
            "gold": "hma.json"
        }
    ],
    "atr_ts.rs": [
        {
            "name": "ATR_TS",
            "title": "ATR Trailing Stop",
            "desc": "A trailing stop based on Average True Range to keep trades in a trend.",
            "params": [
                ("period", "10", "ATR period"),
                ("multiplier", "3.0", "ATR Multiplier")
            ],
            "source": "https://www.tradingview.com/support/solutions/43000589105-average-true-range-atr/",
            "latex": r"Stop = P_{high} - (Multiplier \times ATR)",
            "gold": "atr_ts.json"
        }
    ],
    "tema.rs": [
        {
            "name": "TEMA",
            "title": "Triple Exponential Moving Average",
            "desc": "TEMA reduces the lag of traditional EMAs.",
            "params": [("period", "14", "Smoothing period")],
            "source": "https://www.investopedia.com/terms/t/triple-exponential-moving-average.asp",
            "latex": r"TEMA = (3 \times EMA_1) - (3 \times EMA_2) + EMA_3",
            "gold": "tema.json"
        },
        {
            "name": "ZLEMA",
            "title": "Zero Lag Exponential Moving Average",
            "desc": "ZLEMA attempts to eliminate the inherent lag associated with moving averages.",
            "params": [("period", "14", "Smoothing period")],
            "source": "https://en.wikipedia.org/wiki/Zero_lag_exponential_moving_average",
            "latex": r"ZLEMA = EMA(Price + (Price - Price_{t - (period - 1)/2}))",
            "gold": "zlema.json"
        }
    ],
    "volatility.rs": [
        {
            "name": "TRUE_RANGE",
            "title": "True Range",
            "desc": "True Range measures daily volatility.",
            "params": [],
            "source": "https://www.investopedia.com/terms/a/atr.asp",
            "latex": r"TR = \max(H - L, |H - C_{t-1}|, |L - C_{t-1}|)",
            "gold": "true_range.json"
        },
        {
            "name": "ATR",
            "title": "Average True Range",
            "desc": "ATR represents the average of true ranges over a specified period.",
            "params": [("period", "14", "Smoothing period")],
            "source": "https://www.investopedia.com/terms/a/atr.asp",
            "latex": r"ATR = \frac{ATR_{t-1} \times (n-1) + TR_t}{n}",
            "gold": "atr.json"
        }
    ],
    "vortex.rs": [
        {
            "name": "VORTEX",
            "title": "Vortex Indicator",
            "desc": "The Vortex Indicator helps identify the start of a new trend or the continuation of an existing one.",
            "params": [("period", "14", "Period")],
            "source": "https://www.investopedia.com/terms/v/vortex-indicator-vi.asp",
            "latex": r"VI+ = \frac{\sum VM+}{\sum TR} \\ VI- = \frac{\sum VM-}{\sum TR}",
            "gold": "vortex.json"
        }
    ],
    "wavetrend.rs": [
        {
            "name": "WAVETREND",
            "title": "WaveTrend Oscillator",
            "desc": "WaveTrend is an oscillator that helps identify overbought and oversold conditions.",
            "params": [
                ("n1", "10", "Channel Length"),
                ("n2", "21", "Average Length")
            ],
            "source": "https://www.tradingview.com/script/2KE8wTuF-Indicator-WaveTrend-Oscillator-WT/",
            "latex": r"WT_1 = EMA(ESA, n_2)",
            "gold": "wavetrend.json"
        }
    ],
    "vwap.rs": [
        {
            "name": "VWAP",
            "title": "Anchored VWAP",
            "desc": "Volume Weighted Average Price anchored to a specific starting point.",
            "params": [],
            "source": "https://www.investopedia.com/terms/v/vwap.asp",
            "latex": r"VWAP = \frac{\sum (Price \times Volume)}{\sum Volume}",
            "gold": "vwap.json"
        }
    ],
    "statistics.rs": [
        {
            "name": "STDDEV",
            "title": "Standard Deviation",
            "desc": "Standard Deviation is a statistical measure of market volatility.",
            "params": [("period", "14", "Period")],
            "source": "https://www.investopedia.com/terms/s/standarddeviation.asp",
            "latex": r"\sigma = \sqrt{ \frac{\sum (x_i - \mu)^2}{N} }",
            "gold": "stddev.json"
        },
        {
            "name": "LINREG",
            "title": "Linear Regression",
            "desc": "Linear Regression plots a straight line that best fits the data prices.",
            "params": [("period", "14", "Period")],
            "source": "https://www.investopedia.com/terms/l/linearregression.asp",
            "latex": r"y = a + bx",
            "gold": "linreg.json"
        }
    ],
    "ttm_squeeze.rs": [
        {
            "name": "TTM_SQUEEZE",
            "title": "TTM Squeeze",
            "desc": "TTM Squeeze measures the relationship between Bollinger Bands and Keltner Channels to identify volatility consolidations.",
            "params": [
                ("bb_period", "20", "Bollinger Bands Period"),
                ("bb_mult", "2.0", "Bollinger Bands Multiplier"),
                ("kc_period", "20", "Keltner Channel Period"),
                ("kc_mult", "1.5", "Keltner Channel Multiplier")
            ],
            "source": "https://www.investopedia.com/articles/active-trading/110714/intro-ttm-squeeze-indicator.asp",
            "latex": r"\text{Squeeze} = BB_{width} < KC_{width}",
            "gold": "ttm_squeeze.json"
        }
    ],
    "keltner.rs": [
        {
            "name": "KELTNER",
            "title": "Keltner Channels",
            "desc": "Keltner Channels are volatility-based envelopes set above and below an exponential moving average.",
            "params": [
                ("period", "20", "EMA Period"),
                ("multiplier", "2.0", "ATR Multiplier")
            ],
            "source": "https://www.investopedia.com/terms/k/keltnerchannel.asp",
            "latex": r"UC = EMA + (Multiplier \times ATR)",
            "gold": "keltner.json"
        }
    ],
    "pivot_points.rs": [
        {
            "name": "PIVOT_POINTS",
            "title": "Pivot Points",
            "desc": "Pivot Points are used to determine overall trend over different time frames.",
            "params": [],
            "source": "https://www.investopedia.com/terms/p/pivotpoint.asp",
            "latex": r"P = \frac{H + L + C}{3}",
            "gold": "pivot_points.json"
        }
    ],
    "alma.rs": [
        {
            "name": "ALMA",
            "title": "Arnaud Legoux Moving Average",
            "desc": "ALMA is designed to reduce lag while providing high smoothness.",
            "params": [
                ("period", "9", "Period"),
                ("offset", "0.85", "Offset"),
                ("sigma", "6.0", "Sigma")
            ],
            "source": "https://www.prorealcode.com/prorealtime-indicators/arnaud-legoux-moving-average-alma/",
            "latex": r"ALMA = \sum (W_i \times P_i) / \sum W_i",
            "gold": "alma.json"
        }
    ]
}

def build_struct(meta):
    params_str = ""
    for p in meta["params"]:
        params_str += f'        ParamDef {{ name: "{p[0]}", default: "{p[1]}", description: "{p[2]}" }},\n'
    
    return f"""

pub const {meta['name']}_METADATA: IndicatorMetadata = IndicatorMetadata {{
    name: "{meta['title']}",
    description: "{meta['desc']}",
    params: &[
{params_str}    ],
    formula_source: "{meta['source']}",
    formula_latex: r#"
\\[
{meta['latex']}
\\]
"#,
    gold_standard_file: "{meta['gold']}",
}};
"""

base_dir = "quantwave-core/src/indicators"

for filename, metas in INDICATORS.items():
    filepath = os.path.join(base_dir, filename)
    if not os.path.exists(filepath):
        continue
    
    with open(filepath, "r") as f:
        content = f.read()
    
    # Check if metadata is already imported
    if "use crate::indicators::metadata::" not in content:
        # Add it after the first few lines, or just at the top
        content = "use crate::indicators::metadata::{IndicatorMetadata, ParamDef};\n" + content
        
    for meta in metas:
        if f"{meta['name']}_METADATA" not in content:
            content += build_struct(meta)
            
    with open(filepath, "w") as f:
        f.write(content)

print("Metadata injected successfully.")
