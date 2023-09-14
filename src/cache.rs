use std::fs as std_fs;
use std::path::PathBuf;

use directories::BaseDirs;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, Error, ErrorKind, Result};

use crate::entry::Entry;

pub struct Cache {
    file: File,
}

impl Cache {
    fn get_path() -> Result<PathBuf> {
        let dirs = BaseDirs::new()
            .ok_or(Error::new(ErrorKind::Other, "Cannot determine cache directory because no valid home directory path could be retrieved from the operating system."))?
            .cache_dir()
            .join("dictate");
        std_fs::create_dir_all(&dirs)?;
        Ok(dirs.join("entries.json"))
    }

    pub async fn open(opts: &mut OpenOptions) -> Result<Self> {
        Ok(Self {
            file: opts.append(false).open(&Self::get_path()?).await?,
        })
    }

    async fn get_entries(&mut self) -> Result<Vec<Entry>> {
        let mut json = String::new();
        self.file.seek(std::io::SeekFrom::Start(0)).await?;
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
            .filter(|e| e.word == word)
            .collect())
    }

    pub async fn append(&mut self, entries: &mut Vec<Entry>) -> Result<()> {
        let mut entries_cache = self.get_entries().await?;
        entries_cache.append(entries);

        let json = serde_json::to_string(&entries_cache)?;
        self.file.seek(std::io::SeekFrom::Start(0)).await?;
        self.file.write_all(json.as_bytes()).await?;

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
