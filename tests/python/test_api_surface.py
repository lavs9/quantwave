"""API surface registry + stub hygiene (quantwave-5ipk.3)."""

from __future__ import annotations

import ast
import inspect
import re
from pathlib import Path

import pytest

import quantwave as qw
from quantwave._metadata_generated import GENERATED_ENTRIES
from quantwave._ta_registry_generated import (
    METADATA_SLUG_COUNT,
    SPECIAL_SYMBOLS,
    TA_REGISTRY,
    UNBOUND_SLUGS,
)

ROOT = Path(__file__).resolve().parents[2]
INIT_PY = ROOT / "quantwave-py" / "python" / "quantwave" / "__init__.py"


def _ta_indicator_attrs() -> set[str]:
    return {
        name
        for name in dir(qw.ta)
        if not name.startswith("_") and name not in SPECIAL_SYMBOLS
    }


def test_registry_covers_all_metadata_slugs():
    assert set(TA_REGISTRY.keys()) == set(GENERATED_ENTRIES.keys())
    assert METADATA_SLUG_COUNT == len(GENERATED_ENTRIES) == 221


def test_no_unbound_slugs():
    assert UNBOUND_SLUGS == ()


def test_ta_namespace_matches_metadata_slugs():
    assert _ta_indicator_attrs() == set(GENERATED_ENTRIES.keys())


def test_special_symbols_on_ta():
    for name in SPECIAL_SYMBOLS:
        assert hasattr(qw.ta, name), f"missing special symbol {name}"


def test_indicators_matches_registry():
    assert set(qw.indicators()) == set(TA_REGISTRY.keys())


def test_pyi_stubs_exist():
    pkg = ROOT / "quantwave-py" / "python" / "quantwave"
    for path in (pkg / "ta.pyi", pkg / "py.typed"):
        assert path.is_file(), f"missing stub marker {path}"


def test_init_has_no_broad_except_pass_in_ta_block():
    text = INIT_PY.read_text(encoding="utf-8")
    tree = ast.parse(text)
    offenders: list[int] = []
    for node in ast.walk(tree):
        if not isinstance(node, ast.ExceptHandler):
            continue
        if node.type is None:
            continue
        if not (isinstance(node.type, ast.Name) and node.type.id == "Exception"):
            continue
        if len(node.body) == 1 and isinstance(node.body[0], ast.Pass):
            # Allow optional submodule import guards outside ta population.
            if node.lineno and node.lineno < 165:
                continue
            if node.lineno and node.lineno > 210:
                continue
            offenders.append(node.lineno)
    assert offenders == [], f"broad except-pass in ta population block: lines {offenders}"


def test_registry_entries_have_surface_binding():
    for slug, entry in TA_REGISTRY.items():
        assert (
            entry.get("native_batch")
            or entry.get("native_streaming")
            or entry.get("polars_method")
        ), slug


def test_registry_native_symbols_resolve_against_build():
    """A declared native symbol must actually exist — a name alone proves nothing."""
    import quantwave._quantwave as _q

    missing = [
        (slug, entry[key])
        for slug, entry in TA_REGISTRY.items()
        for key in ("native_batch", "native_streaming")
        if entry.get(key) and not hasattr(_q, entry[key])
    ]
    assert missing == [], f"registry names absent from build: {missing}"


@pytest.mark.parametrize("slug", ["supertrend", "wavetrend", "frac_diff", "cyber_cycle"])
def test_multiword_indicators_are_batch_functions_not_classes(slug):
    """Regression: pascal_to_snake once made every multi-word slug degrade to a class."""
    obj = getattr(qw, slug)
    assert not inspect.isclass(obj), f"qw.{slug} degraded to a streaming class"


def test_streaming_class_uses_registry_for_rsi():
    cls = qw.streaming_class("rsi")
    assert cls is not None
    assert cls.__name__ in {"Rsi", "RSI"}


def test_no_bead_ids_in_init():
    text = INIT_PY.read_text(encoding="utf-8")
    assert not re.search(r"quantwave-(?=[a-z0-9]*\d)[a-z0-9]{3,6}\b", text)