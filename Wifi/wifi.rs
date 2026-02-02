use std::process::Command;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct WifiProfile {
    ssid: String,
    authentication: String,
    encryption: String,
    password: Option<String>,
}

fn main() {
    println!("WiFi Backup Tool - Backing up Windows WiFi profiles...");

    // Step 1: Get list of WiFi profiles
    let profiles_output = Command::new("netsh")
        .args(&["wlan", "show", "profiles"])
        .output()
        .expect("Failed to run netsh command. Ensure you're on Windows and have admin privileges.");

    if !profiles_output.status.success() {
        eprintln!("Error: Could not retrieve WiFi profiles.");
        return;
    }

    let profiles_text = String::from_utf8_lossy(&profiles_output.stdout);
    let mut profiles: Vec<String> = Vec::new();

    // Parse the output to extract profile names (basic parsing for "All User Profile : <SSID>")
    for line in profiles_text.lines() {
        if line.contains("All User Profile") {
            if let Some(ssid) = line.split(": ").nth(1) {
                profiles.push(ssid.trim().to_string());
            }
        }
    }

    if profiles.is_empty() {
        println!("No WiFi profiles found.");
        return;
    }

    // Step 2: For each profile, get details including password
    let mut backup_data: Vec<WifiProfile> = Vec::new();

    for ssid in profiles {
        println!("Backing up profile: {}", ssid);

        let profile_output = Command::new("netsh")
            .args(&["wlan", "show", "profile", &format!("name=\"{}\"", ssid), "key=clear"])
            .output();

        match profile_output {
            Ok(output) if output.status.success() => {
                let details = String::from_utf8_lossy(&output.stdout);
                let mut auth = "Unknown".to_string();
                let mut enc = "Unknown".to_string();
                let mut pass: Option<String> = None;

                // Basic parsing for key details (you could use a regex crate for more robustness)
                for line in details.lines() {
                    if line.contains("Authentication") {
                        if let Some(val) = line.split(": ").nth(1) {
                            auth = val.trim().to_string();
                        }
                    } else if line.contains("Cipher") {
                        if let Some(val) = line.split(": ").nth(1) {
                            enc = val.trim().to_string();
                        }
                    } else if line.contains("Key Content") {
                        if let Some(val) = line.split(": ").nth(1) {
                            pass = Some(val.trim().to_string());
                        }
                    }
                }

                backup_data.push(WifiProfile {
                    ssid: ssid.clone(),
                    authentication: auth,
                    encryption: enc,
                    password: pass,
                });
            }
            _ => {
                eprintln!("Warning: Could not export details for profile '{}'. Skipping.", ssid);
            }
        }
    }

    // Step 3: Save to JSON file
    let json = serde_json::to_string_pretty(&backup_data).expect("Failed to serialize data to JSON.");
    let mut file = File::create("wifi_backup.json").expect("Failed to create backup file.");
    file.write_all(json.as_bytes()).expect("Failed to write to backup file.");

    println!("Backup complete! Saved to 'wifi_backup.json'.");
    println!("Total profiles backed up: {}", backup_data.len());
      }
