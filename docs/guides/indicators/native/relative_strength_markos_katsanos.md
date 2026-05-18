# Relative Strength Markos Katsanos

<div class="indicator-meta"><span class="category-badge">Momentum</span> <span class="kw-badge">relative strength</span> <span class="kw-badge">momentum</span> <span class="kw-badge">benchmark</span> <span class="kw-badge">katsanos</span></div>

An improved relative strength indicator that compares a security to a benchmark, separating periods of strong and weak relative performance.

## Usage

Use as a momentum-based relative strength indicator. Values above zero indicate the security is outperforming the benchmark over the specified period.

## Background

> RSMK calculates the log-ratio momentum of a security relative to a benchmark (e.g., SPY). It measures the difference between current log-relative strength and its value N bars ago, then smooths it with an EMA. This approach identifies trends in relative performance with less lag than traditional methods.

## Parameters

- `length` (default: 90): Momentum lookback period
- `ema_length` (default: 3): EMA smoothing period

## Formula


\[
RSMK = EMA(\ln(\frac{P_t}{B_t}) - \ln(\frac{P_{t-n}}{B_{t-n}}), m) \times 100
\]


[Source](TASC March 2020)
