//! Unified PyO3 (abi3) extension for quantwave (quantwave-6dgg).
//!
//! One cdylib carries three surfaces that used to be three separate crates/wheels:
//!   * `indicators` — the streaming classes + batch fns (was `quantwave._quantwave`)
//!   * `backtest`   — the backtest engine bindings (was `quantwave._backtest`)
//!   * `plugins`    — the `#[polars_expr]` Polars expression plugins (the `.ta` surface)
//!
//! The two class/function surfaces are exposed as submodules registered into
//! `sys.modules` under their historical dotted names, so `from quantwave import
//! _quantwave` / `_backtest` keep working unchanged. The Polars plugins need no
//! Python module — their symbols live in this cdylib and Polars discovers them via
//! `register_plugin_function(plugin_path=<the package dir>)`.

use pyo3::prelude::*;

mod backtest;
mod bars;
mod indicators;
pub mod plugins;

#[pymodule]
fn _lib(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let sys_modules = py.import("sys")?.getattr("modules")?;

    // Indicators -> quantwave._quantwave
    let indicators_mod = PyModule::new(py, "_quantwave")?;
    indicators::register(&indicators_mod)?;
    sys_modules.set_item("quantwave._quantwave", &indicators_mod)?;
    m.add_submodule(&indicators_mod)?;

    // Backtest -> quantwave._backtest
    let backtest_mod = PyModule::new(py, "_backtest")?;
    backtest::register(&backtest_mod)?;
    sys_modules.set_item("quantwave._backtest", &backtest_mod)?;
    m.add_submodule(&backtest_mod)?;

    // Alternative bar construction -> quantwave._bars
    let bars_mod = PyModule::new(py, "_bars")?;
    bars::register(&bars_mod)?;
    sys_modules.set_item("quantwave._bars", &bars_mod)?;
    m.add_submodule(&bars_mod)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
