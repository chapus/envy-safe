// envy-safe/src/secure.rs

use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use std::process::Command;

use super::parse_env_file;

pub fn encrypt_key(env_path: &str, key: &str) -> io::Result<()> {
    let mut vars = parse_env_file(env_path)?;

    if let Some(value) = vars.get(key) {
        let encrypted = encrypt_with_age(value)?;
        let new_line = format!("{}={}", key, encrypted);
        let new_contents = replace_key(env_path, key, &new_line)?;
        fs::write(env_path, new_contents)?;
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Key not found"));
    }

    Ok(())
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
    let output = Command::new("age")
        .args(["-e", "-r", "AGE-RECIPIENT-HERE"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.as_mut().unwrap().write_all(value.as_bytes())?;
            let output = child.wait_with_output()?;
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        })?;

    Ok(output)
}

fn decrypt_with_age(value: &str) -> io::Result<String> {
    let output = Command::new("age")
        .arg("-d")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.as_mut().unwrap().write_all(value.as_bytes())?;
            let output = child.wait_with_output()?;
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        })?;

    Ok(output)
}

fn replace_key(path: &str, key: &str, new_line: &str) -> io::Result<String> {
    let contents = fs::read_to_string(path)?;
    let new_contents = contents
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

    Ok(new_contents)
}
