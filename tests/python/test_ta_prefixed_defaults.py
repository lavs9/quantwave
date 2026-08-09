"""The ``ta_``-prefixed plugins must default to TA-Lib's per-function periods.

Regression cover for a bug where the generated signatures all carried a blanket
``timeperiod=14``. TA-Lib does not use one uniform default: ``BETA`` is 5 and
``CORREL`` is 30, while the LINEARREG family, TSF, ATR and NATR are 14. So
``.ta.ta_correl(other)`` silently computed over 14 bars where TA-Lib and the
non-prefixed sibling ``.ta.correl(other)`` use 30, and ``.ta.ta_beta(other)``
over 14 where TA-Lib and ``.ta.beta(other)`` use 5. The formulas were correct;
only the default diverged, on the one surface whose prefix promises TA-Lib
fidelity.

Assertions are anchored to values computed independently in this file — a
Pearson correlation and TA-Lib's own BETA accumulation, both written out
longhand here — rather than to whatever the implementation currently returns.
"""

import inspect
import math

import pytest

pl = pytest.importorskip("polars")
import quantwave  # noqa: F401,E402  (registers the .ta namespace)
from quantwave._ta_namespace import TaNamespace  # noqa: E402


# TA-Lib's own optInTimePeriod defaults for every ta_-prefixed method that takes
# one. ta_trange takes no period and is therefore absent.
TALIB_DEFAULTS = {
    "ta_atr": 14,
    "ta_beta": 5,
    "ta_correl": 30,
    "ta_linearreg": 14,
    "ta_linearreg_angle": 14,
    "ta_linearreg_intercept": 14,
    "ta_linearreg_slope": 14,
    "ta_natr": 14,
    "ta_tsf": 14,
}


def _pair(n: int = 200):
    """Two correlated-but-distinct random walks.

    Deliberately not ``high = close * k``: a scalar multiple correlates at
    exactly 1.0 at every period, which would make a period-sensitive assertion
    pass no matter what default were used.
    """
    import random

    random.seed(11)
    a = [100.0]
    b = [50.0]
    for _ in range(n - 1):
        shock = random.gauss(0, 0.01)
        a.append(round(a[-1] * (1 + shock), 4))
        b.append(round(b[-1] * (1 + 0.6 * shock + 0.4 * random.gauss(0, 0.01)), 4))
    return pl.DataFrame({"a": a, "b": b})


def _pearson(x, y, period):
    """Pearson correlation over the last ``period`` observations."""
    xs, ys = x[-period:], y[-period:]
    mx = sum(xs) / period
    my = sum(ys) / period
    cov = sum((u - mx) * (v - my) for u, v in zip(xs, ys))
    var_x = sum((u - mx) ** 2 for u in xs)
    var_y = sum((v - my) ** 2 for v in ys)
    return cov / math.sqrt(var_x * var_y)


def _talib_beta(x, y, period):
    """TA-Lib BETA: least-squares slope of ``period`` simple-return pairs."""
    rx, ry = [], []
    for i in range(len(x) - period, len(x)):
        rx.append(x[i] / x[i - 1] - 1.0)
        ry.append(y[i] / y[i - 1] - 1.0)
    sx, sy = sum(rx), sum(ry)
    sxy = sum(u * v for u, v in zip(rx, ry))
    sxx = sum(u * u for u in rx)
    return (period * sxy - sx * sy) / (period * sxx - sx * sx)


class TestSignatureDefaults:
    """The declared defaults themselves, before any data is involved."""

    @pytest.mark.parametrize("method,expected", sorted(TALIB_DEFAULTS.items()))
    def test_default_matches_talib(self, method, expected):
        param = inspect.signature(getattr(TaNamespace, method)).parameters["timeperiod"]
        assert param.default == expected, (
            f".ta.{method} defaults to timeperiod={param.default}, "
            f"TA-Lib uses {expected}"
        )

    def test_not_every_ta_default_is_the_same(self):
        """A blanket default is exactly the bug; assert the set is not uniform."""
        assert len(set(TALIB_DEFAULTS.values())) > 1
        defaults = {
            name: inspect.signature(getattr(TaNamespace, name))
            .parameters["timeperiod"]
            .default
            for name in TALIB_DEFAULTS
        }
        assert len(set(defaults.values())) > 1, (
            f"all ta_* defaults collapsed to one value again: {defaults}"
        )

    def test_every_period_taking_ta_method_is_covered(self):
        """A new ta_* plugin must be added to this table, not left to drift."""
        found = set()
        for name in dir(TaNamespace):
            if not name.startswith("ta_"):
                continue
            try:
                params = inspect.signature(getattr(TaNamespace, name)).parameters
            except (TypeError, ValueError):
                continue
            if "timeperiod" in params:
                found.add(name)
        assert found == set(TALIB_DEFAULTS), (
            f"uncovered: {sorted(found - set(TALIB_DEFAULTS))}, "
            f"stale: {sorted(set(TALIB_DEFAULTS) - found)}"
        )

    def test_ta_trange_takes_no_period(self):
        params = inspect.signature(TaNamespace.ta_trange).parameters
        assert "timeperiod" not in params


class TestBareCallMatchesTalib:
    """Called with no period, the value must be TA-Lib's."""

    def test_ta_correl_bare_is_the_30_bar_correlation(self):
        df = _pair()
        expected = _pearson(df["a"].to_list(), df["b"].to_list(), 30)
        got = (
            df.lazy()
            .with_columns(pl.col("a").ta.ta_correl("b").alias("x"))
            .collect()["x"][-1]
        )
        assert got == pytest.approx(expected, abs=1e-9)

    def test_ta_beta_bare_is_the_5_bar_beta(self):
        df = _pair()
        expected = _talib_beta(df["a"].to_list(), df["b"].to_list(), 5)
        got = (
            df.lazy()
            .with_columns(pl.col("a").ta.ta_beta("b").alias("x"))
            .collect()["x"][-1]
        )
        assert got == pytest.approx(expected, abs=1e-9)

    def test_ta_correl_bare_is_not_the_old_14_bar_value(self):
        """Guards against a silent revert to the blanket default."""
        df = _pair()
        old = _pearson(df["a"].to_list(), df["b"].to_list(), 14)
        got = (
            df.lazy()
            .with_columns(pl.col("a").ta.ta_correl("b").alias("x"))
            .collect()["x"][-1]
        )
        assert not math.isclose(got, old, abs_tol=1e-6)

    def test_ta_beta_bare_is_not_the_old_14_bar_value(self):
        df = _pair()
        old = _talib_beta(df["a"].to_list(), df["b"].to_list(), 14)
        got = (
            df.lazy()
            .with_columns(pl.col("a").ta.ta_beta("b").alias("x"))
            .collect()["x"][-1]
        )
        assert not math.isclose(got, old, abs_tol=1e-6)


class TestTwinsAgreeWithNoPeriod:
    """The point of the fix: bare twin calls must now return the same number."""

    def test_correl_twins_agree_bare(self):
        df = _pair()
        out = (
            df.lazy()
            .with_columns(
                pl.col("a").ta.ta_correl("b").alias("prefixed"),
                pl.col("a").ta.correl("b").alias("sibling"),
            )
            .collect()
        )
        assert out["prefixed"][-1] == pytest.approx(out["sibling"][-1], abs=1e-12)

    def test_beta_twins_agree_bare(self):
        df = _pair()
        out = (
            df.lazy()
            .with_columns(
                pl.col("a").ta.ta_beta("b").alias("prefixed"),
                pl.col("a").ta.beta("b").alias("sibling"),
            )
            .collect()
        )
        assert out["prefixed"][-1] == pytest.approx(out["sibling"][-1], abs=1e-12)

    @pytest.mark.parametrize("slug", ["beta", "correl"])
    def test_twin_defaults_are_identical(self, slug):
        plain = inspect.signature(getattr(TaNamespace, slug)).parameters["timeperiod"]
        prefixed = inspect.signature(
            getattr(TaNamespace, f"ta_{slug}")
        ).parameters["timeperiod"]
        assert plain.default == prefixed.default


class TestGeneratorCannotReintroduceBlanketDefault:
    """The generated file is downstream of scripts/gen_pyo3_plugins_py.py."""

    def _generator_source(self):
        from pathlib import Path

        root = Path(__file__).resolve().parents[2]
        path = root / "scripts" / "gen_pyo3_plugins_py.py"
        if not path.exists():
            pytest.skip("generator script not present in this checkout")
        return path.read_text()

    def test_templates_do_not_hardcode_a_default(self):
        src = self._generator_source()
        assert "timeperiod: int = 14" not in src, (
            "generator template still hard-codes a blanket default; it must call "
            "default_timeperiod(m) so per-function values drive the signature"
        )

    def test_generator_table_matches_the_shipped_defaults(self):
        # Parse the table literally rather than executing the generator, which
        # reads Rust sources and appends to the shipped file as a side effect.
        import ast

        src = self._generator_source()

        tree = ast.parse(src)
        tables = {
            node.targets[0].id: ast.literal_eval(node.value)
            for node in tree.body
            if isinstance(node, ast.Assign)
            and isinstance(node.targets[0], ast.Name)
            and node.targets[0].id.endswith("_DEFAULT_TIMEPERIOD")
        }
        assert tables.get("TALIB_DEFAULT_TIMEPERIOD") == TALIB_DEFAULTS
