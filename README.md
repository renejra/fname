# fname
## A file renaming program, written in Rust

I created this program to quickly rename a bunch of files within a folder,
hoping to use Rust's performance in memory and speed. This is useful, 
f.e. when adding new naming conventions to files within a directory,
specially if performance is of need with a lot of files.

For ease of use, you can just copy your files on the `data/` folder,
and use some of the many optional arguments for customising your bulk renaming.
You can also replace the `data/` directory for the directory you want to rename files on.

The custom `args` for replacing filenames allow for:

- --suffix: Only rename files with specific suffixes
- --prefix: Would add a prefix before the file
- --suffix-to-add: Adds a suffix at the end of the word
- --char-map: You can pass characters and their replacements like `o:n`, `" ":_`

## Prerequisites

Just install Rust from the [official website](https://www.rust-lang.org/).

## Test

Just run the following to run the program to rename the contents of the `data` directory.

```bash
cargo run data
```

If you run the above with an empty directory, you will see that the file `.keep` was renamed to the same name (no change).
In order to actually rename the contents, you need to specify at least one of the optional arguments to make a change.

## Usage

To use the application, run the following command from the project' directory, using a terminal:

```bash
cargo run path/to/files --suffix _old --prefix new_ --suffix-to-add .txt --char-map o:n," ":_
```
The directory where the files are can be specified replacing the `data` directory for your `path/to/files`. 
The program will rename all files in this directory, with the provided customisations.

Additionally, the above code would make use of the optional arguments for prefix and suffix, 
and replace the characters o for n, and spaces for underscores.