use std::path::Path;
use std::error::Error;
use std::io::BufRead;

use crate::error::SimpleError;
use crate::xxhash;

pub enum HashCheckOutcome {
    UpdateNeeded,
    NoActionNeeded,
}

// Return whether or not we should run an update. If any file is out of date, just trigger an
// entire update (since we'll be downloading the entire archive anyway)
pub fn check_hashes_for_update_needed(target_dir: &str, hash_filename: &str) -> Result<HashCheckOutcome, Box<dyn Error>> {
    let maybe_hashes_text = std::fs::read(hash_filename);

    let hashes_text = match maybe_hashes_text {
        Ok(hashes_text) => hashes_text,
        Err(err) => {
            eprintln!("Failed to read hashes file! Will attempt a full update. Error: {err:#?}");
            return Ok(HashCheckOutcome::UpdateNeeded);
        }
    };

    for maybe_line in hashes_text.lines() {
        let line = maybe_line?;
        let (expected_hash, filename_only) = line.rsplit_once("  ").ok_or_else(|| {
            SimpleError("Failed to parse \"{line}\" in \"{hashes_filename}\"!".into())
        })?;

        let filename = Path::join(Path::new(target_dir), filename_only);

        // If a file we expect doesn't exist, just force an update
        let file_exists = filename.try_exists().unwrap_or(false);
        if !file_exists {
            return Ok(HashCheckOutcome::UpdateNeeded);
        }

        let calculated_hash = match xxhash::xxhash_file(&filename) {
            Ok(hash) => hash,
            Err(err) => {
                eprintln!("Error hashing file {}, will assume it needs replacement. Error: {err:#?}", filename.to_string_lossy());
                return Ok(HashCheckOutcome::UpdateNeeded);
            }
        };

        if calculated_hash != expected_hash {
            return Ok(HashCheckOutcome::UpdateNeeded);
        }
    }
    Ok(HashCheckOutcome::NoActionNeeded)
}

#[test]
fn test_check_hashes() {
    
}
