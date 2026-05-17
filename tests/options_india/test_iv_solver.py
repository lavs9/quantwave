import pytest
import quantwave as qw

def test_iv_round_trip():
    s = 25000.0
    k = 25000.0
    r = 0.065
    t = 7/365.0
    sigma_orig = 0.18
    
    price = qw.bs_call_price(s, k, r, t, sigma_orig)
    iv = qw.implied_vol(price, s, k, r, t, True)
    
    assert iv is not None
    assert iv == pytest.approx(sigma_orig, abs=1e-5)

def test_iv_extreme_moneyness():
    # Test LetsBeRational robustness
    s = 100.0
    k = 180.0
    r = 0.03
    t = 0.1
    sigma_orig = 0.2
    
    # From Peter Jaeckel's README example
    price = qw.bs_call_price(s, k, r, t, sigma_orig)
    iv = qw.implied_vol(price, s, k, r, t, True)
    
    assert iv is not None
    assert iv == pytest.approx(sigma_orig, abs=1e-10)

def test_iv_put():
    s = 25000.0
    k = 24500.0
    r = 0.065
    t = 14/365.0
    sigma_orig = 0.22
    
    price = qw.bs_put_price(s, k, r, t, sigma_orig)
    iv = qw.implied_vol(price, s, k, r, t, False)
    
    assert iv is not None
    assert iv == pytest.approx(sigma_orig, abs=1e-5)
