mod xxhash;
mod utils;
mod error;
mod live_update;
mod download;

pub use xxhash::generate_hashfile_for_tarball;
pub use live_update::live_update;

pub(crate) const NUM_RETRIES: u8 = 5;
pub(crate) const DECKTRICKS_DOWNLOAD_URL: &str =
    "https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz";
