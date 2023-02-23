use tokio::fs::{self, File};
use tokio::io::{self, AsyncReadExt};
use xdg::BaseDirectories;

use crate::entry::Entry;

async fn get_cache_file() -> io::Result<File> {
    let dirs = BaseDirectories::with_prefix("dictate")?;
    let cache_path = dirs.place_cache_file("entries.json")?;
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&cache_path)
        .await
}

pub async fn find(word: &str) -> io::Result<Vec<Entry>> {
    let mut cache_file = get_cache_file().await?;

    let mut cache = String::new();
    cache_file.read_to_string(&mut cache).await?;
    if cache.is_empty() {
        cache = "[]".to_string();
    }

    let entries: Vec<Entry> = serde_json::from_str(&cache)?;
    Ok(entries
        .into_iter()
        .filter(|e| e.word.contains(word))
        .collect())
}
