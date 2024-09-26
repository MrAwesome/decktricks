pub(crate) use crate::providers::*;
pub(crate) use crate::actions::*;
pub(crate) use crate::tricks_config::*;
pub use crate::errors::*;

pub(crate) use crate::success;
pub(crate) use crate::err;

pub const GITHUB_LINK: &str = "https://github.com/MrAwesome/decktricks";

pub(crate) const DEBUG: bool = false;

#[cfg(test)]
pub(crate) const TEST_SAFETY: bool = true;

#[cfg(not(test))]
pub(crate) const TEST_SAFETY: bool = false;
