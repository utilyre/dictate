use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum When {
    Auto,
    Never,
    Always,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Word to look up
    pub word: String,

    /// When to use escape sequences
    #[arg(value_enum, short = 'C', long = "color", default_value = "auto")]
    pub color: When,

    /// When to use ASCII characters
    #[arg(short = 'a', long = "ascii", default_value = "auto")]
    pub ascii: When,
}
