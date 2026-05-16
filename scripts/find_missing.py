import os
import re

indicators_dir = "quantwave-core/src/indicators/"
tips_dir = "references/traderstipsreference/"
papers_dir = "references/Ehlers Papers/"

implemented_indicators = [f[:-3] for f in os.listdir(indicators_dir) if f.endswith(".rs")]

def get_title(file_path):
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            title_match = re.search(r'<title>(.*?)</title>', content, re.IGNORECASE)
            if title_match:
                return title_match.group(1).replace("TRADERS&rsquo; TIPS - ", "").replace("TRADERS’ TIPS - ", "")
            
            # If no title, look for first <h1> or specific text
            h1_match = re.search(r'<h1.*?>(.*?)</h1>', content, re.IGNORECASE)
            if h1_match:
                return h1_match.group(1)
            
            # Look for author's article title in <div> or <p>
            article_match = re.search(r'article in this issue, &ldquo;(.*?)&rdquo;', content)
            if article_match:
                return article_match.group(1)
                
    except Exception as e:
        return f"Error reading {file_path}: {e}"
    return os.path.basename(file_path)

missing = []

# Audit Tips
for tip in os.listdir(tips_dir):
    if not tip.endswith(".html"): continue
    path = os.path.join(tips_dir, tip)
    title = get_title(path)
    
    # Try to map title to implemented indicators
    # This is hard because titles don't always match filenames.
    # We'll use the content search as a hint.
    with open(path, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read().lower()
    
    is_implemented = False
    matches = []
    for ind in implemented_indicators:
        if ind.lower().replace('_', ' ') in content or ind.lower() in content:
            # Check if it's a real match or just a common word
            if len(ind) > 3: # Skip very short names for mapping
                matches.append(ind)
    
    # Heuristics for implementation check
    if "ultimate channel" in title.lower() and "ultimate_channel" in implemented_indicators: is_implemented = True
    if "ultimate bands" in title.lower() and "ultimate_bands" in implemented_indicators: is_implemented = True
    if "am and fm" in content and "amfm" in implemented_indicators: is_implemented = True
    
    # If we found direct matches in the text, we might still be missing the *main* indicator of the tip
    # But if it's already in 'implemented/' folder, we ignore it.
    
    missing.append({
        "file": tip,
        "title": title,
        "matches": matches,
        "type": "tip"
    })

# Audit Papers
for paper in os.listdir(papers_dir):
    if not paper.endswith(".pdf"): continue
    missing.append({
        "file": paper,
        "title": paper.replace(".pdf", ""),
        "matches": [],
        "type": "paper"
    })

for m in missing:
    print(f"FILE: {m['file']} | TITLE: {m['title']} | MATCHES: {m['matches']}")
