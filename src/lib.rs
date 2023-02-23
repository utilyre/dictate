use clap::Parser;
use colored::control;
use tokio::{fs, io::AsyncReadExt};
use xdg::BaseDirectories;

use cli::{Args, Color};

pub mod entry;
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

    let dirs = BaseDirectories::with_prefix("dictate").unwrap();
    let cache_path = dirs.place_cache_file("entries.json").unwrap();
    let mut cache_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&cache_path)
        .await
        .unwrap();

    let mut cache = String::new();
    cache_file.read_to_string(&mut cache).await.unwrap();
    if cache.is_empty() {
        cache = "[]".to_string();
    }

    let entries: Vec<Entry> = serde_json::from_str(&cache).unwrap();
    let mut entries: Vec<Entry> = entries
        .into_iter()
        .filter(|e| e.word.contains(&args.word))
        .collect();

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
