# Copilot, for your terminal

A CLI tool that generates shell scripts from a human readable description.

## Installation

You can install `plz` by running the following command in your terminal.

```
curl -fsSL https://raw.githubusercontent.com/m1guelpf/plz-cli/main/install.sh | sh -
```

### Homebrew

You can also install `plz` using [Homebrew](https://brew.sh/).

```sh
$ brew install plz-cli
```

You may need to close and reopen your terminal after installation. Alternatively, you can download the binary corresponding to your OS from the [latest release](https://github.com/m1guelpf/plz-cli/releases/latest).

## Usage

`plz` uses [GPT-3](https://beta.openai.com/). To use it, you'll need to grab an API key from [your dashboard](https://beta.openai.com/), and save it to `PLZ_OPENAI_API_KEY` as follows (you can also save it in your bash/zsh profile for persistance between sessions).

```bash
# PLZ_OPENAI_API_BASE: The base URL for the OpenAI API.
#                     Default: https://api.openai.com/v1
export PLZ_OPENAI_API_BASE="https://api.openai.com/v1"

# PLZ_OPENAI_API_KEY: Your OpenAI API key.
export PLZ_OPENAI_API_KEY='sk-XXXXXXXX'

# PLZ_OPENAI_MODEL: The model to use for generating code.
#                   Default: gpt-3.5-turbo-instruct
export PLZ_OPENAI_MODEL='gpt-3.5-turbo-instruct'

# PLZ_SYSTEM_PROMPT: The system prompt to send to the model.
#                    Default: empty string
export PLZ_SYSTEM_PROMPT=''

# PLZ_POST_COMMAND: A command to run after generating code.
#                   Default: empty string
#                   Example: copy the generated command to the clipboard and paste it into the terminal
export PLZ_POST_COMMAND="printf \"\$PLZ_GENERATED_COMMAND\" | pbcopy; osascript -e 'tell application \"System Events\" to keystroke \"v\" using {command down}'"

# PLZ_LANG: The language you are using.
#           Default: $LANG
export PLZ_LANG='Your language'
```

Once you have configured your environment, run `plz` followed by whatever it is that you want to do (`plz show me all options for the plz cli`).

To get a full overview of all available options, run `plz --help`

```sh
$ plz --help
Generate bash scripts from the command line, using Codex

Usage: plz [OPTIONS] [PROMPT]...

Arguments:
  [PROMPT]...  Description of the command to execute

Options:
  -s, --silent   Silent mode
  -h, --help     Print help information
  -V, --version  Print version information
```

## Develop

Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)). Then, you can build the project by running `cargo build`, and run it with `cargo run`.

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
