# remoji

A simple CLI tool to strip emojis from markdown files. It can process single files or recursively scan directories.

## Installation

You can install remoji using Cargo:

```bash
cargo install --path .
```

## Usage

```bash
remoji --path <PATH> [OPTIONS]
```

### Options

- `-p, --path <PATH>`: Path to a markdown file or directory containing .md files.
- `-r, --recursive`: Recursively process all .md files in the directory (replaces files in-place).
- `-o, --output <FILE>`: Output file path (only works with single file mode, ignored with --recursive).
- `-v, --verbose`: Show detailed processing information.
- `-d, --dry-run`: Preview changes without modifying files.
- `-b, --backup`: Create backup files (.bak) before modifying (only with --recursive).
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

## Examples

Process a single file and print to standard output:

```bash
remoji -p README.md
```

Process a single file and save to a new file:

```bash
remoji -p input.md -o output.md
```

Recursively process a directory (modifies files in-place) and create backups:

```bash
remoji -p ./docs -r -b
```

Preview changes without modifying anything:

```bash
remoji -p ./docs -r --dry-run
```
