import polars as pl
import quantwave as qw
import pytest

def test_sma_parity():
    # Test data
    data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
    df = pl.DataFrame({"close": data})
    
    # 1. Test Polars Namespace (.ta.sma)
    period = 3
    # Use ta.sma since that's what we registered
    result_df = df.lazy().ta.sma("close", period).collect()
    
    # Expected results (SMA period 3)
    # 1.0 -> 1.0 / 1 = 1.0
    # 2.0 -> (1+2) / 2 = 1.5
    # 3.0 -> (1+2+3) / 3 = 2.0
    # 4.0 -> (2+3+4) / 3 = 3.0
    # ...
    expected_sma = [1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    
    # The output column name in quantwave-polars is "sma" for the sma method
    actual_sma = result_df["sma"].to_list()
    assert actual_sma == pytest.approx(expected_sma)
    
    # 2. Test Streaming API (PySMA)
    sma = qw.PySMA(period)
    streaming_results = [sma.next(v) for v in data]
    assert streaming_results == pytest.approx(expected_sma)

if __name__ == "__main__":
    test_sma_parity()
