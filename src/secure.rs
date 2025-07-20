use std::process::{Command, Stdio};
use std::env;
use std::fs;

use std::path::PathBuf;
use dirs::config_dir;
use toml::Value;

fn get_age_recipient() -> Option<String> {
    // 1. Check ENV
    if let Ok(key) = env::var("ENVY_AGE_RECIPIENT") {
        return Some(key);
    }

    // 2. Check config file
    if let Some(mut config_path) = config_dir() {
        config_path.push("envy-safe/config.toml");
        if config_path.exists() {
            if let Ok(contents) = fs::read_to_string(config_path) {
                if let Ok(parsed) = contents.parse::<Value>() {
                    if let Some(recipient) = parsed.get("recipient") {
                        return recipient.as_str().map(String::from);
                    }
                }
            }
        }
    }

    None
}

pub fn encrypt_with_age(value: &str) -> Result<String, String> {
    let recipient = get_age_recipient().ok_or("Missing AGE recipient key. Set ENVY_AGE_RECIPIENT or ~/.config/envy-safe/config.toml")?;

    let mut cmd = Command::new("age")
        .args(["-e", "-r", &recipient])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to run age: {}", e))?;

    if let Some(stdin) = &mut cmd.stdin {
        use std::io::Write;
        stdin
            .write_all(value.as_bytes())
            .map_err(|e| format!("Failed to write to age stdin: {}", e))?;
    }

    let output = cmd
        .wait_with_output()
        .map_err(|e| format!("Failed to read age output: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
