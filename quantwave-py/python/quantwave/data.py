"""quantwave.data — optional, minimally-scoped fetchers for *real* market data.

This module is deliberately small and network-optional:

- ``fetch_yfinance``: a thin wrapper around the third-party ``yfinance``
  package, imported lazily (only inside the function body). ``yfinance`` is
  NOT a dependency of this project (core stays network-free); if it isn't
  installed, calling this function raises a clear ``ImportError`` with an
  install hint rather than failing at import time for everyone else.

- ``fetch_nse_bhavcopy`` / ``fetch_nse_option_chain``: documented stubs.
  Scraping NSE's bhavcopy archive or option-chain JSON endpoints raises ToS
  and rate-limiting concerns that are out of scope for this bead; both are
  left as clearly-documented ``NotImplementedError`` stubs so callers get
  guidance instead of a silent/broken implementation.

None of the functions in this module are used by ``quantwave.datasets``;
``datasets.synthetic()`` / ``datasets.load_sample()`` remain fully
zero-network and have no dependency on anything here.
"""

from __future__ import annotations

from typing import Any

import polars as pl

from .datasets import IST, SCHEMA  # noqa: F401  (re-exported for convenience)


def fetch_yfinance(
    symbol: str,
    start: str | None = None,
    end: str | None = None,
    interval: str = "1d",
) -> pl.DataFrame:
    """Fetch OHLCV history for ``symbol`` via the ``yfinance`` package.

    Requires the optional ``yfinance`` package (NOT a core dependency of
    quantwave). Install it with ``pip install yfinance`` to use this
    function. Timestamps in the returned frame are converted to the
    canonical schema (tz-aware, Asia/Kolkata) via a naive UTC->IST
    conversion; verify this matches your intended market calendar before
    relying on it for anything beyond exploratory analysis.

    Raises
    ------
    ImportError
        If ``yfinance`` is not installed, with an actionable install hint.
    """
    try:
        import yfinance as yf  # type: ignore  # lazy import — optional dependency
    except ImportError as exc:  # pragma: no cover - exercised via mocked/missing-dep test
        raise ImportError(
            "quantwave.data.fetch_yfinance requires the optional 'yfinance' "
            "package, which is not installed. Install it with:\n"
            "    pip install yfinance\n"
            "(yfinance is intentionally NOT a core quantwave dependency.)"
        ) from exc

    raw = yf.download(symbol, start=start, end=end, interval=interval, progress=False)
    if raw is None or len(raw) == 0:
        return pl.DataFrame(schema=dict(SCHEMA))

    raw = raw.reset_index()
    ts_col = "Datetime" if "Datetime" in raw.columns else "Date"
    df = pl.from_pandas(raw)
    df = df.rename(
        {
            ts_col: "ts",
            "Open": "open",
            "High": "high",
            "Low": "low",
            "Close": "close",
            "Volume": "volume",
        }
    )
    df = df.with_columns(
        pl.col("ts").cast(pl.Datetime(time_unit="us")).dt.replace_time_zone("UTC").dt.convert_time_zone(IST),
        pl.col("volume").cast(pl.Float64),
        pl.lit(symbol).alias("symbol"),
    )
    return df.select(["ts", "open", "high", "low", "close", "volume", "symbol"]).sort("ts")


def fetch_nse_bhavcopy(date: str, **_: Any) -> pl.DataFrame:
    """Fetch the NSE end-of-day bhavcopy for ``date`` (YYYY-MM-DD).

    NOT IMPLEMENTED. NSE's bhavcopy archive is published under NSE's own
    terms of service; automated scraping/downloading may be restricted and
    the endpoint/format has changed multiple times historically. This is a
    documented stub rather than a scraping implementation: if you need this
    data, download it manually from nseindia.com (or an authorized data
    vendor) and load it with ``polars.read_csv`` and align it to
    ``quantwave.datasets.SCHEMA`` yourself.

    Raises
    ------
    NotImplementedError
        Always. This function exists to document the intended surface.
    """
    raise NotImplementedError(
        "fetch_nse_bhavcopy is a documented stub. NSE bhavcopy scraping is "
        "out of scope for quantwave core (see NSE's terms of service). "
        "Download the bhavcopy manually and load it with polars.read_csv(), "
        "then align columns to quantwave.datasets.SCHEMA."
    )


def fetch_nse_option_chain(underlying: str, **_: Any) -> pl.DataFrame:
    """Fetch the live NSE option chain for ``underlying`` (e.g. "NIFTY").

    NOT IMPLEMENTED. NSE's option-chain data is served from an
    authenticated/rate-limited JSON endpoint intended for nseindia.com's own
    website; scraping it programmatically is subject to NSE's terms of
    service and is out of scope for this bead. This is a documented stub:
    if you need option-chain data, use an authorized broker/vendor API (for
    example, this repo's ``upstox`` integration) instead.

    Raises
    ------
    NotImplementedError
        Always. This function exists to document the intended surface.
    """
    raise NotImplementedError(
        "fetch_nse_option_chain is a documented stub. Scraping NSE's "
        "option-chain endpoint is out of scope for quantwave core (see "
        "NSE's terms of service). Use an authorized broker/vendor API "
        "(e.g. Upstox) for live option-chain data instead."
    )
