use std::{error::Error, fs, path::PathBuf, process};

/// Detect empty files
#[derive(Debug, clap::Parser)]
pub(crate) struct Config {
    paths: Vec<PathBuf>,
}

pub(crate) fn check(path: PathBuf) -> Result<bool, Box<dyn Error>> {
    let mut stack = vec![path];
    let mut ok = true;
    while let Some(path) = stack.pop() {
        if path.is_file() {
            let sz = fs::metadata(&path)?.len();
            let empty = sz == 0;
            if empty {
                ok = false;
                eprintln!("{}: empty", path.display());
            }
            continue;
        }
        if path.is_dir() {
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                stack.push(entry.path().clone());
            }
        }
    }
    Ok(ok)
}

pub(super) fn go(mut conf: Config) -> Result<(), Box<dyn Error>> {
    let mut ok = false;
    conf.paths.dedup();
    for path in conf.paths {
        ok |= check(path)?;
    }
    if !ok {
        process::exit(1);
    }
    Ok(())
}
