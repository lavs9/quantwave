"""
Result dataclasses for indicators.

These are re-exported here for namespacing.

For backward compatibility during 0.5.x, they are still available
directly from the top-level quantwave module.
"""

from ._quantwave import (
    MacdResult,
    SuperTrendResult,
    BbandsResult,
    StochResult,
    IchimokuResult,
    DonchianResult,
    KeltnerResult,
    PivotPointsResult,
    UltimateBandsResult,
    UltimateChannelResult,
    OIZonesResult,
    GexResult,
    StraddleResult,
    # Add more as needed
)

__all__ = [
    "MacdResult", "SuperTrendResult", "BbandsResult", "StochResult",
    "IchimokuResult", "DonchianResult", "KeltnerResult", "PivotPointsResult",
    "UltimateBandsResult", "UltimateChannelResult", "OIZonesResult",
    "GexResult", "StraddleResult",
]
