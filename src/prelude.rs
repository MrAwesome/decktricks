pub use crate::actions::*;
pub use crate::add_to_steam::*;
pub use crate::command::*;
pub use crate::executor::*;
pub use crate::logging::*;
pub(crate) use crate::tricks_config::*;

// TODO: bring in rayon::spawn here, and use it everywhere so you can
// easily codemode to other spawn methods across crates

pub(crate) use crate::run_system_command::*;

pub(crate) use crate::join_all;
pub(crate) use crate::success;

pub use crate::{log,debug,info,warn,error};

pub use crate::errors::*;
pub use crate::providers::*;

pub const GITHUB_LINK: &str = "https://github.com/MrAwesome/decktricks";
pub const GITHUB_ISSUES_LINK: &str = "https://github.com/MrAwesome/decktricks";
pub const REPO_DIRECTORY_NAME: &str = "decktricks";
pub const DEFAULT_USER: &str = "deck";
pub const PID_ENV_STRING: &str = "DECKTRICKS_TRICK_ID";
pub const INSTALLING_ENV_STRING: &str = "DECKTRICKS_IS_INSTALLING";

pub type ProcessID = String;
pub type TrickID = String;
pub type CategoryID = String;

pub trait StringType: AsRef<str> + Clone + std::fmt::Display + std::fmt::Debug + ToString + Sized {}
impl StringType for String {}
impl StringType for &str {}
impl StringType for &String {}
impl StringType for &&str {}

#[must_use]
pub fn is_debug() -> bool {
    // TODO: switch this to use a flag, and make it do something
    false
}
