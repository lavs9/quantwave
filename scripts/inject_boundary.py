import json
import re
from pathlib import Path
import sys
ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))
from docs.upgrade_to_standards import IndicatorRecord, render_boundary

def main():
    metadata_file = ROOT / "metadata_export.json"
    with open(metadata_file) as f:
        data = json.load(f)

    for item in data:
        slug = item["slug"]
        rec = IndicatorRecord(
            const_name="",
            struct_name=item["struct_name"],
            source_file=Path(item["source_file"]),
            name=item["name"],
            description=item["description"],
            usage=item["usage"],
            boundary_kind=item.get("boundary_kind", "scalar")
        )
        
        raw_name_for_file = rec.name.lower()
        filename = "".join(c if c.isalnum() else "_" for c in raw_name_for_file)
        filename = "_".join(part for part in filename.split("_") if part)
        
        md_path = ROOT / "docs" / "guides" / "indicators" / "native" / f"{filename}.md"
        if not md_path.exists():
            continue
            
        content = md_path.read_text(encoding="utf-8")
        if "## Boundary Behavior" in content:
            continue
            
        boundary_text = render_boundary(rec)
        
        if "## Related Indicators & See Also" in content:
            content = content.replace("## Related Indicators & See Also", boundary_text + "\n## Related Indicators & See Also")
            md_path.write_text(content, encoding="utf-8")
        else:
            print(f"Warning: No 'Related Indicators' section in {md_path.name}")

if __name__ == "__main__":
    main()
