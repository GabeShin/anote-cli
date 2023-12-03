# Anote

## Description

Anote is a command-line tool for taking notes using Vim. It stands out by automatically summarizing and cleaning up notes using the Language Learning Model (LLM) from Ollama. This tool is perfect for those who love the efficiency of Vim for note-taking and seek the added convenience of automatic note processing.

## Features

- Create and Summarize Notes: Easily create notes in Vim and have them summarized by Ollama's LLM.
- Configuration Flexibility: Modify the save path for notes to suit your organizational preferences.
- List Notes: Quickly list all your notes for easy access.
- Helpful Guide: Access a handy guide to Anote commands with a simple command.

## Getting Started

Prerequisites

### Ollama

- Anote requires Ollama to be installed for its language processing features.
- Please visit [Ollama's GitHub](https://github.com/jmorganca/ollama) page and follow the installation instructions.

### Rust

- Ensure you have Rust installed on your machine.
- You can download and install it from [here](https://rust-lang.org)

## Installation

To install Anote, run the following command:

```bash
git clone https://github.com/gabeshin/anote-cli.git
cd anote-cli
cargo install --path .
```

## Usage

### Create a Note

```bash
# This command allows you to create a new note and automatically processes it upon saving.
anote create
```

### Configure Anote

```bash
# Modify settings such as the save path for your notes.
anote configure
```

### List Notes

```bash
# View a list of all your notes.
anote list
```

### Get Help

```bash
# Displays a list of available Anote commands.
anote help
```

## License

This project is licensed under the MIT License.

## Acknowledgements

Thanks to the Ollama team for their innovative language learning model.
