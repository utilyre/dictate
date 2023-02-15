use std::error::Error;

use reqwest::{Client, StatusCode};

use crate::entry::Entry;

pub async fn fetch_word(word: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(format!(
            "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
            word
        ))
        .send()
        .await;

    match response {
        Ok(response) if response.status() == StatusCode::OK => Ok(response.json().await?),
        Ok(response) if response.status() == StatusCode::NOT_FOUND => {
            Err(format!("word `{}` not found", word))?
        }
        Ok(response) => Err(format!("request failed ({})", response.status()))?,
        Err(err) => Err(Box::new(err)),
    }
}
