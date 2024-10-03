pub(crate) use crate::actions::*;
pub use crate::command::*;
pub use crate::executor::*;
pub(crate) use crate::tricks_config::*;

pub(crate) use crate::run_system_command::*;

pub(crate) use crate::err;
pub(crate) use crate::success;
pub(crate) use crate::join_all;

#[allow(unused_imports)]
pub use log::{info, error, warn, debug};

pub use crate::errors::*;
pub use crate::providers::*;

pub const GITHUB_LINK: &str = "https://github.com/MrAwesome/decktricks";

#[must_use]
pub fn is_debug() -> bool {
    // Leverage the global state of the logger, so we don't have to pass a context object around
    matches!(log::max_level(), log::LevelFilter::Debug | log::LevelFilter::Trace)
}
