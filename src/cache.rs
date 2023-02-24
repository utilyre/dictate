use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use xdg::BaseDirectories;

use crate::entry::Entry;

pub struct Cache {
    file: File,
}

impl Cache {
    pub async fn open(opts: &mut OpenOptions) -> Result<Self> {
        let dirs = BaseDirectories::with_prefix("dictate")?;

        let filename = dirs.place_cache_file("entries.json")?;
        let file = opts.append(false).open(&filename).await?;

        Ok(Self { file })
    }

    async fn get_entries(&mut self) -> Result<Vec<Entry>> {
        let mut json = String::new();
        self.file.read_to_string(&mut json).await?;
        if json.is_empty() {
            json = "[]".to_string();
        }

        Ok(serde_json::from_str(&json)?)
    }

    pub async fn lookup_word(&mut self, word: &str) -> Result<Vec<Entry>> {
        let entries_cache = self.get_entries().await?;

        Ok(entries_cache
            .into_iter()
            .filter(|e| e.word.contains(word))
            .collect())
    }

    pub async fn append(&mut self, entries: &Vec<Entry>) -> Result<()> {
        let mut entries_cache = self.get_entries().await?;
        entries_cache.append(&mut entries.clone());

        let json = serde_json::to_string(&entries_cache)?;
        self.file.write(json.as_bytes()).await?;

        Ok(())
    }
}
