# GitHub Actions

Two workflows. Everything else was merged here for clarity.

## CI (`ci.yml`)

**Triggers:** push or PR to `main`, or manual `workflow_dispatch`.

```
changes ──┬──► sanity (always)
          ├──► rust-gate (Rust paths changed)
          ├──► python-gold-parity (Linux + macOS, Python/gold paths changed)
          ├──► plugins (quantwave-plugins/ changed)
          └──► deploy-docs (main push only, after sanity)
```

| Job | What it runs | When |
|-----|----------------|------|
| **Doc & metadata sanity** | metadata/doc/benchmark/hygiene drift checks | Always (~1 min) |
| **Rust quality gate** | `cargo nextest` (core/polars/backtest) | `**/*.rs`, `Cargo.*` changed, or manual dispatch |
| **Python gold parity** | 25+ streaming indicators vs `gold_standard/*.json` | `tests/python`, `quantwave-python`, gold fixtures changed |
| **Plugin wheels** | Build `quantwave-plugins` wheel + pytest | `quantwave-plugins/**` changed, or manual dispatch |
| **Deploy docs** | mkdocs → GitHub Pages | `main` push only, after sanity |

**Full quality gate (local, pre-push):** `./scripts/install-git-hooks.sh` — superset of CI (unified wheel + full pytest).  
**Manual run:** `./scripts/quantwave_verify.sh`

## Release (`release.yml`)

**Triggers:** push tag `v*` (e.g. `v0.6.0`).

```
publish-rust ──► build-python-wheels (matrix) ──► verify-python-wheel ──► publish-python (PyPI)
```

| Job | What |
|-----|------|
| **Publish Rust crates** | `scripts/publish_crates.sh` — core → backtest/plugins → polars → quantwave (idempotent) |
| **Python wheels** | Unified wheel via `scripts/build_unified_wheel.py` (core + backtest + plugins) on linux x64, linux arm64, macOS, Windows |
| **Verify wheel** | Import + RSI smoke on Python 3.9 / 3.11 / 3.12 / 3.13 |
| **Publish Python** | `pypa/gh-action-pypi-publish` via PyPI OIDC trusted publisher (no API token) |

**Secrets:** `CRATES_IO_TOKEN` only. PyPI uses a [trusted publisher](https://docs.pypi.org/trusted-publishers/) on `lavs9/quantwave`, workflow filename `release.yml` (case-sensitive), environment `(Any)`.