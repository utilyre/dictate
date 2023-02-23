use clap::Parser;

use cli::{Args, Color};

pub mod entry;
use colored::control;
pub use entry::Entry;

pub mod client;

pub mod cli;

pub struct Error {
    pub code: i32,
    pub message: String,
}

pub async fn run() -> Result<(), Error> {
    let args = Args::parse();

    match args.color {
        Color::Auto => (),
        Color::Never => control::set_override(false),
        Color::Always => control::set_override(true),
    }

    let entries = client::fetch_word(&args.word).await.or_else(|e| {
        Err(Error {
            code: 1,
            message: e.to_string(),
        })
    })?;

    for (i, entry) in entries.iter().enumerate() {
        println!("{}", entry);
        if i < entries.len() - 1 {
            println!();
        }
    }

    Ok(())
}
