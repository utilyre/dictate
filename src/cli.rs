use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum When {
    Auto,
    Never,
    Always,
}

#[derive(Parser)]
#[command(version, author, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// When to use escape sequences
    #[arg(value_enum, short = 'C', long = "color", default_value = "auto")]
    pub color: When,
}

#[derive(Subcommand)]
pub enum Command {
    /// Lookup word in dictionary
    #[clap(alias = "search")]
    Lookup {
        /// Word to lookup
        word: String,
    },
}
