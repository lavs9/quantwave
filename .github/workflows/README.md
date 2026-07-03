# GitHub Actions

Two workflows. Everything else was merged here for clarity.

## CI (`ci.yml`)

**Triggers:** push or PR to `main`, or manual `workflow_dispatch`.

```
sanity ──┬──► plugins (only if quantwave-plugins/ changed)
         └──► deploy-docs (main push only)
```

| Job | What it runs | When |
|-----|----------------|------|
| **Doc & metadata sanity** | `check_metadata_drift`, `check_doc_drift`, `check_public_metadata` | Always (~1 min) |
| **Plugin wheels** | Build `quantwave-plugins` wheel + pytest | `quantwave-plugins/**` changed, or manual dispatch |
| **Deploy docs** | mkdocs → GitHub Pages | `main` push only, after sanity |

**Full quality gate (local, pre-push):** `./scripts/install-git-hooks.sh` then push as usual.  
**Manual run:** `./scripts/quantwave_verify.sh`

## Release (`release.yml`)

**Triggers:** push tag `v*` (e.g. `v0.6.0`).

```
publish-rust ──► build-python-wheels (matrix) ──► publish-python (PyPI)
```

| Job | What |
|-----|------|
| **Publish Rust crates** | `scripts/publish_crates.sh` — core → backtest/plugins → polars → quantwave (idempotent) |
| **Python wheels** | Unified wheel via `scripts/build_unified_wheel.py` (core + backtest + plugins) on linux x64, linux arm64, macOS, Windows |
| **Publish Python** | `twine upload` to PyPI |

**Secrets:** `CRATES_IO_TOKEN`, `PYPI_API_TOKEN`