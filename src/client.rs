use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub word: String,
    pub phonetics: Vec<Phonetic>,
    pub meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize)]
pub struct Phonetic {
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,

    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    pub brief: String,
    pub example: Option<String>,
}

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
