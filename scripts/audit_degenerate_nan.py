#!/usr/bin/env python3
"""
Phase 0 degenerate-input NaN/Inf audit for quantwave streaming indicators.
See br issue quantwave-1jqv (Phase 0 design) for full context.

READ-ONLY AUDIT TOOL. Does not modify any indicator source. Produces
docs/generated/degenerate_nan_audit.json.

This is a throwaway/reusable harness written to the scratchpad per the task
instructions; not committed to the repo. Flagged in the final report for
human review on whether it's worth keeping as `scripts/audit_degenerate_nan.py`.
"""
import json
import math
import re
import sys
import datetime
import traceback

import quantwave as qw
from quantwave._metadata_generated import GENERATED_ENTRIES as G

# ---------------------------------------------------------------------------
# Construction layer -- metadata's optional_params/required_params keys and
# types don't always match the real PyO3 constructor. See report for the
# full list of mismatches found. These overrides were reverse-engineered by
# probing the real TypeErrors at runtime.
# ---------------------------------------------------------------------------

REQUIRED_PARAM_DEFAULTS = {
    'market_structure': {'swing_strength': 5},
    'rsi': {'period': 14},
    'supertrend': {'period': 10, 'multiplier': 3.0},
}

POSITIONAL_OVERRIDE = {
    'hamming_filter', 'hann_filter', 'hurst_exponent', 'ichimoku',
    'kalman_filter', 'mesa_stochastic', 'my_rsi', 'noise_elimination',
    'projected_moving_average', 'recursive_median', 'reversion_index',
    'ttm_squeeze', 'undersampled_double_ma',
}


def _ctor_keltner():
    return qw.streaming_class('keltner')(ema_period=20, atr_period=20, multiplier=1.5)


def _ctor_frac_diff():
    return qw.streaming_class('frac_diff')(d=0.4, threshold=1e-5)


def _ctor_swiss_army_knife():
    return qw.streaming_class('swiss_army_knife')(mode=qw.SwissMode.BandPass, period=20, delta=0.1)


def _ctor_kama():
    return qw.streaming_class('kama')(10)


BESPOKE_CTOR = {
    'keltner': _ctor_keltner,
    'frac_diff': _ctor_frac_diff,
    'swiss_army_knife': _ctor_swiss_army_knife,
    'kama': _ctor_kama,
}

NO_SANE_DEFAULT = {
    'gaussian_hmm': (
        "GaussianHmmFilterPy has no __init__; requires .from_params(...) "
        "with pretrained HMM parameter arrays (transition/emission matrices) "
        "that have no sane synthetic default for a degenerate-input sweep."
    ),
}


def build_kwargs(name):
    meta = G[name]
    kwargs = dict(meta['optional_params'])
    kwargs.update(REQUIRED_PARAM_DEFAULTS.get(name, {}))
    for p in meta['required_params']:
        if p not in kwargs:
            kwargs[p] = 1
    fixed = {}
    for k, v in kwargs.items():
        if isinstance(v, str):
            try:
                v2 = float(v)
                if v2.is_integer() and '.' not in v and 'e' not in v.lower():
                    v2 = int(v2)
                v = v2
            except ValueError:
                pass
        fixed[k] = v
    return fixed


def make_instance(name):
    if name in NO_SANE_DEFAULT:
        return None, NO_SANE_DEFAULT[name]
    cls = qw.streaming_class(name)
    if cls is None:
        return None, "no streaming implementation (quantwave.streaming_class returned None)"
    if name in BESPOKE_CTOR:
        try:
            return BESPOKE_CTOR[name](), None
        except (KeyboardInterrupt, SystemExit):
            raise
        except BaseException as e:
            return None, f"bespoke ctor failed: {type(e).__name__}: {e}"
    kwargs = build_kwargs(name)
    if name in POSITIONAL_OVERRIDE:
        vals = list(kwargs.values())
        try:
            return cls(*vals), None
        except (KeyboardInterrupt, SystemExit):
            raise
        except BaseException as e:
            return None, f"positional ctor failed with {vals}: {type(e).__name__}: {e}"
    try:
        return cls(**kwargs), None
    except (KeyboardInterrupt, SystemExit):
        raise
    except BaseException as e:
        return None, f"kwarg ctor failed with {kwargs}: {type(e).__name__}: {e}"


def probe_arity(instance):
    try:
        instance.next()
        return [], None
    except TypeError as e:
        msg = str(e)
        m = re.search(r'missing (\d+) required positional argument', msg)
        if not m:
            return None, f"unexpected TypeError shape: {msg}"
        names = re.findall(r"'(\w+)'", msg)
        return names, None
    except (KeyboardInterrupt, SystemExit):
        raise
    except BaseException as e:
        return None, f"unexpected exception on 0-arg probe: {type(e).__name__}: {e}"


def warmup_or_proxy(name):
    meta = G[name]
    wb = meta['warmup_bars']
    if wb is not None:
        return wb, False
    kwargs = build_kwargs(name)
    ints = [v for v in kwargs.values() if isinstance(v, int) and not isinstance(v, bool) and v > 0]
    if ints:
        return max(ints), True
    return 50, True


# ---------------------------------------------------------------------------
# Scenario generation. `arity` is the ordered list of real .next() parameter
# names (recovered at runtime, NOT trusted from metadata's data_inputs,
# which is wrong for the vast majority of multi-input indicators -- see
# report). Every scenario builds a dict of per-field arrays of length N,
# and next() args are assembled by looking up each arity name in that dict.
# ---------------------------------------------------------------------------

N = 250


def _fill(fields, n, close, open_=None, high=None, low=None, volume=None, anchor=None):
    return {
        'input': list(close),
        'price': list(close),
        'close': list(close),
        'open': list(open_ if open_ is not None else close),
        'high': list(high if high is not None else close),
        'low': list(low if low is not None else close),
        'volume': list(volume if volume is not None else [1000.0 + i for i in range(n)]),
        'anchor': list(anchor if anchor is not None else [False] * n),
    }


def scenario_constant_price(n=N):
    close = [100.0] * n
    return _fill(None, n, close)


def scenario_zero_volume(n=N):
    close = [100.0 + 2.0 * math.sin(i * 0.1) for i in range(n)]
    high = [c + 0.5 for c in close]
    low = [c - 0.5 for c in close]
    open_ = [c - 0.1 for c in close]
    volume = [0.0] * n
    return _fill(None, n, close, open_, high, low, volume)


def scenario_zero_range_ohlc(n=N):
    close = [100.0 + 2.0 * math.sin(i * 0.1) for i in range(n)]
    return _fill(None, n, close, close, close, close)


def scenario_flat_then_spike(n=N, spike_idx=200):
    close = [100.0] * n
    high = [100.0] * n
    low = [100.0] * n
    open_ = [100.0] * n
    volume = [1000.0] * n
    close[spike_idx] = 1000.0
    high[spike_idx] = 1000.0
    low[spike_idx] = 100.0
    open_[spike_idx] = 100.0
    volume[spike_idx] = 100000.0
    return _fill(None, n, close, open_, high, low, volume)


def scenario_all_zero(n=N):
    close = [0.0] * n
    volume = [0.0] * n
    return _fill(None, n, close, close, close, close, volume)


def scenario_minimal_length(n):
    close = [100.0 + 5.0 * math.sin(i * 0.2) for i in range(n)]
    high = [c + 0.3 for c in close]
    low = [c - 0.3 for c in close]
    open_ = [c - 0.2 for c in close]
    volume = [1000.0 + 10.0 * i for i in range(n)]
    return _fill(None, n, close, open_, high, low, volume)


def scenario_nan_passthrough(n=N, nan_idx=100, primary='close'):
    close = [100.0 + 5.0 * math.sin(i * 0.2) for i in range(n)]
    high = [c + 0.3 for c in close]
    low = [c - 0.3 for c in close]
    open_ = [c - 0.2 for c in close]
    volume = [1000.0 + 10.0 * i for i in range(n)]
    fields = _fill(None, n, close, open_, high, low, volume)
    if primary in fields:
        fields[primary][nan_idx] = float('nan')
    else:
        fields['close'][nan_idx] = float('nan')
    return fields


# ---------------------------------------------------------------------------
# Output extraction: handle plain float/int returns, tuples, and PyO3
# result-wrapper objects (which may have nested sub-objects). Extraction is
# generic (by real attribute names) rather than trusting metadata's
# declared `outputs` list, which does not match the real attribute names
# for several multi-output indicators (see report -- e.g. ht_phasor
# ('inphase'->'in_phase'), supertrend ('supertrend'->'value'), ttm_squeeze
# ('histogram'/'is_squeezed' -> 'value'/'direction'), market_structure
# (12 declared outputs vs 6 real top-level attrs with nested sub-objects).
# ---------------------------------------------------------------------------

def extract_fields(result, prefix=''):
    out = {}
    if result is None:
        return out
    if isinstance(result, bool):
        return out
    if isinstance(result, (int, float)):
        out[prefix or 'value'] = float(result)
        return out
    if isinstance(result, tuple):
        for i, v in enumerate(result):
            out.update(extract_fields(v, f'{prefix}out{i}' if not prefix else f'{prefix}.out{i}'))
        return out
    # object with attributes (PyO3 result wrapper)
    for a in dir(result):
        if a.startswith('_'):
            continue
        try:
            v = getattr(result, a)
        except Exception:
            continue
        if callable(v):
            continue
        key = f'{prefix}.{a}' if prefix else a
        if isinstance(v, bool):
            continue
        if isinstance(v, (int, float)):
            out[key] = float(v)
        elif hasattr(v, '__class__') and v.__class__.__module__ == 'builtins' and not isinstance(v, (str, list, dict)):
            # one level of recursion into nested result structs
            out.update(extract_fields(v, key))
    return out


def run_series(instance, arity, fields, n):
    """Feed n bars; return list of per-bar output dicts, or raise with index info."""
    per_bar = []
    for i in range(n):
        args = [fields[name][i] for name in arity]
        try:
            r = instance.next(*args)
        except (KeyboardInterrupt, SystemExit):
            raise
        except BaseException as e:
            # Rust panics surface as pyo3_runtime.PanicException, which does
            # NOT subclass Exception -- must catch BaseException to trap it.
            raise RuntimeError(f"exception at bar {i}: {type(e).__name__}: {e}") from e
        per_bar.append(extract_fields(r))
    return per_bar


def analyze(per_bar, warmup, field_names):
    """Return dict field_name -> finding or None."""
    findings = {}
    n = len(per_bar)
    for fname in field_names:
        first_bad = None
        for i in range(warmup, n):
            v = per_bar[i].get(fname)
            if v is None:
                continue
            if math.isnan(v) or math.isinf(v):
                first_bad = i
                break
        if first_bad is None:
            continue
        # contiguous bad run starting at first_bad
        run_len = 0
        recovered = False
        for i in range(first_bad, n):
            v = per_bar[i].get(fname)
            bad = v is not None and (math.isnan(v) or math.isinf(v))
            if bad:
                run_len += 1
            else:
                recovered = True
                break
        findings[fname] = {
            'first_bad_index': first_bad,
            'bars_bad_post_warmup': run_len,
            'recovered_before_series_end': recovered,
        }
    return findings


SCENARIOS = ['constant_price', 'zero_volume', 'zero_range_ohlc', 'flat_then_spike',
             'all_zero', 'minimal_length', 'nan_passthrough']


def applicable_scenarios(arity):
    has_vol = 'volume' in arity
    has_ohlc_shape = len({'open', 'high', 'low', 'close'} & set(arity)) >= 2
    scenarios = ['constant_price', 'flat_then_spike', 'all_zero', 'minimal_length', 'nan_passthrough']
    if has_vol:
        scenarios.append('zero_volume')
    if has_ohlc_shape:
        scenarios.append('zero_range_ohlc')
    return scenarios


def main():
    findings = []
    could_not_instantiate = []
    exceptions = []
    tested = []
    clean = []
    unusual_notes = []

    for name in sorted(G.keys()):
        meta = G[name]
        inst0, err = make_instance(name)
        if err:
            could_not_instantiate.append({'indicator': name, 'reason': err})
            continue
        names, aerr = probe_arity(inst0)
        if aerr:
            could_not_instantiate.append({'indicator': name, 'reason': f"arity probe failed: {aerr}"})
            continue
        arity = names
        warmup, is_proxy = warmup_or_proxy(name)

        tested.append(name)
        had_finding = False
        had_exception = False

        for scenario in applicable_scenarios(arity):
            # fresh instance per scenario
            inst, cerr = make_instance(name)
            if cerr:
                continue
            if scenario == 'minimal_length':
                n = max(warmup, 1)
                fields = scenario_minimal_length(n)
            elif scenario == 'nan_passthrough':
                primary = 'close' if 'close' in arity else ('input' if 'input' in arity else arity[0])
                fields = scenario_nan_passthrough(primary=primary)
                n = N
            else:
                fields = globals()[f'scenario_{scenario}']()
                n = N

            try:
                per_bar = run_series(inst, arity, fields, n)
            except RuntimeError as e:
                had_exception = True
                m = re.search(r'exception at bar (\d+): (.*)', str(e))
                bar_idx = int(m.group(1)) if m else None
                msg = m.group(2) if m else str(e)
                findings.append({
                    'indicator': name,
                    'category': meta['category'],
                    'scenario': scenario,
                    'output_field': None,
                    'first_bad_index': bar_idx,
                    'bars_bad_post_warmup': None,
                    'recovered_before_series_end': None,
                    'exception_message': msg,
                })
                continue
            except Exception as e:
                had_exception = True
                findings.append({
                    'indicator': name,
                    'category': meta['category'],
                    'scenario': scenario,
                    'output_field': None,
                    'first_bad_index': None,
                    'bars_bad_post_warmup': None,
                    'recovered_before_series_end': None,
                    'exception_message': f"{type(e).__name__}: {e}",
                })
                continue

            # union of field names across all bars (some result objects may
            # vary in populated keys but shouldn't in practice)
            field_names = set()
            for b in per_bar:
                field_names |= set(b.keys())
            eff_warmup = warmup if scenario != 'minimal_length' else 0
            per_field_findings = analyze(per_bar, eff_warmup, field_names)
            for fname, res in per_field_findings.items():
                had_finding = True
                findings.append({
                    'indicator': name,
                    'category': meta['category'],
                    'scenario': scenario,
                    'output_field': fname,
                    'first_bad_index': res['first_bad_index'],
                    'bars_bad_post_warmup': res['bars_bad_post_warmup'],
                    'recovered_before_series_end': res['recovered_before_series_end'],
                    'exception_message': None,
                })

        if had_exception:
            exceptions.append(name)
        if not had_finding and not had_exception:
            clean.append(name)

    summary = {
        'total_indicators_in_metadata': len(G),
        'total_indicators_tested': len(tested),
        'total_could_not_instantiate': len(could_not_instantiate),
        'total_clean': len(clean),
        'total_with_at_least_one_finding': len(set(f['indicator'] for f in findings if f['exception_message'] is None)),
        'total_that_raised_exceptions': len(exceptions),
    }

    out = {
        'generated_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'summary': summary,
        'findings': findings,
        'could_not_instantiate': could_not_instantiate,
    }

    with open('docs/generated/degenerate_nan_audit.json', 'w') as f:
        json.dump(out, f, indent=2)

    print(json.dumps(summary, indent=2))
    print("could_not_instantiate:", len(could_not_instantiate))
    print("findings:", len(findings))


if __name__ == '__main__':
    main()
