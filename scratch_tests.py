import re
import glob

core_dir = "quantwave-core/src/indicators"
talib_pattern = re.compile(r'talib_[^!]+!\s*\(\s*([A-Za-z0-9_]+)\s*,?')

indicators = []
for path in glob.glob(f"{core_dir}/*.rs"):
    with open(path, "r") as f:
        content = f.read()
        for match in talib_pattern.finditer(content):
            ind = match.group(1)
            # check if there's a test for it
            # usually test_ta_{ind.lower()}_parity or something similar
            indicators.append((ind, content))

missing_tests = []
for ind, content in indicators:
    ind_lower = ind.lower()
    # strip "ta" if it starts with it
    if ind_lower.startswith("ta"):
        short_ind = ind_lower[2:]
    else:
        short_ind = ind_lower
    
    # look for test_{ind_lower}_parity or test_{short_ind}_parity or similar
    if f"test_{ind_lower}_parity" not in content and f"test_{short_ind}_parity" not in content and f"test_ta_{short_ind}_parity" not in content:
        # Check if there is any proptest containing the name
        if not re.search(r'test_.*' + re.escape(short_ind) + r'.*_parity', content, re.IGNORECASE):
            missing_tests.append(ind)

print(f"Total indicators: {len(indicators)}")
print(f"Missing parity tests: {len(missing_tests)}")
for m in sorted(missing_tests):
    print(m)
