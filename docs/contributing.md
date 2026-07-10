# Contributing to QuantWave First off, thank you for considering contributing to QuantWave! It's people like you who make it such a great tool. ## Development Workflow ### Prerequisites
- Rust (2024 edition)
- Python 3.12+
- `cargo-nextest` ### Local Setup
1. Clone the repository: ```bash git clone https://github.com/lavs9/quantwave cd quantwave ```
2. Set up the virtual environment: ```bash python -m venv .venv source .venv/bin/activate pip install -r requirements-docs.txt ```
3. Build the project: ```bash cargo build ```
4. Install the pre-push quality gate (runs `./scripts/quantwave_verify.sh` before each push): ```bash ./scripts/install-git-hooks.sh ``` Emergency bypass: `SKIP_PRE_PUSH_VERIFY=1 git push` Force full rebuild: `VERIFY_NO_CACHE=1 git push` Cache status: `python3 scripts/verify_cache.py status` ### Running Tests
We use `nextest` for Rust tests:
```bash
cargo nextest run
```
For Python tests:
```bash
pytest
``` ## Adding a New Indicator 1. **Implement Core Logic**: Add the indicator to `/src/indicators/` implementing the `Next<T>` trait.
2. **Add Polars Expression**: Expose the indicator in `ns` or `s`.
3. **Write Tests**: - Unit tests in ``. - Parity tests (Streaming vs. Batch). - Add to `gold_standard` if applicable.
4. **Document**: Add the indicator to `metadata.rs` and regenerate the metadata registry (`python scripts/regenerate_metadata_registry.py`). Then generate its documentation skeleton using `python scripts/generate_native_docs.py` (which emits into `docs/guides/indicators/native/`). Hand-enrich the generated page with visuals following [DOCUMENTATION_STANDARDS.md](DOCUMENTATION_STANDARDS.md). Generate previews with `python docs/generate_all_previews.py --sync-docs`. Run `python docs/upgrade_to_standards.py --lint` and `python docs/upgrade_to_standards.py --depth-lint` before landing doc changes. ## Style Guidelines
- Follow idiomatic Rust (run `cargo clippy`).
- Ensure all public functions have docstrings.
- Keep performance in mind; avoid unnecessary allocations. ## Reporting Issues
Please use the GitHub issue tracker to report bugs or request features.
