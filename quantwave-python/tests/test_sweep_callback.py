import polars as pl
import pytest
import quantwave as qw

def test_sweep_callback_single_param_rebuilds_signal():
    df = pl.DataFrame({
        "bar": range(10),
        "close": [10.0, 10.5, 9.8, 11.2, 11.5, 12.0, 11.8, 12.5, 13.0, 12.8],
        "feature": [0.1, 0.5, 0.2, 0.8, 0.9, 1.2, 0.7, 1.5, 1.6, 1.1]
    }).lazy()

    def build_fn(ldf, params):
        threshold = params["threshold"]
        return ldf.with_columns(
            signal=pl.when(pl.col("feature") > threshold).then(1.0).otherwise(0.0)
        )

    grid = {"threshold": [0.5, 1.0, 1.5]}
    sweep_res = df.bt.sweep_callback(
        param_grid=grid,
        build_fn=build_fn,
        signal="signal",
        close_col="close",
        timestamp_col="bar"
    )

    assert isinstance(sweep_res, pl.DataFrame)
    assert len(sweep_res) == 3
    assert "threshold" in sweep_res.columns

def test_sweep_callback_two_params_cartesian():
    df = pl.DataFrame({
        "bar": range(10),
        "close": [10.0, 10.5, 9.8, 11.2, 11.5, 12.0, 11.8, 12.5, 13.0, 12.8],
        "feature1": [0.1, 0.5, 0.2, 0.8, 0.9, 1.2, 0.7, 1.5, 1.6, 1.1],
        "feature2": [10, 20, 15, 25, 30, 35, 25, 40, 45, 35]
    }).lazy()

    def build_fn(ldf, params):
        return ldf.with_columns(
            signal=pl.when((pl.col("feature1") > params["a"]) & (pl.col("feature2") > params["b"])).then(1.0).otherwise(0.0)
        )

    grid = {"a": [0.5, 1.0], "b": [15, 25]}
    sweep_res = df.bt.sweep_callback(param_grid=grid, build_fn=build_fn, signal="signal", close_col="close", timestamp_col="bar")
    
    assert len(sweep_res) == 4
    assert "a" in sweep_res.columns
    assert "b" in sweep_res.columns

def test_sweep_callback_metrics_match_manual_sweep():
    df = pl.DataFrame({
        "bar": range(10),
        "close": [10.0, 10.5, 9.8, 11.2, 11.5, 12.0, 11.8, 12.5, 13.0, 12.8],
        "sig_1": [0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0],
        "sig_2": [1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]
    }).lazy()
    
    # manual sweep
    manual_res = df.bt.sweep(param_values=[1, 2], signal_cols=["sig_1", "sig_2"], param_name="variant", close_col="close", timestamp_col="bar")
    
    def build_fn(ldf, params):
        return ldf.with_columns(signal=pl.col(f"sig_{params['variant']}"))
        
    cb_res = df.bt.sweep_callback(param_grid={"variant": [1, 2]}, build_fn=build_fn, signal="signal", close_col="close", timestamp_col="bar")
    
    assert manual_res.select("num_trades").to_dict(as_series=False) == cb_res.select("num_trades").to_dict(as_series=False)

def test_sweep_callback_invalid_grid_raises():
    df = pl.DataFrame({"bar": [1], "close": [1]}).lazy()
    
    with pytest.raises(ValueError):
        df.bt.sweep_callback(param_grid={}, build_fn=lambda x,y: x, signal="signal", close_col="close", timestamp_col="bar")
    
    with pytest.raises(ValueError):
        df.bt.sweep_callback(param_grid={"a": [1]}, build_fn="not callable", signal="signal", close_col="close", timestamp_col="bar")
        
    def bad_build_fn(ldf, params):
        return ldf # missing signal col
        
    with pytest.raises(ValueError):
        df.bt.sweep_callback(param_grid={"a": [1]}, build_fn=bad_build_fn, signal="signal", close_col="close", timestamp_col="bar")
