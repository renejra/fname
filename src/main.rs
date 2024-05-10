use clap::{App, Arg};
use std::fs;
use std::path::Path;

fn main() {
    let matches = App::new("File Renamer")
        .version("1.0")
        .author("Your Name")
        .about("Renames files in a directory")
        .arg(
            Arg::with_name("path")
                .required(true)
                .index(1)
                .help("Sets the path of the directory"),
        )
        .arg(
            Arg::with_name("suffix")
                .takes_value(true)
                .short('s')
                .long("suffix")
                .help("Sets the suffix to filter for in filenames"),
        )
        .arg(
            Arg::with_name("prefix")
                .takes_value(true)
                .short('p')
                .long("prefix")
                .help("Sets the prefix to add to filenames"),
        )
        .arg(
            Arg::with_name("suffix_to_add")
                .takes_value(true)
                .short('a')
                .long("suffix-to-add")
                .help("Sets the suffix to add to filenames"),
        )
        .arg(
            Arg::with_name("char_map")
                .takes_value(true)
                .short('m')
                .long("char-map")
                .help("Sets the character map for replacing characters"),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let suffix_filter = matches.value_of("suffix");
    let prefix = matches.value_of("prefix").unwrap_or("");
    let suffix_to_add = matches.value_of("suffix_to_add").unwrap_or("");
    let char_map = matches.value_of("char_map").unwrap_or("");

    if let Err(e) = rename_files(path, suffix_filter, prefix, suffix_to_add, char_map) {
        eprintln!("Error: {}", e);
    }
}

fn rename_files(path: &str, suffix_filter: Option<&str>, prefix: &str, suffix_to_add: &str, char_map: &str) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(path)?;

    // Convert char_map to Vec<(char, char)>
    let char_map_vec = parse_char_map(char_map);

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

        // Check if the file matches the suffix filter
        if let Some(suffix) = suffix_filter {
            if !file_name.ends_with(suffix) {
                continue;
            }
        }

        // Modify the file name
        let mut new_file_name = format!("{}{}{}", prefix, file_name.trim_end_matches(suffix_filter.unwrap_or("")), suffix_to_add);
        
        // Apply character map
        for (from_char, to_char) in &char_map_vec {
            new_file_name = new_file_name.replace(&from_char.to_string(), &to_char.to_string());
        }

        // Rename the file
        let new_file_path = Path::new(&file_path).with_file_name(&new_file_name);
        fs::rename(&file_path, &new_file_path)?;
        println!("Renamed {} to {}", file_name, new_file_name);
    }

    Ok(())
}

fn parse_char_map(char_map_arg: &str) -> Vec<(char, char)> {
    let mut char_map = Vec::new();
    let mappings: Vec<&str> = char_map_arg.split(',').collect();

    for mapping in mappings {
        let parts: Vec<&str> = mapping.split(':').collect();
        if parts.len() == 2 {
            let from_char = parts[0].chars().next().unwrap();
            let to_char = parts[1].chars().next().unwrap();
            char_map.push((from_char, to_char));
        }
    }

    char_map
}
