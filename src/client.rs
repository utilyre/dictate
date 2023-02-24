use std::error::Error;

use reqwest::{Client, StatusCode};

use crate::entry::Entry;

const BASE_URL: &str = "https://api.dictionaryapi.dev/api/v2";

pub async fn lookup_word(word: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(format!("{}/entries/en/{}", BASE_URL, word))
        .send()
        .await;

    match response {
        Ok(response) if response.status() == StatusCode::OK => Ok(response.json().await?),
        Ok(response) if response.status() == StatusCode::NOT_FOUND => {
            Err(format!("word `{}` not found", word).into())
        }
        Ok(response) => Err(format!("request failed ({})", response.status()).into()),
        Err(err) => Err(Box::new(err)),
    }
}
