use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use chrono::{Utc, Datelike, Timelike};

fn main() {
    // Auto-detect the flash disk path based on the executable's location.
    // Assumes the executable is run from the flash disk's root.
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let flashdisk_path = exe_path
        .parent()
        .expect("Executable has no parent directory")
        .to_path_buf();

    // Get the user's profile directory path from the environment variable.
    let user_profile = env::var("USERPROFILE").expect("USERPROFILE environment variable not set");

    // Generate a timestamp for the backup folder name (format: YYYY-MM-DD_HHMMSS).
    let now = Utc::now();
    let timestamp = format!(
        "{:04}-{:02}-{:02}_{:02}{:02}{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    let backup_root = flashdisk_path.join(format!("Backup_{}", timestamp));

    // Create the backup root directory if it doesn't exist.
    fs::create_dir_all(&backup_root).expect("Failed to create backup directory");

    // List of folder names to skip during backup (blacklist).
    let blacklist: Vec<&str> = vec![
        "AppData",
        "Local Settings",
        "Searches",
        "Links",
        "Saved Games",
    ];

    // List of standard user folders to always include in the backup.
    let standard_folders: Vec<&str> = vec![
        "Desktop",
        "Documents",
        "Downloads",
    ];

    // Scan the user profile directory for folders to back up.
    let user_path = Path::new(&user_profile);
    if let Ok(entries) = fs::read_dir(user_path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let folder_name = entry.file_name().to_string_lossy().to_string();

                    // Skip hidden folders (those starting with '.').
                    if folder_name.starts_with('.') {
                        continue;
                    }

                    // Skip folders in the blacklist.
                    if blacklist.contains(&folder_name.as_str()) {
                        continue;
                    }

                    // Include standard folders or any other non-empty named folders.
                    // This backs up standard folders plus any custom user-created folders.
                    if standard_folders.contains(&folder_name.as_str()) || !folder_name.is_empty() {
                        let source_dir = entry.path();
                        let dest_dir = backup_root.join(&folder_name);

                        println!("Backing up: {}", folder_name);
                        copy_recursive(&source_dir, &dest_dir);
                    }
                }
            }
        }
    }

    println!("Backup completed to: {}", backup_root.display());
}

/// Recursively copies the contents of the source directory to the destination directory.
/// Uses WalkDir to traverse the directory tree.
/// Creates directories as needed and copies files.
/// Prints warnings for any failures but continues the process.
fn copy_recursive(source: &Path, dest: &Path) {
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let source_path = entry.path();
        let relative_path = source_path.strip_prefix(source).unwrap();
        let dest_path = dest.join(relative_path);

        if entry.file_type().is_dir() {
            // Create the directory in the destination.
            if let Err(e) = fs::create_dir_all(&dest_path) {
                eprintln!("Warning: Failed to create directory {}: {}", dest_path.display(), e);
            }
        } else {
            // Copy the file to the destination.
            if let Err(e) = fs::copy(source_path, &dest_path) {
                eprintln!("Warning: Failed to copy {} to {}: {}", source_path.display(), dest_path.display(), e);
            }
        }
    }
}

// Made with love for everyone
