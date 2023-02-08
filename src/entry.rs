use std::fmt::{self, Display};

use colored::Colorize;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entry {
    word: String,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
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

            stringified.push_str(&format!(" {}\n", text.italic().bright_black()));
        }

        for meaning in self.meanings.iter() {
            stringified.push_str(&format!(
                "\n{:indent$}{}",
                "",
                meaning.part_of_speech.italic().underline(),
                indent = 2
            ));

            for definition in meaning.definitions.iter() {
                stringified.push_str(&format!(
                    "\n{:indent$}{} {}\n",
                    "",
                    "•".blue(),
                    definition.brief,
                    indent = 4
                ));

                if let Some(example) = &definition.example {
                    stringified.push_str(&format!(
                        "{:indent$}{}\n",
                        "",
                        format!("\"{}\"", example).bright_black(),
                        indent = 6
                    ));
                }
            }
        }

        write!(f, "{}", stringified)
    }
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
    #[serde(rename = "definition")]
    brief: String,
    example: Option<String>,
}
