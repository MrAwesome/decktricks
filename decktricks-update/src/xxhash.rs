use crate::error::SimpleError;
use std::error::Error;
use std::hash::Hasher;
use std::path::Path;
use std::process::Command;
use twox_hash::XxHash64;

fn xxhash_file<F: AsRef<Path>>(fullpath: F) -> Result<String, Box<dyn Error>> {
    let data = std::fs::read(fullpath)?;
    xxhash_data(&data)
}

fn xxhash_data(data: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut xxhash = XxHash64::default();
    xxhash.write(data);
    let hash = xxhash.finish();
    Ok(format!("{:X}", hash).to_lowercase())
}

// TODO: use crates
fn untar<F: AsRef<Path>, D: AsRef<Path>>(
    filename: F,
    dest_dir: D,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut c = Command::new("tar");
    c.args([
        "-xvf",
        &filename.as_ref().to_string_lossy(),
        "-C",
        &dest_dir.as_ref().to_string_lossy(),
    ]);
    let output = c.output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let full_filenames = text.lines();
    Ok(full_filenames.map(ToString::to_string).collect())
}

fn generate_hashes_for_tarball<F: AsRef<Path>>(
    tarball_filename: F,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let d = tempfile::tempdir()?;
    let tempdir_path = d
        .path()
        .to_str()
        .ok_or_else(|| SimpleError("Failed to get str for tempdir path! Dir: {d:#?}".into()))?;

    let inner_filenames = untar(&tarball_filename, tempdir_path)?;

    let inner_hashes = inner_filenames
        .iter()
        .map(|temp_filename| Path::join(d.path(), temp_filename))
        .map(xxhash_file)
        .collect::<Result<Vec<_>, _>>()?;

    // Note that we return only the filename of the tarball, as we don't want our local paths
    // reflected in the hash sums file.
    let tarball_filename_only: String = tarball_filename
        .as_ref()
        .file_name()
        .ok_or_else(|| {
            SimpleError("Failed to get filename for tarball path! Path: {tarball_filename}".into())
        })?
        .to_string_lossy()
        .into_owned();

    let tarball_filename_to_hash = (tarball_filename_only, xxhash_file(tarball_filename)?);

    let filename_to_hash = std::iter::zip(inner_filenames, inner_hashes)
        .chain(vec![tarball_filename_to_hash])
        .collect::<Vec<(String, String)>>();

    Ok(filename_to_hash)
}

pub fn generate_hashfile_for_tarball<F: AsRef<Path>>(
    filename: F,
) -> Result<String, Box<dyn Error>> {
    let filename_to_hash = generate_hashes_for_tarball(filename)?;
    let text = filename_to_hash
        .iter()
        .map(|(f, h)| format!("{f} {h}"))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(text)
}

#[cfg(test)]
mod tests {
    use crate::generate_hashfile_for_tarball;
    use crate::xxhash::{xxhash_data, xxhash_file};
    use std::io::Write;

    const KNOWN_HASHES: &[(&str, &str)] =
        &[("lol", "58bc5111c453ba82"), ("lawl", "8c806046e05f883d")];

    const TEST_DATA_TAR_FILENAME: &str = "test-data/test-data.tar.xz";
    const EXPECTED_TAR_SUMS: &str = "lawl.txt 8c806046e05f883d
lol.txt 58bc5111c453ba82
test-data.tar.xz 16612534869068c7";

    #[test]
    fn hash_simple_text() {
        let lol_hashed = xxhash_data(KNOWN_HASHES[0].0.as_bytes());
        assert_eq!(KNOWN_HASHES[0].1, lol_hashed.unwrap());
    }

    #[test]
    fn hash_simple_text_file() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        write!(f, "{}", KNOWN_HASHES[1].0).unwrap();
        let filename = f.path();
        let lawl_hashed = xxhash_file(filename);
        assert_eq!(KNOWN_HASHES[1].1, lawl_hashed.unwrap());
    }

    #[test]
    fn hash_contents_of_tar() {
        let contents = generate_hashfile_for_tarball(TEST_DATA_TAR_FILENAME).unwrap();
        assert_eq!(EXPECTED_TAR_SUMS, contents);
    }
}
