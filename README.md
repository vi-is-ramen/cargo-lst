# cargo-lst

[![Crates.io](https://img.shields.io/crates/v/cargo-lst.svg)](https://crates.io/crates/cargo-lst)
[![Documentation](https://docs.rs/cargo-lst/badge.svg)](https://docs.rs/cargo-lst)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**cargo-lst** is a Rust-based command-line tool that generates a Markdown snapshot of your current directory:

- **Directory structure** (tree-like listing)
- **Contents of all UTF‑8 text files**
- **Respects `.gitignore`** and optional **`.lstignore`** (same format as `.gitignore`)
- **Outputs to** stdout, stderr, or a file

It is designed to be run as a Cargo subcommand (`cargo lst`), but can also be invoked directly as `lst` (or `cargo-lst`).

---

## Installation

Install from [crates.io](https://crates.io/crates/cargo-lst):

```sh
cargo install cargo-lst
```

This will install the `cargo-lst` binary. You can then run it as:

```sh
cargo lst
```

or directly as:

```sh
lst
```

---

## Usage

```bash
cargo lst [options]
```

### Options

| Option          | Description                                                                 |
|-----------------|-----------------------------------------------------------------------------|
| `-2`            | Send output to **stderr** instead of stdout (default is stdout).            |
| `-o FILE`       | Write output to the specified **FILE** instead of stdout/stderr.            |
| `--help`        | Show help message.                                                          |

If both `-2` and `-o` are given, `-o` takes precedence (output goes to the file).

---

## Output Format

The output is a valid **Markdown** document containing:

1. A top-level heading `# Directory Structure` followed by a bullet list of all files and directories.
2. A second-level heading `# File Contents` followed by an `## <filepath>` heading and a fenced code block (with `text` language) for each UTF‑8‑decodable file.

Non‑UTF‑8 files (e.g., binaries, images) are skipped silently.

### Example

For a project with:

```
.
├── src/
│   └── main.rs
├── .gitignore
└── README.md
```

The output might look like:

````markdown
# Directory Structure

- src/
  - main.rs
- .gitignore
- README.md

# File Contents

## `src/main.rs`

```
fn main() {
    println!("Hello, world!");
}
```

## `README.md`

```
# My Project
...
```
````

---

## Ignore Files

cargo-lst automatically respects:

- **`.gitignore`** – standard Git ignore rules.
- **`.lstignore`** – if present in the current directory, its rules are also applied (same format as `.gitignore`). This allows you to have separate ignore rules for this tool without affecting Git.

Both files are read from the current working directory.

---

## Use Cases

- **Documentation** – embed an up‑to‑date directory snapshot in your project’s README or wiki.
- **Code reviews** – quickly share a project’s structure and all source files in a single Markdown file.
- **Debugging** – capture the exact state of a directory for reproduction in bug reports.
- **Archiving** – create a textual representation of a project for offline reference.

---

## License

This project is dual‑licensed under the [MIT License](LICENSE-MIT) and the [Apache License, Version 2.0](LICENSE-APACHE). You may choose either license at your option.
