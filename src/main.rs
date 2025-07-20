use clap::{arg, command, Arg, ArgMatches, Command};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

mod secure;

fn main() {
    let matches = Command::new("envy-safe")
        .version("0.1.0")
        .author("Sergio Pereda <sergio@adtv.io>")
        .about("Secure and safe .env file validator and helper")
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .help("Check for required keys in .env against .env.example")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sync")
                .short('s')
                .long("sync")
                .help("Sync missing keys from .env.example to .env")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("encrypt")
                .long("encrypt")
                .value_name("KEY")
                .help("Encrypt the value of a key in .env file using age")
                .num_args(1),
        )
        .arg(
            Arg::new("decrypt")
                .long("decrypt")
                .value_name("KEY")
                .help("Decrypt the value of a key in .env file using age")
                .num_args(1),
        )
        .get_matches();

    let example_env = ".env.example";
    let actual_env = ".env";

    if matches.get_flag("check") {
        match check_env(example_env, actual_env) {
            Ok(_) => println!("✅ .env file is valid."),
            Err(e) => eprintln!("❌ Validation failed: {}", e),
        }
    }

    if matches.get_flag("sync") {
        match sync_env(example_env, actual_env) {
            Ok(_) => println!("✅ Synced .env with missing keys from .env.example."),
            Err(e) => eprintln!("❌ Sync failed: {}", e),
        }
    }

    if let Some(key) = matches.get_one::<String>("encrypt") {
        match secure::encrypt_key(actual_env, key) {
            Ok(_) => println!("🔐 Encrypted key '{}'", key),
            Err(e) => eprintln!("❌ Encryption failed: {}", e),
        }
    }

    if let Some(key) = matches.get_one::<String>("decrypt") {
        match secure::decrypt_key(actual_env, key) {
            Ok(value) => println!("🔓 {} = {}", key, value),
            Err(e) => eprintln!("❌ Decryption failed: {}", e),
        }
    }
}

fn parse_env_file(path: &str) -> io::Result<HashMap<String, String>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect())
}

fn check_env(example_path: &str, env_path: &str) -> io::Result<()> {
    let example_vars = parse_env_file(example_path)?;
    let actual_vars = parse_env_file(env_path)?;

    let missing: Vec<_> = example_vars
        .keys()
        .filter(|k| !actual_vars.contains_key(*k))
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        for key in missing {
            eprintln!("Missing key: {}", key);
        }
        Err(io::Error::new(io::ErrorKind::Other, "Missing required keys."))
    }
}

fn sync_env(example_path: &str, env_path: &str) -> io::Result<()> {
    let example_vars = parse_env_file(example_path)?;
    let actual_vars = parse_env_file(env_path)?;

    let missing: Vec<_> = example_vars
        .iter()
        .filter(|(k, _)| !actual_vars.contains_key(*k))
        .collect();

    if !missing.is_empty() {
        let mut file = fs::OpenOptions::new().append(true).open(env_path)?;
        for (key, value) in missing {
            writeln!(file, "{}={}", key, value)?;
        }
    }

    Ok(())
}
