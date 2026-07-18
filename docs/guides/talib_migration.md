# TA-Lib Migration & the Abstract API

QuantWave exposes a **TA-Lib `abstract`-style introspection registry** so screeners,
no-code UIs, optimizers, and migration tools can drive **every** indicator
generically — discover its inputs, parameters (with defaults), and outputs, then
call it — without hardcoding per-indicator signatures.

If you're coming from `talib.abstract`, the surface will feel familiar.

## Discovery

```python
import quantwave as qw

qw.get_functions()          # -> ['ad', 'adx', 'alma', ..., 'wma']  (all indicators)
qw.get_function_groups()    # -> {'Momentum': ['adx', 'cci', ...], 'Overlap': [...], ...}
```

`get_function_groups()` is derived from `get_functions()`, so the union of every
group is exactly the function list — no extras, no duplicates. That invariant is
enforced by a test, which makes it safe to build catalogs and menus on top of it.

## The `Function` façade

```python
from quantwave.abstract import Function

f = Function("RSI")
f.input_names      # ['close']
f.parameters       # {'timeperiod': 14}
f.output_names     # ['rsi']
f.group            # 'Momentum'
f.info             # {'name': 'RSI', 'group': 'Momentum', 'input_names': [...], ...}
```

Calling it accepts either **named inputs** (a `dict` or a Polars `DataFrame`) or
**positional arrays** in canonical `open, high, low, close, volume` order — just
like classic `talib`:

```python
import numpy as np

close = np.asarray(prices, dtype=float)

Function("RSI")(close, timeperiod=14)                       # -> np.ndarray
Function("ATR")({"high": h, "low": l, "close": c})          # named inputs
Function("ATR")(h, l, c, timeperiod=14)                     # positional (H, L, C)

macd, signal, hist = Function("MACD")(close)                # multi-output -> tuple
```

Values are computed by the same parity-tested Rust plugins that back the Polars
`.ta` namespace, so `Function(name)(...)` and `pl.col(...).ta.<name>(...)` return
identical results.

## Migrating from `talib`

| `talib`                              | QuantWave                                   |
| ------------------------------------ | ------------------------------------------- |
| `talib.RSI(close, timeperiod=14)`    | `qw.talib.RSI(close, timeperiod=14)`        |
| `talib.get_functions()`              | `qw.get_functions()`                        |
| `talib.get_function_groups()`        | `qw.get_function_groups()`                  |
| `talib.abstract.Function('RSI')`     | `qw.abstract.Function('RSI')`               |
| `Function('RSI').input_names`        | `Function('RSI').input_names`               |
| `Function('RSI').parameters`         | `Function('RSI').parameters`                |
| `Function('RSI').output_names`       | `Function('RSI').output_names`              |

The `quantwave.talib` module (161 functions covering the classic TA-Lib surface)
provides the drop-in uppercase functions; `quantwave.abstract` provides the generic,
metadata-driven access.

## Example: a generic screener

Because every indicator is reachable through the registry, a screener can iterate
the whole momentum group without knowing any signature ahead of time:

```python
import numpy as np
import quantwave as qw
from quantwave.abstract import Function

def screen(frame: dict, group: str = "Momentum"):
    """Compute every indicator in `group` over `frame` and return the last value."""
    out = {}
    for name in qw.get_function_groups().get(group, []):
        f = Function(name)
        # Only run indicators whose inputs are present in the frame.
        if not set(f.input_names).issubset(frame):
            continue
        try:
            result = f({k: frame[k] for k in f.input_names})
        except Exception:
            continue
        last = result[0][-1] if isinstance(result, tuple) else result[-1]
        out[name] = float(last)
    return out

frame = {"open": o, "high": h, "low": l, "close": c, "volume": v}
signals = screen(frame, "Momentum")
```

Swap `"Momentum"` for any key of `qw.get_function_groups()` (e.g. `"Overlap"`,
`"Volume"`, `"Candlestick"`) to sweep a different family.

For the common case of "compute everything a frame supports," skip the manual
loop above and use `df.ta.all()` (below) — it does the same
input-satisfiability filtering, but as one batched Polars pipeline.

## Bulk-compute: `df.ta.all()`

`df.ta.all()` / `lf.ta.all()` drive the *entire* registry generically over a
Polars DataFrame or LazyFrame in a single lazy pass: every batch-capable
indicator whose required input columns are present gets computed and added as
new columns, in one `collect()`. Streaming-only indicators (no batch `.ta`
implementation) and indicators whose inputs aren't satisfiable by the frame
(missing OHLCV columns, or multi-series indicators like `beta`/`correl` that
need arbitrary paired series) are skipped with a recorded reason — nothing
raises.

```python
import polars as pl
import quantwave as qw

df = pl.DataFrame({"open": o, "high": h, "low": l, "close": c, "volume": v})

features, manifest = df.ta.all()
# features: df + one column per single-output indicator (e.g. "rsi"),
#           and f"{name}_{field}" per multi-output indicator (e.g. "bbands_upper")
# manifest: {"computed": [...], "skipped": [{"name", "reason"}, ...], "columns": [...]}

lazy_features, manifest = df.lazy().ta.all()   # LazyFrame in -> LazyFrame out (uncollected)
```

Filter the universe with `include`/`exclude`/`groups` (set algebra: start from
all indicators, restrict to `groups`, restrict further to `include`, subtract
`exclude`), or override any indicator's parameters per-call via `params`:

```python
features, manifest = df.ta.all(
    groups=["Momentum", "Volatility"],
    exclude=["mama"],
    params={"rsi": {"timeperiod": 21}},
)
```

Pass `timing=True` to add a `manifest["timing"]` map of per-indicator wall
time (computed via a separate pass, so it trades away some of the batched
pipeline's parallelism — only pay for it when profiling).

`qw.feature_matrix(df, **same_kwargs)` is a thin alias returning
`(df, feature_column_names)` for quick ML pipeline wiring, in place of the
richer manifest.

## See also

- [Plugin vs `.ta`](plugin_vs_ta.md) — when to use expression plugins vs the accessor
- [Full Catalog](indicators/native/index.md) — every native indicator page
- [ML Features](ml_features.md) — bulk feature extraction
