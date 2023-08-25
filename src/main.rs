use regex::Regex;
use std::env;
use std::fs;

fn main() {
    // Get the current working directory.
    if let Ok(current_dir) = env::current_dir() {
        println!("Current working directory: {}", current_dir.display());

        // Process the files in the current directory.
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let old_path = entry.path();
                    if let Some(old_name) = old_path.file_name() {
                        if let Some(old_str) = old_name.to_str() {
                            // Check if the file name contains "S01E01" or similar using regex
                            if let Some(new_name) = parse_file_name(old_str) {
                                // Preserve the original file extension (if it exists)
                                let extension = old_path.extension();
                                let new_filename = if let Some(ext) = extension {
                                    format!("{}.{}", new_name, ext.to_string_lossy())
                                } else {
                                    new_name.to_string()
                                };

                                // Create the new path in the current directory.
                                let new_path = current_dir.join(new_filename);

                                // Rename the file and output changed files.
                                if let Err(e) = fs::rename(&old_path, &new_path) {
                                    eprintln!("Error renaming file: {:?}", e);
                                } else {
                                    println!(
                                        "Renamed: {} -> {}",
                                        old_path.display(),
                                        new_path.display()
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to get the current working directory.");
    }
}

// Parse the original filename and trim after "S01E01" or similar
fn parse_file_name(filename: &str) -> Option<String> {
    let regex = Regex::new(r"(?i)(.*S\d+E\d+)").unwrap();
    if let Some(captures) = regex.captures(filename) {
        Some(captures.get(1).unwrap().as_str().to_string())
    } else {
        None
    }
}
