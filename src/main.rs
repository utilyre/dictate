use std::process;

use clap::Parser;
use colored::control;
use dictate::{
    cache::Cache,
    cli::{Args, Color},
    client,
};
use tokio::fs::OpenOptions;

struct Error {
    code: i32,
    message: String,
}

#[tokio::main]
async fn main() {
    run().await.unwrap_or_else(|e| {
        eprintln!("dictate: {}", e.message);
        process::exit(e.code);
    });
}

async fn run() -> Result<(), Error> {
    let args = Args::parse();

    match args.color {
        Color::Auto => (),
        Color::Never => control::set_override(false),
        Color::Always => control::set_override(true),
    }

    let mut cache = Cache::open(OpenOptions::new().read(true).write(true).create(true))
        .await
        .or_else(|e| {
            Err(Error {
                code: 1,
                message: e.to_string(),
            })
        })?;

    let entries = match cache.lookup_word(&args.word).await {
        Ok(entries) => {
            if entries.is_empty() {
                let entries = client::lookup_word(&args.word).await.or_else(|e| {
                    Err(Error {
                        code: 1,
                        message: e.to_string(),
                    })
                })?;

                cache.append(&mut entries.clone()).await.or_else(|e| {
                    Err(Error {
                        code: 1,
                        message: e.to_string(),
                    })
                })?;

                entries
            } else {
                entries
            }
        }
        Err(err) => {
            return Err(Error {
                code: 1,
                message: err.to_string(),
            })
        }
    };

    for (i, entry) in entries.iter().enumerate() {
        println!("{}", entry);
        if i < entries.len() - 1 {
            println!();
        }
    }

    Ok(())
}
