import re
import os

with open('core_talib.txt', 'r') as f:
    core_inds = [line.strip() for line in f if line.strip()]

with open('quantwave-polars/src/lib.rs', 'r') as f:
    polars_content = f.read()

missing = []
found = 0
for ind in core_inds:
    pattern_generic = re.compile(rf'<{ind}>', re.IGNORECASE)
    
    if not pattern_generic.search(polars_content):
        func_name = ind.lower()
        if ind.startswith("Ta"):
            func_name = "ta_" + ind[2:].lower()
            
        if f"fn {func_name}(" not in polars_content:
            missing.append(ind)
        else:
            found += 1
    else:
        found += 1

print(f"Total checked: {len(core_inds)}")
print(f"Found: {found}")
print(f"Missing: {len(missing)}")
for m in sorted(missing):
    print(m)
