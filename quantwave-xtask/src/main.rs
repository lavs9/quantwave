use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("Generating documentation...");

    let workspace_root =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()))
            .parent()
            .unwrap()
            .to_path_buf();
    let indicators_base = workspace_root.join("docs/guides/indicators");
    let indicators_dir = workspace_root.join("quantwave-core/src/indicators");

    if !workspace_root.exists() {
        return Err(anyhow::anyhow!("Workspace root does not exist: {:?}", workspace_root));
    }

    if !indicators_dir.exists() {
        return Err(anyhow::anyhow!("Indicators directory does not exist: {:?}", indicators_dir));
    }

    fs::create_dir_all(indicators_base.join("native")).context("Failed to create indicators/native directory")?;

    // Remove legacy TA-Lib wrapper docs (all indicators are native now).
    let talib_dir = indicators_base.join("talib");
    if talib_dir.exists() {
        fs::remove_dir_all(&talib_dir).context("Failed to remove legacy indicators/talib directory")?;
    }

    run_python(&workspace_root, "scripts/generate_native_docs.py")?;
    run_python(&workspace_root, "scripts/sync_indicator_docs.py")?;

    println!("Documentation generation complete.");
    Ok(())
}

fn run_python(workspace_root: &PathBuf, script: &str) -> Result<()> {
    println!("Running {script}...");
    let status = std::process::Command::new("python3")
        .arg(workspace_root.join(script))
        .current_dir(workspace_root)
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("{script} failed"));
    }
    Ok(())
}