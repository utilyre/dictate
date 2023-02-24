use std::{error::Error, process};

use clap::Parser;
use colored::control;
use dictate::{
    cache::Cache,
    cli::{Args, Color},
    client,
};
use tokio::fs::OpenOptions;

#[tokio::main]
async fn main() {
    run().await.unwrap_or_else(|e| {
        eprintln!("dictate: {}", e.to_string());
        process::exit(1);
    });
}

async fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.color {
        Color::Auto => (),
        Color::Never => control::set_override(false),
        Color::Always => control::set_override(true),
    }

    let mut cache = Cache::open(OpenOptions::new().read(true).write(true).create(true)).await?;
    let mut entries = cache.lookup_word(&args.word).await?;
    if entries.is_empty() {
        entries = client::lookup_word(&args.word).await?;
        cache.append(&mut entries.clone()).await?;
    }

    for (i, entry) in entries.iter().enumerate() {
        println!("{}", entry);
        if i < entries.len() - 1 {
            println!();
        }
    }

    Ok(())
}
