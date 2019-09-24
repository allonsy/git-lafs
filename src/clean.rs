use crate::PROTOCOL_VERSION;
use super::hash;
use std::fs::File;
use std::io::Result;

pub fn clean_file(fname: &str) -> Result<String> {
    let metadata = File::open(fname)?.metadata()?;
    let size = metadata.len() as usize;

    let hash = hash::hash_file(fname)?;
    Ok(format_hash(&hash, size))
}

fn format_hash(hash: &str, size: usize) -> String {
    return format!("version {}\noid sha256:{}\nsize {}", PROTOCOL_VERSION, hash, size)
}
