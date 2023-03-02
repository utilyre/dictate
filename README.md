<div align="center">

# dictate

Lookup words in [dictionaryapi.dev][dictionaryapi] right from the terminal
without interrupting your workflow.

[dictionaryapi]: https://dictionaryapi.dev/

</div>

## 📦 Installation

-   [crates.io][dictate]

    ```bash
    cargo install dictate
    ```

[dictate]: https://crates.io/crates/dictate

## 🚀 Usage

-   Lookup "hello" in the dictionary

    ```bash
    dictate lookup hello
    ```

    **NOTE**: This also caches the output for later `lookup`s.

-   Clean up the cached entries

    ```bash
    dictate clean -c
    ```

## 💻 Shell Completion

-   Bash

    1. Run the following

        ```bash
        mkdir -p ~/.local/share/bash-completion/completions
        dictate complete bash > ~/.local/share/bash-completion/completions/rustup
        ```

    2. Restart bash and you should have tab-completion now.

-   Zsh

    1. Append `~/.zfunc` to `fpath` in your zsh config

        **NOTICE**: Make sure to insert the following line before initializing
        `compinit`.

        ```bash
        fpath+=("$HOME/.zfunc")
        ```

    2. Create the appended directory and dump completion definition to
       `~/.zfunc/_dictate`

        ```bash
        mkdir -p ~/.zfunc
        dictate complete zsh > ~/.zfunc/_dictate
        ```

    3. Restart zsh and you should have tab-completion now.

    **NOTE**: In case it's not working yet, delete `~/.zcompdump` and restart
    your shell again.

-   Fish

    1. Run the following

        ```fish
        mkdir -p ~/.config/fish/completions
        dictate complete fish > ~/.config/fish/completions/rustup.fish
        ```

    2. Restart fish and you should have tab-completion now.
