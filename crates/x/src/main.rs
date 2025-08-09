mod cli;
mod empty;
mod lint;
mod wasm;
mod whitespace;
mod xref;

use std::error::Error;

use cli::{Cli, Command};

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = clap::Parser::parse();
    match cli.cmd {
        Command::Empty(conf) => empty::go(conf)?,
        Command::Lint(conf) => lint::go(conf)?,
        Command::Whitespace(conf) => whitespace::go(conf)?,
        Command::Xref(conf) => xref::go(conf)?,
        Command::Wasm(conf) => wasm::go(conf)?,
    }
    Ok(())
}
