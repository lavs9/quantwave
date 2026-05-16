import os
import re

indicators_dir = "quantwave-core/src/indicators/"
tips_dir = "references/traderstipsreference/"
papers_dir = "references/Ehlers Papers/"

implemented_indicators = [f[:-3] for f in os.listdir(indicators_dir) if f.endswith(".rs")]

def get_article_info(file_path):
    try:
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            # Look for author's article title in &ldquo; (smart quotes)
            article_match = re.search(r'article in this issue, &ldquo;(.*?)&rdquo;', content)
            if not article_match:
                article_match = re.search(r'article in this issue, "(.*?)"', content)
            
            title = article_match.group(1) if article_match else "Unknown Topic"
            
            # Look for indicators mentioned in TradeStation or MetaStock code blocks
            code_indicators = []
            # TradeStation Indicator: Name
            ts_matches = re.findall(r'Indicator:\s*(.*?)\n', content)
            code_indicators.extend(ts_matches)
            # MetaStock
            ms_matches = re.findall(r'<b>(.*?):</b>', content)
            code_indicators.extend(ms_matches)
            
            return title, code_indicators
                
    except Exception as e:
        return f"Error reading {file_path}", []

results = []

# Audit Tips
for tip in os.listdir(tips_dir):
    if not tip.endswith(".html"): continue
    path = os.path.join(tips_dir, tip)
    topic, code_inds = get_article_info(path)
    
    # Check if any of the code_indicators or the topic seems to be implemented
    is_implemented = False
    
    # Normalize for comparison
    norm_topic = topic.lower().replace(' ', '_').replace('-', '_')
    if norm_topic in implemented_indicators:
        is_implemented = True
    
    for ci in code_inds:
        norm_ci = ci.lower().replace(' ', '_').replace('-', '_')
        if norm_ci in implemented_indicators:
            is_implemented = True
            
    # Hardcoded known implementations
    if "Ultimate Channels" in topic and "ultimate_channel" in implemented_indicators: is_implemented = True
    if "AM and FM" in topic and "amfm" in implemented_indicators: is_implemented = True
    if "Vortex" in topic and "vortex" in implemented_indicators: is_implemented = True
    if "WaveTrend" in topic and "wavetrend" in implemented_indicators: is_implemented = True
    if "SuperTrend" in topic and "supertrend" in implemented_indicators: is_implemented = True

    results.append({
        "file": tip,
        "topic": topic,
        "indicators": code_inds,
        "implemented": is_implemented
    })

# Sort by date (roughly)
results.sort(key=lambda x: x["file"])

for r in results:
    status = "✅" if r["implemented"] else "❌"
    print(f"{status} | {r['file']} | {r['topic']} | {r['indicators']}")
