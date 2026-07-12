import os
import re

POLARS_LIB_PATH = "quantwave-polars/src/lib.rs"
PLUGINS_SRC_DIR = "quantwave-py/src/plugins/"

implemented = set()
for file in os.listdir(PLUGINS_SRC_DIR):
    if not file.endswith(".rs"): continue
    with open(os.path.join(PLUGINS_SRC_DIR, file), "r") as f:
        content = f.read()
        matches = re.findall(r'fn\s+([a-zA-Z0-9_]+)\s*\(\s*inputs', content)
        for m in matches:
            implemented.add(m.lower())

print(f"Already implemented plugins: {len(implemented)}")

with open(POLARS_LIB_PATH, "r") as f:
    polars_content = f.read()

# match pub fn name(self, ...) -> LazyFrame
methods = re.findall(r'pub fn ([a-zA-Z0-9_]+)\s*\(\s*self[^)]*\)\s*->\s*LazyFrame', polars_content)
all_methods = set(m.lower() for m in methods)

missing = all_methods - implemented
print(f"Total in polars: {len(all_methods)}")
print(f"Missing in plugins: {len(missing)}")

# Group missing by the pattern they match in quantwave-polars/src/lib.rs
# E.g. self.math_transform_1_in_1_out
patterns = {}
for m in missing:
    # find the body of the function
    # simple heuristic: find pub fn m(self ... ) -> LazyFrame { ... }
    body_match = re.search(r'pub fn ' + m + r'\s*\([^)]*\)\s*->\s*LazyFrame\s*\{([^}]*)\}', polars_content)
    if body_match:
        body = body_match.group(1).strip()
        # extract self.pattern::<
        pattern_match = re.search(r'self\.([a-zA-Z0-9_]+)::<', body)
        if pattern_match:
            pat = pattern_match.group(1)
            patterns.setdefault(pat, []).append(m)
        else:
            patterns.setdefault("custom", []).append(m)

import json
print(json.dumps(patterns, indent=2))
