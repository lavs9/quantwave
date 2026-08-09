# How *Not* To Use QuantWave

Every item below was reproduced against QuantWave 0.7.0. They share one property:
**QuantWave returns a plausible number and no error.** Nothing here throws. That is
what makes them dangerous — a wrong backtest looks exactly like a right one.

Ordered by how much damage they do.

---

## 1. Assuming `signal=1` means "go long with my capital"

**It means "hold 1 unit."** Not 100% of equity, not one lot — one share.

```python
df.lazy().bt.backtest_with_report(signal="signal", initial_cash=100_000)
```

With `initial_cash=100_000` and a ~$100 stock, a `signal` of `1` opens a position worth
**$100 — 0.1% of the account.** Verified: quantity `1.0`, `total_return` `2e-05` on a
strategy that caught a clean +2% move.

**Why it burns you:** returns look near-zero, so the strategy reads as worthless; or
`sharpe_ratio` comes back at `7.98` on one trade and reads as brilliant. Both
conclusions are about the sizing default, not the strategy.

**Do instead** — size explicitly with `size_multiplier_col`:

```python
df = df.with_columns(
    (pl.lit(100_000) * 0.95 / pl.col("close")).alias("size_mult")   # must be Float64
)
report = df.lazy().bt.backtest_with_report(signal="signal", size_multiplier_col="size_mult")
```

`size_multiplier_col` **rejects `Int64`**. Cast to `Float64`.

> Equity-fraction sizing (`signal_type={"shares","weight","target_pct"}`) is not shipped
> yet. Until it lands, sizing is your job.

---

## 2. Trusting `.metrics()` on an unsized or single-trade backtest

Undefined ratios come back as `NaN`, never `inf` — one winning trade with no losses
gives `profit_factor=nan`, `sortino_ratio=nan`, `calmar_ratio=nan`, `win_rate=1.0`.
Test with `math.isnan()`; `NaN == NaN` is `False`, so an equality check lies.

**Never report a metric without `num_trades` next to it.** Below ~30 closed trades, the
ratios are noise. Check `report.metrics()["num_trades"]` first, every time.

---

## 3. `roc` is ×100. `rocp` is the ratio.

```
roc(10)  = -0.830565      # percent, TA-Lib convention
rocp(10) = -0.008306      # plain (p / p_n) - 1
```

Feed `roc` into anything expecting a fractional return and you are **100× off**, silently.
Returns-based features, vol targeting, and Sharpe calculations all expect `rocp`.

---

## 4. `stddev` is population (ddof=0). pandas `.std()` is sample (ddof=1).

Verified on the same 10 values: QuantWave `1.9653244`, pandas default `2.0716338`.
They differ by `sqrt(N/(N-1))` — ~5% at N=10, and it does not vanish; it just shrinks.

Porting a pandas z-score or Bollinger band straight across gives **tighter bands and more
signals** than the original. If you need ddof=1, rescale it yourself.

---

## 5. `atr` means different things on different surfaces

> **Historical note.** Before this was fixed, the `ta_`-prefixed plugins took
> `(high, low, close)` positionally with generated `in2`/`in3` parameter names, so
> the receiver had to be **high** while the sibling `.ta.atr` takes **close**.
> Writing the call the obvious way silently permuted the inputs and returned a
> plausible wrong number. **That is fixed** — `ta_atr`, `ta_natr` and `ta_trange`
> now take close in the receiver and are named `(high, low)`, matching their
> siblings. If you are on an older release, check the argument order.

Both of these are now correct and agree:

```python
pl.col("close").ta.atr("high", "low", timeperiod=14)      # 1.384819
pl.col("close").ta.ta_atr("high", "low", timeperiod=14)   # 1.384819
```

The live trap is not argument order — it is that **the same slug means different
math on different surfaces**.

### What `atr` actually is on each surface

```
pl.col("close").ta.atr("high","low",14)   = 1.384819   # Wilder RMA — matches TA-Lib
quantwave.talib.ATR(h,l,c,timeperiod=14)  = Wilder     # matches TA-Lib
qw.ta.atr(14, h, l, c)   (list API)       = EMA variant, alpha=2/(period+1)
```

The Polars plugin `.ta.atr` **is** Wilder and is correct — reach for it freely. But the
list-based `qw.ta.atr(...)` computes an EMA-smoothed variant under the same name, so the
two disagree. The EMA variant has **no recorded authoritative source**.

When a number must match an external chart, verify it on the exact surface you are
calling rather than trusting the slug name, and confirm against `qw.metadata()`.

---

## 6. `execution_delay="same_bar"` is the default — and it is optimistic

`same_bar` fills at the **signal bar's own close**. If your signal is derived from that
same close (RSI crossing 30 computed on bar *t*, filled at bar *t*'s close), you are
executing on information available only at the instant the bar ends.

Verified: `same_bar` entry `100.5`, `next_bar` entry `101.0` on the same signal.

**For any strategy claiming realism, pass `execution_delay="next_bar"`.** Keep `same_bar`
only for signals built purely from bar *t-1* data or for close-auction execution you can
actually achieve.

---

## 7. `hmm_bull_bear` batch-fits the whole series — do not backtest with it

It fits over the entire input, so every bar's regime label is informed by the future.
Any backtest using it is look-ahead-contaminated and the equity curve is fiction.

Also, its contract is not what you would guess:
- feed it **returns, not price** — on price it degenerates to a constant `1`
- states are `{1, 2}`, not `{0, 1}`, and `2` is empirically bear

Use it for **post-hoc regime description only**.

Treat any batch-fitted regime/clustering output (GMM, PELT, k-means labels) with the same
suspicion unless you have confirmed a causal/streaming fit path.

---

## 8. Passing unsorted multi-symbol data

`portfolio_backtest` requires the frame sorted by **`["timestamp", "symbol"]`** in that
order. This one *does* raise — `ValueError: Data must be sorted by timestamp (and symbol
for multi-symbol runs)` — but the fix is non-obvious:

```python
df.sort(["timestamp", "symbol"]).lazy().bt.portfolio_backtest(signal="signal")
```

Sorting by `["symbol", "timestamp"]` (the natural per-instrument grouping) fails.

---

## 9. Cleaning warmup with `drop_nulls()` — it removes nothing

Warmup is **`NaN`, not `null`**. Polars treats those as different things:

```
30 rows -> drop_nulls() -> 30 rows    # no-op, null_count is 0
30 rows -> drop_nans()  -> 16 rows    # 14 warmup bars actually removed
```

The idiomatic pandas reflex (`.drop_nulls()` / `.dropna()`) is a **silent no-op here**,
and the warmup rows sail straight into your backtest, your feature matrix, or your
`mean()`. Use `drop_nans()`, `.is_not_nan()`, or — better, because it is deterministic
and preserves alignment across differently-warmed indicators — slice by warmup:

```python
n = qw.warmup_bars("rsi", {"period": 14})
report = df.slice(n).lazy().bt.backtest_with_report(signal="signal")
```

Slice **after** computing indicators, **before** backtesting. When combining several
indicators, slice by the **largest** warmup among them.

Note that comparison signals degrade quietly rather than loudly: `NaN < 30` evaluates to
`false`, so `(rsi < 30).cast(pl.Float64)` yields `0.0` across the whole warmup. You get a
run of flat bars that is indistinguishable from a genuine no-signal period.

---

## 10. Misreading the units in `.metrics()`

Everything return-like is a **fraction, not a percent**, and `max_drawdown_pct` is a
**positive** fraction:

- `total_return=0.05` is 5%, not 5 basis points
- `max_drawdown_pct=0.10` is a 10% *decline* — do not negate it, do not multiply by 100 twice
- `var_95` / `cvar_95` are positive loss magnitudes
- `stop_loss_pct=0.02` is 2% — passing `2.0` means a 200% stop, i.e. never triggers

`entry_ts` / `exit_ts` are **epoch seconds** (i64), not datetimes. `side` is `1`/`-1`.

---

## 11. Reaching for streaming when you meant batch (and vice versa)

- Expression plugins (`pl.col().ta.*`) are **stateless** — they cannot carry position or
  indicator state across calls. Do not try to drive live bars through them one row at a time.
- `streaming_class` instances are **stateful and single-series** — do not reuse one
  instance across symbols. One instance per symbol.
- Do not re-implement an indicator in Python "for the live path." That is the exact
  duplication `Next<T>` exists to remove, and it silently breaks parity. Use the same
  slug, and assert it:

```python
qw.assert_parity("supertrend", {"period": 10, "multiplier": 3.0}, closes)
```

---

## 12. Guessing parameter names

`rsi(timeperiod=14)` but `ema(period=20)`. `stddev(timeperiod=5, nbdev=1.0)`.
`supertrend(high, low, period=10, multiplier=3.0)`.

There is no rule you can infer — TA-Lib-lineage indicators kept `timeperiod`, native ones
use `period`. Call `qw.metadata("<slug>")` and read `params`. A wrong keyword raises, but
a *plausible* wrong positional argument may not.

---

## Not-yet-supported — don't build on these

| Want | Status |
|---|---|
| Live order routing / broker execution | Not shipped (Nautilus bridge deferred) |
| Wide-format matrix portfolio optimization | Out of scope |
| Equity-fraction `signal_type` sizing | Not shipped as of 0.7.0 |
| Intra-trade position resizing | Risk overlays size **at entry only** |

Pyramiding does not exist in `order_backtest`: the model is flat-or-single-position, a
same-side fill while in a position is **silently ignored**, and any open position is
force-flattened at the final bar's close.
