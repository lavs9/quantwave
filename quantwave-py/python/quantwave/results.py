"""
Result dataclasses for indicators.

These are re-exported here for namespacing.

For backward compatibility during 0.5.x, they are still available
directly from the top-level quantwave module.
"""

# Resilient imports for gqem namespace cleanup (P1 in-progress).
# Newer Result types (OIZones*, Gex*, Straddle*) may only be in the uniffi
# generated bindings or added when PA rich-struct work (06sz, cu03) fully lands
# in the PyO3 module. We load what we can so "import quantwave" and
# "from quantwave.results import ..." succeed for the common ones.
# Missing ones become None (clear error on actual use).
# Combined with the selective population in __init__.py (no *Result to top-level
# globals), this removes the bulk of the top-level namespace pollution.

for _res_name in [
    "MacdResult", "SuperTrendResult", "BbandsResult", "StochResult",
    "IchimokuResult", "DonchianResult", "KeltnerResult", "PivotPointsResult",
    "UltimateBandsResult", "UltimateChannelResult",
]:
    try:
        exec(f"from ._quantwave import {_res_name}")
    except Exception:
        exec(f"{_res_name} = None")

for _res_name in ["OIZonesResult", "GexResult", "StraddleResult"]:
    try:
        exec(f"from ._quantwave import {_res_name}")
    except Exception:
        exec(f"{_res_name} = None")

__all__ = [
    "MacdResult", "SuperTrendResult", "BbandsResult", "StochResult",
    "IchimokuResult", "DonchianResult", "KeltnerResult", "PivotPointsResult",
    "UltimateBandsResult", "UltimateChannelResult", "OIZonesResult",
    "GexResult", "StraddleResult",
]
