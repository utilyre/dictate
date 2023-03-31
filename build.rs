use std::{env, fs, io, path::PathBuf};

use clap::CommandFactory;
use clap_complete::Shell;

include!("src/cli.rs");

struct CompletionGenerator {
    outdir: PathBuf,
    command: clap::Command,
}

impl CompletionGenerator {
    fn new() -> Self {
        Self {
            outdir: PathBuf::from(
                env::var_os("OUT_DIR").expect("`OUT_DIR` should be set during compilation"),
            ),
            command: Cli::command(),
        }
    }

    fn generate(&self, shell: Shell) -> Result<PathBuf, io::Error> {
        let directory = &self.outdir.join(shell.to_string());
        fs::create_dir_all(directory)?;

        clap_complete::generate_to(shell, &mut self.command.clone(), "dictate", directory)
    }
}

fn main() -> Result<(), io::Error> {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let generator = CompletionGenerator::new();
    generator.generate(Shell::Bash)?;
    generator.generate(Shell::Elvish)?;
    generator.generate(Shell::Fish)?;
    generator.generate(Shell::PowerShell)?;
    generator.generate(Shell::Zsh)?;

    Ok(())
}
