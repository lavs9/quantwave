# QuantWave Indicator Documentation Standards **Official Standard** — Version 1.0 **Effective Date**: 2026-05-31 IST **Epic**: documentation program (Professionalize and Complete QuantWave Documentation Website) **Task**: documentation standards; combined with 6br5) **Applies To**: All 223+ pages in `docs/guides/indicators/native/*.md`, Ehlers DSP pages, and any future dedicated pages for rich struct/event indicators (Market Structure, Geometric Patterns, SRMonitor, etc.). This document defines the **mandatory, enforceable template** and guidelines for all indicator documentation. It replaces the previous inconsistent "established template" references. All new pages and all updates to existing pages **must** conform. ## Core Principles 1. **User-First & Practical**: Written for traders, quantitative developers, ML feature engineers, and strategy builders. Every section answers "What is it?", "Why/when use it?", "How do I use it in production code (both batch and streaming)?", "What can go wrong?", "What does it look like?", and "What else should I consider?". 2. **Parity & Fidelity**: Explicitly note that QuantWave guarantees bit-identical results between Polars batch (`.ta()` expressions) and streaming (`Next<T>` implementations). This is validated via proptests against gold-standard vectors in `/tests/gold_standard/`. Reference the universal indicator pattern (see `/src/traits.rs` and Agents.md). 3. **Source of Truth & Provenance** (per Agents.md): - The authoritative calculation **must** be recorded in the corresponding `pub const METADATA` in `/src/indicators/<name>.rs` (see `metadata.rs`). - Documentation **must** cite primary sources. Preferred (in priority order): - Original papers / books in `references/Ehlers Papers/implemented/` - MQL5 Price Action series (for PA tools) - TradingView Pine Script reference implementations - StockCharts, Sierra Chart, Devexperts, QuantConnect, TASC - TA-Lib source + explicit rule translation for patterns - **Never** rely solely on generic Investopedia as the primary source. Local references/ links are preferred when available. "Source of calculation" rule is non-negotiable — if uncertain, validate before writing. 4. **No Internal Leaks**: Zero references to beads (bfg, n6e7, etc.), agent notes, internal task IDs, or planning docs in public-facing content. (See prior decision in DOCUMENTATION_DECISIONS.md for PA cleanup.) 5. **Depth Over Breadth, Consistency**: Eliminate copy-paste (e.g. the identical "Steve Nison 1991" paragraph that appeared on 50+ candlestick pages). Tailor content by indicator type (see §4). Use the `IndicatorMetadata` (description, usage, ehlers_summary, params, formula_latex, formula_source) as the starting point — never duplicate it verbatim without enrichment. 6. **Visual Intuition Mandatory**: No page is complete without at least one visual (sparkline preview, annotated chart, or detailed placeholder + description). Visuals are generated/maintained via `docs/gen_indicator_previews.py` (extend as needed). 7. **Cross-Linking & Navigation**: Every page strengthens the site graph. Link to gallery, native index, related indicators, relevant notebooks, and references/. ## Required Page Structure (Enforceable Template) All pages **must** use exactly these top-level `##` headings in this order. Minor subsections (###) are allowed inside. ```markdown
# Human-Friendly Title (e.g. "Relative Strength Index (RSI)") <div class="indicator-meta">
<span class="category-badge">Classic</span>
<span class="kw-badge">momentum</span>
<span class="kw-badge">oscillator</span>
<span class="kw-badge">overbought</span>
<span class="kw-badge">oversold</span>
</div> One-paragraph lead (synthesized from metadata.description + significance for the target audience). ## Visual Example [Image or detailed placeholder + caption explaining what the reader should observe. REQUIRED.] ## Description Expanded practical explanation (2–4 paragraphs). Cover:
- What it measures and its theoretical foundation (DSP for Ehlers, market psychology for patterns).
- When and why a practitioner chooses it over alternatives.
- Common strategy / ML feature applications. ## Formula / Specification Full mathematical definition using KaTeX (`$$` blocks supported by mkdocs). For scalar/oscillator/bands: complete equations + recursive state where relevant. For candlestick / geometric patterns: **explicit step-by-step recognition logic or pseudocode** (never "TA-Lib Internal" or vague). Include confirmation filters if implemented. For rich struct/event indicators (MarketStructure, GeometricPatternScanner, etc.): full definition of all output structs / event types with field names, types, and semantics. Include any adaptive / state-machine behavior. ## Parameters Use a clean table (preferred) or bulleted list with defaults and descriptions. Align exactly with `METADATA.params`. | Parameter | Default | Description |
|-----------------|---------|-------------|
| `timeperiod` | 14 | Lookback window... | ## Usage Examples **Mandatory**. Show real, minimal, correct usage for the three surfaces. Use tabs or clear headings. Copy style from `docs/examples/batch-streaming.md` and the PA notebook. ### Streaming (Rust)
```rust
use quantwave_core::indicators::RSI;
use quantwave_core::traits::Next; let mut rsi = RSI::new(14);
for price in &prices { let value = rsi.next(*price); // ...
}
``` ### Streaming (Python)
```python
from quantwave import RSI rsi = RSI(14)
for price in prices: value = rsi.next(price)
``` ### Polars Batch (Python — primary research / feature surface)
```python
import polars as pl
import quantwave as qw # or: from quantwave import ta df = ( pl.read_csv("ohlcv.csv") .lazy() .with_columns([ pl.col("close").ta.rsi(14).alias("rsi"), # For multi-input: pl.col("high").ta.xxx(...) etc. ]) .collect()
)
``` For rich/event indicators, additionally demonstrate:
- Extracting struct fields (`.struct.field("...")`)
- Filtering confirmed events only
- Deriving ML features or position sizing (e.g. `pole_length_atr`) Always include a one-sentence parity note in this section. ## Edge Cases & Limitations Non-negotiable section (3–8 bullets or short paragraphs). Cover:
- Warm-up period / initial NaN or None values (exact count of bars before first valid output).
- Behavior on flat/zero-volatility series, division-by-zero, extreme outliers.
- Look-ahead bias (confirm none exists).
- Minimum viable data length.
- Performance characteristics (especially recursive Ehlers filters).
- When the indicator produces noisy or low-value signals (e.g. patterns in strong trends without confirmation).
- Any known divergences from reference implementations and why. ## Related Indicators & See Also 4–8 high-value links:
- Other native pages with similar purpose or common pairing (relative paths, e.g. `[Stochastic Oscillator](stochastic_oscillator.md)`).
- Gallery, native index, Ehlers index, PA notebook.
- Relevant references/ PDFs or MQL5 articles.
- Higher-level guides (regimes, ML features). Example ending:
*See also: [Indicator Gallery](guides/indicators/gallery.md), [Price Action Patterns notebook](examples/notebooks/pa_flag_breakout_strategy.md), [RSI example](guides/indicators/native/relative_strength_index_rsi.md)* ## Sources & References (footer area) **Primary Source**: [exact citation + link, preferably local `references/...` or authoritative URL] **Gold Standard Test Vector**: `/tests/gold_standard/xxx.json` (if applicable) **Implementation Provenance**: `/src/indicators/xxx.rs` (METADATA) ``` **Notes on structure**:
- The legacy `## Background` and `## Usage` (one-liner) sections are **deprecated**. Merge useful historical context into Description or a short "Context" subsection inside Description. Never paste generic web text.
- For pattern pages that previously shared identical Nison boilerplate: replace with specific, differentiated content per pattern. ## Tone, Depth, and Style Guidelines - **Tone**: Professional, precise, confident but humble. Practical over promotional. "Traders and quant developers use X to...", "This produces sparse, high-quality events suitable for...". Use "you" only in direct instructional code comments.
- **Depth**: - Classic scalar (SMA, RSI, MACD, SuperTrend, ATR...): 500–900 words + 3 code blocks + edges + visuals. - Patterns (candlesticks, fractals): 350–650 words focused on exact rules + psychology + confirmation + strong visuals. - Ehlers DSP: Emphasize lag reduction, cycle isolation, performance numbers (use the style already present in `ehlers/cyber_cycle.md` and enrich). - Rich PA / event-based (MarketStructure family): Model directly on the quality achieved in `examples/notebooks/pa_flag_breakout_strategy.md` (rich field tables, strategy integration, ML ideas, sizing examples, confirmation logic).
- **Formatting**: KaTeX for math, language-specific fences, consistent badge markup at top. Use admonitions sparingly for warnings.
- **Length & Scannability**: Short paragraphs. Tables preferred for params. Bullet lists for edges.
- **Dates**: Use IST (Asia/Kolkata) for any temporal references in examples or notes.
- **Cross-linking density**: At least 5–7 internal links per mature page. ## Type-Specific Guidance + Good vs. Current Typical Examples ### Type A: Classic Scalar Indicator (e.g. RSI, SMA, MACD, Keltner, SuperTrend) **Current Typical (pre-standards, unacceptable)** — from `relative_strength_index_rsi.md` and `simple_moving_average.md` (and 200+ siblings): ```markdown
# Relative Strength Index (RSI) <div class="indicator-meta">...badges...</div> A momentum oscillator that measures the speed and change of price movements. ## Usage
Use to identify overbought (>70) and oversold (<30) conditions... ## Background
> Developed by J. Welles Wilder... (generic 4-line blockquote) ## Parameters
- `timeperiod` (default: 14)... ## Formula
[RS formula]
[Source](https://www.investopedia.com/...)
``` **Problems**: No code, no visual, no edges, generic background, weak source, no ML/strategy depth, promises in gallery/index not delivered. **Good Example** (target quality — excerpt): ```markdown
# Relative Strength Index (RSI) <div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> ... </div> The Relative Strength Index (RSI) is a bounded momentum oscillator (0–100) that quantifies the magnitude of recent price gains versus losses over a lookback window. It is one of the most widely applied tools for identifying potential reversals via overbought/oversold levels and divergence analysis. ## Visual Example
![RSI (14) preview](../../../assets/indicator-previews/rsi.png){ width="420" } *Typical RSI oscillation between 30/70 bands with clear divergence at the second peak.* ## Description
... ## Formula / Specification
$$
RS = \frac{\text{Avg Gain}}{\text{Avg Loss}}, \quad RSI = 100 - \frac{100}{1 + RS}
$$
(Include Wilder’s exact smoothing method.) ## Parameters
... ## Usage Examples
[Full three surfaces with correct imports per batch-streaming.md] ## Edge Cases & Limitations
- First 14 periods typically return None / NaN while the average-gain window fills.
- In strong trends RSI can remain >70 or <30 for extended periods ("overbought can stay overbought").
- Low-volatility or thinly traded assets produce noisy RSI; combine with ATR or regime filters.
- ... ## Related Indicators & See Also
- [Stochastic Oscillator](stochastic_oscillator.md)
- [Chande Momentum Oscillator](chande_momentum_oscillator_cmo.md)
- [MarketState](marketstate.md) (for regime-aware RSI usage)
- ... **Primary Source**: Wilder, "New Concepts in Technical Trading Systems" (1978) + StockCharts School implementation details. **Gold Standard**: `/tests/gold_standard/...` (or applicable)
``` ### Type B: Candlestick / Simple Pattern (e.g. Engulfing, Morning Star, Doji family, Three White Soldiers) **Current Typical (unacceptable)** — identical across dozens of files: ```markdown
# Engulfing / Morning Star A pattern where a larger candle completely covers... ## Usage
Signals a strong shift in momentum. ## Background
> Candlestick patterns were popularized in the West by Steve Nison in his 1991 book... (exact same 3 sentences on 50+ pages) ## Formula
\text{Pattern Recognition Logic (TA-Lib Internal)}
[Source](investopedia bullish patterns)
``` **Good Example** (target — differentiated per pattern): ```markdown
# Bullish Engulfing <div class="indicator-meta"><span class="category-badge">Patterns</span> <span class="kw-badge">candlestick</span> <span class="kw-badge">reversal</span></div> A two-candle bullish reversal pattern in which a large green body completely engulfs the prior red body's range. ## Visual Example
[Placeholder chart: downtrend, small red candle, large green engulfing candle with volume spike. Annotation: "Engulfing body covers prior high-low entirely; close near high of pattern."] ## Description
The pattern reflects a sudden shift in control from sellers to buyers. The second candle opens within or below the prior body and closes well above the prior high. ## Formula / Specification
**Recognition Rules (exact implementation)**:
1. Prior bar: close < open (red/black body).
2. Current bar: close > open (green/white body).
3. Current open ≤ prior close.
4. Current close ≥ prior open.
5. (Optional filter in higher strategies: current volume > prior volume or > 20-bar avg.) Returns boolean (or signal enum) on the bar of completion. No smoothing or state carried between calls beyond the two-bar window. ## Parameters
None (pure pattern recognizer). Some surfaces accept a `confirm_volume` flag. ## Usage Examples
[Streaming Rust/Python showing accumulation of pattern signals; Polars `.ta.bullish_engulfing()` or equivalent returning boolean column + optional strength.] ## Edge Cases & Limitations
- Requires at least two complete bars; first bar of any series yields no signal.
- False positives are common in chop; always require trend context or higher-timeframe confirmation (see MarketStructure).
- Body-only vs wick-inclusive definitions exist in literature — QuantWave follows the TA-Lib-compatible body engulfment rule (document exact choice).
- ... ## Related...
**Primary Source**: TA-Lib pattern definitions cross-checked against Nison "Japanese Candlestick Charting Techniques" (1991) + MQL5 implementations. Exact logic in `/src/indicators/pattern.rs`.
``` ### Type C: Rich Event / Struct-Based (MarketState, Geometric Patterns, SRMonitor — future dedicated pages) **Current State**: MarketState has decent formula but lacks examples/edges/visuals. Rich PA tools intentionally documented primarily in the professional PA notebook (see n6e7 decision) + native index. **Good Example** (model after pa_flag... + expand for dedicated page): Follow the structure used in `pa_flag_breakout_strategy.md`:
- Rich output field tables with types and usage (bias, structure_strength, pole_length_atr, breakout_confirmed, etc.).
- Both streaming (`state, flag, hs = scanner.next((h,l))`) and Polars struct extraction.
- Visual placeholders with **specific annotations** ("BOS flip at bar 87, strength=3").
- Concrete strategy usage (sizing = f(pole_length_atr × ATR)) and ML feature list.
- Confirmation rules ("flips only after structure_count >= 2").
- Links back to the notebook for full runnable code.
- MQL5 article citations (Parts 21/66/67/69). Dedicated pages for these should be **supplementary** to the notebook, not duplicative. They focus on API surface + field reference + edge cases of the detector itself. ## Visual Standards - **Minimum**: One image or high-quality placeholder per page.
- **Preferred assets**: Place in `docs/assets/indicator-previews/<slug>.png`. Use the existing gen script (sparkline style for overview) or extend it to call real QuantWave indicators on representative synthetic data (trending + cyclic + noisy regimes).
- **For complex/PA**: Use detailed textual visual descriptions + markdown image placeholders. Example alt text must describe labels, arrows, and key observations.
- **Styling**: Respect theme (light/dark). Widths ~420px for previews; full-width for annotated charts via `width="100%"` or CSS.
- **Future**: Hook preview generation into mkdocs build and/or xtask so visuals stay in sync with code.
- **Placeholder discipline**: Always pair a placeholder with a precise caption of what the final annotated chart must show. ## Enforcement, Tooling & Maintenance - **contributing.md** (updated in this task) now points here explicitly.
- **Metadata sync rule** (already in metadata.rs): When modifying an indicator implementation, update its `METADATA` **and** the corresponding documentation page in the same session.
- **Generator alignment**: Future `` doc generator (see planning/quantwave/Documentation.md and metadata.rs comments) **must** emit pages that satisfy this template (skeleton from metadata + reserved slots for "Usage Examples", "Edge Cases", and "Visual").
- **Review checklist** (for contributors and PRs): 1. All required sections present and non-empty? 2. At least one visual (or explicit high-quality placeholder)? 3. Three usage surfaces with correct, project-consistent code? 4. Authoritative source cited (not generic web)? 5. No bead/agent/internal references? 6. Links to at least 4 related pages + gallery/index? 7. Edges & limitations section substantive? 8. Matches tone/depth for its type? ## Rollout Plan **Phase 0 (this task — complete)**: Publish this standards document + update contributing.md + append decision record to DOCUMENTATION_DECISIONS.md. Create 2–3 exemplar pages (one per type) that fully conform and can serve as copy-paste references. **Phase 1 (complete — 2026-06-25 IST)**: Bulk rollout via `docs/upgrade_to_standards.py` upgraded all 202 non-compliant native pages to the mandatory template (metadata-seeded, 3-surface examples, edge cases, sources; Nison boilerplate eliminated). Nineteen hand-upgraded exemplars with committed PNGs preserved. Gallery and native index updated. Lint: `python docs/upgrade_to_standards.py --lint`. **Phase 2 (complete — 2026-06-25 IST)**: `docs/generate_all_previews.py` + `docs/visual_utils.py` generate committed PNGs for all native indicator pages (161 sparklines/charts + 61 candle schematics). Run `python docs/generate_all_previews.py --sync-docs` to regenerate and embed. Hand-crafted exemplars from `gen_indicator_previews.py` / `gen_candle_previews.py` preserved. **Phase 3**: Implement/activate the xtask documentation generator so new indicators automatically receive a conforming skeleton (metadata + formula + params + stub sections for examples/edges). Manual enrichment remains required for examples, visuals, and edges. **Phase 4 (ongoing)**: Quarterly audit of a random 10% sample against this standard. Update standards document itself (append to this file) when new patterns emerge (e.g. regime-conditioned indicators). **Tracking**: All work logged under documentation epic. Individual page rewrites tracked as child tasks in bd. Related docs (changelog.md, roadmap.md, contributing.md, gallery, indices) updated in the same logical change sets. **Success Criteria**: 100% of indicator pages pass the review checklist above; gallery and index claims about "formulas, parameters, and usage examples" are accurate; no copy-paste background text remains; every page has a visual and at least one practical code example; sources are authoritative and recorded. --- *This standard was established after direct review of representative current pages (SMA, RSI, Bollinger Bands, Cyber Cycle, Engulfing, Morning Star, MarketState, Ichimoku, SuperTrend, plus the improved PA notebook and ehlers/ dedicated pages) and alignment with Agents.md, existing metadata system, MkDocs setup, preview generator, batch-streaming canonical examples, and prior PA documentation decisions (n6e7).* **Next action for maintainers**: Begin Phase 1 rewrites using the Good examples in this document as the quality bar.
