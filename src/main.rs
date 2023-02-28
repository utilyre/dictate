use std::{error::Error, process};

use clap::Parser;
use colored::control;
use dictate::{
    cache::Cache,
    charset,
    cli::{Args, When},
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

    // TODO: extract to its own function
     match args.color {
        When::Auto => (),
        When::Never => {
            control::set_override(false);
            charset::set_override(false);
        }
        When::Always => {
            control::set_override(true);
            charset::set_override(true);
        }
    };

    let mut cache = Cache::open(OpenOptions::new().read(true).write(true).create(true)).await?;
    let mut entries = cache.lookup_word(&args.word).await?;
    if entries.is_empty() {
        entries = client::lookup_word(&args.word).await?;
        let entries = entries.clone();
        cache.append(&mut entries.clone()).await?;
    }

    // TODO: only print empty line on the first iteration
    for entry in entries.iter() {
        println!();
        println!("{}", entry);
    }

    Ok(())
}
