import polars as pl
import quantwave as qw
from quantwave.polars import options as opt_expr
import pytest

def test_polars_options_analytics():
    # Setup a synthetic option chain
    spot = 25000.0
    strikes = [24800.0, 24900.0, 25000.0, 25100.0, 25200.0]
    r = 0.065
    t = 7/365.0
    
    # Calculate LTPs with sigma=0.18
    ce_ltps = [qw.bs_call_price(spot, k, r, t, 0.18) for k in strikes]
    pe_ltps = [qw.bs_put_price(spot, k, r, t, 0.18) for k in strikes]
    
    df = pl.DataFrame({
        "strike": strikes,
        "ce_ltp": ce_ltps,
        "pe_ltp": pe_ltps,
        "t_years": [t] * len(strikes)
    })
    
    # Calculate IV and Greeks using Polars
    result = df.with_columns([
        opt_expr.implied_vol("ce_ltp", spot, "strike", r, "t_years", is_call=True).alias("iv"),
    ]).with_columns([
        opt_expr.bs_delta("iv", spot, "strike", r, "t_years", is_call=True).alias("delta"),
        opt_expr.bs_gamma("iv", spot, "strike", r, "t_years").alias("gamma"),
    ])
    
    # Verify results
    # IV should be around 0.18
    assert result["iv"][2] == pytest.approx(0.18, abs=1e-5)
    
    # Delta of ATM CE should be around 0.5
    assert result["delta"][2] == pytest.approx(0.5, abs=0.1)
    
    # Gamma should be positive
    assert all(g > 0 for g in result["gamma"])

if __name__ == "__main__":
    test_polars_options_analytics()
