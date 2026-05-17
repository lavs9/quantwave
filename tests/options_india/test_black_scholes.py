import pytest
import quantwave as qw

def test_hull_reference_values():
    # S=42, K=40, r=0.10, T=0.5, sigma=0.20
    s = 42.0
    k = 40.0
    r = 0.10
    t = 0.5
    sigma = 0.20
    
    call_price = qw.bs_call_price(s, k, r, t, sigma)
    put_price = qw.bs_put_price(s, k, r, t, sigma)
    
    # Values from Hull "Options, Futures, and Other Derivatives"
    assert call_price == pytest.approx(4.7594, abs=1e-4)
    assert put_price == pytest.approx(0.8086, abs=1e-4)

def test_greek_directions():
    s = 25000.0
    k = 25000.0
    r = 0.065
    t = 7/365.0
    sigma = 0.18
    
    delta_ce = qw.bs_delta(s, k, r, t, sigma, True)
    delta_pe = qw.bs_delta(s, k, r, t, sigma, False)
    
    assert 0.0 <= delta_ce <= 1.0
    assert -1.0 <= delta_pe <= 0.0
    
    gamma = qw.bs_gamma(s, k, r, t, sigma)
    assert gamma > 0.0
    
    vega = qw.bs_vega(s, k, r, t, sigma)
    assert vega > 0.0
    
    theta_ce = qw.bs_theta(s, k, r, t, sigma, True)
    theta_pe = qw.bs_theta(s, k, r, t, sigma, False)
    assert theta_ce < 0.0
    assert theta_pe < 0.0
    
    rho_ce = qw.bs_rho(s, k, r, t, sigma, True)
    rho_pe = qw.bs_rho(s, k, r, t, sigma, False)
    assert rho_ce > 0.0
    assert rho_pe < 0.0
