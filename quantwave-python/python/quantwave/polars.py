"""Polars expression helpers for Options India analytics."""

from __future__ import annotations

from pathlib import Path
from typing import Union

import polars as pl
from polars.plugins import register_plugin_function

# Whole-chain reductions (max pain, OI zones, GEX flip, ATM straddle) reuse the
# exact Rust core math via a single per-batch FFI call. The per-strike and
# lookup analytics are expressed as native Polars expressions (no FFI, no
# per-row Python loops) — see the individual methods below.
from ._quantwave import (
    max_pain as core_max_pain,
    oi_zones as core_oi_zones,
    gex_flip_strike as core_gex_flip_strike,
    atm_straddle as core_atm_straddle,
)

# NSE lot sizes mirror quantwave_core::options_india::india::nse_lot_size.
# Kept in lockstep by test_polars_chain / test_chain_analytics parity assertions.
_NSE_LOT_SIZES: dict[str, int] = {
    "NIFTY": 50,
    "BANKNIFTY": 15,
    "FINNIFTY": 40,
    "MIDCPNIFTY": 75,
    "SENSEX": 10,
}


# Type aliases for Polars Expr builders: a bare column reference vs. a value
# that may be a column, an Expr, or a scalar broadcast across the chain.
_Col = Union[str, pl.Expr]
_ColOrVal = Union[str, pl.Expr, float, int]


def _as_expr(arg: Union[str, pl.Expr, float, int]) -> Union[pl.Expr, float, int]:
    """Column name -> pl.col; Expr / scalar passed through for use in expressions."""
    if isinstance(arg, str):
        return pl.col(arg)
    return arg


def _plugin_path() -> Path:
    import quantwave_plugins

    return Path(quantwave_plugins.__file__).resolve().parent


def _options_plugin(
    args: list[pl.Expr],
    function_name: str,
    *,
    kwargs: dict | None = None,
) -> pl.Expr:
    return register_plugin_function(
        args=args,
        plugin_path=_plugin_path(),
        function_name=function_name,
        is_elementwise=True,
        kwargs=kwargs or {},
    )


class options:
    """Polars Expr builders for NSE options chain analytics."""

    @staticmethod
    def _handle_arg(
        exprs: list,
        arg: Union[str, pl.Expr, float, int],
        default_name: str,
        *,
        length_ref: str | None = None,
    ) -> str:
        if isinstance(arg, str):
            exprs.append(pl.col(arg))
            return arg
        if isinstance(arg, pl.Expr):
            # Broadcast a length-1 literal (e.g. pl.lit(0.18)) to the reference
            # column length; a full-length column expression is unchanged by
            # `col*0 + expr`, so this is safe for both.
            if length_ref is not None:
                exprs.append((pl.col(length_ref) * 0 + arg).alias(default_name))
            else:
                exprs.append(arg.alias(default_name))
            return default_name
        if length_ref is not None:
            exprs.append((pl.col(length_ref) * 0 + arg).alias(default_name))
        else:
            exprs.append(pl.lit(arg).alias(default_name))
        return default_name

    @staticmethod
    def _greeks_inputs(
        iv_col_or_val,
        strike_col: str,
        r_col_or_val,
        t_col_or_val,
    ) -> list[pl.Expr]:
        """strike, sigma, r, t column order for native BS plugins."""
        exprs: list[pl.Expr] = [pl.col(strike_col)]
        options._handle_arg(exprs, iv_col_or_val, "_iv_val", length_ref=strike_col)
        options._handle_arg(exprs, r_col_or_val, "_r_val", length_ref=strike_col)
        options._handle_arg(exprs, t_col_or_val, "_t_val", length_ref=strike_col)
        return exprs

    @staticmethod
    def implied_vol(
        price_col: str,
        spot: float,
        strike_col: str,
        r_col_or_val: _ColOrVal,
        t_col_or_val: _ColOrVal,
        is_call: bool = True,
    ) -> pl.Expr:
        """Implied Black volatility from market price (annualized decimal).

        Args:
            price_col: Column with option LTP (same units as spot).
            spot: Underlying spot price.
            strike_col: Strike column.
            r_col_or_val: Risk-free rate (annualized decimal, e.g. 0.065), as
                a column name, Expr, or float.
            t_col_or_val: Time to expiry in years, as a column name, Expr, or float.
            is_call: True for call, False for put.

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"k": [100.0], "px": [5.0], "t": [0.25]})
            >>> df.select(options.implied_vol("px", 100.0, "k", 0.05, "t"))  # doctest: +SKIP
        """
        exprs: list[pl.Expr] = [pl.col(price_col), pl.col(strike_col)]
        options._handle_arg(exprs, r_col_or_val, "_r_val", length_ref=strike_col)
        options._handle_arg(exprs, t_col_or_val, "_t_val", length_ref=strike_col)
        return _options_plugin(
            exprs,
            "options_implied_vol",
            kwargs={"spot": spot, "is_call": is_call},
        )

    @staticmethod
    def bs_call_price(
        spot: float,
        strike_col: str,
        r_col_or_val: _ColOrVal,
        t_col_or_val: _ColOrVal,
        sigma_col_or_val: _ColOrVal,
    ) -> pl.Expr:
        """Black–Scholes call price (vectorized native plugin)."""
        exprs: list[pl.Expr] = [pl.col(strike_col)]
        options._handle_arg(exprs, r_col_or_val, "_r_val", length_ref=strike_col)
        options._handle_arg(exprs, t_col_or_val, "_t_val", length_ref=strike_col)
        options._handle_arg(exprs, sigma_col_or_val, "_s_val", length_ref=strike_col)
        return _options_plugin(exprs, "options_bs_call_price", kwargs={"spot": spot})

    @staticmethod
    def bs_put_price(
        spot: float,
        strike_col: str,
        r_col_or_val: _ColOrVal,
        t_col_or_val: _ColOrVal,
        sigma_col_or_val: _ColOrVal,
    ) -> pl.Expr:
        """Black–Scholes put price (vectorized native plugin)."""
        exprs: list[pl.Expr] = [pl.col(strike_col)]
        options._handle_arg(exprs, r_col_or_val, "_r_val", length_ref=strike_col)
        options._handle_arg(exprs, t_col_or_val, "_t_val", length_ref=strike_col)
        options._handle_arg(exprs, sigma_col_or_val, "_s_val", length_ref=strike_col)
        return _options_plugin(exprs, "options_bs_put_price", kwargs={"spot": spot})

    @staticmethod
    def bs_delta(
        iv_col_or_val: _ColOrVal,
        spot: float,
        strike_col: str,
        r_col_or_val: _ColOrVal,
        t_col_or_val: _ColOrVal,
        is_call: bool = True,
    ) -> pl.Expr:
        """Black–Scholes delta (vectorized native plugin)."""
        return _options_plugin(
            options._greeks_inputs(iv_col_or_val, strike_col, r_col_or_val, t_col_or_val),
            "options_bs_delta",
            kwargs={"spot": spot, "is_call": is_call},
        )

    @staticmethod
    def bs_gamma(
        iv_col_or_val: _ColOrVal,
        spot: float,
        strike_col: str,
        r_col_or_val: _ColOrVal,
        t_col_or_val: _ColOrVal,
    ) -> pl.Expr:
        """Black–Scholes gamma (vectorized native plugin)."""
        return _options_plugin(
            options._greeks_inputs(iv_col_or_val, strike_col, r_col_or_val, t_col_or_val),
            "options_bs_gamma",
            kwargs={"spot": spot},
        )

    @staticmethod
    def bs_vega(
        iv_col_or_val: _ColOrVal,
        spot: float,
        strike_col: str,
        r_col_or_val: _ColOrVal,
        t_col_or_val: _ColOrVal,
    ) -> pl.Expr:
        """Black–Scholes vega (vectorized native plugin)."""
        return _options_plugin(
            options._greeks_inputs(iv_col_or_val, strike_col, r_col_or_val, t_col_or_val),
            "options_bs_vega",
            kwargs={"spot": spot},
        )

    @staticmethod
    def bs_theta(
        iv_col_or_val: _ColOrVal,
        spot: float,
        strike_col: str,
        r_col_or_val: _ColOrVal,
        t_col_or_val: _ColOrVal,
        is_call: bool = True,
    ) -> pl.Expr:
        """Black–Scholes theta per calendar day (vectorized native plugin)."""
        return _options_plugin(
            options._greeks_inputs(iv_col_or_val, strike_col, r_col_or_val, t_col_or_val),
            "options_bs_theta",
            kwargs={"spot": spot, "is_call": is_call},
        )

    @staticmethod
    def bs_rho(
        iv_col_or_val: _ColOrVal,
        spot: float,
        strike_col: str,
        r_col_or_val: _ColOrVal,
        t_col_or_val: _ColOrVal,
        is_call: bool = True,
    ) -> pl.Expr:
        """Black–Scholes rho (vectorized native plugin)."""
        return _options_plugin(
            options._greeks_inputs(iv_col_or_val, strike_col, r_col_or_val, t_col_or_val),
            "options_bs_rho",
            kwargs={"spot": spot, "is_call": is_call},
        )

    @staticmethod
    def max_pain(
        strikes_col: _Col,
        ce_oi_col: _Col,
        pe_oi_col: _Col,
        lot_size_col_or_val: _ColOrVal,
    ) -> pl.Expr:
        """Max-pain strike for one option chain (whole-chain reduction).

        Returns the strike at which total buyer loss is minimised. This is an
        O(n^2) reduction over the whole chain, computed in a single native Rust
        call (``quantwave_core::options_india::chain_analytics::max_pain``) — no
        per-row Python loop. Apply per expiry snapshot (one chain per call, or
        inside ``group_by(expiry)``).

        Args:
            strikes_col: Strike prices, as a column name or Expr.
            ce_oi_col: Call open interest (integer contracts), as a column name or Expr.
            pe_oi_col: Put open interest (integer contracts), as a column name or Expr.
            lot_size_col_or_val: Contract lot size (scalar per chain), as an
                int, float, column name, or Expr.

        Returns:
            pl.Expr: Length-1 Float64 (the max-pain strike).

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"k": [24800.0, 25000.0, 25200.0],
            ...                    "ce": [10000, 5000, 2000], "pe": [2000, 5000, 10000]})
            >>> df.select(options.max_pain("k", "ce", "pe", 50).alias("x"))["x"][0]
            25000.0
        """
        exprs: list[pl.Expr] = []
        s_name = options._handle_arg(exprs, strikes_col, "_s_col")
        c_name = options._handle_arg(exprs, ce_oi_col, "_c_col")
        p_name = options._handle_arg(exprs, pe_oi_col, "_p_col")
        l_name = options._handle_arg(exprs, lot_size_col_or_val, "_l_val")

        def _max_pain(batch: pl.Series) -> pl.Series:
            lot = int(batch.struct.field(l_name).to_list()[0])
            value = core_max_pain(
                batch.struct.field(s_name).to_list(),
                batch.struct.field(c_name).to_list(),
                batch.struct.field(p_name).to_list(),
                lot,
            )
            return pl.Series([value])

        return pl.struct(exprs).map_batches(_max_pain, return_dtype=pl.Float64)

    @staticmethod
    def strike_pcr(ce_oi_col: _Col, pe_oi_col: _Col) -> pl.Expr:
        """Per-strike Put-Call Ratio (PE_OI / CE_OI), native Polars expression.

        Matches ``chain_analytics::strike_pcr``: 0.0 where CE_OI is zero.

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"ce": [10000, 2000], "pe": [2000, 10000]})
            >>> df.select(options.strike_pcr("ce", "pe").alias("x"))["x"].to_list()
            [0.2, 5.0]
        """
        ce = _as_expr(ce_oi_col)
        pe = _as_expr(pe_oi_col)
        return pl.when(ce == 0).then(0.0).otherwise(pe / ce)

    @staticmethod
    def chain_pcr(ce_oi_col: _Col, pe_oi_col: _Col) -> pl.Expr:
        """Chain-level Put-Call Ratio: sum(PE_OI) / sum(CE_OI) (0.0 if no CE_OI).

        Native Polars aggregation; returns a length-1 Float64.

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"ce": [10000, 5000], "pe": [2000, 8000]})
            >>> df.select(options.chain_pcr("ce", "pe").alias("x"))["x"][0]
            0.6666666666666666
        """
        ce = _as_expr(ce_oi_col)
        pe = _as_expr(pe_oi_col)
        total_ce = ce.sum()
        return pl.when(total_ce == 0).then(0.0).otherwise(pe.sum() / total_ce)

    @staticmethod
    def synthetic_futures(strikes_col: _Col, ce_ltp_col: _Col, pe_ltp_col: _Col) -> pl.Expr:
        """Per-strike synthetic future (CE_LTP - PE_LTP + Strike), native expression.

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"k": [24800.0], "ce": [250.0], "pe": [10.0]})
            >>> df.select(options.synthetic_futures("k", "ce", "pe").alias("x"))["x"][0]
            25040.0
        """
        return _as_expr(ce_ltp_col) - _as_expr(pe_ltp_col) + _as_expr(strikes_col)

    @staticmethod
    def oi_zones(
        strikes_col: _Col,
        ce_oi_col: _Col,
        pe_oi_col: _Col,
        n_col_or_val: _ColOrVal,
    ) -> pl.Expr:
        """Top-N OI support/resistance strikes for one chain (whole-chain reduction).

        Resistance = strikes with highest CE_OI; support = highest PE_OI. Computed
        in a single native Rust call (``chain_analytics::oi_zones``); returns a
        length-1 struct of two Float64 lists. Apply per expiry snapshot.

        Returns:
            pl.Expr: Struct{resistance_strikes: list[f64], support_strikes: list[f64]}.

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"k": [24800.0, 25000.0, 25200.0],
            ...                    "ce": [10000, 5000, 2000], "pe": [2000, 5000, 10000]})
            >>> df.select(options.oi_zones("k", "ce", "pe", 1).alias("z"))["z"][0]["resistance_strikes"]
            [24800.0]
        """
        exprs: list[pl.Expr] = []
        s_name = options._handle_arg(exprs, strikes_col, "_s_col")
        c_name = options._handle_arg(exprs, ce_oi_col, "_c_col")
        p_name = options._handle_arg(exprs, pe_oi_col, "_p_col")
        n_name = options._handle_arg(exprs, n_col_or_val, "_n_val")

        def _oi_zones(batch: pl.Series) -> pl.Series:
            n = int(batch.struct.field(n_name).to_list()[0])
            res = core_oi_zones(
                batch.struct.field(s_name).to_list(),
                batch.struct.field(c_name).to_list(),
                batch.struct.field(p_name).to_list(),
                n,
            )
            return pl.Series([{
                "resistance_strikes": res.resistance_strikes,
                "support_strikes": res.support_strikes,
            }])

        return pl.struct(exprs).map_batches(
            _oi_zones,
            return_dtype=pl.Struct([
                pl.Field("resistance_strikes", pl.List(pl.Float64)),
                pl.Field("support_strikes", pl.List(pl.Float64)),
            ]),
        )

    @staticmethod
    def gex_per_strike(
        spot_col_or_val: _ColOrVal,
        strikes_col: _Col,
        ce_gamma_col: _Col,
        pe_gamma_col: _Col,
        ce_oi_col: _Col,
        pe_oi_col: _Col,
        lot_size_col_or_val: _ColOrVal,
    ) -> pl.Expr:
        """Per-strike Gamma Exposure (native Polars expression).

        CE GEX = CE_OI * CE_gamma * spot * lot * 0.01; PE GEX uses -0.01; the
        struct also carries net_gex = CE + PE. Mirrors ``chain_analytics::
        gex_per_strike`` exactly, with no per-row FFI.

        Returns:
            pl.Expr: Struct{ce_gex: f64, pe_gex: f64, net_gex: f64} (same length as input).

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"k": [25000.0], "cg": [0.0005], "pg": [0.0005],
            ...                    "ce": [4000], "pe": [5000]})
            >>> df.select(options.gex_per_strike(25000.0, "k", "cg", "pg", "ce", "pe", 50)
            ...          )["k"][0]["net_gex"]  # doctest: +SKIP
        """
        spot = _as_expr(spot_col_or_val)
        lot = _as_expr(lot_size_col_or_val)
        ce_gex = _as_expr(ce_oi_col) * _as_expr(ce_gamma_col) * spot * lot * 0.01
        pe_gex = _as_expr(pe_oi_col) * _as_expr(pe_gamma_col) * spot * lot * -0.01
        return pl.struct(
            ce_gex.alias("ce_gex"),
            pe_gex.alias("pe_gex"),
            (ce_gex + pe_gex).alias("net_gex"),
        )

    @staticmethod
    def gex_flip_strike(strikes_col: _Col, net_gex_col: _Col) -> pl.Expr:
        """Strike where cumulative Net GEX first changes sign (whole-chain reduction).

        Single native Rust call (``chain_analytics::gex_flip_strike``); returns a
        length-1 Float64 (null if no sign change). Apply per expiry snapshot.

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"k": [24800.0, 25000.0], "gex": [4.0, -0.5]})
            >>> df.select(options.gex_flip_strike("k", "gex").alias("x"))["x"][0]
            24800.0
        """
        exprs: list[pl.Expr] = []
        s_name = options._handle_arg(exprs, strikes_col, "_s_col")
        n_name = options._handle_arg(exprs, net_gex_col, "_n_col")

        def _flip(batch: pl.Series) -> pl.Series:
            return pl.Series([
                core_gex_flip_strike(
                    batch.struct.field(s_name).to_list(),
                    batch.struct.field(n_name).to_list(),
                )
            ])

        return pl.struct(exprs).map_batches(_flip, return_dtype=pl.Float64)

    @staticmethod
    def atm_straddle(
        spot_col_or_val: _ColOrVal,
        strikes_col: _Col,
        ce_ltp_col: _Col,
        pe_ltp_col: _Col,
    ) -> pl.Expr:
        """ATM straddle analytics for one chain (whole-chain reduction).

        Finds the strike closest to spot and returns its straddle premium and
        implied move. Single native Rust call (``chain_analytics::atm_straddle``);
        returns a length-1 struct. Apply per expiry snapshot.

        Returns:
            pl.Expr: Struct{atm_strike: f64, straddle_premium: f64, implied_move_pct: f64}.

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"k": [24800.0, 25000.0], "ce": [250.0, 100.0],
            ...                    "pe": [10.0, 60.0]})
            >>> df.select(options.atm_straddle(25000.0, "k", "ce", "pe").alias("s"))["s"][0]["atm_strike"]
            25000.0
        """
        exprs: list[pl.Expr] = []
        sp_name = options._handle_arg(exprs, spot_col_or_val, "_sp_val")
        st_name = options._handle_arg(exprs, strikes_col, "_st_col")
        cl_name = options._handle_arg(exprs, ce_ltp_col, "_cl_col")
        pl_name = options._handle_arg(exprs, pe_ltp_col, "_pl_col")

        def _straddle(batch: pl.Series) -> pl.Series:
            spot = batch.struct.field(sp_name).to_list()[0]
            res = core_atm_straddle(
                spot,
                batch.struct.field(st_name).to_list(),
                batch.struct.field(cl_name).to_list(),
                batch.struct.field(pl_name).to_list(),
            )
            return pl.Series([{
                "atm_strike": res.atm_strike,
                "straddle_premium": res.straddle_premium,
                "implied_move_pct": res.implied_move_pct,
            }])

        return pl.struct(exprs).map_batches(
            _straddle,
            return_dtype=pl.Struct([
                pl.Field("atm_strike", pl.Float64),
                pl.Field("straddle_premium", pl.Float64),
                pl.Field("implied_move_pct", pl.Float64),
            ]),
        )

    @staticmethod
    def moneyness(spot: float, strike_col: _Col) -> pl.Expr:
        """Per-strike moneyness (ITM/ATM/OTM) from a call perspective, native expression.

        ATM band is spot +/- 0.2% (mirrors ``options_india::india::moneyness``):
        strike < spot*0.998 -> ITM, strike > spot*1.002 -> OTM, else ATM.

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"k": [24800.0, 25000.0, 25200.0]})
            >>> df.select(options.moneyness(25000.0, "k").alias("x"))["x"].to_list()
            ['ITM', 'ATM', 'OTM']
        """
        strike = _as_expr(strike_col)
        return (
            pl.when(strike < spot * 0.998)
            .then(pl.lit("ITM"))
            .when(strike > spot * 1.002)
            .then(pl.lit("OTM"))
            .otherwise(pl.lit("ATM"))
        )

    @staticmethod
    def nse_lot_size(symbol_col: _Col) -> pl.Expr:
        """Map an NSE index symbol column to its lot size (native expression).

        Mirrors ``options_india::india::nse_lot_size``; unknown symbols map to
        null. Case-insensitive.

        Examples:
            >>> import polars as pl
            >>> from quantwave.polars import options
            >>> df = pl.DataFrame({"sym": ["NIFTY", "BANKNIFTY"]})
            >>> df.select(options.nse_lot_size("sym").alias("x"))["x"].to_list()
            [50, 15]
        """
        return (
            _as_expr(symbol_col)
            .str.to_uppercase()
            .replace_strict(_NSE_LOT_SIZES, default=None, return_dtype=pl.Int64)
        )