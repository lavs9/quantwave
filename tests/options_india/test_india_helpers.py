import pytest
import quantwave as qw
from datetime import date

def test_nse_lot_sizes():
    assert qw.nse_lot_size("NIFTY") == 50
    assert qw.nse_lot_size("BANKNIFTY") == 15
    assert qw.nse_lot_size("FINNIFTY") == 40
    assert qw.nse_lot_size("MIDCPNIFTY") == 75
    assert qw.nse_lot_size("SENSEX") == 10
    assert qw.nse_lot_size("UNKNOWN") is None

def test_moneyness():
    spot = 25000.0
    assert qw.moneyness(spot, 24000.0) == "ITM"
    assert qw.moneyness(spot, 26000.0) == "OTM"
    assert qw.moneyness(spot, 25010.0) == "ATM"

def test_risk_free_rate():
    assert qw.nse_risk_free_rate() == 0.065
