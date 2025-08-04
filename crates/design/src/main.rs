mod metrics;
mod string;
mod top;

use std::{
    fs,
    io::{self, Read as _},
    iter,
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use clap::Parser as _;
use lesshand::{
    abbrev::ABBREVS, dec::dec_with_default_rules, enc::enc_with_default_rules, reqs::check,
    words::WORDS,
};

/// Tool for designing Lesshand
#[derive(Debug, clap::Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) cmd: Command,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Command {
    Abbrevs,
    Debug(Debug),
    Metrics(Metrics),
    Suggest(Suggest),
    Top(Top),
}

/// Debug issues
#[derive(Debug, clap::Parser)]
pub(crate) struct Debug {
    pub(crate) which: DebugType,
    /// Path to file, pass "-" to use stdin
    pub(crate) path: PathBuf,
}

/// Print encoding and decoding metrics for a file
#[derive(Clone, Debug, clap::ValueEnum)]
pub(crate) enum DebugType {
    Decode,
    Double,
    Roundtrip,
}

/// Print encoding and decoding metrics for a file
#[derive(Debug, clap::Parser)]
pub(crate) struct Metrics {
    /// Path to file, pass "-" to use stdin
    pub(crate) path: PathBuf,
}

/// Suggest words to abbreviate
#[derive(Debug, clap::Parser)]
pub(crate) struct Suggest {
    #[clap(short, long, default_value_t = 100)]
    pub(crate) limit: usize,
    #[clap(short, long, default_value_t = 3)]
    pub(crate) min_length: usize,
}

/// Show top rules
#[derive(Debug, clap::Parser)]
pub(crate) struct Top {
    #[clap(short, long, default_value_t = 25)]
    pub(crate) limit: usize,
    /// Path to file, pass "-" to use stdin
    pub(crate) path: PathBuf,
}

#[allow(dead_code)] // TODO
fn read_file(p: &Path) -> anyhow::Result<String> {
    fs::read_to_string(p).with_context(|| format!("Failed to read file {}", p.display()))
}

fn stdin_string() -> anyhow::Result<String> {
    let cap = 256; // tiny strings are unlikely, what's the use?
    let mut stdin_str: String = String::with_capacity(cap);
    io::stdin()
        .read_to_string(&mut stdin_str)
        .context("Failed to read input from stdin")?;
    Ok(stdin_str)
}

fn read_path(p: &Path) -> anyhow::Result<String> {
    if p.as_os_str().as_encoded_bytes() == "-".as_bytes() {
        return stdin_string();
    }
    read_file(p)
}

fn abbrevs() {
    for (long, short, _savings) in ABBREVS {
        println!("{long} --> {short}");
    }
}

fn do_diff(s0: &str, s: &str) {
    for (l0, l) in iter::zip(s0.lines(), s.lines()) {
        // Encoding / decoding always change length, and don't introduce or
        // remove newlines.
        if l0.len() != l.len() {
            let diff = prettydiff::diff_words(l0, l);
            eprintln!("{diff}");
        }
    }
}

fn debug(d: Debug) -> anyhow::Result<()> {
    let s0 = read_path(d.path.as_path())?;
    match d.which {
        DebugType::Decode => {
            let s = dec_with_default_rules(&s0);
            do_diff(&s0, &s);
            Ok(())
        }
        DebugType::Double => {
            let s0 = enc_with_default_rules(&s0);
            let s = enc_with_default_rules(&s0);
            do_diff(&s0, &s);
            Ok(())
        }
        DebugType::Roundtrip => {
            let s = dec_with_default_rules(&enc_with_default_rules(&s0));
            do_diff(&s0, &s);
            Ok(())
        }
    }
}

fn metrics(m: Metrics) -> anyhow::Result<()> {
    let s = read_path(m.path.as_path())?;
    let m = metrics::metrics(&s);
    m.print();
    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
fn suggest(s: Suggest) -> Result<(), anyhow::Error> {
    'word: for (i0, word) in (WORDS[0..s.limit]).iter().copied().enumerate() {
        if word.len() < s.min_length {
            continue;
        }
        for (long, _short, _) in ABBREVS.iter().copied() {
            if long == word {
                continue 'word;
            }
        }
        let abbrev = format!(
            "{}{}",
            word.chars().next().unwrap_or('?'),
            word.chars().last().unwrap_or('?')
        );
        let mut abbrev_ok = true;
        for (_, short, _) in ABBREVS.iter().copied() {
            if abbrev == short {
                abbrev_ok = false;
                break;
            }
        }
        for (i, w) in WORDS.iter().copied().enumerate() {
            if i == i0 {
                continue;
            }
            abbrev_ok &= !abbrev.starts_with(w.chars().next().unwrap_or('?'))
                || !abbrev.ends_with(w.chars().last().unwrap_or('?'));
        }
        let abbrevs = &[(word, abbrev.as_str())];
        abbrev_ok &= check(abbrevs).is_ok();
        if abbrev_ok && !WORDS.contains(&abbrev.as_str()) {
            println!("{word}: {abbrev}");
        } else {
            println!("{word}");
        }
    }
    Ok(())
}

fn top(t: Top) -> Result<(), anyhow::Error> {
    let s0 = read_path(t.path.as_path())?;
    top::top(io::stderr(), &s0, t.limit)?;
    Ok(())
}

fn dispatch(cli: Cli) -> anyhow::Result<()> {
    match cli.cmd {
        Command::Abbrevs => {
            abbrevs();
            Ok(())
        }
        Command::Debug(d) => debug(d),
        Command::Metrics(m) => metrics(m),
        Command::Suggest(s) => suggest(s),
        Command::Top(t) => top(t),
    }
}

fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}
