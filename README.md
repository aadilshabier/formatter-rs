# Formatter-rs

A simple file formatter. This is a project to help me learn the Rust programming language.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## Aims

- [x] Whitespace remover
- [ ] Tab to spaces

## Programs

1. **whitespace**: Removes unwanted whitespaces in a file and returns the formatted file.

   ### Usage:

   ```bash
   $ formatter-rs whitespace file1.txt file2.txt
   ```

   This formats the contents of file1.txt and writes it into a new file named file2.txt.

## Usage

1. Ensure rustc and cargo in installed in your operating system. If not follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install).
1. Go to a directory where you wish to download the source code and type this:

   ```bash
   $ git clone https://github.com/aadilshabier/formatter-rs.git
   ```

1. Go to the installation directory and build the program using cargo with the release option
   ```bash
   $ cargo build --release
   ```
1. The executable `formatter-rs` in target/release/ can now be run as a standalone program.
