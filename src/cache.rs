use std::path::PathBuf;

use tokio::fs;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use xdg::BaseDirectories;

use crate::entry::Entry;

async fn get_cache_path() -> io::Result<PathBuf> {
    let dirs = BaseDirectories::with_prefix("dictate")?;
    dirs.place_cache_file("entries.json")
}

async fn get_entries() -> io::Result<Vec<Entry>> {
    let mut cache_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&get_cache_path().await?)
        .await?;

    let mut cache = String::new();
    cache_file.read_to_string(&mut cache).await?;
    if cache.is_empty() {
        cache = "[]".to_string();
    }

    Ok(serde_json::from_str::<Vec<Entry>>(&cache)?)
}

pub async fn find(word: &str) -> io::Result<Vec<Entry>> {
    let entries = get_entries().await?;

    Ok(entries
        .into_iter()
        .filter(|e| e.word.contains(word))
        .collect())
}

pub async fn append(entries: &mut Vec<Entry>) -> io::Result<()> {
    let mut cache = get_entries().await?;
    cache.append(entries);

    let mut cache_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(get_cache_path().await?)
        .await?;

    let json = serde_json::to_string(&cache)?;
    cache_file.write(json.as_bytes()).await?;

    Ok(())
}
