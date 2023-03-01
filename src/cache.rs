use std::io::SeekFrom;
use std::path::PathBuf;

use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, Result};
use xdg::BaseDirectories;

use crate::entry::Entry;

pub struct Cache {
    file: File,
}

impl Cache {
    fn get_path() -> Result<PathBuf> {
        let dirs = BaseDirectories::with_prefix("dictate")?;
        Ok(dirs.place_cache_file("entries.json")?)
    }

    pub async fn open(opts: &mut OpenOptions) -> Result<Self> {
        Ok(Self {
            file: opts.append(false).open(&Self::get_path()?).await?,
        })
    }

    async fn get_entries(&mut self) -> Result<Vec<Entry>> {
        let mut json = String::new();
        self.file.seek(SeekFrom::Start(0)).await?;
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

    pub async fn append(&mut self, entries: &mut Vec<Entry>) -> Result<()> {
        let mut entries_cache = self.get_entries().await?;
        entries_cache.append(entries);

        let json = serde_json::to_string(&entries_cache)?;
        self.file.seek(SeekFrom::Start(0)).await?;
        self.file.write(json.as_bytes()).await?;

        Ok(())
    }

    pub async fn clean(self) -> Result<()> {
        drop(self.file);

        let mut parent = Self::get_path()?;
        parent.pop();
        fs::remove_dir_all(parent).await?;

        Ok(())
    }
}
