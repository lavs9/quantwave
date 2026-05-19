import polars as pl
import pytest
from quantwave.polars import options as opt_expr

def test_polars_chain_analytics():
    strikes = [24800.0, 25000.0, 25200.0]
    ce_oi = [10000, 5000, 2000]
    pe_oi = [2000, 5000, 10000]
    ce_ltp = [250.0, 100.0, 20.0]
    pe_ltp = [10.0, 60.0, 180.0]
    
    df = pl.DataFrame({
        "strike": strikes,
        "ce_oi": ce_oi,
        "pe_oi": pe_oi,
        "ce_ltp": ce_ltp,
        "pe_ltp": pe_ltp
    })
    
    # Test Max Pain (returns a single value, but broadcast/wrapped in Series)
    # At 25000: CE pain 200*10000=2M, PE pain 200*10000=2M. Total 4M (Min).
    mp = df.select([
        opt_expr.max_pain("strike", "ce_oi", "pe_oi", 50).alias("max_pain")
    ])
    assert mp["max_pain"][0] == 25000.0
    
    # Test Strike PCR (returns a Series of same length)
    pcr = df.with_columns([
        opt_expr.strike_pcr("ce_oi", "pe_oi").alias("strike_pcr")
    ])
    assert pcr["strike_pcr"][0] == 2000/10000
    assert pcr["strike_pcr"][2] == 10000/2000
    
    # Test Synthetic Futures
    # Future = CE - PE + Strike
    syn = df.with_columns([
        opt_expr.synthetic_futures("strike", "ce_ltp", "pe_ltp").alias("syn_future")
    ])
    assert syn["syn_future"][0] == 250.0 - 10.0 + 24800.0 # 25040.0
    assert syn["syn_future"][1] == 100.0 - 60.0 + 25000.0 # 25040.0

    # Test BS Pricing
    pricing = df.with_columns([
        opt_expr.bs_call_price(25000.0, "strike", 0.065, 7/365.0, pl.lit(0.18)).alias("calc_ce"),
        opt_expr.bs_put_price(25000.0, "strike", 0.065, 7/365.0, pl.lit(0.18)).alias("calc_pe")
    ])
    assert pricing["calc_ce"][1] == pytest.approx(264.3355426718899, rel=1e-5) # Check ATM


    # Test Moneyness
    mny = df.with_columns([
        opt_expr.moneyness(25000.0, "strike").alias("mny")
    ])
    assert mny["mny"][0] == "ITM" # CE 24800 is ITM if spot is 25000
    assert mny["mny"][1] == "ATM"
    assert mny["mny"][2] == "OTM"

    # Test NSE Lot Size
    lots = pl.DataFrame({"symbol": ["NIFTY", "BANKNIFTY"]}).with_columns([
        opt_expr.nse_lot_size("symbol").alias("lot")
    ])
    assert lots["lot"][0] == 50
    assert lots["lot"][1] == 15

    # Test OI Zones (returns a struct with lists)
    # Highest CE OI is at 24800 (10000) -> Resistance
    # Highest PE OI is at 25200 (10000) -> Support
    zones = df.select([
        opt_expr.oi_zones("strike", "ce_oi", "pe_oi", 1).alias("zones")
    ])
    assert 24800.0 in zones["zones"][0]["resistance_strikes"]
    assert 25200.0 in zones["zones"][0]["support_strikes"]


    # Test GEX Per Strike (returns a struct)
    # CE OI: 10000, 4000, 2000
    # PE OI: 2000, 5000, 10000
    # At 24800: Net GEX = 10000*0.0005 - 2000*0.0005 = +4.0 (scaled)
    # At 25000: Net GEX = 4000*0.0005 - 5000*0.0005 = -0.5
    # At 25200: Net GEX = 2000*0.0005 - 10000*0.0005 = -4.0
    # A flip should be detected between 24800 and 25000.
    
    ce_oi_alt = [10000, 4000, 2000]
    pe_oi_alt = [2000, 5000, 10000]
    df_alt = df.with_columns([
        pl.Series("ce_oi", ce_oi_alt),
        pl.Series("pe_oi", pe_oi_alt)
    ])
    
    gex = df_alt.with_columns([
        opt_expr.gex_per_strike(25000.0, "strike", pl.lit(0.0005), pl.lit(0.0005), "ce_oi", "pe_oi", 50).alias("gex")
    ])
    
    # Test GEX Flip Strike
    net_gex_series = gex["gex"].struct.field("net_gex")
    flip = df_alt.with_columns([
        pl.Series("net_gex", net_gex_series)
    ]).select([
        opt_expr.gex_flip_strike("strike", "net_gex").alias("flip")
    ])
    # Sign changes between +4.0 (at 24800) and -0.5 (at 25000)
    assert flip["flip"][0] == 24800.0



    # Test ATM Straddle
    straddle = df.select([
        opt_expr.atm_straddle(25000.0, "strike", "ce_ltp", "pe_ltp").alias("straddle")
    ])
    assert straddle["straddle"][0]["atm_strike"] == 25000.0
    assert straddle["straddle"][0]["straddle_premium"] == 100.0 + 60.0 # 160.0


    
if __name__ == "__main__":
    test_polars_chain_analytics()
