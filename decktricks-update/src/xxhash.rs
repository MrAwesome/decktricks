use std::hash::Hasher;
use twox_hash::XxHash64;

fn xxhash_file(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut xxhash = XxHash64::default();
    let data = std::fs::read(filename)?;
    xxhash.write(&data);
    let hash = xxhash.finish();
    Ok(format!("{:X}", hash).to_lowercase())
}

pub fn generate_hashes(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let x = todo!();
}
