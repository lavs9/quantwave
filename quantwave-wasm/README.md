# quantwave-wasm

WebAssembly (`wasm-bindgen`) bindings for [QuantWave](https://github.com/lavs9/quantwave)'s `quantwave-core` streaming indicators (`Next<T>`), enabling bit-identical technical analysis in browsers and Node.js with zero Python or server runtime.

## Quick Start

```js
import init, { SuperTrend, Rsi, Macd, Bbands } from "quantwave-wasm";

await init();
const st = new SuperTrend(10, 3.0);
const rsi = new Rsi(14);
const macd = new Macd(12, 26, 9);

const [stValue, stDirection] = st.next(105.0, 95.0, 102.0);
const rsiValue = rsi.next(102.0);
const [macdLine, signalLine, hist] = macd.next(102.0);
```

## Included Indicators

- **Trend / Overlap**: `Sma`, `Ema`, `Wma`, `SuperTrend`, `Bbands`, `Ichimoku`
- **Momentum**: `Rsi`, `Macd`, `Stoch`, `Adx`, `Adxr`, `TtmSqueeze`
- **Volatility / Filtering**: `Atr`, `SuperSmoother`, `CyberCycle`, `RoofingFilter`, `KalmanFilter`
- **Price Action / Alternative Bars**: `MarketStructure`, `HarmonicScanner`, `Renko`, `Kagi`

## Building & Testing

```bash
./scripts/test_wasm.sh
```
