use std::{
    error::Error,
    fmt::{self, Display},
};

use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub word: String,
    pub phonetics: Vec<Phonetic>,
    pub meanings: Vec<Meaning>,
}

impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stringified = format!(
            "{}{}{}",
            "".white(),
            self.word.bold().black().on_white(),
            "".white()
        );

        if let Some(phonetic) = self.phonetics.iter().find(|p| p.text.is_some()) {
            let text = phonetic
                .text
                .as_ref()
                .expect("should never be the None variant");

            stringified.push_str(&format!(" {}", text.italic().bright_black()))
        }

        write!(f, "{}", stringified)
    }
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
    #[serde(rename = "definition")]
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
