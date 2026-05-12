import os
import re

INDICATORS_DIR = "/Users/mayanklavania/moonshot_projects/quantwave/quantwave-core/src/indicators"

EHLERS_INDICATORS = ["frama.rs", "cyber_cycle.rs", "wavetrend.rs"]

def update_file(filename):
    filepath = os.path.join(INDICATORS_DIR, filename)
    with open(filepath, 'r') as f:
        content = f.read()

    if "IndicatorMetadata = IndicatorMetadata {" not in content:
        return

    if "category:" in content:
        print(f"Skipping {filename}, category already exists")
        return

    category = "Ehlers DSP" if filename in EHLERS_INDICATORS else "Classic"
    
    # Find the IndicatorMetadata block
    # We want to insert 'category: "...",' before the closing '};' of the struct literal
    
    # Use regex to find the end of the struct literal
    # It usually looks like:
    #     gold_standard_file: "...",
    # };
    
    match = re.search(r'(gold_standard_file:\s*"[^"]*",?\s*)\};', content, re.DOTALL)
    if match:
        replacement = match.group(1) + f'    category: "{category}",\n}};'
        new_content = content[:match.start()] + replacement + content[match.end():]
        with open(filepath, 'w') as f:
            f.write(new_content)
        print(f"Updated {filename} with category: {category}")
    else:
        print(f"Could not find metadata end in {filename}")

if __name__ == "__main__":
    for filename in os.listdir(INDICATORS_DIR):
        if filename.endswith(".rs") and filename != "mod.rs" and filename != "metadata.rs":
            update_file(filename)
