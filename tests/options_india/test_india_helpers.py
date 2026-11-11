import pytest
import quantwave as qw
from datetime import date

def test_nse_lot_sizes():
    # Current active contract lot sizes (Nov 2025+ revision for NIFTY is 65)
    assert qw.nse_lot_size("NIFTY") == 65
    assert qw.nse_lot_size("BANKNIFTY") == 15
    assert qw.nse_lot_size("FINNIFTY") == 40
    assert qw.nse_lot_size("MIDCPNIFTY") == 75
    assert qw.nse_lot_size("SENSEX") == 10
    assert qw.nse_lot_size("UNKNOWN") is None

    # Historical date-aware lot sizes for NIFTY
    assert qw.options.nse_lot_size("NIFTY", as_of="2020-01-01") == 75
    assert qw.options.nse_lot_size("NIFTY", as_of="2023-01-01") == 50
    assert qw.options.nse_lot_size("NIFTY", as_of="2024-06-01") == 25
    assert qw.options.nse_lot_size("NIFTY", as_of="2025-05-01") == 75
    assert qw.options.nse_lot_size("NIFTY", as_of="2026-01-01") == 65
    assert qw.options.nse_lot_size("NIFTY", as_of=date(2026, 1, 1)) == 65

def test_moneyness():
    spot = 25000.0
    assert qw.moneyness(spot, 24000.0) == "ITM"
    assert qw.moneyness(spot, 26000.0) == "OTM"
    assert qw.moneyness(spot, 25010.0) == "ATM"

def test_risk_free_rate():
    assert qw.nse_risk_free_rate() == 0.065
