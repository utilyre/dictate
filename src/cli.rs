use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum Color {
    Auto,
    Never,
    Always,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Word to look up in dictionary.
    pub word: String,

    /// Whether to use escape sequences
    #[arg(value_enum, short = 'C', long = "color", default_value = "auto")]
    pub color: Color,
}
