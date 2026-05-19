import polars as pl
from ._quantwave import (
    bs_call_price as core_bs_call_price,
    bs_put_price as core_bs_put_price,
    bs_delta, bs_gamma, bs_theta, bs_vega, bs_rho,
    implied_vol as core_implied_vol,
    max_pain as core_max_pain,
    strike_pcr as core_strike_pcr,
    chain_pcr as core_chain_pcr,
    oi_zones as core_oi_zones,
    gex_per_strike as core_gex_per_strike,
    gex_flip_strike as core_gex_flip_strike,
    atm_straddle as core_atm_straddle,
    synthetic_futures as core_synthetic_futures,
)

class options:
    @staticmethod
    def _handle_arg(exprs, arg, default_name):
        if isinstance(arg, str):
            exprs.append(pl.col(arg))
            return arg
        elif isinstance(arg, pl.Expr):
            exprs.append(arg.alias(default_name))
            return default_name
        else:
            exprs.append(pl.lit(arg).alias(default_name))
            return default_name

    @staticmethod
    def implied_vol(price_col, spot, strike_col, r_col_or_val, t_col_or_val, is_call=True):
        exprs = [pl.col(price_col), pl.col(strike_col)]
        r_name = options._handle_arg(exprs, r_col_or_val, "_r_val")
        t_name = options._handle_arg(exprs, t_col_or_val, "_t_val")

        return pl.struct(exprs).map_batches(
            lambda s: pl.Series([
                core_implied_vol(
                    row[price_col], 
                    spot, 
                    row[strike_col], 
                    row[r_name],
                    row[t_name],
                    is_call
                )
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_call_price(spot, strike_col, r_col_or_val, t_col_or_val, sigma_col_or_val):
        exprs = [pl.col(strike_col)]
        r_name = options._handle_arg(exprs, r_col_or_val, "_r_val")
        t_name = options._handle_arg(exprs, t_col_or_val, "_t_val")
        s_name = options._handle_arg(exprs, sigma_col_or_val, "_s_val")
            
        return pl.struct(exprs).map_batches(
            lambda s: pl.Series([
                core_bs_call_price(
                    spot, 
                    row[strike_col], 
                    row[r_name],
                    row[t_name],
                    row[s_name]
                )
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_put_price(spot, strike_col, r_col_or_val, t_col_or_val, sigma_col_or_val):
        exprs = [pl.col(strike_col)]
        r_name = options._handle_arg(exprs, r_col_or_val, "_r_val")
        t_name = options._handle_arg(exprs, t_col_or_val, "_t_val")
        s_name = options._handle_arg(exprs, sigma_col_or_val, "_s_val")
            
        return pl.struct(exprs).map_batches(
            lambda s: pl.Series([
                core_bs_put_price(
                    spot, 
                    row[strike_col], 
                    row[r_name],
                    row[t_name],
                    row[s_name]
                )
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_delta(iv_col_or_val, spot, strike_col, r_col_or_val, t_col_or_val, is_call=True):
        exprs = [pl.col(strike_col)]
        iv_name = options._handle_arg(exprs, iv_col_or_val, "_iv_val")
        r_name = options._handle_arg(exprs, r_col_or_val, "_r_val")
        t_name = options._handle_arg(exprs, t_col_or_val, "_t_val")
            
        return pl.struct(exprs).map_batches(
            lambda s: pl.Series([
                bs_delta(
                    spot, 
                    row[strike_col], 
                    row[r_name],
                    row[t_name],
                    row[iv_name],
                    is_call
                )
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_gamma(iv_col_or_val, spot, strike_col, r_col_or_val, t_col_or_val):
        exprs = [pl.col(strike_col)]
        iv_name = options._handle_arg(exprs, iv_col_or_val, "_iv_val")
        r_name = options._handle_arg(exprs, r_col_or_val, "_r_val")
        t_name = options._handle_arg(exprs, t_col_or_val, "_t_val")
            
        return pl.struct(exprs).map_batches(
            lambda s: pl.Series([
                bs_gamma(
                    spot, 
                    row[strike_col], 
                    row[r_name],
                    row[t_name],
                    row[iv_name]
                )
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_vega(iv_col_or_val, spot, strike_col, r_col_or_val, t_col_or_val):
        exprs = [pl.col(strike_col)]
        iv_name = options._handle_arg(exprs, iv_col_or_val, "_iv_val")
        r_name = options._handle_arg(exprs, r_col_or_val, "_r_val")
        t_name = options._handle_arg(exprs, t_col_or_val, "_t_val")
            
        return pl.struct(exprs).map_batches(
            lambda s: pl.Series([
                bs_vega(
                    spot, 
                    row[strike_col], 
                    row[r_name],
                    row[t_name],
                    row[iv_name]
                )
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_theta(iv_col_or_val, spot, strike_col, r_col_or_val, t_col_or_val, is_call=True):
        exprs = [pl.col(strike_col)]
        iv_name = options._handle_arg(exprs, iv_col_or_val, "_iv_val")
        r_name = options._handle_arg(exprs, r_col_or_val, "_r_val")
        t_name = options._handle_arg(exprs, t_col_or_val, "_t_val")
            
        return pl.struct(exprs).map_batches(
            lambda s: pl.Series([
                bs_theta(
                    spot, 
                    row[strike_col], 
                    row[r_name],
                    row[t_name],
                    row[iv_name],
                    is_call
                )
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def bs_rho(iv_col_or_val, spot, strike_col, r_col_or_val, t_col_or_val, is_call=True):
        exprs = [pl.col(strike_col)]
        iv_name = options._handle_arg(exprs, iv_col_or_val, "_iv_val")
        r_name = options._handle_arg(exprs, r_col_or_val, "_r_val")
        t_name = options._handle_arg(exprs, t_col_or_val, "_t_val")
            
        return pl.struct(exprs).map_batches(
            lambda s: pl.Series([
                bs_rho(
                    spot, 
                    row[strike_col], 
                    row[r_name],
                    row[t_name],
                    row[iv_name],
                    is_call
                )
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
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
                    row[l_name]
                )
                for row in s.to_list()
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def strike_pcr(ce_oi_col, pe_oi_col):
        return pl.struct([ce_oi_col, pe_oi_col]).map_batches(
            lambda s: pl.Series(
                core_strike_pcr(
                    s.struct.field(ce_oi_col).to_list(),
                    s.struct.field(pe_oi_col).to_list()
                )
            ),
            return_dtype=pl.Float64
        )

    @staticmethod
    def chain_pcr(ce_oi_col, pe_oi_col):
        return pl.struct([ce_oi_col, pe_oi_col]).map_batches(
            lambda s: pl.Series([
                core_chain_pcr(
                    s.struct.field(ce_oi_col).to_list(),
                    s.struct.field(pe_oi_col).to_list()
                )
            ]),
            return_dtype=pl.Float64
        )

    @staticmethod
    def synthetic_futures(strikes_col, ce_ltp_col, pe_ltp_col):
        return pl.struct([strikes_col, ce_ltp_col, pe_ltp_col]).map_batches(
            lambda s: pl.Series(
                core_synthetic_futures(
                    s.struct.field(strikes_col).to_list(),
                    s.struct.field(ce_ltp_col).to_list(),
                    s.struct.field(pe_ltp_col).to_list()
                )
            ),
            return_dtype=pl.Float64
        )

    @staticmethod
    def oi_zones(strikes_col, ce_oi_col, pe_oi_col, n_col_or_val):
        exprs = []
        s_name = options._handle_arg(exprs, strikes_col, "_s_col")
        c_name = options._handle_arg(exprs, ce_oi_col, "_c_col")
        p_name = options._handle_arg(exprs, pe_oi_col, "_p_col")
        n_name = options._handle_arg(exprs, n_col_or_val, "_n_val")

        def _get_zones(s):
            # Batch size here is N rows. For chain analytics we often want 1 row = 1 chain
            # but if it's multiple rows, we process row by row if parameters vary
            results = []
            for row in s.to_list():
                res = core_oi_zones(
                    s.struct.field(s_name).to_list(),
                    s.struct.field(c_name).to_list(),
                    s.struct.field(p_name).to_list(),
                    row[n_name]
                )
                results.append({"resistance_strikes": res.resistance_strikes, "support_strikes": res.support_strikes})
            return pl.Series(results)
            
        return pl.struct(exprs).map_batches(
            _get_zones,
            return_dtype=pl.Struct([pl.Field("resistance_strikes", pl.List(pl.Float64)), pl.Field("support_strikes", pl.List(pl.Float64))])
        )

    @staticmethod
    def gex_per_strike(spot_col_or_val, strikes_col, ce_gamma_col, pe_gamma_col, ce_oi_col, pe_oi_col, lot_size_col_or_val):
        exprs = []
        sp_name = options._handle_arg(exprs, spot_col_or_val, "_sp_val")
        st_name = options._handle_arg(exprs, strikes_col, "_st_col")
        cg_name = options._handle_arg(exprs, ce_gamma_col, "_cg_col")
        pg_name = options._handle_arg(exprs, pe_gamma_col, "_pg_col")
        co_name = options._handle_arg(exprs, ce_oi_col, "_co_col")
        po_name = options._handle_arg(exprs, pe_oi_col, "_po_col")
        lt_name = options._handle_arg(exprs, lot_size_col_or_val, "_lt_val")

        def _get_gex(s):
            # We assume all rows in this batch belong to the SAME chain for these calculations
            # or we do it row by row which is safer but slower.
            # GEX per strike is inherently a whole-chain operation.
            row = s.to_list()[0]
            res = core_gex_per_strike(
                row[sp_name],
                s.struct.field(st_name).to_list(),
                s.struct.field(cg_name).to_list(),
                s.struct.field(pg_name).to_list(),
                s.struct.field(co_name).to_list(),
                s.struct.field(po_name).to_list(),
                row[lt_name]
            )
            return pl.Series([{"ce_gex": r.ce_gex, "pe_gex": r.pe_gex, "net_gex": r.net_gex} for r in res])

        return pl.struct(exprs).map_batches(
            _get_gex,
            return_dtype=pl.Struct([pl.Field("ce_gex", pl.Float64), pl.Field("pe_gex", pl.Float64), pl.Field("net_gex", pl.Float64)])
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
                    s.struct.field(n_name).to_list()
                )
            ]),
            return_dtype=pl.Float64
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
                s.struct.field(pl_name).to_list()
            )
            return pl.Series([{"atm_strike": res.atm_strike, "straddle_premium": res.straddle_premium, "implied_move_pct": res.implied_move_pct}])

        return pl.struct(exprs).map_batches(
            _get_straddle,
            return_dtype=pl.Struct([pl.Field("atm_strike", pl.Float64), pl.Field("straddle_premium", pl.Float64), pl.Field("implied_move_pct", pl.Float64)])
        )

    @staticmethod
    def moneyness(spot, strike_col):
        from ._quantwave import moneyness as core_moneyness
        return pl.col(strike_col).map_batches(
            lambda s: pl.Series([core_moneyness(spot, k) for k in s.to_list()]),
            return_dtype=pl.String
        )

    @staticmethod
    def nse_lot_size(symbol_col):
        from ._quantwave import nse_lot_size as core_nse_lot_size
        return pl.col(symbol_col).map_batches(
            lambda s: pl.Series([core_nse_lot_size(sym) for sym in s.to_list()]),
            return_dtype=pl.Int64
        )
