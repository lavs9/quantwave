#!/usr/bin/env python3
"""Generate explicit Python API registry and .pyi stubs (quantwave-5ipk.3)."""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
METADATA_GEN = ROOT / "quantwave-py" / "python" / "quantwave" / "_metadata_generated.py"
REGISTRY_RS = ROOT / "quantwave-core" / "src" / "indicators" / "metadata_registry.rs"
LIB_RS = ROOT / "quantwave-py" / "src" / "indicators.rs"
PLUGINS_DIR = ROOT / "quantwave-py" / "python" / "quantwave"
ALIASES = ROOT / "scripts" / "api_slug_aliases.json"
OVERLAY = ROOT / "scripts" / "metadata_overlay.json"

OUT_REGISTRY = ROOT / "quantwave-py" / "python" / "quantwave" / "_ta_registry_generated.py"
OUT_TA_PYI = ROOT / "quantwave-py" / "python" / "quantwave" / "ta.pyi"
# Unified crate: plugins ta.pyi is the same file as the package ta.pyi.
OUT_PLUGINS_TA_PYI = OUT_TA_PYI
OUT_TYPED = ROOT / "quantwave-py" / "python" / "quantwave" / "py.typed"

# ML / PA / regime helpers — explicit surface (not metadata slugs).
SPECIAL_SYMBOLS: dict[str, str] = {
    "CyberCycleFeatureExtractor": "CyberCycleFeatureExtractor",
    "HurstFeatureExtractor": "HurstFeatureExtractor",
    "InstantaneousTrendlineFeatureExtractor": "InstantaneousTrendlineFeatureExtractor",
    "TrendflexFeatureExtractor": "TrendflexFeatureExtractor",
    "GriffithsDominantCycleFeatureExtractor": "GriffithsDominantCycleFeatureExtractor",
    "BullBearHMM": "BullBearHmm",
    "GaussianHmmFilterPy": "GaussianHmmFilterPy",
    "GaussianHmmDiagnosticsPy": "GaussianHmmDiagnosticsPy",
    "fit_gaussian_hmm": "fit_gaussian_hmm",
    "gaussian_hmm_diagnostics": "gaussian_hmm_diagnostics",
    "gaussian_hmm_forecast_state": "gaussian_hmm_forecast_state",
    "gaussian_hmm_forecast_vol": "gaussian_hmm_forecast_vol",
    "regime_to_features": "regime_to_features",
    "hurst_features": "hurst_features",
    "cyber_cycle_features": "cyber_cycle_features",
    "trendflex_features": "trendflex_features",
    "instantaneous_trendline_features": "instantaneous_trendline_features",
    "griffiths_dominant_cycle_features": "griffiths_dominant_cycle_features",
    "MarketStructure": "MarketStructure",
    "GeometricPatternScanner": "GeometricPatternScanner",
    "market_structure_batch": "market_structure_batch",
}


def norm(name: str) -> str:
    return name.lower().replace("_", "")


def pascal_to_snake(name: str) -> str:
    if not name:
        return ""
    out: list[str] = []
    for i, ch in enumerate(name):
        if ch.isupper() and i:
            out.append("_")
        out.append(ch.lower())
    return "".join(out)


def load_generated_slugs() -> list[str]:
    text = METADATA_GEN.read_text(encoding="utf-8")
    return sorted(set(re.findall(r'"([a-z][a-z0-9_]*)":\s*\{', text)))


def load_registry_entries() -> dict[str, str]:
    text = REGISTRY_RS.read_text(encoding="utf-8")
    entries: dict[str, str] = {}
    for m in re.finditer(
        r'slug: "([^"]+)",\s*meta: &[^,]+,\s*struct_name: "([^"]*)",\s*source_file:',
        text,
        re.S,
    ):
        entries[m.group(1)] = m.group(2)
    return entries


def load_overlay_aliases() -> dict[str, str]:
    if not OVERLAY.exists():
        return {}
    data = json.loads(OVERLAY.read_text(encoding="utf-8"))
    out: dict[str, str] = {}
    for key, val in data.items():
        if key.startswith("_"):
            continue
        if isinstance(val, dict) and "slug_alias" in val:
            out[key] = val["slug_alias"]
    return out


def load_manual_aliases() -> dict[str, dict[str, Any]]:
    if not ALIASES.exists():
        return {}
    data = json.loads(ALIASES.read_text(encoding="utf-8"))
    return {k: v for k, v in data.items() if not k.startswith("_")}


def parse_native_symbols() -> tuple[set[str], set[str]]:
    text = LIB_RS.read_text(encoding="utf-8")
    batch = set(re.findall(r"^pub fn ([a-z][a-z0-9_]*)\(", text, re.M))
    streaming: set[str] = set()
    # Hand-written streaming classes: a #[pyclass] struct with an `inner:` state field.
    # Honor an explicit #[pyclass(name = "...")] (PyO3 keeps the raw Rust identifier,
    # so the re-cased uniffi-era symbols are pinned via a name attr).
    for m in re.finditer(
        r"#\[pyclass(\([^)]*\))?\]\s*\npub struct (\w+)\s*\{[^}]*?\binner\s*:", text
    ):
        attrs, struct = m.group(1) or "", m.group(2)
        name_attr = re.search(r'name\s*=\s*"([^"]+)"', attrs)
        streaming.add(name_attr.group(1) if name_attr else struct)
    for type_name in re.findall(r"export_[a-z0-9_]+!\s*\(\s*(\w+)", text):
        # uniffi's export_*! macros emit `pub fn [<$name:lower>]` — all-lowercase,
        # no separators. pascal_to_snake would yield `super_trend`, which never exists.
        batch.add(type_name.lower())
        streaming.add(type_name)
    return batch, streaming


def parse_plugin_methods() -> dict[str, str]:
    methods: dict[str, str] = {}
    for path in sorted(PLUGINS_DIR.glob("*.py")):
        text = path.read_text(encoding="utf-8")
        for m in re.finditer(r"^\s+def ([a-zA-Z_][a-zA-Z0-9_]*)\s*\(", text, re.M):
            name = m.group(1)
            if name in {"__init__", "_handle_arg"}:
                continue
            methods.setdefault(norm(name), name)
    return methods


def parse_plugin_signatures() -> list[tuple[str, str]]:
    """Return (method_name, signature_line) from TaNamespace + custom mixins."""
    sigs: list[tuple[str, str]] = []
    seen: set[str] = set()
    for path in sorted(PLUGINS_DIR.glob("*.py")):
        text = path.read_text(encoding="utf-8")
        for m in re.finditer(
            r"^\s+def ([a-zA-Z_][a-zA-Z0-9_]*)\s*(\([^)]*\))\s*->\s*([^:]+):",
            text,
            re.M,
        ):
            name, args, ret = m.group(1), m.group(2), m.group(3).strip()
            if name in {"__init__", "_handle_arg"} or name in seen:
                continue
            seen.add(name)
            sigs.append((name, f"def {name}{args} -> {ret}: ..."))
    return sorted(sigs, key=lambda x: x[0])


def resolve_polars_method(slug: str, plugin_by_norm: dict[str, str], manual: dict[str, Any]) -> str | None:
    if slug in manual and manual[slug].get("polars_method"):
        return manual[slug]["polars_method"]
    if slug in plugin_by_norm.values():
        return slug
    hit = plugin_by_norm.get(norm(slug))
    if hit:
        return hit
    # CDL patterns: insert underscore after cdl prefix
    if slug.startswith("cdl") and len(slug) > 3:
        cdl_variant = "cdl_" + slug[3:]
        hit = plugin_by_norm.get(norm(cdl_variant))
        if hit:
            return hit
    return None


def resolve_native_batch(
    slug: str,
    struct_name: str,
    batch_syms: set[str],
    manual: dict[str, Any],
) -> str | None:
    if slug in manual and manual[slug].get("native_batch"):
        return manual[slug]["native_batch"]
    candidates = [slug]
    if struct_name:
        candidates.append(pascal_to_snake(struct_name))
        candidates.append(struct_name.lower())  # uniffi paste![<$name:lower>]
    for cand in candidates:
        if cand in batch_syms:
            return cand
    for cand in candidates:
        for sym in batch_syms:
            if norm(sym) == norm(cand):
                return sym
    return None


def resolve_native_streaming(
    slug: str,
    struct_name: str,
    stream_syms: set[str],
    manual: dict[str, Any],
) -> str | None:
    if slug in manual and manual[slug].get("native_streaming"):
        return manual[slug]["native_streaming"]
    candidates: list[str] = []
    if struct_name:
        candidates.append(struct_name)
    # PascalCase from slug
    candidates.append("".join(part.capitalize() for part in slug.split("_")))
    for cand in candidates:
        if cand in stream_syms:
            return cand
    for cand in candidates:
        for sym in stream_syms:
            if norm(sym) == norm(cand):
                return sym
    return None


def build_registry() -> tuple[dict[str, dict[str, Any]], list[str]]:
    slugs = load_generated_slugs()
    struct_by_slug = load_registry_entries()
    overlay_alias = load_overlay_aliases()
    manual = load_manual_aliases()
    batch_syms, stream_syms = parse_native_symbols()
    plugin_by_norm = parse_plugin_methods()

    registry: dict[str, dict[str, Any]] = {}
    unbound: list[str] = []

    for slug in slugs:
        canonical = overlay_alias.get(slug, slug)
        if canonical in manual and manual[canonical].get("slug_alias"):
            canonical = manual[canonical]["slug_alias"]
        struct_name = struct_by_slug.get(slug, "")
        entry = {
            "slug": slug,
            "polars_method": resolve_polars_method(canonical, plugin_by_norm, manual)
            or resolve_polars_method(slug, plugin_by_norm, manual),
            "native_batch": resolve_native_batch(canonical, struct_name, batch_syms, manual)
            or resolve_native_batch(slug, struct_name, batch_syms, manual),
            "native_streaming": resolve_native_streaming(canonical, struct_name, stream_syms, manual)
            or resolve_native_streaming(slug, struct_name, stream_syms, manual),
        }
        if not any(entry[k] for k in ("polars_method", "native_batch", "native_streaming")):
            unbound.append(slug)
        registry[slug] = entry
    return registry, unbound


def render_registry_py(registry: dict[str, dict[str, Any]], unbound: list[str]) -> str:
    lines = [
        '"""AUTO-GENERATED by scripts/generate_api_stubs.py — do not edit by hand.',
        "Re-run: `python scripts/generate_api_stubs.py`",
        '"""',
        "",
        "from __future__ import annotations",
        "",
        "from typing import Any, TypedDict",
        "",
        "",
        "class TaRegistryEntry(TypedDict, total=False):",
        "    slug: str",
        "    polars_method: str | None",
        "    native_batch: str | None",
        "    native_streaming: str | None",
        "",
        "",
        "TA_REGISTRY: dict[str, TaRegistryEntry] = {",
    ]
    for slug, entry in registry.items():
        parts = [f'    "{slug}": {{', f'        "slug": "{slug}",']
        for key in ("polars_method", "native_batch", "native_streaming"):
            val = entry.get(key)
            if val:
                parts.append(f'        "{key}": "{val}",')
            else:
                parts.append(f'        "{key}": None,')
        parts.append("    },")
        lines.extend(parts)
    lines.append("}")
    lines.append("")
    lines.append(f"METADATA_SLUG_COUNT: int = {len(registry)}")
    lines.append("")
    lines.append("UNBOUND_SLUGS: tuple[str, ...] = (")
    for slug in unbound:
        lines.append(f'    "{slug}",')
    lines.append(")")
    lines.append("")
    lines.append("SPECIAL_SYMBOLS: dict[str, str] = {")
    for pub, native in SPECIAL_SYMBOLS.items():
        lines.append(f'    "{pub}": "{native}",')
    lines.append("}")
    lines.append("")
    return "\n".join(lines)


def render_plugins_ta_pyi(signatures: list[tuple[str, str]]) -> str:
    lines = [
        '"""Type stubs for ``pl.col("x").ta`` — generated by scripts/generate_api_stubs.py."""',
        "",
        "from __future__ import annotations",
        "",
        "import polars as pl",
        "",
        "",
        "class TaNamespace:",
        "    _expr: pl.Expr",
        "    def __init__(self, expr: pl.Expr) -> None: ...",
        "",
    ]
    for _, sig in signatures:
        lines.append(f"    {sig}")
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def render_quantwave_ta_pyi(registry: dict[str, dict[str, Any]]) -> str:
    lines = [
        '"""Type stubs for ``quantwave.ta`` — generated by scripts/generate_api_stubs.py."""',
        "",
        "from __future__ import annotations",
        "",
        "from typing import Any, Callable, Type",
        "",
        "",
        "class ta:",
        '    """Explicit indicator namespace (one attribute per metadata slug)."""',
        "",
    ]
    for slug in sorted(registry):
        lines.append(f"    {slug}: Any")
    for pub in sorted(SPECIAL_SYMBOLS):
        lines.append(f"    {pub}: Any")
    lines.append("")
    return "\n".join(lines)


def main() -> None:
    registry, unbound = build_registry()
    OUT_REGISTRY.write_text(render_registry_py(registry, unbound), encoding="utf-8")
    OUT_TA_PYI.write_text(render_quantwave_ta_pyi(registry), encoding="utf-8")
    # Unified crate: OUT_PLUGINS_TA_PYI == OUT_TA_PYI, so no separate plugins stub write.
    OUT_TYPED.write_text("", encoding="utf-8")
    print(f"Wrote TA registry ({len(registry)} slugs, {len(unbound)} unbound) -> {OUT_REGISTRY}")
    print(f"Wrote stubs -> {OUT_TA_PYI}")
    if unbound:
        print("Unbound slugs:", ", ".join(unbound[:12]), ("..." if len(unbound) > 12 else ""))


if __name__ == "__main__":
    main()