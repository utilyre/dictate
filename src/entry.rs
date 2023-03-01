use std::fmt::{self, Display};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use textwrap::Options;

use crate::charset;

#[derive(Clone, Serialize, Deserialize)]
pub struct Entry {
    pub word: String,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
}

impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = charset::get();
        let list = String::from(ch.list);
        let section_left = String::from(ch.section_left);
        let section_right = String::from(ch.section_right);

        write!(
            f,
            "{}{}{}",
            section_left.white(),
            self.word.bold().black().on_white(),
            section_right.white()
        )?;

        if let Some(phonetic) = self.phonetics.iter().find(|p| p.text.is_some()) {
            write!(
                f,
                " {}",
                phonetic
                    .text
                    .clone()
                    .expect("`text` should exist since the `None` variants have been filtered out")
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

            for (i, definition) in meaning.definitions.iter().enumerate() {
                if i > 0 {
                    write!(f, "\n")?;
                }

                write!(
                    f,
                    "\n{}",
                    textwrap::fill(&format!("{} {}", list.blue(), definition.brief), &depth2)
                )?;

                if let Some(example) = &definition.example {
                    write!(
                        f,
                        "\n{}",
                        textwrap::fill(
                            &format!(
                                "{} {}{}{}",
                                list.yellow(),
                                "\"".italic().bright_black(),
                                example.italic().bright_black(),
                                "\"".italic().bright_black()
                            ),
                            &depth3
                        )
                    )?;
                }
            }

            if !meaning.synonyms.is_empty() {
                write!(
                    f,
                    "\n\n{}",
                    textwrap::fill(
                        &format!(
                            "{} {}",
                            "Synonyms:".green(),
                            meaning
                                .synonyms
                                .iter()
                                .map(|s| format!(
                                    "{}{}{}",
                                    section_left.white(),
                                    s.black().on_white(),
                                    section_right.white()
                                ))
                                .collect::<Vec<_>>()
                                .join(" ")
                        ),
                        &depth2
                    )
                )?;
            }

            if !meaning.antonyms.is_empty() {
                write!(
                    f,
                    "\n\n{}",
                    textwrap::fill(
                        &format!(
                            "{} {}",
                            "Antonyms:".green(),
                            meaning
                                .antonyms
                                .iter()
                                .map(|a| format!(
                                    "{}{}{}",
                                    section_left.white(),
                                    a.black().on_white(),
                                    section_right.white()
                                ))
                                .collect::<Vec<_>>()
                                .join(" ")
                        ),
                        &depth2
                    )
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Phonetic {
    text: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Definition>,

    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Definition {
    #[serde(rename = "definition")]
    brief: String,
    example: Option<String>,
}
