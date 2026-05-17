import polars as pl
from ._quantwave import (
    bs_call_price, bs_put_price, bs_delta, bs_gamma, bs_theta, bs_vega, bs_rho,
    implied_vol as core_implied_vol
)

class options:
    @staticmethod
    def implied_vol(price_col, spot, strike_col, r, t_col, is_call=True):
        return pl.struct([price_col, strike_col, t_col]).map_batches(
            lambda s: pl.Series([
                core_implied_vol(row[price_col], spot, row[strike_col], r, row[t_col], is_call)
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_delta(iv_col, spot, strike_col, r, t_col, is_call=True):
        return pl.struct([iv_col, strike_col, t_col]).map_batches(
            lambda s: pl.Series([
                bs_delta(spot, row[strike_col], r, row[t_col], row[iv_col], is_call)
                if row[iv_col] is not None else None
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_gamma(iv_col, spot, strike_col, r, t_col):
        return pl.struct([iv_col, strike_col, t_col]).map_batches(
            lambda s: pl.Series([
                bs_gamma(spot, row[strike_col], r, row[t_col], row[iv_col])
                if row[iv_col] is not None else None
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_vega(iv_col, spot, strike_col, r, t_col):
        return pl.struct([iv_col, strike_col, t_col]).map_batches(
            lambda s: pl.Series([
                bs_vega(spot, row[strike_col], r, row[t_col], row[iv_col])
                if row[iv_col] is not None else None
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_theta(iv_col, spot, strike_col, r, t_col, is_call=True):
        return pl.struct([iv_col, strike_col, t_col]).map_batches(
            lambda s: pl.Series([
                bs_theta(spot, row[strike_col], r, row[t_col], row[iv_col], is_call)
                if row[iv_col] is not None else None
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )
