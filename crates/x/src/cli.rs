#[derive(Debug, clap::Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) cmd: Command,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Command {
    Empty(crate::empty::Config),
    Whitespace(crate::whitespace::Config),
    Xref(crate::xref::Config),
    Wasm(crate::wasm::Config),
}
