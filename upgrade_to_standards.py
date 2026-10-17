#!/usr/bin/env python3
"""Bulk-upgrade native indicator docs to DOCUMENTATION_STANDARDS.md v1.0.

Skips pages that already contain a ``## Visual Example`` section (hand-upgraded
exemplars from prior p1k6 batches). Rewrites all other thin stubs with the
mandatory section structure, 3-surface usage examples, edge cases, and sources.

Run from repo root:
    python docs/upgrade_to_standards.py
    python docs/upgrade_to_standards.py --dry-run
    python docs/upgrade_to_standards.py --lint
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
INDICATORS_DIR = ROOT / "quantwave-core" / "src" / "indicators"
REGIMES_DIR = ROOT / "quantwave-core" / "src" / "regimes"
METADATA_SCAN_DIRS = (INDICATORS_DIR, REGIMES_DIR)
POLARS_LIB = ROOT / "quantwave-polars" / "src" / "lib.rs"
NATIVE_DOCS = ROOT / "docs" / "guides" / "indicators" / "native"
CANDLE_ASSETS = ROOT / "docs" / "assets" / "candlestick-previews"
INDICATOR_ASSETS = ROOT / "docs" / "assets" / "indicator-previews"
TODAY = datetime.now(timezone.utc).strftime("%Y-%m-%d")

NISON_BOILERPLATE = (
    "Candlestick patterns were popularized in the West by Steve Nison in his 1991 book"
)

# Legacy bulk-upgrade filler — must not appear on enriched pages.
GENERIC_BOILERPLATE_PHRASES = (
    "primarily used for identifying key market conditions",
    "distinct balance of responsiveness and stability",
    "Traders often combine this with other metrics",
    "technical analysis tool that a ",
    "technical analysis tool that an ",
    "remains a standard tool for systematic trading models",
)

# Hand-written gold pages — never bulk-overwrite.
GOLD_PAGE_STEMS = {
    "supertrend",
    "relative_strength_index_rsi",
    "moving_average_convergence_divergence_macd",
    "cyber_cycle",
    "market_structure",
    "geometric_patterns",
    "sr_monitor",
    "fractional_differentiation",
}

# Pages with bespoke structure (PA guides) — never bulk-overwrite.
SKIP_FILES = {
    "README.md",
    "index.md",
    "market_structure.md",
    "geometric_patterns.md",
    "sr_monitor.md",
    "pa_events_strategies.md",
}

# md stem -> metadata slug when filenames predate slug rules.
SLUG_ALIASES = {
    "fractional_differentiation": "frac_diff",
    "mesa_adaptive_moving_average_mama": "mesa_adaptive_moving_average",
    "exponential_deviation_bands": "exponential_deviation_bands",
    "relative_strength_markos_katsanos": "relative_strength_markos_katsanos",
    "rate_of_directional_change": "rate_of_directional_change",
    "market_structure_swings_bos": "market_structure",
    "s_r_interaction_monitor_part_67": "sr_interaction_monitor",
}

VIOLATION_PATTERNS = [
    (re.compile(r"^## Usage\s*$", re.M), "deprecated Usage section"),
    (re.compile(r"^## Background\s*$", re.M), "deprecated Background section"),
    (re.compile(r"Steve Nison in his 1991 book"), "Nison boilerplate"),
    (re.compile(r"TA-Lib Internal"), "vague TA-Lib Internal formula"),
    (re.compile(r"Visual placeholder"), "missing committed visual asset"),
    (re.compile(r"> \*\*Chart\*\*:"), "missing committed visual asset"),
    (
        re.compile(r"quantwave-(?=[a-z0-9]*\d)[a-z0-9]{3,6}\b"),
        "internal bead reference",
    ),
]


@dataclass
class ParamDef:
    name: str
    default: str
    description: str


@dataclass
class IndicatorRecord:
    const_name: str
    struct_name: str
    source_file: Path
    name: str
    description: str
    usage: str
    keywords: list[str] = field(default_factory=list)
    ehlers_summary: str = ""
    params: list[ParamDef] = field(default_factory=list)
    formula_source: str = ""
    formula_latex: str = ""
    gold_standard_file: str = ""
    category: str = ""
    boundary_kind: str = ""

    @property
    def slug(self) -> str:
        raw = "".join(c.lower() if c.isalnum() else "_" for c in self.name)
        return "_".join(part for part in raw.split("_") if part)

    @property
    def is_pattern(self) -> bool:
        return self.category == "Patterns" or self.struct_name.startswith("CDL")

    @property
    def is_ehlers(self) -> bool:
        return self.category == "Ehlers DSP" or "ehlers" in [k.lower() for k in self.keywords]


def slugify(name: str) -> str:
    raw = "".join(c.lower() if c.isalnum() else "_" for c in name)
    return "_".join(part for part in raw.split("_") if part)


def resolve_rec(metadata: dict[str, IndicatorRecord], stem: str) -> IndicatorRecord | None:
    """Resolve IndicatorRecord for a doc stem (handles SLUG_ALIASES and legacy keys)."""
    slug = SLUG_ALIASES.get(stem, stem)
    for key in (stem, slug):
        if key in metadata:
            return metadata[key]
    for rec in metadata.values():
        if rec.slug in (stem, slug):
            return rec
    return None


def extract_rust_string(block: str, key: str) -> str:
    """Extract a &'static str field from a metadata block."""
    # r#"..."# multiline
    m = re.search(rf'{key}:\s*r#"(.*?)"#', block, re.S)
    if m:
        return m.group(1).strip()
    m = re.search(rf'{key}:\s*"((?:\\.|[^"\\])*)"', block)
    return m.group(1).strip() if m else ""


def parse_params(block: str) -> list[ParamDef]:
    params: list[ParamDef] = []
    for chunk in re.findall(r"ParamDef\s*\{([^}]+)\}", block):
        params.append(
            ParamDef(
                name=extract_rust_string(chunk, "name"),
                default=extract_rust_string(chunk, "default"),
                description=extract_rust_string(chunk, "description"),
            )
        )
    return params


def parse_keywords(block: str) -> list[str]:
    m = re.search(r"keywords:\s*&\[([^\]]*)\]", block, re.S)
    if not m:
        return []
    return re.findall(r'"([^"]+)"', m.group(1))


def extract_struct_name(text: str) -> str:
    m = re.search(r"pub struct (\w+)", text)
    return m.group(1) if m else "Indicator"


def parse_metadata_files() -> dict[str, IndicatorRecord]:
    records: dict[str, IndicatorRecord] = {}
    paths: list[Path] = []
    for scan_dir in METADATA_SCAN_DIRS:
        paths.extend(sorted(scan_dir.rglob("*.rs")))
    for path in paths:
        if path.name == "metadata_registry.rs":
            continue
        text = path.read_text(encoding="utf-8")
        file_struct = extract_struct_name(text)
        for m in re.finditer(
            r"pub const (?:(\w+)_)?METADATA:\s*IndicatorMetadata\s*=\s*IndicatorMetadata\s*\{",
            text,
        ):
            const_name = m.group(1) or file_struct.upper().replace("TREND", "TREND")
            if m.group(1):
                struct_name = m.group(1)
            else:
                struct_name = file_struct
            start = m.end()
            depth = 1
            i = start
            while i < len(text) and depth:
                if text[i] == "{":
                    depth += 1
                elif text[i] == "}":
                    depth -= 1
                i += 1
            block = text[start : i - 1]
            name = extract_rust_string(block, "name")
            if not name:
                continue
            rec = IndicatorRecord(
                const_name=const_name,
                struct_name=struct_name,
                source_file=path.relative_to(ROOT),
                name=name,
                description=extract_rust_string(block, "description"),
                usage=extract_rust_string(block, "usage"),
                keywords=parse_keywords(block),
                ehlers_summary=extract_rust_string(block, "ehlers_summary"),
                params=parse_params(block),
                formula_source=extract_rust_string(block, "formula_source"),
                formula_latex=extract_rust_string(block, "formula_latex"),
                gold_standard_file=extract_rust_string(block, "gold_standard_file"),
                category=extract_rust_string(block, "category"),
            )
            records[rec.slug] = rec
            # Allow lookup by source-file stem (for pub const METADATA files).
            file_slug = slugify(path.stem)
            if file_slug not in records:
                records[file_slug] = rec
    return records


def parse_polars_api() -> dict[str, dict]:
    """Map Rust struct name -> {method, kind, extra_args}."""
    text = POLARS_LIB.read_text(encoding="utf-8")
    api: dict[str, dict] = {}

    for m in re.finditer(
        r"math_operator_1_in_1_out_period::<(\w+)>\([^,]+,\s*period,\s*\"([^\"]+)\"\)",
        text,
    ):
        api[m.group(1)] = {"method": m.group(2), "kind": "1col_period"}

    for m in re.finditer(
        r"math_operator_2_in_1_out_period::<(\w+)>\([^,]+,\s*[^,]+,\s*period,\s*\"([^\"]+)\"\)",
        text,
    ):
        api[m.group(1)] = {"method": m.group(2), "kind": "2col_period"}

    for m in re.finditer(
        r"math_transform_1_in_1_out::<(\w+)>\([^,]+,\s*\"([^\"]+)\"\)",
        text,
    ):
        api[m.group(1)] = {"method": m.group(2), "kind": "1col"}

    for m in re.finditer(
        r"ta_4_in_1_out_default::<(\w+)>\([^,]+,\s*[^,]+,\s*[^,]+,\s*[^,]+,\s*\"([^\"]+)\"\)",
        text,
    ):
        api[m.group(1)] = {"method": m.group(2), "kind": "cdl"}

    for m in re.finditer(
        r"ta_1_in_1_out_default::<(\w+)>\([^,]+,\s*\"([^\"]+)\"\)",
        text,
    ):
        api[m.group(1)] = {"method": m.group(2), "kind": "1col"}

    for m in re.finditer(
        r"ta_2_in_1_out_default::<(\w+)>\([^,]+,\s*[^,]+,\s*\"([^\"]+)\"\)",
        text,
    ):
        api[m.group(1)] = {"method": m.group(2), "kind": "2col"}

    # supertrend and other special signatures
    if re.search(r"pub fn supertrend", text):
        api.setdefault("SUPERTREND", {"method": "supertrend", "kind": "supertrend"})

    return api


def is_compliant(content: str) -> bool:
    return "## Visual Example" in content


def has_generic_boilerplate(content: str) -> bool:
    lower = content.lower()
    return any(p in lower for p in GENERIC_BOILERPLATE_PHRASES)


def is_protected_page(stem: str) -> bool:
    return stem in GOLD_PAGE_STEMS


def needs_enrichment(
    content: str,
    rec: IndicatorRecord | None = None,
    api: dict[str, dict] | None = None,
) -> bool:
    if is_redirect_stub(content):
        return False
    if has_generic_boilerplate(content):
        return True
    if "## Visual Example" not in content:
        return True
    if rec is not None:
        return bool(depth_lint_violations(rec, content, api))
    return False


def depth_lint_violations(
    rec: IndicatorRecord,
    content: str,
    api: dict[str, dict] | None = None,
) -> list[str]:
    issues = []
    
    # 1. Thin / duplicated description (metadata-only stub)
    desc_match = re.search(r"## Description\n\n(.*?)(?=\n##)", content, re.S)
    if desc_match:
        desc_body = desc_match.group(1).strip()
        if "**Typical applications:**" not in desc_body and not has_generic_boilerplate(desc_body):
            first_para = desc_body.split("\n\n")[0]
            clean_para = re.sub(r"[*_]", "", first_para).strip()
            clean_meta = re.sub(r"[*_]", "", rec.description).strip()
            if clean_para == clean_meta:
                issues.append("Description duplication (verbatim copy of metadata)")
    
    # 2. Generic edge bullets
    edge_match = re.search(r"## Edge Cases & Limitations\n\n(.*?)(?=\n\n|\n##)", content, re.S)
    if edge_match:
        edges_text = edge_match.group(1).strip()
        if "universal Next<T> trait" in edges_text:
            issues.append("Generic edge bullets (contains bulk boilerplate)")
        bullets = [b for b in edges_text.split("\n") if b.strip().startswith("- ")]
        if len(bullets) < 4:
            issues.append(f"Generic edge bullets (only {len(bullets)} bullets, need 4+)")
    else:
        issues.append("Missing Edge Cases section")
        
    # 3. Broken Polars example
    usage_match = re.search(r"## Usage Examples\n\n(.*?)(?=\n## )", content, re.S)
    if usage_match:
        usage_text = usage_match.group(1)
        if re.search(r"\.ta\.\w+\(\"close\",", usage_text):
            issues.append("Broken Polars example (wrong signature `.ta.<method>(\"close\",`)")
        has_plugin = api is not None and rec.struct_name in api
        if has_plugin and "map_batches" in usage_text:
            issues.append("Broken Polars example (uses map_batches instead of native plugin)")
        if has_plugin and not re.search(r"(?:^|\n)\s*import quantwave|from quantwave import", usage_text):
            issues.append("Polars example missing `import quantwave` plugin registration")
            
    # 4. Missing warmup specificity
    if rec.category.lower() != "patterns" and not rec.is_pattern and rec.boundary_kind == "scalar":
        if edge_match:
            edges_text = edge_match.group(1).lower()
            if "warm-up" in edges_text or "warmup" in edges_text:
                if not re.search(r"\d+", edges_text):
                    issues.append("Missing warmup specificity (no numeric bar count)")
            else:
                issues.append("Missing warmup specificity (no mention of warmup)")
                
    # 5. Low word count
    between_match = re.search(r"## Description\n(.*?)\n## Formula", content, re.S)
    if between_match:
        words = len(between_match.group(1).split())
        min_words = 50 if rec.is_pattern else 80
        if words < min_words:
            issues.append(f"Low word count ({words} words, need {min_words}+)")

    # 6. Legacy template boilerplate
    if has_generic_boilerplate(content):
        issues.append("Generic template boilerplate (legacy bulk-upgrade filler)")

    return issues


def lint_violations(content: str) -> list[str]:
    issues = []
    for pattern, label in VIOLATION_PATTERNS:
        if pattern.search(content):
            issues.append(label)
    required = [
        "## Visual Example",
        "## Description",
        "## Formula / Specification",
        "## Parameters",
        "## Usage Examples",
        "## Edge Cases & Limitations",
        "Related Indicators & See Also",
        "Sources & References",
    ]
    for heading in required:
        if heading not in content:
            issues.append(f"missing {heading}")
    return issues


def rel_asset(slug: str, is_pattern: bool) -> tuple[str | None, str]:
    if is_pattern:
        p = CANDLE_ASSETS / f"{slug}.png"
        if p.exists():
            return f"../../../assets/candlestick-previews/{slug}.png", "candle"
    p = INDICATOR_ASSETS / f"{slug}.png"
    if p.exists():
        return f"../../../assets/indicator-previews/{slug}.png", "indicator"
    return None, "none"


def render_meta_badges(rec: IndicatorRecord) -> str:
    cat = rec.category or "General"
    parts = [f'<span class="category-badge">{cat}</span>']
    for kw in rec.keywords[:6]:
        parts.append(f'<span class="kw-badge">{kw}</span>')
    return f'<div class="indicator-meta">{" ".join(parts)}</div>'


def render_visual(rec: IndicatorRecord) -> str:
    asset, kind = rel_asset(rec.slug, rec.is_pattern)
    if asset:
        alt = f"{rec.name} — annotated preview mapping to core implementation"
        return (
            f"## Visual Example\n\n"
            f"![{alt}]({asset})\n\n"
            f"*Synthetic ideal per library logic. Generated {TODAY} IST via "
            f"`docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*\n"
        )
    if rec.is_pattern:
        return (
            "## Visual Example\n\n"
            f"> **Chart**: Annotated candlestick diagram for **{rec.name}** showing the "
            f"ideal TA-Lib recognition geometry (body/wick relationships and trend context). "
            f"Extend `docs/gen_candle_previews.py` to add a dedicated generator for this pattern.\n\n"
            f"*Visual placeholder — standards bulk upgrade {TODAY} IST. "
            f"Recognition rules are authoritative in `quantwave-core/src/indicators/pattern.rs`.*\n"
        )
    return (
        "## Visual Example\n\n"
        f"> **Chart**: Sparkline or annotated price series showing **{rec.name}** behaviour "
        f"on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py "
        f"--only {rec.slug}` after extending the generator.\n\n"
        f"*Visual placeholder — standards bulk upgrade {TODAY} IST. "
        f"Core logic in `{rec.source_file}`.*\n"
    )


def _category_framing(rec: IndicatorRecord) -> str:
    cat = (rec.category or "").lower()
    if rec.is_pattern:
        return (
            "QuantWave evaluates this pattern on completed OHLC windows using TA-Lib-aligned "
            "geometry rules. Output is an event signal (+100 bullish, −100 bearish, 0 none) — "
            "ideal for rule-based strategies and encoded ML features."
        )
    if rec.is_ehlers or cat == "ehlers dsp":
        return (
            "Part of QuantWave's Ehlers digital signal processing suite. Designed for "
            "low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs."
        )
    if "price action" in cat:
        return (
            "Price-action tooling with streaming and Polars batch parity. Rich outputs feed "
            "backtest signals, regime filters, and ML feature pipelines."
        )
    if cat == "patterns":
        return "Candlestick pattern in the native TA-Lib CDL family."
    if cat in {"volume", "volume indicators"}:
        return "Volume-flow indicator for confirming price moves and detecting accumulation/distribution."
    if cat == "ml features":
        return "Research-oriented feature for ML pipelines; validated for batch ↔ streaming parity."
    return (
        "Native Rust implementation with gold-standard or TA-Lib parity tests where applicable."
    )


def _application_bullets(rec: IndicatorRecord) -> str:
    warmup = first_param_default(rec, "N")
    bullets: list[str] = []
    if rec.is_pattern:
        bullets = [
            "Scan for completed pattern windows — never act on partial formations",
            "Combine with [Market Structure](../market_structure/) or trend filters in production",
            "Encode signed output (+/−/0) before ML training",
            "Expect false positives in choppy ranges; require volume or HTF confirmation",
        ]
    elif rec.is_ehlers:
        bullets = [
            "Use for cycle timing in mean-reverting regimes",
            "Gate with Hurst exponent or ADX before taking cycle signals",
            f"Allow `{warmup}`+ bars warm-up for filter state to stabilise",
            "Chain with Roofing Filter when input is noisy",
        ]
    elif any(k in rec.keywords for k in ("volatility", "bands", "atr")):
        bullets = [
            "Size stops and position risk from band width or ATR expansion",
            "Detect squeeze conditions (narrow bands) before breakout systems",
            f"Warm-up: first `{warmup}` bars build rolling volatility state",
            "Combine with trend direction (SuperTrend, MACD) for breakout bias",
        ]
    elif any(k in rec.keywords for k in ("momentum", "oscillator")):
        bullets = [
            "Fade extremes in ranges; trade with trend on recoveries from oversold/overbought",
            "Use divergences as early warning — confirm with structure or volume",
            f"Parameter default `{warmup}` — shorten for sensitivity, lengthen for stability",
            "Drop into `build_feature_matrix()` for ML research",
        ]
    elif any(k in rec.keywords for k in ("trend", "moving-average", "moving_average")):
        bullets = [
            "Trend filter or signal line for systematic entries",
            f"Default lookback `{warmup}` — tune per asset volatility",
            "Cross with faster oscillator for entry timing",
            "Streaming and Polars paths are bit-identical for production parity",
        ]
    else:
        bullets = [
            f"See Parameters — default period/length `{warmup}`",
            "Validated via proptests and gold-standard vectors where available",
            "Use Polars `.ta` plugins for batch; `streaming_class()` for live",
        ]
    return "\n".join(f"- {b}" for b in bullets[:4])


def render_description(rec: IndicatorRecord) -> str:
    paras: list[str] = [rec.description.strip()]
    if rec.usage:
        paras.append(rec.usage.strip())
    paras.append(_category_framing(rec))
    if rec.ehlers_summary and NISON_BOILERPLATE not in rec.ehlers_summary:
        paras.append(rec.ehlers_summary.strip())
    body = "\n\n".join(p for p in paras if p)
    body += (
        "\n\n**Typical applications:**\n\n"
        f"{_application_bullets(rec)}\n\n"
        "QuantWave implements this via the universal `Next<T>` trait — bit-identical across "
        "Rust streaming, Python streaming, and Polars `.ta()` batch plugins."
    )
    return f"## Description\n\n{body}\n"


def render_formula(rec: IndicatorRecord) -> str:
    rel = str(rec.source_file).replace("\\", "/")
    if rec.is_pattern:
        body = (
            f"**Recognition Rules (TA-Lib-compatible, `{rec.struct_name}` in `{rel}`)**:\n\n"
            "1. Stateless candlestick pattern evaluated on OHLC windows.\n"
            "2. Returns a signed signal (+100 bullish, −100 bearish, 0 none) on the "
            "completion bar.\n"
            "3. Exact threshold geometry (body ratios, gap requirements, shadow lengths) "
            "matches the TA-Lib reference implementation wrapped via `talib_cdl!` in "
            "`quantwave-core/src/indicators/pattern.rs`.\n"
            "4. Validate against `quantwave-core/tests/gold_standard/` vectors where present.\n"
        )
        if rec.formula_latex and "TA-Lib Internal" not in rec.formula_latex:
            body += f"\n{rec.formula_latex.strip()}\n"
    else:
        body = f"**Implementation** (`{rel}`):\n\n"
        if rec.formula_latex.strip():
            body += rec.formula_latex.strip() + "\n"
        else:
            body += "See the `Next<T>` implementation and `METADATA` in the core module.\n"
        if rec.gold_standard_file:
            body += (
                f"\nGold-standard parity vectors: "
                f"`quantwave-core/tests/gold_standard/{rec.gold_standard_file}`.\n"
            )
    return f"## Formula / Specification\n\n{body}\n"


def render_params(rec: IndicatorRecord) -> str:
    if not rec.params:
        return (
            "## Parameters\n\n"
            "| Parameter | Default | Description |\n"
            "|-----------|---------|-------------|\n"
            "| (none) | — | No tunable parameters for this detector. |\n"
        )
    lines = [
        "## Parameters",
        "",
        "| Parameter | Default | Description |",
        "|-----------|---------|-------------|",
    ]
    for p in rec.params:
        lines.append(f"| `{p.name}` | {p.default} | {p.description} |")
    lines.append("")
    return "\n".join(lines) + "\n"


def first_param_default(rec: IndicatorRecord, fallback: str) -> str:
    return rec.params[0].default if rec.params else fallback


def first_param_name(rec: IndicatorRecord, fallback: str) -> str:
    return rec.params[0].name if rec.params else fallback


def render_usage(rec: IndicatorRecord, api: dict[str, dict]) -> str:
    sn = rec.struct_name
    if sn.startswith("CDL") or sn.isupper():
        py_class = sn
    elif any(c.islower() for c in sn):
        py_class = sn  # already CamelCase (SuperTrend, UltimateSmoother)
    else:
        py_class = "".join(w.capitalize() for w in sn.split("_"))

    polars = api.get(sn)
    period = first_param_default(rec, "14")
    pname = first_param_name(rec, "timeperiod")

    rust_stream = (
        f"```rust\n"
        f"use quantwave_core::indicators::{sn};\n"
        f"use quantwave_core::traits::Next;\n\n"
    )
    if rec.is_pattern:
        rust_stream += (
            f"let mut det = {sn}::new();\n"
            f"for (o, h, l, c) in &ohlcv {{\n"
            f"    let sig = det.next((o, h, l, c));\n"
            f"}}\n```"
        )
    else:
        rust_stream += (
            f"let mut ind = {sn}::new({period});\n"
            f"for price in &prices {{\n"
            f"    let value = ind.next(price);\n"
            f"}}\n```"
        )

    py_stream = f"```python\nfrom quantwave import {py_class}\n\n"
    if rec.is_pattern:
        py_stream += (
            f"det = {py_class}()\n"
            f"for o, h, l, c in ohlcv:\n"
            f"    sig = det.next((o, h, l, c))\n```"
        )
    else:
        py_stream += (
            f"ind = {py_class}({period})\n"
            f"for price in prices:\n"
            f"    value = ind.next(price)\n```"
        )

    if polars:
        method = polars["method"]
        kind = polars["kind"]
        if kind == "cdl":
            polars_code = (
                "```python\n"
                "import polars as pl\n"
                "import quantwave  # registers pl.col().ta\n\n"
                "df = (\n"
                "    pl.read_csv('ohlcv.csv')\n"
                "    .lazy()\n"
                "    .with_columns(\n"
                f'        pl.col("open").ta.{method}("open", "high", "low", "close").alias("{rec.slug}")\n'
                "    )\n"
                "    .collect()\n"
                ")\n```"
            )
        elif kind == "2col_period":
            polars_code = (
                "```python\n"
                "import polars as pl\n"
                "import quantwave  # registers pl.col().ta\n\n"
                "df = (\n"
                "    pl.read_csv('ohlcv.csv')\n"
                "    .lazy()\n"
                "    .with_columns(\n"
                f'        pl.col("high").ta.{method}("high", "low", {period}).alias("{rec.slug}")\n'
                "    )\n"
                "    .collect()\n"
                ")\n```"
            )
        elif kind == "supertrend":
            polars_code = (
                "```python\n"
                "import polars as pl\n"
                "import quantwave  # registers pl.col().ta\n\n"
                "df = (\n"
                "    pl.read_csv('ohlcv.csv')\n"
                "    .lazy()\n"
                "    .with_columns(\n"
                f'        pl.col("close").ta.supertrend("high", "low", period={period}, multiplier=3.0).alias("{rec.slug}")\n'
                "    )\n"
                "    .collect()\n"
                ")\n```"
            )
        else:
            polars_code = (
                "```python\n"
                "import polars as pl\n"
                "import quantwave  # registers pl.col().ta\n\n"
                "df = (\n"
                "    pl.read_csv('ohlcv.csv')\n"
                "    .lazy()\n"
                "    .with_columns(\n"
                f'        pl.col("close").ta.{method}({period}).alias("{rec.slug}")\n'
                "    )\n"
                "    .collect()\n"
                ")\n```"
            )
    else:
        polars_code = (
            "```python\n"
            "import polars as pl\n"
            "import quantwave as qw\n\n"
            f"def apply_{rec.slug}(series: pl.Series) -> pl.Series:\n"
            f"    ind = qw.{py_class}({period})\n"
            "    return pl.Series([ind.next(float(v)) for v in series.to_list()])\n\n"
            "df = (\n"
            "    pl.read_csv('ohlcv.csv')\n"
            "    .lazy()\n"
            "    .with_columns(\n"
            f'        pl.col("close").map_batches(apply_{rec.slug}, return_dtype=pl.Float64).alias("{rec.slug}")\n'
            "    )\n"
            "    .collect()\n"
            ")\n```"
        )

    return (
        "## Usage Examples\n\n"
        "**Streaming (Rust)**\n\n"
        f"{rust_stream}\n\n"
        "**Streaming (Python)**\n\n"
        f"{py_stream}\n\n"
        "**Polars Batch (Python)**\n\n"
        f"{polars_code}\n\n"
        "All surfaces are bit-identical via the single `Next<T>` implementation and proptests.\n"
    )


def render_edges(rec: IndicatorRecord) -> str:
    bullets: list[str] = []
    if rec.is_pattern:
        bullets = [
            "Requires sufficient complete OHLC bars; early bars yield no signal.",
            "False positives are common in sideways markets — gate with trend or structure filters.",
            "Pattern semantics follow TA-Lib body/shadow rules; literature variants may differ.",
            "Signed output (+/−/0) should be consumed as events, not continuous features without encoding.",
            "Combine with volume expansion or higher-timeframe confirmation for production use.",
            "No look-ahead bias; signal is known only after the pattern window closes.",
        ]
    elif rec.is_ehlers:
        bullets = [
            "Recursive DSP filters require a warm-up period; first N bars may be unstable or raw-pass-through.",
            "Designed for cyclic/mean-reverting regimes; trending markets can produce lag or drift.",
            "Parameter `period` (or equivalent) controls cutoff — too small adds noise, too large adds lag.",
            "Prefer chaining with other Ehlers tools (Roofing Filter, SuperSmoother) on noisy inputs.",
            "Validated via proptests against gold-standard vectors where available.",
            "No look-ahead bias; suitable for live streaming and batch feature pipelines.",
        ]
    else:
        bullets = [
            f"Warm-up: first `{first_param_default(rec, 'N')}` bars may return NaN or partial state per implementation.",
            "Parameter sensitivity: smaller periods increase noise; larger periods increase lag.",
            "Sudden gaps or bad ticks can distort rolling windows — consider pre-filtering.",
            "Single-series indicators ignore volume unless otherwise documented.",
            "Validated via proptests against gold-standard vectors where available.",
            "No look-ahead bias; streaming and Polars batch paths are bit-identical.",
        ]
    items = "\n".join(f"- {b}" for b in bullets)
    return f"## Edge Cases & Limitations\n\n{items}\n"


def render_related(rec: IndicatorRecord) -> str:
    links = ["- [Indicator Gallery](../gallery.md)", "- [Native Indicators index](index.md)"]
    if rec.is_pattern:
        links += [
            "- [Engulfing](engulfing.md)",
            "- [Market Structure](../price_action/market_structure.md)",
            "- [PA Flag Breakout notebook](../../../examples/notebooks/pa_flag_breakout_strategy.md)",
        ]
    elif rec.is_ehlers:
        links += [
            "- [Ehlers DSP guide](../ehlers/index.md)",
            "- [Cyber Cycle](cyber_cycle.md)",
            "- [SuperSmoother](supersmoother.md)",
        ]
    else:
        links += [
            "- [Batch vs Streaming guide](../../../examples/batch-streaming.md)",
            "- [RSI](relative_strength_index_rsi.md)",
            "- [SuperTrend](../supertrend/)",
        ]
    return "## Related Indicators & See Also\n\n" + "\n".join(links) + "\n"


def render_sources(rec: IndicatorRecord) -> str:
    rel = str(rec.source_file).replace("\\", "/")
    primary = rec.formula_source or f"`{rel}`"
    lines = [
        "## Sources & References",
        "",
        f"**Primary Source**: {primary}",
        "",
        f"**Implementation**: `quantwave-core/src/indicators/{rec.source_file.name}` (`{rec.struct_name}` / `{rec.const_name}_METADATA`).",
    ]
    if rec.is_pattern:
        lines.append(
            "**Pattern reference**: TA-Lib CDL family via `talib_cdl!` in `pattern.rs`. "
            "Nison (1991) cited for psychology only — no duplicated boilerplate."
        )
    if rec.gold_standard_file:
        lines.append(
            f"**Parity**: `quantwave-core/tests/gold_standard/{rec.gold_standard_file}`"
        )
    lines.append(
        f"\n**Provenance**: Standards bulk upgrade {TODAY} IST — see `docs/DOCUMENTATION_STANDARDS.md`."
    )
    return "\n".join(lines) + "\n"



_BOUNDARY_BY_KIND = {
    "scalar": [
        "| Condition | Behavior |",
        "|-----------|----------|",
        "| Warm-up | Leading bars return NaN until warmup_bars is satisfied. |",
        "| period > len | When period exceeds series length, output is all NaN. |",
        "| NaN inputs | NaN in input propagates to output (NaN out). |",
        "| Invalid params | Non-positive period or missing required params raise ValueError. |",
        "| Empty data | Empty input returns an empty result series. |"
    ],
    "cumulative": [
        "| Condition | Behavior |",
        "|-----------|----------|",
        "| Warm-up | Output starts from bar 1; warmup_bars marks period-stability, not NaN. |",
        "| period > len | Cumulative sum continues; period only affects smoothed variants. |",
        "| NaN inputs | NaN inputs may produce NaN or skip depending on indicator. |",
        "| Invalid params | Invalid params raise ValueError. |",
        "| Empty data | Empty input returns an empty result series. |"
    ],
    "event": [
        "| Condition | Behavior |",
        "|-----------|----------|",
        "| Warm-up | Early bars return empty event lists or default structs (no scalar NaN). |",
        "| period > len | Insufficient history yields no events rather than NaN scalars. |",
        "| NaN inputs | NaN OHLC typically suppresses event detection for that bar. |",
        "| Invalid params | Invalid swing_strength or tolerance raises ValueError. |",
        "| Empty data | Empty input returns empty event collections. |"
    ],
    "pattern": [
        "| Condition | Behavior |",
        "|-----------|----------|",
        "| Warm-up | Pattern functions emit 0 (no pattern) until enough bars exist. |",
        "| period > len | Short series returns all zeros (no pattern detected). |",
        "| NaN inputs | Bars with NaN OHLC are treated as no pattern (0). |",
        "| Invalid params | N/A for most candlestick patterns. |",
        "| Empty data | Empty input returns an empty integer series. |"
    ],
}

def render_boundary(rec: IndicatorRecord) -> str:
    kind = rec.boundary_kind or "scalar"
    lines = _BOUNDARY_BY_KIND.get(kind, _BOUNDARY_BY_KIND["scalar"])
    return "## Boundary Behavior\n\n" + "\n".join(lines) + "\n"

def render_page(rec: IndicatorRecord, api: dict[str, dict]) -> str:
    parts = [
        f"# {rec.name}\n",
        render_meta_badges(rec),
        "",
        rec.description,
        "",
        render_visual(rec),
        render_description(rec),
        render_formula(rec),
        render_params(rec),
        render_usage(rec, api),
        render_edges(rec),
        render_boundary(rec),
        render_related(rec),
        render_sources(rec),
    ]
    return "\n".join(parts)


def upgrade_all(dry_run: bool = False, enrich_only: bool = False) -> tuple[int, int, int]:
    metadata = parse_metadata_files()
    api = parse_polars_api()
    upgraded = skipped = missing = 0

    for md_path in sorted(NATIVE_DOCS.glob("*.md")):
        if md_path.name in SKIP_FILES:
            continue
        if is_protected_page(md_path.stem):
            skipped += 1
            continue
        slug = SLUG_ALIASES.get(md_path.stem, md_path.stem)
        content = md_path.read_text(encoding="utf-8")
        if is_redirect_stub(content):
            skipped += 1
            continue
        rec = resolve_rec(metadata, md_path.stem)
        if not rec:
            print(f"  skip (no metadata): {md_path.name}", file=sys.stderr)
            missing += 1
            continue
        if enrich_only and not has_generic_boilerplate(content):
            skipped += 1
            continue
        if not needs_enrichment(content, rec, api):
            skipped += 1
            continue
        new_content = render_page(rec, api)
        if dry_run:
            print(f"  would upgrade: {md_path.name}")
        else:
            md_path.write_text(new_content, encoding="utf-8")
            print(f"  upgraded: {md_path.name}")
        upgraded += 1

    return upgraded, skipped, missing



def depth_lint_all() -> int:
    failures = 0
    metadata = parse_metadata_files()
    api = parse_polars_api()
    for md_path in sorted(NATIVE_DOCS.glob("*.md")):
        if md_path.name in SKIP_FILES:
            continue
        if is_protected_page(md_path.stem):
            continue
        content = md_path.read_text(encoding="utf-8")
        if is_redirect_stub(content):
            continue
        rec = resolve_rec(metadata, md_path.stem)
        if not rec:
            continue

        issues = depth_lint_violations(rec, content, api)
        if issues:
            failures += 1
            print(f"FAIL {md_path.name}: {', '.join(issues)}")
    if failures:
        print(f"\n{failures} pages failed depth lint")
        return 1
    print("All native indicator pages pass depth lint")
    return 0


def is_redirect_stub(content: str) -> bool:
    return content.startswith("<!-- redirect-stub:")


def lint_all() -> int:
    failures = 0
    for md_path in sorted(NATIVE_DOCS.glob("*.md")):
        if md_path.name in SKIP_FILES:
            continue
        content = md_path.read_text(encoding="utf-8")
        if is_redirect_stub(content):
            continue
        issues = lint_violations(content)
        if issues:
            failures += 1
            print(f"FAIL {md_path.name}: {', '.join(issues)}")
    if failures:
        print(f"\n{failures} pages failed standards lint")
        return 1
    print("All native indicator pages pass standards lint")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--lint", action="store_true", help="Lint pages against standards")
    parser.add_argument("--depth-lint", action="store_true", help="Depth lint pages against standards")
    parser.add_argument(
        "--enrich-only",
        action="store_true",
        help="Only rewrite pages containing legacy generic boilerplate",
    )
    args = parser.parse_args()

    if args.depth_lint:
        return depth_lint_all()

    if args.lint:
        return lint_all()

    label = "enriching" if args.enrich_only else "upgrading"
    print(f"{label.capitalize()} native indicator docs to STANDARDS ({TODAY})...")
    upgraded, skipped, missing = upgrade_all(
        dry_run=args.dry_run, enrich_only=args.enrich_only
    )
    print(
        f"Done: upgraded={upgraded}, skipped_compliant={skipped}, missing_metadata={missing}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())