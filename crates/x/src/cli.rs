#[derive(Debug, clap::Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) cmd: Command,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Command {
    Empty(crate::empty::Config),
    Lint(crate::lint::Config),
    Whitespace(crate::whitespace::Config),
    Xref(crate::xref::Config),
}
