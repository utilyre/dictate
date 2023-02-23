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
            write!(
                f,
                " {}",
                phonetic
                    .text
                    .clone()
                    .expect("`text` should exist since `None` variants have been filtered")
                    .italic()
                    .bright_black()
            )?;
        }

        let depth1 = Options::with_termwidth()
            .initial_indent("  ")
            .subsequent_indent("  ");
        let depth2 = Options::with_termwidth()
            .initial_indent("    ")
            .subsequent_indent("    ");
        let depth3 = Options::with_termwidth()
            .initial_indent("      ")
            .subsequent_indent("      ");

        for meaning in self.meanings.iter() {
            write!(
                f,
                "\n\n{}",
                textwrap::fill(
                    &meaning.part_of_speech.italic().underline().to_string(),
                    &depth1
                )
            )?;

            for (i,definition) in meaning.definitions.iter().enumerate() {
                if i > 0 {
                    write!(f, "\n")?;
                }

                write!(
                    f,
                    "\n{}",
                    textwrap::fill(&format!("{} {}", "•".blue(), definition.brief), &depth2)
                )?;

                if let Some(example) = &definition.example {
                    write!(
                        f,
                        "\n{}",
                        textwrap::fill(
                            &format!(
                                "{} {}{}{}",
                                "•".yellow(),
                                "\"".italic().bright_black(),
                                example.italic().bright_black(),
                                "\"".italic().bright_black()
                            ),
                            &depth3
                        )
                    )?;
                }
            }

            //
        }

        /*
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
        */

        Ok(())
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
