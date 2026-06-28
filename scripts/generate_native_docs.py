import json
import argparse
import sys
from pathlib import Path
ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))
from docs.upgrade_to_standards import IndicatorRecord, render_page, parse_polars_api, SKIP_FILES, is_compliant

ROOT = Path(__file__).resolve().parent.parent
NATIVE_DOCS = ROOT / "docs" / "guides" / "indicators" / "native"

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--force", action="store_true", help="Overwrite existing hand-enriched files")
    args = parser.parse_args()

    metadata_file = ROOT / "metadata_export.json"
    with open(metadata_file) as f:
        data = json.load(f)

    api = parse_polars_api()
    skipped = 0
    generated = 0

    for item in data:
        slug = item["slug"]
        rec = IndicatorRecord(
            const_name="",
            struct_name=item["struct_name"],
            source_file=Path(item["source_file"]),
            name=item["name"],
            description=item["description"],
            usage=item["usage"],
            keywords=item.get("keywords", []),
            ehlers_summary=item.get("ehlers_summary", ""),
            params=[type("ParamDef", (), p) for p in item.get("params", [])],
            formula_source=item.get("formula_source", ""),
            formula_latex=item.get("formula_latex", ""),
            gold_standard_file=item.get("gold_standard_file", ""),
            category=item.get("category", ""),
            boundary_kind=item.get("boundary_kind", "scalar")
        )
        
        # Legacy xtask generated filenames from the indicator name
        raw_name_for_file = rec.name.lower()
        filename = "".join(c if c.isalnum() else "_" for c in raw_name_for_file)
        filename = "_".join(part for part in filename.split("_") if part)
        
        md_path = NATIVE_DOCS / f"{filename}.md"
        if md_path.name in SKIP_FILES:
            skipped += 1
            continue
            
        is_existing_compliant = False
        if md_path.exists():
            content = md_path.read_text(encoding="utf-8")
            # If a visual example was added, it's considered hand-enriched.
            if is_compliant(content):
                is_existing_compliant = True
                
        if is_existing_compliant and not args.force:
            skipped += 1
            continue

        md_path.parent.mkdir(parents=True, exist_ok=True)
        md_path.write_text(render_page(rec, api), encoding="utf-8")
        generated += 1

    print(f"Done: generated={generated}, skipped_enriched={skipped}")

if __name__ == "__main__":
    main()
