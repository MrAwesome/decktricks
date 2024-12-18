mod xxhash;
mod check_hashes;
mod utils;
mod error;
mod live_update;
mod download;

pub use xxhash::*;
pub use live_update::*;
pub use check_hashes::*;

pub(crate) const NUM_RETRIES: u8 = 5;
pub(crate) const DECKTRICKS_DOWNLOAD_URL: &str =
    "https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz";
