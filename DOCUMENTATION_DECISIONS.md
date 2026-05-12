# Documentation System Decisions (Grill-Me)

This document tracks the decisions made regarding the documentation architecture outlined in `planning/quantwave/Documentation.md`. 
We need to reach a shared understanding on the implementation details. 

## Round 1: Indicator Metadata & talib-rs Wrappers

### Question 1: Metadata Struct Location & Pattern
The plan calls for an `IndicatorMetadata` struct and an instance per indicator. We have 27 files in `quantwave-core/src/indicators/`.
**Option A**: Add `IndicatorMetadata` directly in `quantwave-core/src/indicators/mod.rs` (or a `metadata.rs`) and require every indicator file to export a `pub const METADATA: IndicatorMetadata`.
**Option B**: Create a separate lightweight crate `quantwave-metadata` so the `xtask` generator doesn't need to compile the heavy `quantwave-core` dependencies to extract metadata.

*Recommended Answer*: Option A is simpler, but compiling `quantwave-core` in an `xtask` to read metadata can be slow or tricky unless you use `syn` to parse the AST rather than compiling it. Since the plan mentions using `syn + quote`, Option A works best (we just parse the `.rs` files as text using `syn`).

<Answer>
Option A is ok to go ahead. 
</Answer>

### Question 2: talib-rs Wrapper Metadata Auto-generation
We have 158 talib-rs wrappers. Manually writing metadata for these is tedious. 
**Option A**: Write a Rust macro in `quantwave-core` that automatically injects a `METADATA` constant when wrapping the talib-rs functions.
**Option B**: The `xtask` documentation generator can automatically scrape the talib C header or documentation (or a mapping config) and generate the metadata on the fly without it existing in the `quantwave-core` source.

*Recommended Answer*: Option A. By modifying `talib_wrapper.rs` (or however they are currently generated) to inject a `METADATA` struct for each wrapper, the source of truth remains in the code, and the `xtask` generator can uniformly parse all indicators using `syn`.

<Answer>
For talib-rs the metadata should be available in the library itself ideally. If not can we scrape from the C header or generate from there ?
</Answer>

## Finalized Decisions

1. **IndicatorMetadata Struct**: We will go with Option A. `IndicatorMetadata` and `ParamDef` structs will be defined in `quantwave-core/src/indicators/metadata.rs`. Every native indicator module will export a `pub const METADATA: IndicatorMetadata`. The `xtask` documentation generator will parse these Rust files using `syn` to extract the metadata without compiling the core library.

2. **TA-Lib Metadata Extraction**: Since `talib-rs` does not expose rich metadata (descriptions, default params, etc.), we will have the `xtask` generator parse the official `ta_func_api.xml` (which TA-Lib uses internally to describe all 158 functions). The generator will automatically map the Rust wrapper names to the XML entries to auto-generate the mdBook pages, completely avoiding manual metadata entry for the 158 wrappers.
