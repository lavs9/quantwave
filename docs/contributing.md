# Contributing to QuantWave

First off, thank you for considering contributing to QuantWave! It's people like you who make it such a great tool.

## Development Workflow

### Prerequisites
- Rust (2024 edition)
- Python 3.12+
- `cargo-nextest`

### Local Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/lavs9/quantwave
   cd quantwave
   ```
2. Set up the virtual environment:
   ```bash
   python -m venv .venv
   source .venv/bin/activate
   pip install -r requirements-docs.txt
   ```
3. Build the project:
   ```bash
   cargo build
   ```

### Running Tests
We use `nextest` for Rust tests:
```bash
cargo nextest run
```
For Python tests:
```bash
pytest
```

## Adding a New Indicator

1. **Implement Core Logic**: Add the indicator to `quantwave-core/src/indicators/` implementing the `Next<T>` trait.
2. **Add Polars Expression**: Expose the indicator in `quantwave-plugins` or `quantwave-polars`.
3. **Write Tests**: 
   - Unit tests in `quantwave-core`.
   - Parity tests (Streaming vs. Batch).
   - Add to `gold_standard` if applicable.
4. **Document**: Add a new page in `docs/guides/indicators/` following the established template.

## Style Guidelines
- Follow idiomatic Rust (run `cargo clippy`).
- Ensure all public functions have docstrings.
- Keep performance in mind; avoid unnecessary allocations.

## Reporting Issues
Please use the GitHub issue tracker to report bugs or request features.
