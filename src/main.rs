use regex::Regex;
use std::env;
use std::fs;

fn main() {
    if let Ok(current_dir) = env::current_dir() {
        println!("Current working directory: {}", current_dir.display());

        if let Ok(entries) = fs::read_dir(&current_dir) {
            let regex = Regex::new(r"(?i)(.*S\d+E\d+)").unwrap();

            for entry in entries.flatten() {
                let old_name = entry.file_name();
                if let Some(old_str) = old_name.to_str() {
                    if let Some(new_name) = parse_file_name(&regex, old_str) {
                        let extension = entry.path().extension().map(|ext| ext.to_string_lossy().into_owned());

                        let new_filename = if let Some(ext) = extension.as_ref() {
                            format!("{}.{}", new_name, ext)
                        } else {
                            new_name.to_string()
                        };

                        let new_path = current_dir.join(new_filename);

                        if let Err(e) = fs::rename(&entry.path(), &new_path) {
                            eprintln!("Error renaming file: {:?}", e);
                        } else {
                            println!("Renamed: {} -> {}", entry.path().display(), new_path.display());
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to get the current working directory.");
    }
}

fn parse_file_name(regex: &Regex, filename: &str) -> Option<String> {
    regex.captures(filename).map(|captures| captures[1].to_string())
}
