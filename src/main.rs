use serde::Deserialize;
use std::{env, error::Error, process};

#[derive(Debug, Deserialize)]
struct Entry {
    word: String,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize)]
struct Phonetic {
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Definition>,

    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Definition {
    definition: String,
    example: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let Some(word) = args.next() else {
        println!("expected word as second argument");
        process::exit(2);
    };

    let body = reqwest::get(format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        word
    ))
    .await?
    .json::<Vec<Entry>>()
    .await?;

    dbg!(body);

    Ok(())
}
