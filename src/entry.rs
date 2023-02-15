use std::fmt::{self, Display};

use colored::Colorize;
use serde::Deserialize;
use textwrap::Options;

#[derive(Debug, Deserialize)]
pub struct Entry {
    word: String,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
}

impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            "".white(),
            self.word.bold().black().on_white(),
            "".white()
        )?;

        if let Some(phonetic) = self.phonetics.iter().find(|p| p.text.is_some()) {
            write!(f, " {}", phonetic)?;
        }

        write!(f, "\n")?;
        for meaning in self.meanings.iter() {
            write!(
                f,
                "{}",
                textwrap::indent(&meaning.to_string(), &" ".repeat(2))
            )?;
        }

        Ok(())
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
                .unwrap_or("_".to_string())
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

impl Display for Meaning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}", self.part_of_speech.italic().underline())?;

        for definition in self.definitions.iter() {
            write!(
                f,
                "\n{}\n",
                textwrap::indent(&definition.to_string(), &" ".repeat(2))
            )?;
        }

        if !self.synonyms.is_empty() || !self.antonyms.is_empty() {
            write!(f, "\n")?;
        }
        if !self.synonyms.is_empty() {
            write!(
                f,
                "{:indent$}{} {}\n",
                "",
                "Synonyms:".green(),
                self.synonyms
                    .iter()
                    .map(|s| format!("{}{}{}", "".white(), s.black().on_white(), "".white()))
                    .collect::<Vec<String>>()
                    .join(" "),
                indent = 2
            )?;
        }
        if !self.antonyms.is_empty() {
            write!(
                f,
                "{:indent$}{} {}\n",
                "",
                "Antonyms:".green(),
                self.antonyms
                    .iter()
                    .map(|s| format!("{}{}{}", "".white(), s.black().on_white(), "".white()))
                    .collect::<Vec<String>>()
                    .join(" "),
                indent = 2
            )?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct Definition {
    #[serde(rename = "definition")]
    brief: String,
    example: Option<String>,
}

impl Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let options = Options::with_termwidth().subsequent_indent("  ");

        write!(
            f,
            "{} {}",
            "•".blue(),
            textwrap::fill(&self.brief, &options)
        )?;

        if let Some(example) = &self.example {
            write!(
                f,
                "\n{:indent$}{}{}{}",
                "",
                "\"".bright_black(),
                textwrap::fill(&example, &options).bright_black(),
                "\"".bright_black(),
                indent = 2
            )?;
        }

        Ok(())
    }
}
