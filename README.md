# Irregular verbs memo app

A command-line application for memorizing irregular verbs forms.

## Installation

To install `verb_memorizer_beta`, you can download the latest release from the releases section of this repository and run `cargo install --path path/to/release`.

## Usage

To use `verb_memorizer_beta`, you will need to have Rust installed on your machine.

You can use it by running `verb_memorizer_beta` followed by a subcommand. The available subcommands are:

- `memo`: Display a list of irregular verbs to memorize.
- `verbs`: Display a list of all irregular verbs.
- `check`: Check if a given verb and its forms are correct.

### Examples

To start training mode, run:

`verb_memorizer_beta memo`

To display a list of all irregular verbs, run:

`verb_memorizer_beta verbs`

To check if a given verb and its forms are correct, run:

`verb_memorizer_beta check -f "past simple form" "past participle form" -v "verb"`

## Arguments

- `FILE`: The file with verbs to memorize. The default value is `irregular_verbs.txt`.
- `FORMS`: Two forms of verb for check. This argument is required when using the `check` subcommand.
- `VERB`: Verb for check. This argument is required when using the `check` subcommand.

## Author

Maskedball