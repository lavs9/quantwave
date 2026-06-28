import re

file_path = "docs/upgrade_to_standards.py"
with open(file_path, "r") as f:
    content = f.read()

# 1. Add boundary_kind to IndicatorRecord
content = re.sub(
    r'    category: str = ""\n',
    '    category: str = ""\n    boundary_kind: str = ""\n',
    content
)

# 2. Add depth_lint_violations
depth_lint_func = """
def depth_lint_violations(rec: IndicatorRecord, content: str) -> list[str]:
    issues = []
    
    # 1. Description duplication
    desc_match = re.search(r"## Description\\n\\n(.*?)(?=\\n\\n|\\n##)", content, re.S)
    if desc_match:
        first_para = desc_match.group(1).strip()
        clean_para = re.sub(r'[*_]', '', first_para).strip()
        clean_meta = re.sub(r'[*_]', '', rec.description).strip()
        if clean_para == clean_meta:
            issues.append("Description duplication (verbatim copy of metadata)")
    
    # 2. Generic edge bullets
    edge_match = re.search(r"## Edge Cases & Limitations\\n\\n(.*?)(?=\\n\\n|\\n##)", content, re.S)
    if edge_match:
        edges_text = edge_match.group(1).strip()
        if "universal Next<T> trait" in edges_text:
            issues.append("Generic edge bullets (contains bulk boilerplate)")
        bullets = [b for b in edges_text.split("\\n") if b.strip().startswith("- ")]
        if len(bullets) < 4:
            issues.append(f"Generic edge bullets (only {len(bullets)} bullets, need 4+)")
    else:
        issues.append("Missing Edge Cases section")
        
    # 3. Broken Polars example
    usage_match = re.search(r"## Usage Examples\\n\\n(.*?)(?=\\n\\n|\\n##)", content, re.S)
    if usage_match:
        usage_text = usage_match.group(1)
        if re.search(r"\\.ta\\.\\w+\\(\\"close\\",", usage_text):
            issues.append("Broken Polars example (wrong signature `.ta.<method>(\\"close\\",`)")
        if "map_batches" in usage_text:
            issues.append("Broken Polars example (uses map_batches instead of native plugin)")
            
    # 4. Missing warmup specificity
    if rec.category.lower() != "patterns" and not rec.is_pattern and rec.boundary_kind == "scalar":
        if edge_match:
            edges_text = edge_match.group(1).lower()
            if "warm-up" in edges_text or "warmup" in edges_text:
                if not re.search(r"\\d+", edges_text):
                    issues.append("Missing warmup specificity (no numeric bar count)")
            else:
                issues.append("Missing warmup specificity (no mention of warmup)")
                
    # 5. Low word count
    between_match = re.search(r"## Description\\n(.*?)\\n## Formula", content, re.S)
    if between_match:
        words = len(between_match.group(1).split())
        min_words = 50 if rec.is_pattern else 80
        if words < min_words:
            issues.append(f"Low word count ({words} words, need {min_words}+)")
            
    return issues
"""

content = content.replace("def lint_violations(content: str) -> list[str]:", depth_lint_func + "\n\ndef lint_violations(content: str) -> list[str]:")

# 3. Add depth_lint_all
depth_lint_all_func = """
def depth_lint_all() -> int:
    failures = 0
    metadata = parse_metadata_files()
    for md_path in sorted(NATIVE_DOCS.glob("*.md")):
        if md_path.name in SKIP_FILES:
            continue
        slug = SLUG_ALIASES.get(md_path.stem, md_path.stem)
        rec = metadata.get(slug)
        if not rec:
            rec = next((r for r in metadata.values() if r.slug == slug), None)
        if not rec:
            continue
            
        issues = depth_lint_violations(rec, md_path.read_text(encoding="utf-8"))
        if issues:
            failures += 1
            print(f"FAIL {md_path.name}: {', '.join(issues)}")
    if failures:
        print(f"\\n{failures} pages failed depth lint")
        return 1
    print("All native indicator pages pass depth lint")
    return 0
"""
content = content.replace("def lint_all() -> int:", depth_lint_all_func + "\n\ndef lint_all() -> int:")

# 4. Add arg parse
argparse_block = """    parser.add_argument("--depth-lint", action="store_true", help="Depth lint pages against standards")
    args = parser.parse_args()

    if args.depth_lint:
        return depth_lint_all()
"""
content = re.sub(r'    args = parser.parse_args\(\)\n\n    if args.lint:', argparse_block + '\n    if args.lint:', content)

# 5. Add render_boundary
boundary_block = """
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
    return "## Boundary Behavior\\n\\n" + "\\n".join(lines) + "\\n"
"""

content = content.replace("def render_page(rec: IndicatorRecord, api: dict[str, dict]) -> str:", boundary_block + "\ndef render_page(rec: IndicatorRecord, api: dict[str, dict]) -> str:")
content = content.replace("        render_edges(rec),", "        render_edges(rec),\n        render_boundary(rec),")

with open(file_path, "w") as f:
    f.write(content)
