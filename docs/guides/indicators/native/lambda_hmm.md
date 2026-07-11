# lambda_hmm

<div class="indicator-meta"><span class="category-badge">Regime</span> <span class="kw-badge">regime</span> <span class="kw-badge">hmm</span> <span class="kw-badge">lambda</span> <span class="kw-badge">ecld</span> <span class="kw-badge">ldhmm</span> <span class="kw-badge">leptokurtic</span></div>

Lambda-distribution (ecld) emission HMM for leptokurtic returns — ldhmm parity mode.

## Visual Example

![Lambda HMM — leptokurtic emission schematic](../../../assets/indicator-previews/supertrend.png)

*Lambda (ecld) emissions use β=2/λ generalized-normal tails; λ>1 improves fit on leptokurtic returns (`hmm_lambda_2state.json`).*

## Description

**Lambda-distribution HMM** mode for leptokurtic financial returns — the core differentiator of the ldhmm package (Lihn, SSRN 2979516). Each state emits from a symmetric exponential-power density with `(μ, σ, λ)`; **λ=1** reduces to Gaussian, nesting the [gaussian_hmm](../gaussian_hmm/) mode.

Enable via `fit_lambdas=True` on `fit_gaussian_hmm` / `.ta().hmm_fit(..., fit_lambdas=True)`. The M-step alternates profile likelihood updates for λ and σ per state while EM refits transitions and means. Use when return series show excess kurtosis that degrades Gaussian HMM fit quality.

Validated against `hmm_lambda_2state.json` (generic 2-state fixture). Pairs with [hmm_forecast](../hmm_forecast/) for mixture volatility forecasts used in ldhmm vol studies.

## Formula / Specification

**Implementation** (`quantwave-core/src/regimes/gaussian_hmm.rs`):

See the `Next<T>` implementation and `METADATA` in the core module.

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/hmm_lambda_2state.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `n_states` | 2 | Number of latent states. |
| `max_iter` | 100 | Maximum EM iterations. |
| `fit_lambdas` | true | Estimate per-state λ in the M-step. |


## Usage Examples

Same API as [gaussian_hmm](../gaussian_hmm/) with `fit_lambdas=True` (lambda / ecld emissions, ldhmm parity).

**Polars**

```python
df = (
    df.lazy()
    .ta()
    .hmm_fit("returns", n_states=2, max_iter=100, fit_lambdas=True)
    .collect()
)
```

**Python**

```python
import quantwave as qw

fit = qw.fit_gaussian_hmm(returns, n_states=2, max_iter=100, fit_lambdas=True)
print(fit.params.lambdas)  # per-state λ (≥ 1 for leptokurtic tails)
```

## Edge Cases & Limitations

- Warm-up: first `2` bars may return NaN or partial state per implementation.
- Parameter sensitivity: smaller periods increase noise; larger periods increase lag.
- Sudden gaps or bad ticks can distort rolling windows — consider pre-filtering.
- Single-series indicators ignore volume unless otherwise documented.
- Validated via proptests against gold-standard vectors where available.
- No look-ahead bias; streaming and Polars batch paths are bit-identical.

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars return NaN until warmup_bars is satisfied. |
| period > len | When period exceeds series length, output is all NaN. |
| NaN inputs | NaN in input propagates to output (NaN out). |
| Invalid params | Non-positive period or missing required params raise ValueError. |
| Empty data | Empty input returns an empty result series. |

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](../supertrend/)

## Sources & References

**Primary Source**: references/ldhmm/ssrn-2979516.pdf; references/ldhmm/ldhmm-cran-reference.pdf

**Implementation**: `quantwave-core/src/indicators/gaussian_hmm.rs` (`LAMBDA_HMM` / `LAMBDA_HMM_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/hmm_lambda_2state.json`

**Provenance**: Standards bulk upgrade 2026-07-04 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
