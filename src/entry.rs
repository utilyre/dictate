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
            stringified.push_str(&format!(" {}", phonetic));
        }

        stringified.push('\n');
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

            if !meaning.synonyms.is_empty() || !meaning.antonyms.is_empty() {
                stringified.push('\n');
            }
            if !meaning.synonyms.is_empty() {
                stringified.push_str(&format!(
                    "{:indent$}{} {}\n",
                    "",
                    "Synonyms:".green(),
                    meaning
                        .synonyms
                        .iter()
                        .map(|s| format!("{}{}{}", "".white(), s.black().on_white(), "".white()))
                        .collect::<Vec<String>>()
                        .join(" "),
                    indent = 4
                ));
            }
            if !meaning.antonyms.is_empty() {
                stringified.push_str(&format!(
                    "{:indent$}{} {}\n",
                    "",
                    "Antonyms:".green(),
                    meaning
                        .antonyms
                        .iter()
                        .map(|s| format!("{}{}{}", "".white(), s.black().on_white(), "".white()))
                        .collect::<Vec<String>>()
                        .join(" "),
                    indent = 4
                ));
            }
        }

        write!(f, "{}", stringified)
    }
}

#[derive(Debug, Deserialize)]
struct Phonetic {
    text: Option<String>,
}

impl Display for Phonetic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.text
                .clone()
                .unwrap_or("bruh".to_string())
                .italic()
                .bright_black()
        )
    }
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
