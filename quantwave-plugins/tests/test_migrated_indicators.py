import polars as pl
import quantwave_plugins
import math

def test_ema_math():
    df = pl.DataFrame({
        "close": [10.0, 20.0, 30.0, 40.0, 50.0, 40.0, 30.0, 20.0, 10.0, 20.0, 30.0, 40.0, 50.0]
    })
    res = df.lazy().with_columns(pl.col("close").ta.ema(period=3).alias("out")).collect().get_column("out").to_list()
    expected = [10.0, 15.0, 22.5, 31.25, 40.625, 40.3125, 35.15625, 27.578125, 18.7890625, 19.39453125, 24.697265625, 32.3486328125, 41.17431640625]
    for r, e in zip(res, expected):
        if math.isnan(e):
            assert math.isnan(r)
        else:
            assert abs(r - e) < 1e-9

def test_rsi_math():
    df = pl.DataFrame({
        "close": [10.0, 20.0, 30.0, 40.0, 50.0, 40.0, 30.0, 20.0, 10.0, 20.0, 30.0, 40.0, 50.0]
    })
    res = df.lazy().with_columns(pl.col("close").ta.rsi(timeperiod=3).alias("out")).collect().get_column("out").to_list()
    expected = [float('nan'), float('nan'), float('nan'), 100.0, 100.0, 66.66666666666666, 44.444444444444436, 29.629629629629633, 19.75308641975309, 46.50205761316872, 64.33470507544581, 76.22313671696388, 84.14875781130925]
    for r, e in zip(res, expected):
        if math.isnan(e):
            assert math.isnan(r)
        else:
            assert abs(r - e) < 1e-9

def test_macd_math():
    df = pl.DataFrame({
        "close": [10.0, 20.0, 30.0, 40.0, 50.0, 40.0, 30.0, 20.0, 10.0, 20.0, 30.0, 40.0, 50.0]
    })
    res = df.lazy().with_columns(pl.col("close").ta.macd(fast=3, slow=6, signal=3).alias("out")).collect().get_column("out").to_list()
    expected = [{'macd': float('nan'), 'signal': float('nan'), 'hist': float('nan')}, {'macd': float('nan'), 'signal': float('nan'), 'hist': float('nan')}, {'macd': float('nan'), 'signal': float('nan'), 'hist': float('nan')}, {'macd': float('nan'), 'signal': float('nan'), 'hist': float('nan')}, {'macd': float('nan'), 'signal': float('nan'), 'hist': float('nan')}, {'macd': float('nan'), 'signal': float('nan'), 'hist': float('nan')}, {'macd': float('nan'), 'signal': float('nan'), 'hist': float('nan')}, {'macd': 0.3401360544217731, 'signal': 5.827664399092974, 'hist': -5.487528344671201}, {'macd': -3.6856171039844483, 'signal': 1.071023647554263, 'hist': -4.756640751538711}, {'macd': -2.4540122171317478, 'signal': -0.6914942847887424, 'hist': -1.7625179323430054}, {'macd': 0.4792769877630363, 'signal': -0.10610864851285307, 'hist': 0.5853856362758894}, {'macd': 3.6012692769736, 'signal': 1.7475803142303734, 'hist': 1.8536889627432265}, {'macd': 6.344656626409716, 'signal': 4.046118470320044, 'hist': 2.2985381560896716}]
    for r, e in zip(res, expected):
        for k in e:
            if math.isnan(e[k]):
                assert math.isnan(r[k])
            else:
                assert abs(r[k] - e[k]) < 1e-9

def test_bbands_math():
    df = pl.DataFrame({
        "close": [10.0, 20.0, 30.0, 40.0, 50.0, 40.0, 30.0, 20.0, 10.0, 20.0, 30.0, 40.0, 50.0]
    })
    res = df.lazy().with_columns(pl.col("close").ta.bbands(timeperiod=3).alias("out")).collect().get_column("out").to_list()
    expected = [{'upper': float('nan'), 'middle': float('nan'), 'lower': float('nan')}, {'upper': float('nan'), 'middle': float('nan'), 'lower': float('nan')}, {'upper': 36.32993161855451, 'middle': 20.0, 'lower': 3.6700683814454855}, {'upper': 46.32993161855451, 'middle': 30.0, 'lower': 13.670068381445486}, {'upper': 56.329931618554504, 'middle': 40.0, 'lower': 23.670068381445496}, {'upper': 52.76142374915405, 'middle': 43.33333333333333, 'lower': 33.90524291751261}, {'upper': 56.329931618554504, 'middle': 40.0, 'lower': 23.670068381445496}, {'upper': 46.32993161855451, 'middle': 30.0, 'lower': 13.670068381445486}, {'upper': 36.32993161855451, 'middle': 20.0, 'lower': 3.6700683814454855}, {'upper': 26.094757082487313, 'middle': 16.666666666666664, 'lower': 7.238576250846018}, {'upper': 36.32993161855451, 'middle': 20.0, 'lower': 3.6700683814454855}, {'upper': 46.32993161855451, 'middle': 30.0, 'lower': 13.670068381445486}, {'upper': 56.329931618554504, 'middle': 40.0, 'lower': 23.670068381445496}]
    for r, e in zip(res, expected):
        for k in e:
            if math.isnan(e[k]):
                assert math.isnan(r[k])
            else:
                assert abs(r[k] - e[k]) < 1e-9
