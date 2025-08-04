use std::{
    fs,
    io::{self, Read as _},
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use clap::Parser as _;
use lesshand::{dec, enc};

/// Command-line interface for Lesshand, a shorthand for the 21st century
#[derive(Debug, clap::Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) cmd: Command,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Command {
    Decode(Decode),
    Encode(Encode),
}

/// Decode (decompress) Lesshand into English
#[derive(Debug, clap::Parser)]
pub(crate) struct Decode {
    /// Path to file to decode, pass "-" to decode stdin
    pub(crate) path: PathBuf,
}

/// Encode (compress) Lesshand into English
#[derive(Debug, clap::Parser)]
pub(crate) struct Encode {
    /// Path to file to encode, pass "-" to encode stdin
    pub(crate) path: PathBuf,
}

fn read_file(p: &Path) -> anyhow::Result<String> {
    fs::read_to_string(p).with_context(|| format!("Failed to read file {}", p.display()))
}

#[inline]
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

fn decode(d: Decode) -> Result<(), anyhow::Error> {
    let s = read_path(d.path.as_path())?;
    let decoded = dec::dec_with_default_rules(s.as_str());
    println!("{decoded}");
    Ok(())
}

fn encode(e: Encode) -> Result<(), anyhow::Error> {
    let s = read_path(e.path.as_path())?;
    let encoded = enc::enc_with_default_rules(s.as_str());
    println!("{encoded}");
    Ok(())
}

fn dispatch(cli: Cli) -> anyhow::Result<()> {
    match cli.cmd {
        Command::Decode(d) => decode(d),
        Command::Encode(e) => encode(e),
    }
}

fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}
