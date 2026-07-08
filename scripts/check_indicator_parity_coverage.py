#!/usr/bin/env python3
"""Enforce per-indicator streaming/batch parity proptest coverage (quantwave-ruh0.1)."""

from __future__ import annotations

import json
import re
import sys
from collections import Counter
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover
    import tomli as tomllib  # type: ignore

ROOT = Path(__file__).resolve().parent.parent
CORE = ROOT / "quantwave-core"
INDICATORS = CORE / "src" / "indicators"
REGIMES = CORE / "src" / "regimes"
REGISTRY = INDICATORS / "metadata_registry.rs"
EXEMPTIONS = CORE / "tests" / "parity_exemptions.toml"
GOLD_DIR = CORE / "tests" / "gold_standard"
REPORT = ROOT / "docs" / "generated" / "parity_coverage.json"

REGISTERED_RE = re.compile(
    r'RegisteredMetadata \{ slug: "([^"]+)", meta: &\w+, struct_name: "[^"]*", source_file: "([^"]+)" \}'
)

# Slug -> extra proptest function name stems (without test_ prefix / _parity suffix).
PARITY_ALIASES: dict[str, list[str]] = {
    "linreg": ["linearreg", "ta_linearreg"],
    "true_range": ["trange", "tatrange", "ta_trange"],
    "natr": ["tanatr", "ta_natr"],
    "stddev": ["tastddev", "ta_stddev"],
    "beta": ["tabeta", "ta_beta"],
    "correl": ["tacorrel", "ta_correl"],
    "atr": ["taatr", "ta_atr"],
    "sma": ["talib_sma"],
    "ema": ["talib_ema"],
}


def _parity_stems(slug: str) -> list[str]:
    stems = {slug}
    stems.add(f"ta_{slug}")
    stems.add(f"talib_{slug}")
    compact = slug.replace("_", "")
    stems.add(f"ta{compact}")
    for alias in PARITY_ALIASES.get(slug, []):
        stems.add(alias)
    return sorted(stems)


def _has_slug_parity_proptest(slug: str, text: str) -> bool:
    if "proptest!" not in text:
        return False
    for stem in _parity_stems(slug):
        if re.search(rf"fn\s+test_{re.escape(stem)}_parity", text):
            return True
    return False


def _resolve_source(stem: str) -> Path | None:
    for base in (INDICATORS, REGIMES):
        if not base.exists():
            continue
        for path in base.rglob(f"{stem}.rs"):
            return path
    return None


def _load_registered() -> list[tuple[str, str]]:
    text = REGISTRY.read_text(encoding="utf-8")
    return REGISTERED_RE.findall(text)


def _load_exemptions() -> dict[str, dict]:
    if not EXEMPTIONS.exists():
        return {}
    data = tomllib.loads(EXEMPTIONS.read_text(encoding="utf-8"))
    out: dict[str, dict] = {}
    for entry in data.get("exemption", []):
        slug = entry.get("slug", "").strip()
        reason = (entry.get("reason") or "").strip()
        if not slug:
            raise ValueError("parity_exemptions.toml: exemption missing slug")
        if len(reason) < 10:
            raise ValueError(f"parity_exemptions.toml: exemption '{slug}' needs reason (>=10 chars)")
        if slug in out:
            raise ValueError(f"parity_exemptions.toml: duplicate slug '{slug}'")
        out[slug] = entry
    return out


def _scan_rust_sources() -> dict[str, str]:
    cache: dict[str, str] = {}
    for path in CORE.rglob("*.rs"):
        try:
            cache[str(path)] = path.read_text(encoding="utf-8")
        except OSError:
            continue
    return cache


def _parity_location(slug: str, stem: str, stem_counts: Counter[str], sources: dict[str, str]) -> str | None:
    source_path = _resolve_source(stem)
    if source_path is not None:
        text = sources.get(str(source_path), "")
        if stem_counts[stem] == 1 and "proptest!" in text:
            return str(source_path.relative_to(ROOT))
        if _has_slug_parity_proptest(slug, text):
            return str(source_path.relative_to(ROOT))

    for path_str, text in sources.items():
        if _has_slug_parity_proptest(slug, text):
            return str(Path(path_str).relative_to(ROOT))
    return None


def _gold_gap(slug: str, stem: str, sources: dict[str, str]) -> str | None:
    source_path = _resolve_source(stem)
    if source_path is None:
        return None
    text = sources.get(str(source_path), "")
    gold_ref = ""
    for line in text.splitlines():
        if "gold_standard_file:" in line:
            m = re.search(r'gold_standard_file:\s*"([^"]*)"', line)
            if m:
                gold_ref = m.group(1).strip()
            break
    if not gold_ref:
        return None
    if (GOLD_DIR / gold_ref).exists():
        return None
    if "load_gold_standard" in text and re.search(
        r"/\*[\s\S]*?(?:load_gold_standard|test_\w*gold)[\s\S]*?\*/", text
    ):
        return f"missing on-disk gold fixture {gold_ref!r} (test commented out)"
    if re.search(r"//\s*TODO:.*gold", text, re.IGNORECASE):
        return f"missing on-disk gold fixture {gold_ref!r} (restore pending)"
    return None


def collect_coverage() -> dict:
    registered = _load_registered()
    exemptions = _load_exemptions()
    stem_counts = Counter(stem for _, stem in registered)
    sources = _scan_rust_sources()

    rows: list[dict] = []
    failures: list[str] = []

    for slug, stem in registered:
        parity_path = _parity_location(slug, stem, stem_counts, sources)
        gold_issue = _gold_gap(slug, stem, sources)
        exempt = slug in exemptions

        parity_ok = parity_path is not None or exempt
        gold_ok = gold_issue is None or exempt

        row = {
            "slug": slug,
            "source_file": stem,
            "parity_proptest": parity_path,
            "exempt": exempt,
            "gold_gap": gold_issue,
            "status": "ok" if parity_ok and gold_ok else "gap",
        }
        rows.append(row)

        if not parity_ok:
            failures.append(f"{slug}: missing streaming/batch proptest parity")
        if not gold_ok:
            failures.append(f"{slug}: {gold_issue}")

    covered = sum(1 for r in rows if r["parity_proptest"])
    exempt_count = sum(1 for r in rows if r["exempt"])
    gaps = [r["slug"] for r in rows if r["status"] == "gap" and not r["exempt"]]

    return {
        "indicator_count": len(rows),
        "parity_proptest_count": covered,
        "exemption_count": exempt_count,
        "gap_count": len(gaps),
        "gaps": gaps,
        "indicators": rows,
        "failures": failures,
    }


def write_report(data: dict) -> None:
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    payload = {k: v for k, v in data.items() if k != "failures"}
    REPORT.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def main() -> int:
    if not REGISTRY.exists():
        print(f"check_indicator_parity_coverage: missing {REGISTRY}", file=sys.stderr)
        return 1

    try:
        data = collect_coverage()
    except ValueError as exc:
        print(f"check_indicator_parity_coverage: {exc}", file=sys.stderr)
        return 1

    write_report(data)

    if data["failures"]:
        print("check_indicator_parity_coverage: FAILED", file=sys.stderr)
        for item in data["failures"]:
            print(f"  - {item}", file=sys.stderr)
        print(
            f"  covered={data['parity_proptest_count']}/{data['indicator_count']} "
            f"exemptions={data['exemption_count']}",
            file=sys.stderr,
        )
        return 1

    print(
        "check_indicator_parity_coverage: OK "
        f"({data['parity_proptest_count']}/{data['indicator_count']} proptest, "
        f"{data['exemption_count']} exemptions)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())