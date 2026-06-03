import re
import os
import glob

core_dir = "quantwave-core/src/indicators"
polars_lib = "quantwave-polars/src/lib.rs"

talib_indicators = []
pattern = re.compile(r'talib_[^!]+!\s*\(\s*([A-Za-z0-9_]+)\s*,?')

for path in glob.glob(f"{core_dir}/*.rs"):
    with open(path, "r") as f:
        for line in f:
            m = pattern.search(line)
            if m:
                indicator = m.group(1).lower()
                talib_indicators.append(indicator)

print(f"Total talib indicators in core: {len(talib_indicators)}")

with open(polars_lib, "r") as f:
    polars_content = f.read().lower()

missing = []
for ind in talib_indicators:
    if f"fn {ind}(" not in polars_content and f"<{ind}>" not in polars_content:
        missing.append(ind)

print(f"Missing indicators in polars: {len(missing)}")
for m in sorted(missing):
    print(m)
