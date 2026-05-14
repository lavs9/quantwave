import os
import re

indicators_dir = "/Users/mayanklavania/moonshot_projects/quantwave/quantwave-core/src/indicators/"
tips_dir = "/Users/mayanklavania/moonshot_projects/quantwave/references/traderstipsreference/"
implemented_tips_dir = os.path.join(tips_dir, "implemented/")

# Get all tips files
tips_files = [f for f in os.listdir(tips_dir) if f.endswith(".html")]
implemented_tips = [f for f in os.listdir(implemented_tips_dir) if f.endswith(".html")]

ehlers_audit = []

for tip_file in tips_files:
    file_path = os.path.join(tips_dir, tip_file)
    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    # Filter for Ehlers
    if "ehlers" not in content.lower():
        continue
        
    title_match = re.search(r'<title>(.*?)</title>', content, re.IGNORECASE)
    title = title_match.group(1) if title_match else tip_file
    
    ehlers_audit.append({
        "file": tip_file,
        "title": title,
        "implemented": tip_file in implemented_tips
    })

print("| File | Topic | Implemented |")
print("| --- | --- | --- |")
for res in sorted(ehlers_audit, key=lambda x: x["file"]):
    impl_str = "✅" if res["implemented"] else "❌"
    print(f"| {res['file']} | {res['title']} | {impl_str} |")
