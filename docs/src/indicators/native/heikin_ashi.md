# Heikin-Ashi

Heikin-Ashi candles filter market noise to reveal the underlying trend.

## Formula


\[
HA_{Close} = \frac{O + H + L + C}{4} \\ HA_{Open} = \frac{HA_{Open, t-1} + HA_{Close, t-1}}{2}
\]


[Source](https://www.investopedia.com/trading/heikin-ashi-better-candlestick/)
