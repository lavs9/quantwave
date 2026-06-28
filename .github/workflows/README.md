# GitHub Actions

Two workflows. Everything else was merged here for clarity.

## CI (`ci.yml`)

**Triggers:** push or PR to `main`, or manual `workflow_dispatch`.

```
verify ──┬──► plugins (only if quantwave-plugins/ changed)
         └──► deploy-docs (main push only)
```

| Job | What it runs | When |
|-----|----------------|------|
| **Quality gate** | `./scripts/quantwave_verify.sh` — metadata drift, doc lint/depth, nextest, pytest | Always |
| **Plugin wheels** | Build `quantwave-plugins` wheel + pytest | `quantwave-plugins/**` changed, or manual dispatch |
| **Deploy docs** | mkdocs → GitHub Pages | `main` push only, after quality gate |

**Local parity:** `./scripts/quantwave_verify.sh`

## Release (`release.yml`)

**Triggers:** push tag `v*` (e.g. `v0.6.0`).

```
publish-rust ──► build-python-wheels (matrix) ──► publish-python (PyPI)
              └──► build-python-sdist ────────────┘
```

| Job | What |
|-----|------|
| **Publish Rust crates** | `cargo publish` chain on crates.io |
| **Python wheels** | linux x64, linux arm64, macOS, Windows |
| **Python sdist** | source tarball |
| **Publish Python** | `twine upload` to PyPI |

**Secrets:** `CRATES_IO_TOKEN`, `PYPI_API_TOKEN`