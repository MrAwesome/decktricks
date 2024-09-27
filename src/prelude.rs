pub(crate) use crate::providers::*;
pub(crate) use crate::actions::*;
pub(crate) use crate::tricks_config::*;
pub use crate::errors::*;

pub(crate) use crate::success;
pub(crate) use crate::err;

pub const GITHUB_LINK: &str = "https://github.com/MrAwesome/decktricks";

pub(crate) const DEBUG: bool = false;

pub(crate) const fn am_in_test() -> bool {
    #[cfg(test)]
    return true;
    #[cfg(not(test))]
    return false;
}

