import os
import re

indicators_dir = "/Users/mayanklavania/moonshot_projects/quantwave/quantwave-core/src/indicators/"
tips_dir = "/Users/mayanklavania/moonshot_projects/quantwave/references/traderstipsreference/"
implemented_tips_dir = os.path.join(tips_dir, "implemented/")

# Get implemented indicator filenames (base names without .rs)
implemented_indicators = [f[:-3] for f in os.listdir(indicators_dir) if f.endswith(".rs")]

# Get all tips files
tips_files = [f for f in os.listdir(tips_dir) if f.endswith(".html")]
implemented_tips = [f for f in os.listdir(implemented_tips_dir) if f.endswith(".html")]

audit_results = []

for tip_file in tips_files:
    file_path = os.path.join(tips_dir, tip_file)
    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    # Extract title or main topic
    title_match = re.search(r'<title>(.*?)</title>', content, re.IGNORECASE)
    title = title_match.group(1) if title_match else tip_file
    
    # Check for keywords in content to guess indicators
    # This is a bit rough but helpful
    mentions = []
    for ind in implemented_indicators:
        if ind.lower() in content.lower():
            mentions.append(ind)
    
    audit_results.append({
        "file": tip_file,
        "title": title,
        "implemented": tip_file in implemented_tips,
        "potential_matches": list(set(mentions))
    })

# Print report
print("| File | Topic | Implemented | Matches |")
print("| --- | --- | --- | --- |")
for res in sorted(audit_results, key=lambda x: x["file"]):
    impl_str = "✅" if res["implemented"] else "❌"
    matches = ", ".join(res["potential_matches"][:5]) + ("..." if len(res["potential_matches"]) > 5 else "")
    print(f"| {res['file']} | {res['title']} | {impl_str} | {matches} |")
