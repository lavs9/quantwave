"""Gold-standard parity cases (quantwave-5ipk.2).

Constructor args mirror quantwave-core unit tests that load each fixture.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class GoldParityCase:
    fixture: str
    streaming_name: str
    kind: str
    kwargs: dict[str, Any] | None = None
    args: tuple[Any, ...] | None = None
    field: str | None = None
    rtol: float = 1e-6


# 25 streaming cases backed by quantwave-core/tests/gold_standard/*.json
GOLD_PARITY_CASES: tuple[GoldParityCase, ...] = (
    GoldParityCase("sma_5", "sma", "scalar", kwargs={"period": 3}),
    GoldParityCase("alma_9_085_6", "alma", "scalar", kwargs={"period": 9, "offset": 0.85, "sigma": 6.0}),
    GoldParityCase("hma_14", "hma", "close_series", kwargs={"period": 14}, field="expected_hma"),
    GoldParityCase("tema_14", "tema", "close_series", kwargs={"period": 14}, field="expected_tema"),
    GoldParityCase("mad", "mad", "scalar", args=(8, 23)),
    GoldParityCase("rsih", "rsih", "scalar", kwargs={"length": 14}),
    GoldParityCase("frac_diff", "frac_diff", "scalar", kwargs={"d": 0.4, "threshold": 0.05}),
    GoldParityCase("ehlers_stochastic", "ehlers_stochastic", "scalar", args=(48, 10, 20)),
    GoldParityCase("mesa_stochastic", "mesa_stochastic", "scalar", args=(20, 48, 10)),
    GoldParityCase("voss_predictor", "voss_predictor", "tuple", kwargs={"period": 20, "predict": 3}),
    GoldParityCase("ehlers_autocorrelation", "ehlers_autocorrelation", "vec", args=(20, 10)),
    GoldParityCase("cycle_trend_analytics", "cycle_trend_analytics", "cycle_trend", args=(5, 15)),
    GoldParityCase("ehlers_loops", "ehlers_loops", "loops", args=(20, 125)),
    GoldParityCase("oc_price_rsi", "oc_price_rsi", "oc", kwargs={"period": 14}),
    GoldParityCase("donchian_5", "donchian", "donchian", kwargs={"period": 5}),
    GoldParityCase("keltner_20_20_15", "keltner", "keltner", kwargs={"ema_period": 20, "atr_period": 20, "multiplier": 1.5}),
    GoldParityCase("atr_ts_14_25", "atr_ts", "atr_ts", kwargs={"period": 14, "multiplier": 2.5}),
    GoldParityCase("vortex_14", "vortex", "vortex", kwargs={"period": 14}),
    GoldParityCase("ttm_squeeze_20_2_15", "ttm_squeeze", "ttm_squeeze", args=(20, 2.0, 1.5)),
    GoldParityCase("wavetrend_10_21_4", "wavetrend", "wavetrend", args=(10, 21, 4)),
    GoldParityCase("supertrend_10_3", "supertrend", "supertrend", kwargs={"period": 10, "multiplier": 3.0}),
    GoldParityCase("heikin_ashi", "heikin_ashi", "heikin_ashi"),
    GoldParityCase("ichimoku", "ichimoku", "ichimoku", kwargs={"tenkan": 9, "kijun": 26, "senkou_b": 52}),
    GoldParityCase("fractals", "fractals", "fractals"),
    GoldParityCase("pivot_points", "pivot_points", "pivot_points"),
)

# Fixtures present on disk but not yet wired (tracked follow-up for 5ipk.2).
GOLD_PARITY_DEFERRED: tuple[tuple[str, str], ...] = (
    ("hmm_gaussian_2state", "regime model — multi-field JSON, separate suite"),
    ("hmm_lambda_2state", "regime model — multi-field JSON, separate suite"),
)