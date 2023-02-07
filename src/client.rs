use std::error::Error;

use crate::entry::Entry;

pub async fn fetch_word(word: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    let entries = reqwest::get(format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        word
    ))
    .await?
    .json::<Vec<Entry>>()
    .await?;

    Ok(entries)
}
