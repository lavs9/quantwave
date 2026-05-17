import pytest
import quantwave as qw

def test_max_pain():
    strikes = [24800.0, 25000.0, 25200.0]
    ce_oi = [10000, 5000, 2000]
    pe_oi = [2000, 5000, 10000]
    lot_size = 50
    
    # At 24800: CE pain 0, PE pain (25000-24800)*5000 + (25200-24800)*10000 = 200*5000 + 400*10000 = 1M + 4M = 5M
    # At 25000: CE pain (25000-24800)*10000 = 200*10000 = 2M, PE pain (25200-25000)*10000 = 200*10000 = 2M. Total 4M.
    # At 25200: CE pain (25200-24800)*10000 + (25200-25000)*5000 = 4M + 1M = 5M, PE pain 0.
    # So max pain strike is 25000.
    
    result = qw.max_pain(strikes, ce_oi, pe_oi, lot_size)
    assert result == 25000.0

def test_pcr():
    ce_oi = [1000, 2000]
    pe_oi = [500, 4000]
    
    strike_pcr = qw.strike_pcr(ce_oi, pe_oi)
    assert strike_pcr == [0.5, 2.0]
    
    chain_pcr = qw.chain_pcr(ce_oi, pe_oi)
    assert chain_pcr == 4500 / 3000

def test_oi_zones():
    strikes = [24800.0, 24900.0, 25000.0, 25100.0]
    ce_oi = [100, 500, 1000, 200]
    pe_oi = [1000, 200, 100, 50]
    
    zones = qw.oi_zones(strikes, ce_oi, pe_oi, 2)
    assert zones.resistance_strikes == [25000.0, 24900.0]
    assert zones.support_strikes == [24800.0, 24900.0]

def test_gex():
    spot = 25000.0
    strikes = [25000.0]
    ce_gamma = [0.0005]
    pe_gamma = [0.0005]
    ce_oi = [1000]
    pe_oi = [1000]
    lot_size = 50
    
    # CE GEX = 1000 * 0.0005 * 25000 * 50 * 0.01 = 0.5 * 250 * 50 * 0.01 = 125 * 0.5 = 62.5? No.
    # 1000 * 0.0005 = 0.5
    # 0.5 * 25000 = 12500
    # 12500 * 50 = 625000
    # 625000 * 0.01 = 6250.0
    
    gex = qw.gex_per_strike(spot, strikes, ce_gamma, pe_gamma, ce_oi, pe_oi, lot_size)
    assert gex[0].ce_gex == pytest.approx(6250.0)
    assert gex[0].pe_gex == pytest.approx(-6250.0)
    assert gex[0].net_gex == pytest.approx(0.0)
