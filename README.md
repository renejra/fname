# fname
## A file renaming program, written in Rust

I created this program to quickly rename a bunch of files within a folder.
This is useful, f.e. when adding new naming conventions to files within a directory.

## Prerequisites

Just install Rust from the [official website](https://www.rust-lang.org/).

## Usage

Run the following from the project' directory, using a terminal:

```bash
cargo run -- /path/to/files --suffix _old --prefix new_ --suffix-to-add .txt --char-map o:n," ":_
```
