use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::{fs, io};

pub fn get_file_hash(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(&path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    let string = format!("{:x}", hash);
    Ok(string)
}

pub fn string_ends_with_any(s: String, suffixes: Vec<&str>) -> bool {
    return suffixes.iter().any(|&suffix| s.ends_with(suffix));
}

pub fn validate_fetch_type(s: String) -> bool {
    let types: Vec<&str> = vec!["hot", "hour", "day", "week", "month", "year", "all"];
    if string_ends_with_any(s, types) {
        true
    } else {
        false
    }
}
