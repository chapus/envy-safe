use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};

use dirs::config_dir;
use toml::Value;

use super::parse_env_file;

pub fn encrypt_key(env_path: &str, key: &str) -> io::Result<()> {
    let vars = parse_env_file(env_path)?;

    if let Some(value) = vars.get(key) {
        let encrypted = encrypt_with_age(value)?;
        let new_line = format!("{}={}", key, encrypted);
        let new_contents = replace_key(env_path, key, &new_line)?;
        fs::write(env_path, new_contents)?;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Key not found"))
    }
}

pub fn decrypt_key(env_path: &str, key: &str) -> io::Result<String> {
    let vars = parse_env_file(env_path)?;

    if let Some(value) = vars.get(key) {
        let decrypted = decrypt_with_age(value)?;
        Ok(decrypted)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Key not found"))
    }
}

fn encrypt_with_age(value: &str) -> io::Result<String> {
    let recipient = get_age_recipient().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            "Missing AGE recipient key. Set ENVY_AGE_RECIPIENT or use ~/.config/envy-safe/config.toml",
        )
    })?;

    let mut child = Command::new("age")
        .args(["-e", "-r", &recipient])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child.stdin.as_mut().unwrap().write_all(value.as_bytes())?;
    let output = child.wait_with_output()?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn decrypt_with_age(value: &str) -> io::Result<String> {
    let mut child = Command::new("age")
        .arg("-d")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child.stdin.as_mut().unwrap().write_all(value.as_bytes())?;
    let output = child.wait_with_output()?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn replace_key(path: &str, key: &str, new_line: &str) -> io::Result<String> {
    let contents = fs::read_to_string(path)?;
    let replaced = contents
        .lines()
        .map(|line| {
            if line.trim_start().starts_with(&format!("{}=", key)) {
                new_line.to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(replaced)
}

fn get_age_recipient() -> Option<String> {
    if let Ok(val) = std::env::var("ENVY_AGE_RECIPIENT") {
        return Some(val);
    }

    let mut config_path = config_dir()?;
    config_path.push("envy-safe/config.toml");

    if config_path.exists() {
        let contents = fs::read_to_string(config_path).ok()?;
        let parsed: Value = contents.parse().ok()?;
        return parsed.get("recipient")?.as_str().map(String::from);
    }

    None
}
