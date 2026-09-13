---
name: Bug report
about: Something is broken — including numeric parity mismatches
title: "fix: "
labels: bug
assignees: ''
---

<!--
If this is a numeric parity mismatch (QuantWave returns a plausible-looking but
wrong number — e.g. disagreeing with TA-Lib, pandas, or between the .ta / .bt /
streaming surfaces), fill in the "Parity mismatch" section below in addition to
the general sections. Otherwise you can delete that section.

Before filing: check .claude/skills/quantwave/PITFALLS.md — several "wrong"
results are documented, intentional behavior (unit conventions, ddof, sizing
defaults, etc.), not bugs.
-->

## Describe the bug

A clear, concise description of what's broken.

## To reproduce

Minimal steps or code to reproduce:

```python
import quantwave as qw
# ...
```

## Expected behavior

What you expected to happen.

## Environment

- QuantWave version: `quantwave --version` / `pip show quantwave`
- Python version:
- OS:
- Installed via: `pip install quantwave` / `pip install "quantwave[polars]"` / built from source

---

## Parity mismatch (fill in if applicable, delete otherwise)

**Indicator / function:** e.g. `rsi`, `.ta.atr`, `talib.MACD`

**Reproducing input series** (paste the exact input data, or attach a minimal
`.csv`/`.parquet` — a short synthetic series is fine as long as it reproduces
the mismatch):

```
[list of input values, or a code snippet that generates them]
```

**Parameters used:**

```python
{"period": 14, ...}
```

**Expected output** (from TA-Lib, pandas, another surface of QuantWave, or a
hand-computed reference):

```
[expected values]
```

**Actual output** (what QuantWave returned):

```
[actual values]
```

**Which surface(s) did you check?** (`pl.col().ta.*` / `lf.ta()` / `quantwave.talib`
/ streaming `streaming_class` / `qw.assert_parity(...)`) — mismatches that only show
up on one surface are especially useful to know about.
