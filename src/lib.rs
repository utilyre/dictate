use clap::Parser;
use cli::{Args, Color};
use colored::control;

mod cache;
mod cli;
mod client;
mod entry;

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

    let mut entries = cache::find(&args.word).await.or_else(|e| {
        Err(Error {
            code: 1,
            message: e.to_string(),
        })
    })?;

    if entries.is_empty() {
        entries = client::fetch_word(&args.word).await.or_else(|e| {
            Err(Error {
                code: 1,
                message: e.to_string(),
            })
        })?;
    }

    for (i, entry) in entries.iter().enumerate() {
        println!("{}", entry);
        if i < entries.len() - 1 {
            println!();
        }
    }

    Ok(())
}
