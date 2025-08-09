use std::{env, fs, process::Command};

#[derive(Debug, clap::Args)]
pub(crate) struct Config {}

pub(crate) fn go(_conf: Config) -> Result<(), Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    let out_dir = cwd.join("doc/wasm");
    fs::create_dir_all(&out_dir)?;
    let status = Command::new("wasm-pack")
        .args(["build", "--target", "web", "--release"])
        .arg("--out-dir")
        .arg(out_dir.join("pkg"))
        .arg("crates/wasm")
        .status();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => {
            Err(format!(
                "wasm-pack failed with exit code: {}. Install it with `cargo install wasm-pack` if missing.",
                s.code().map_or_else(|| "unknown".to_string(), |c| c.to_string())
            ).into())
        }
        Err(e) => {
            Err(format!(
                "Failed to execute wasm-pack: {e}. Install it with `cargo install wasm-pack` if missing."
            ).into())
        }
    }
}
