use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Word to look up in dictionary.
    pub word: String,
}
