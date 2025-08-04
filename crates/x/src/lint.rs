use std::{env, error::Error, ffi::OsStr, fs, path::PathBuf, process};

use crate::{empty, whitespace::fix_whitespace};

/// Lint
#[derive(Debug, clap::Parser)]
pub(crate) struct Config {
    #[command(subcommand)]
    pub(crate) tool: Tool,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Tool {
    All,
    Clippy,
    Empty,
    Mdlynx,
    Ruff,
    Typos,
    Whitespace,
}

fn files_with_ext(
    dir: PathBuf,
    ext: &str,
    recursive: bool,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut paths = Vec::<PathBuf>::with_capacity(16); // guess
    let mut stack = Vec::<PathBuf>::with_capacity(16); // guess
    stack.push(dir);
    while let Some(path) = stack.pop() {
        if path.is_file()
            && path
                .as_os_str()
                .as_encoded_bytes()
                .ends_with(ext.as_bytes())
        {
            paths.push(path.clone());
        }
        if path.is_dir() && (recursive || stack.is_empty()) {
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                stack.push(entry.path().clone());
            }
        }
    }
    Ok(paths)
}

fn run<T: AsRef<OsStr>>(command: &str, args: &[T]) -> Result<(), Box<dyn Error>> {
    let arg_strs = args
        .iter()
        .map(|a| a.as_ref().to_string_lossy())
        .collect::<Vec<_>>();
    eprintln!("{command} {}", arg_strs.join(" "));
    let output = process::Command::new(command)
        .args(args)
        .stdin(process::Stdio::inherit())
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .output()?;
    if !output.status.success() {
        process::exit(1);
    }
    Ok(())
}

fn lint_clippy() -> Result<(), Box<dyn Error>> {
    run("cargo", &["fmt", "--check"])?;
    run(
        "cargo",
        &[
            "--quiet",
            "clippy",
            "--all-targets",
            "--",
            "--deny",
            "warnings",
        ],
    )?;
    Ok(())
}

fn lint_empty() -> Result<(), Box<dyn Error>> {
    let mut ok = true;
    let here = PathBuf::from(".");
    let mds = files_with_ext(here.clone(), ".md", true)?;
    for path in mds {
        ok |= empty::check(path)?;
    }
    let pys = files_with_ext(here.clone(), ".py", true)?;
    for path in pys {
        ok |= empty::check(path)?;
    }
    let rss = files_with_ext(here.clone(), ".rs", true)?;
    for path in rss {
        ok |= empty::check(path)?;
    }
    let shs = files_with_ext(here.clone(), ".sh", true)?;
    for path in shs {
        ok |= empty::check(path)?;
    }
    if !ok {
        process::exit(1);
    }
    Ok(())
}

fn lint_mdlynx() -> Result<(), Box<dyn Error>> {
    let here = PathBuf::from(".");
    env::set_current_dir("doc/dev/")?;
    let mds = files_with_ext(here.clone(), ".md", false)?;
    run("mdlynx", mds.as_slice())?;
    env::set_current_dir("../../")?;
    // TODO: needs support for header links:
    /*
    run("mdlynx", &["README.md"])?;
    env::set_current_dir("doc/")?;
    let mds = files_with_ext(here.clone(), ".md", false)?;
    run("mdlynx", mds.as_slice())?;
    env::set_current_dir("../../")?;
    */
    Ok(())
}

fn lint_ruff() -> Result<(), Box<dyn Error>> {
    let here = PathBuf::from(".");
    let py_paths = files_with_ext(here.clone(), ".py", true)?;
    let pys = py_paths
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    // ref: ruff-format
    let mut fmt = vec![
        "--quiet".to_owned(),
        "format".to_owned(),
        "--check".to_owned(),
    ];
    fmt.extend(pys.clone());
    run("ruff", &fmt)?;
    // ref: ruff-check
    let mut check = vec!["--quiet".to_owned(), "check".to_owned()];
    check.extend(pys.clone());
    run("ruff", &check)?;
    Ok(())
}

fn lint_typos() -> Result<(), Box<dyn Error>> {
    let here = PathBuf::from(".");
    let all_mds = files_with_ext(here.clone(), ".md", true)?;
    run("typos", all_mds.as_slice())?;
    Ok(())
}

fn lint_whitespace() -> Result<(), Box<dyn Error>> {
    let mut ok = true;
    let here = PathBuf::from(".");
    let mds = files_with_ext(here.clone(), ".md", true)?;
    for path in mds {
        ok |= fix_whitespace(&path, true, /* tab width */ 4)?;
    }
    let shs = files_with_ext(here.clone(), ".sh", false)?;
    for path in shs {
        ok |= fix_whitespace(&path, true, /* tab width */ 4)?;
    }
    if !ok {
        process::exit(1);
    }
    Ok(())
}

pub(super) fn go(conf: Config) -> Result<(), Box<dyn Error>> {
    match conf.tool {
        Tool::All => {
            lint_clippy()?;
            lint_empty()?;
            lint_mdlynx()?;
            lint_ruff()?;
            lint_typos()?;
            lint_whitespace()?;
            Ok(())
        }
        Tool::Clippy => lint_clippy(),
        Tool::Empty => lint_empty(),
        Tool::Mdlynx => lint_mdlynx(),
        Tool::Ruff => lint_ruff(),
        Tool::Typos => lint_typos(),
        Tool::Whitespace => lint_whitespace(),
    }
}

#[cfg(test)]
mod test {
    use std::{error::Error, path::PathBuf};

    #[test]
    fn test_files_with_ext() -> Result<(), Box<dyn Error>> {
        let here = PathBuf::from(".");
        let tomls = super::files_with_ext(here, ".toml", false)?;
        assert!(!tomls.is_empty());
        Ok(())
    }
}
