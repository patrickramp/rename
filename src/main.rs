use regex::Regex;
use std::env;
use std::fs;

fn main() {
    // Get the current working directory.
    if let Ok(current_dir) = env::current_dir() {
        println!("Current working directory: {}", current_dir.display());

        // Read directory entries.
        if let Ok(entries) = fs::read_dir(&current_dir) {
            // Prepare a regex pattern to match filenames (You can change this to fit your needs).
            let regex = Regex::new(r"(?i)(.*S\d+E\d+)").unwrap();

            // Iterate through directory entries.
            for entry in entries.flatten() {
                // Extract the filename as a string.
                let old_name = entry.file_name();
                if let Some(old_str) = old_name.to_str() {
                    // Replace " " with "." in the filename.
                    let dot_str = old_str.replace(" ", ".");
                    // Parse the filename using the regex.
                    if let Some(new_name) = parse_file_name(&regex, &dot_str) {
                        // Get the file extension as an Option<String>.
                        let extension = entry
                            .path()
                            .extension()
                            .map(|ext| ext.to_string_lossy().into_owned());

                        // Create the new filename using the parsed name and extension.
                        let new_filename = if let Some(ext) = extension.as_ref() {
                            format!("{}.{}", new_name, ext)
                        } else {
                            new_name.to_string()
                        };

                        // Construct the new path for the renamed file.
                        let new_path = current_dir.join(new_filename);

                        // Rename the file and handle errors.
                        if let Err(e) = fs::rename(&entry.path(), &new_path) {
                            eprintln!("Error renaming file: {:?}", e);
                        } else {
                            println!(
                                "Renamed: {} -> {}",
                                entry.path().display(),
                                new_path.display()
                            );
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to get the current working directory.");
    }
}

// Parse the original filename and extract relevant information.
fn parse_file_name(regex: &Regex, filename: &str) -> Option<String> {
    // Use the regex to capture the desired part of the filename.
    regex
        .captures(filename)
        .map(|captures| captures[1].to_string())
}
