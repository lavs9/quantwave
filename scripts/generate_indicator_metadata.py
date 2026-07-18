#!/usr/bin/env python3
"""
Generate Python indicator metadata from Rust IndicatorMetadata registry.

Pipeline:
  1. scripts/regenerate_metadata_registry.py  (Rust registry from *_METADATA consts)
  2. cargo run -p quantwave-core --bin export_metadata  (JSON on stdout)
  3. This script  -> quantwave-py/python/quantwave/_metadata_generated.py

Hand-curated overrides live in scripts/metadata_overlay.json (data_inputs, outputs,
warmup_bars, slug aliases). The main _metadata.py merges generated + hand entries.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "quantwave-py" / "python" / "quantwave" / "_metadata_generated.py"
OVERLAY = ROOT / "scripts" / "metadata_overlay.json"

# Rust param names -> Python param names
PARAM_ALIASES = {
    "timeperiod": "period",
    "fastperiod": "fast",
    "slowperiod": "slow",
    "signalperiod": "signal",
    "fastk_period": "fastk",
    "slowk_period": "slowk",
    "slowd_period": "slowd",
    "nbdevup": "std_dev",
    "nbdevdn": "std_dev",
}


def run_export() -> list[dict[str, Any]]:
    proc = subprocess.run(
        ["cargo", "run", "-p", "quantwave-core", "--bin", "export_metadata"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=True,
    )
    return json.loads(proc.stdout)


def load_overlay() -> dict[str, dict[str, Any]]:
    if not OVERLAY.exists():
        return {}
    data = json.loads(OVERLAY.read_text(encoding="utf-8"))
    return {k: v for k, v in data.items() if not k.startswith("_")}


def normalize_param(name: str) -> str:
    return PARAM_ALIASES.get(name, name)


def parse_int_default(val: str) -> Any:
    try:
        if "." in val:
            return float(val)
        return int(val)
    except ValueError:
        return val


def warmup_from_params(params: list[dict], overlay: dict | None) -> int | None:
    if overlay and "warmup_bars" in overlay:
        return overlay["warmup_bars"]
    nums = []
    for p in params:
        v = parse_int_default(p.get("default", "0"))
        if isinstance(v, (int, float)):
            nums.append(int(v))
    return max(nums) if nums else None


def indicator_meta_from_rust(entry: dict, overlay: dict | None) -> dict[str, Any]:
    slug = entry["slug"]
    if overlay and "slug_alias" in overlay:
        slug = overlay["slug_alias"]

    params = entry.get("params") or []
    required: list[str] = []
    optional: dict[str, Any] = {}

    if overlay:
        required = list(overlay.get("required_params", []))
        optional.update(overlay.get("optional_params", {}))

    for p in params:
        pname = normalize_param(p["name"])
        default = parse_int_default(p.get("default", "0"))
        if pname in required:
            continue
        if pname not in optional:
            optional[pname] = default

    data_inputs = (overlay or {}).get("data_inputs", ["close"])
    outputs = (overlay or {}).get("outputs", [slug])
    warmup = warmup_from_params(params, overlay)
    category = entry.get("category") or "Other"
    desc = entry.get("description") or entry.get("name", slug)
    
    boundary_docs = (
        "\n\nBoundary Conditions & Error Behavior:\n"
        "- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n"
        "- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n"
        "- Negative Params: Negative period/length parameters will raise a ValueError."
    )
    desc += boundary_docs

    # Faithful raw param defaults (un-aliased, unfiltered by required/optional) —
    # keyed by the actual Rust/`.ta` param name so the abstract introspection
    # layer can supply an authoritative default for any param the `.ta` signature
    # leaves required. `optional_params` above drops required params and renames
    # via PARAM_ALIASES, so it cannot serve this purpose.
    param_defaults = {
        p["name"]: parse_int_default(p.get("default", "0"))
        for p in params
        if p.get("default", "") not in ("", None)
    }

    return {
        "slug": slug,
        "required_params": required,
        "optional_params": optional,
        "param_defaults": param_defaults,
        "data_inputs": data_inputs,
        "outputs": outputs,
        "warmup_bars": warmup,
        "category": category,
        "description": desc,
    }


def render_python(items: dict[str, dict[str, Any]]) -> str:
    lines = [
        '"""',
        "AUTO-GENERATED from Rust IndicatorMetadata by scripts/generate_indicator_metadata.py",
        "Do not edit by hand. Re-run: python scripts/generate_indicator_metadata.py",
        '"""',
        "",
        "from typing import Any",
        "",
        "# Raw entries merged into IndicatorMeta by _metadata.py (avoids circular imports).",
        "GENERATED_ENTRIES: dict[str, dict[str, Any]] = {",
    ]
    for slug in sorted(items.keys()):
        m = items[slug]
        blob = {
            "required_params": m["required_params"],
            "optional_params": m["optional_params"],
            "data_inputs": m["data_inputs"],
            "outputs": m["outputs"],
            "warmup_bars": m["warmup_bars"],
            "category": m["category"],
            "description": m["description"],
        }
        lines.append(f'    "{slug}": {repr(blob)},')
    lines.append("}")
    lines.append("")
    # Sibling map: authoritative raw-name param defaults for the abstract-API
    # introspection layer (quantwave.abstract). Kept separate from
    # GENERATED_ENTRIES so it is not folded into IndicatorMeta.
    lines.append("PARAM_DEFAULTS: dict[str, dict[str, Any]] = {")
    for slug in sorted(items.keys()):
        pd = items[slug].get("param_defaults") or {}
        if pd:
            lines.append(f'    "{slug}": {repr(pd)},')
    lines.append("}")
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    print("Exporting metadata from Rust...")
    exported = run_export()
    overlay_all = load_overlay()
    print(f"  {len(exported)} Rust entries, {len(overlay_all)} overlay keys")

    items: dict[str, dict[str, Any]] = {}
    for entry in exported:
        ov = overlay_all.get(entry["slug"])
        meta = indicator_meta_from_rust(entry, ov)
        slug = meta.pop("slug")
        if slug in items:
            # keep richer description on collision
            if len(meta.get("description", "")) > len(items[slug].get("description", "")):
                items[slug] = meta
        else:
            items[slug] = meta

    # overlay-only entries not in Rust export
    for key, ov in overlay_all.items():
        alias = ov.get("slug_alias", key)
        if alias not in items:
            desc = ov.get("description", alias)
            boundary_docs = (
                "\n\nBoundary Conditions & Error Behavior:\n"
                "- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n"
                "- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n"
                "- Negative Params: Negative period/length parameters will raise a ValueError."
            )
            desc += boundary_docs
            
            items[alias] = {
                "required_params": list(ov.get("required_params", [])),
                "optional_params": dict(ov.get("optional_params", {})),
                "data_inputs": ov.get("data_inputs", ["close"]),
                "outputs": ov.get("outputs", [alias]),
                "warmup_bars": ov.get("warmup_bars"),
                "category": ov.get("category", "Price Action"),
                "description": desc,
            }

    OUT.write_text(render_python(items), encoding="utf-8")
    print(f"Wrote {len(items)} entries to {OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())