# Español CLI Dictionary

## Introduction

Español CLI brings the power of a Spanish dictionary directly to your command-line interface, making it easier than ever for developers, language learners, and enthusiasts to access Spanish word definitions without leaving the terminal. This tool is designed to be simple, fast, and efficient, fitting seamlessly into your workflow.

## Features

- **Lightweight and Fast:** Quickly lookup Spanish words directly from your CLI.
- **Easy to Install:** Get up and running with just a few commands.
- **Customizable Dictionary Path:** Use the default dictionary or specify your own path for personalized word lists.

## Installation

Before using Español CLI, you must first install it. Installation instructions will be specific to your operating system and setup. After installation, you'll need to build the dictionary database.

## Usage

### Building the Dictionary

Before your first lookup, build the dictionary database with:

```bash
espanol build --path <optional_path_to_dictionary_file>
```

- `--path` is optional. If not specified, Español CLI uses the default path: `~/.espanol/dictionary.db`.

### Lookup Words

To find the definition of a Spanish word:

```bash
espanol lookup <word> --path <optional_path_to_dictionary_file>
```

- If `--path` is not specified, it defaults to `~/.espanol/dictionary.db`.

## Contributing

We welcome contributions of all kinds from the community! Whether it's adding new words, improving the documentation, or reporting bugs, every bit of help is valuable. Please read our CONTRIBUTING.md file for guidelines on how to contribute.
