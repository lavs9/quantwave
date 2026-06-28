import json
import sys
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))
from docs.upgrade_to_standards import SKIP_FILES

NATIVE_DOCS = ROOT / "docs" / "guides" / "indicators" / "native"
METADATA_FILE = ROOT / "metadata_export.json"

def main():
    # Run export_metadata to ensure we have the latest
    subprocess.run(["cargo", "run", "--bin", "export_metadata"], cwd=ROOT, check=True)
    
    if not METADATA_FILE.exists():
        print("metadata_export.json not found!", file=sys.stderr)
        sys.exit(1)
        
    with open(METADATA_FILE) as f:
        data = json.load(f)
        
    expected_files = set()
    for item in data:
        # Match legacy xtask naming logic
        raw_name = item["name"].lower()
        filename = "".join(c if c.isalnum() else "_" for c in raw_name)
        filename = "_".join(part for part in filename.split("_") if part)
        md_name = f"{filename}.md"
        if md_name not in SKIP_FILES:
            expected_files.add(md_name)
        
    actual_files = {p.name for p in NATIVE_DOCS.glob("*.md") if p.name not in SKIP_FILES}
    
    missing = expected_files - actual_files
    orphans = actual_files - expected_files
    
    failed = False
    if missing:
        print("FAIL: Missing documentation pages for registered indicators:", file=sys.stderr)
        for m in sorted(missing):
            print(f"  - {m}", file=sys.stderr)
        failed = True
        
    if orphans:
        print("FAIL: Orphan documentation pages found (no corresponding indicator in registry):", file=sys.stderr)
        for o in sorted(orphans):
            print(f"  - {o}", file=sys.stderr)
        failed = True
        
    if failed:
        sys.exit(1)
        
    print("Doc drift check passed: native pages perfectly match metadata registry.")

if __name__ == "__main__":
    main()
