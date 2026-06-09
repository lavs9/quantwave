import polars as pl
import quantwave_plugins
import math

def test_all_indicators():
    df = pl.DataFrame({
        "open": [9.0, 11.0, 14.0, 13.0, 15.0, 17.0, 19.0, 21.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0],
        "high": [10.5, 12.5, 15.5, 14.5, 16.5, 18.5, 20.5, 22.5, 24.5, 25.5, 26.5, 27.5, 28.5, 29.5, 30.5],
        "low": [9.5, 11.5, 14.5, 13.5, 15.5, 17.5, 19.5, 21.5, 23.5, 24.5, 25.5, 26.5, 27.5, 28.5, 29.5],
        "close": [10.0, 12.0, 15.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0],
        "volume": [100.0, 150.0, 200.0, 100.0, 120.0, 180.0, 300.0, 250.0, 210.0, 190.0, 150.0, 110.0, 105.0, 200.0, 300.0]
    })
    
    res = df.lazy().with_columns([
        # Base
        pl.col("close").ta.sma(period=3).alias("sma"),
        pl.col("close").ta.ema(period=3).alias("ema"),
        pl.col("close").ta.rsi(timeperiod=3).alias("rsi"),
        pl.col("close").ta.macd(fast=3, slow=6, signal=3).alias("macd"),
        pl.col("close").ta.bbands(timeperiod=3).alias("bbands"),
        
        # Momentum & Trend
        pl.col("close").ta.cci("high", "low", timeperiod=3).alias("cci"),
        pl.col("close").ta.cmo(timeperiod=3).alias("cmo"),
        pl.col("close").ta.mom(timeperiod=3).alias("mom"),
        pl.col("close").ta.roc(timeperiod=3).alias("roc"),
        pl.col("close").ta.rocp(timeperiod=3).alias("rocp"),
        pl.col("close").ta.rocr(timeperiod=3).alias("rocr"),
        pl.col("close").ta.rocr100(timeperiod=3).alias("rocr100"),
        pl.col("close").ta.trix(timeperiod=3).alias("trix"),
        pl.col("close").ta.willr("high", "low", timeperiod=3).alias("willr"),
        pl.col("close").ta.apo(fastperiod=3, slowperiod=6).alias("apo"),
        pl.col("close").ta.ppo(fastperiod=3, slowperiod=6).alias("ppo"),
        pl.col("low").ta.sar("high").alias("sar"),
        pl.col("low").ta.aroon("high", timeperiod=3).alias("aroon"),
        pl.col("low").ta.aroonosc("high", timeperiod=3).alias("aroonosc"),
        pl.col("close").ta.stoch("high", "low", fastk_period=3).alias("stoch"),
        pl.col("close").ta.stochf("high", "low", fastk_period=3).alias("stochf"),
        pl.col("close").ta.stochrsi(timeperiod=3).alias("stochrsi"),
        pl.col("close").ta.ultosc("high", "low", timeperiod1=2, timeperiod2=3, timeperiod3=4).alias("ultosc"),
        pl.col("close").ta.adx("high", "low", timeperiod=3).alias("adx"),
        pl.col("close").ta.adxr("high", "low", timeperiod=3).alias("adxr"),
        pl.col("close").ta.dx("high", "low", timeperiod=3).alias("dx"),
        pl.col("close").ta.plus_di("high", "low", timeperiod=3).alias("plus_di"),
        pl.col("close").ta.minus_di("high", "low", timeperiod=3).alias("minus_di"),
        pl.col("low").ta.plus_dm("high", timeperiod=3).alias("plus_dm"),
        pl.col("low").ta.minus_dm("high", timeperiod=3).alias("minus_dm"),

        # Volatility
        pl.col("close").ta.atr("high", "low", timeperiod=3).alias("atr"),
        pl.col("close").ta.natr("high", "low", timeperiod=3).alias("natr"),
        pl.col("close").ta.trange("high", "low").alias("trange"),

        # Volume
        pl.col("close").ta.ad("high", "low", "volume").alias("ad"),
        pl.col("close").ta.adosc("high", "low", "volume").alias("adosc"),

        # Price Transform
        pl.col("close").ta.avgprice("open", "high", "low").alias("avgprice"),
        pl.col("high").ta.medprice("low").alias("medprice"),
        pl.col("high").ta.typprice("low", "close").alias("typprice"),
        pl.col("high").ta.wclprice("low", "close").alias("wclprice"),

        # Overlap
        pl.col("close").ta.trima(timeperiod=3).alias("trima"),
        pl.col("close").ta.midpoint(timeperiod=3).alias("midpoint"),
        pl.col("high").ta.midprice("low", timeperiod=3).alias("midprice"),
        pl.col("close").ta.kama(timeperiod=3).alias("kama"),
        pl.col("close").ta.t3(timeperiod=3).alias("t3"),
        pl.col("close").ta.dema(timeperiod=3).alias("dema"),
        pl.col("close").ta.macdext().alias("macdext"),
        pl.col("close").ta.macdfix().alias("macdfix"),

        # Statistics
        pl.col("close").ta.stddev(timeperiod=3).alias("stddev"),
        pl.col("close").ta.var(timeperiod=3).alias("var"),
        pl.col("close").ta.linearreg(timeperiod=3).alias("linearreg"),
        pl.col("close").ta.linearreg_slope(timeperiod=3).alias("linearreg_slope"),
        pl.col("close").ta.linearreg_intercept(timeperiod=3).alias("linearreg_intercept"),
        pl.col("close").ta.linearreg_angle(timeperiod=3).alias("linearreg_angle"),
        pl.col("close").ta.tsf(timeperiod=3).alias("tsf"),
        pl.col("close").ta.correl("high", timeperiod=3).alias("correl"),
        pl.col("close").ta.beta("high", timeperiod=3).alias("beta"),

        # Hilbert
        pl.col("close").ta.ht_dcperiod().alias("ht_dcperiod"),
        pl.col("close").ta.ht_dcphase().alias("ht_dcphase"),
        pl.col("close").ta.ht_phasor().alias("ht_phasor"),
        pl.col("close").ta.ht_sine().alias("ht_sine"),
        pl.col("close").ta.ht_trendmode().alias("ht_trendmode"),
        pl.col("close").ta.ht_trendline().alias("ht_trendline")
    ]).collect()
    
    # Just checking they run and output correctly formatted structs/floats
    print(res)
    print("All indicators successfully bound and executed!")

if __name__ == "__main__":
    test_all_indicators()
