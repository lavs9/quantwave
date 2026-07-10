"""Polars expression helpers for Options India analytics."""

from __future__ import annotations

from pathlib import Path
from typing import Union

import polars as pl
from polars.plugins import register_plugin_function

from ._quantwave import (
    max_pain as core_max_pain,
    strike_pcr as core_strike_pcr,
    chain_pcr as core_chain_pcr,
    oi_zones as core_oi_zones,
    gex_per_strike as core_gex_per_strike,
    gex_flip_strike as core_gex_flip_strike,
    atm_straddle as core_atm_straddle,
    synthetic_futures as core_synthetic_futures,
)


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
        r_col_or_val,
        t_col_or_val,
        is_call: bool = True,
    ) -> pl.Expr:
        """Implied Black volatility from market price (annualized decimal).

        Parameters
        ----------
        price_col : str
            Column with option LTP (same units as spot).
        spot : float
            Underlying spot price.
        strike_col : str
            Strike column.
        r_col_or_val : str, Expr, or float
            Risk-free rate (annualized decimal, e.g. 0.065).
        t_col_or_val : str, Expr, or float
            Time to expiry in years.
        is_call : bool
            True for call, False for put.

        Examples
        --------
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
        r_col_or_val,
        t_col_or_val,
        sigma_col_or_val,
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
        r_col_or_val,
        t_col_or_val,
        sigma_col_or_val,
    ) -> pl.Expr:
        """Black–Scholes put price (vectorized native plugin)."""
        exprs: list[pl.Expr] = [pl.col(strike_col)]
        options._handle_arg(exprs, r_col_or_val, "_r_val", length_ref=strike_col)
        options._handle_arg(exprs, t_col_or_val, "_t_val", length_ref=strike_col)
        options._handle_arg(exprs, sigma_col_or_val, "_s_val", length_ref=strike_col)
        return _options_plugin(exprs, "options_bs_put_price", kwargs={"spot": spot})

    @staticmethod
    def bs_delta(
        iv_col_or_val,
        spot: float,
        strike_col: str,
        r_col_or_val,
        t_col_or_val,
        is_call: bool = True,
    ) -> pl.Expr:
        """Black–Scholes delta (vectorized native plugin)."""
        return _options_plugin(
            options._greeks_inputs(iv_col_or_val, strike_col, r_col_or_val, t_col_or_val),
            "options_bs_delta",
            kwargs={"spot": spot, "is_call": is_call},
        )

    @staticmethod
    def bs_gamma(iv_col_or_val, spot: float, strike_col: str, r_col_or_val, t_col_or_val) -> pl.Expr:
        """Black–Scholes gamma (vectorized native plugin)."""
        return _options_plugin(
            options._greeks_inputs(iv_col_or_val, strike_col, r_col_or_val, t_col_or_val),
            "options_bs_gamma",
            kwargs={"spot": spot},
        )

    @staticmethod
    def bs_vega(iv_col_or_val, spot: float, strike_col: str, r_col_or_val, t_col_or_val) -> pl.Expr:
        """Black–Scholes vega (vectorized native plugin)."""
        return _options_plugin(
            options._greeks_inputs(iv_col_or_val, strike_col, r_col_or_val, t_col_or_val),
            "options_bs_vega",
            kwargs={"spot": spot},
        )

    @staticmethod
    def bs_theta(
        iv_col_or_val,
        spot: float,
        strike_col: str,
        r_col_or_val,
        t_col_or_val,
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
        iv_col_or_val,
        spot: float,
        strike_col: str,
        r_col_or_val,
        t_col_or_val,
        is_call: bool = True,
    ) -> pl.Expr:
        """Black–Scholes rho (vectorized native plugin)."""
        return _options_plugin(
            options._greeks_inputs(iv_col_or_val, strike_col, r_col_or_val, t_col_or_val),
            "options_bs_rho",
            kwargs={"spot": spot, "is_call": is_call},
        )

    @staticmethod
    def max_pain(strikes_col, ce_oi_col, pe_oi_col, lot_size_col_or_val):
        exprs = []
        s_name = options._handle_arg(exprs, strikes_col, "_s_col")
        c_name = options._handle_arg(exprs, ce_oi_col, "_c_col")
        p_name = options._handle_arg(exprs, pe_oi_col, "_p_col")
        l_name = options._handle_arg(exprs, lot_size_col_or_val, "_l_val")

        return pl.struct(exprs).map_batches(
            lambda s: pl.Series([
                core_max_pain(
                    s.struct.field(s_name).to_list(),
                    s.struct.field(c_name).to_list(),
                    s.struct.field(p_name).to_list(),
                    row[l_name],
                )
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64,
        )

    @staticmethod
    def strike_pcr(ce_oi_col, pe_oi_col):
        return pl.struct([ce_oi_col, pe_oi_col]).map_batches(
            lambda s: pl.Series(
                core_strike_pcr(
                    s.struct.field(ce_oi_col).to_list(),
                    s.struct.field(pe_oi_col).to_list(),
                )
            ),
            return_dtype=pl.Float64,
        )

    @staticmethod
    def chain_pcr(ce_oi_col, pe_oi_col):
        return pl.struct([ce_oi_col, pe_oi_col]).map_batches(
            lambda s: pl.Series([
                core_chain_pcr(
                    s.struct.field(ce_oi_col).to_list(),
                    s.struct.field(pe_oi_col).to_list(),
                )
            ]),
            return_dtype=pl.Float64,
        )

    @staticmethod
    def synthetic_futures(strikes_col, ce_ltp_col, pe_ltp_col):
        return pl.struct([strikes_col, ce_ltp_col, pe_ltp_col]).map_batches(
            lambda s: pl.Series(
                core_synthetic_futures(
                    s.struct.field(strikes_col).to_list(),
                    s.struct.field(ce_ltp_col).to_list(),
                    s.struct.field(pe_ltp_col).to_list(),
                )
            ),
            return_dtype=pl.Float64,
        )

    @staticmethod
    def oi_zones(strikes_col, ce_oi_col, pe_oi_col, n_col_or_val):
        exprs = []
        s_name = options._handle_arg(exprs, strikes_col, "_s_col")
        c_name = options._handle_arg(exprs, ce_oi_col, "_c_col")
        p_name = options._handle_arg(exprs, pe_oi_col, "_p_col")
        n_name = options._handle_arg(exprs, n_col_or_val, "_n_val")

        def _get_zones(s):
            results = []
            for row in s.to_list():
                res = core_oi_zones(
                    s.struct.field(s_name).to_list(),
                    s.struct.field(c_name).to_list(),
                    s.struct.field(p_name).to_list(),
                    row[n_name],
                )
                results.append({
                    "resistance_strikes": res.resistance_strikes,
                    "support_strikes": res.support_strikes,
                })
            return pl.Series(results)

        return pl.struct(exprs).map_batches(
            _get_zones,
            return_dtype=pl.Struct([
                pl.Field("resistance_strikes", pl.List(pl.Float64)),
                pl.Field("support_strikes", pl.List(pl.Float64)),
            ]),
        )

    @staticmethod
    def gex_per_strike(
        spot_col_or_val,
        strikes_col,
        ce_gamma_col,
        pe_gamma_col,
        ce_oi_col,
        pe_oi_col,
        lot_size_col_or_val,
    ):
        exprs = []
        sp_name = options._handle_arg(exprs, spot_col_or_val, "_sp_val")
        st_name = options._handle_arg(exprs, strikes_col, "_st_col")
        cg_name = options._handle_arg(exprs, ce_gamma_col, "_cg_col")
        pg_name = options._handle_arg(exprs, pe_gamma_col, "_pg_col")
        co_name = options._handle_arg(exprs, ce_oi_col, "_co_col")
        po_name = options._handle_arg(exprs, pe_oi_col, "_po_col")
        lt_name = options._handle_arg(exprs, lot_size_col_or_val, "_lt_val")

        def _get_gex(s):
            row = s.to_list()[0]
            res = core_gex_per_strike(
                row[sp_name],
                s.struct.field(st_name).to_list(),
                s.struct.field(cg_name).to_list(),
                s.struct.field(pg_name).to_list(),
                s.struct.field(co_name).to_list(),
                s.struct.field(po_name).to_list(),
                row[lt_name],
            )
            return pl.Series([
                {"ce_gex": r.ce_gex, "pe_gex": r.pe_gex, "net_gex": r.net_gex} for r in res
            ])

        return pl.struct(exprs).map_batches(
            _get_gex,
            return_dtype=pl.Struct([
                pl.Field("ce_gex", pl.Float64),
                pl.Field("pe_gex", pl.Float64),
                pl.Field("net_gex", pl.Float64),
            ]),
        )

    @staticmethod
    def gex_flip_strike(strikes_col, net_gex_col):
        exprs = []
        s_name = options._handle_arg(exprs, strikes_col, "_s_col")
        n_name = options._handle_arg(exprs, net_gex_col, "_n_col")

        return pl.struct(exprs).map_batches(
            lambda s: pl.Series([
                core_gex_flip_strike(
                    s.struct.field(s_name).to_list(),
                    s.struct.field(n_name).to_list(),
                )
            ]),
            return_dtype=pl.Float64,
        )

    @staticmethod
    def atm_straddle(spot_col_or_val, strikes_col, ce_ltp_col, pe_ltp_col):
        exprs = []
        sp_name = options._handle_arg(exprs, spot_col_or_val, "_sp_val")
        st_name = options._handle_arg(exprs, strikes_col, "_st_col")
        cl_name = options._handle_arg(exprs, ce_ltp_col, "_cl_col")
        pl_name = options._handle_arg(exprs, pe_ltp_col, "_pl_col")

        def _get_straddle(s):
            row = s.to_list()[0]
            res = core_atm_straddle(
                row[sp_name],
                s.struct.field(st_name).to_list(),
                s.struct.field(cl_name).to_list(),
                s.struct.field(pl_name).to_list(),
            )
            return pl.Series([{
                "atm_strike": res.atm_strike,
                "straddle_premium": res.straddle_premium,
                "implied_move_pct": res.implied_move_pct,
            }])

        return pl.struct(exprs).map_batches(
            _get_straddle,
            return_dtype=pl.Struct([
                pl.Field("atm_strike", pl.Float64),
                pl.Field("straddle_premium", pl.Float64),
                pl.Field("implied_move_pct", pl.Float64),
            ]),
        )

    @staticmethod
    def moneyness(spot, strike_col):
        from ._quantwave import moneyness as core_moneyness

        return pl.col(strike_col).map_batches(
            lambda s: pl.Series([core_moneyness(spot, k) for k in s.to_list()]),
            return_dtype=pl.String,
        )

    @staticmethod
    def nse_lot_size(symbol_col):
        from ._quantwave import nse_lot_size as core_nse_lot_size

        return pl.col(symbol_col).map_batches(
            lambda s: pl.Series([core_nse_lot_size(sym) for sym in s.to_list()]),
            return_dtype=pl.Int64,
        )