use crate::prelude::*;
use std::fmt::Display;
use std::{backtrace::Backtrace, fmt::Debug};
use urlencoding::encode;

pub type DeckResult<T> = Result<T, KnownError>;
pub type DynamicError = Box<dyn std::error::Error + Send + Sync>;

// TODO: make a constructor macro for these that snags file and line number where they originated?
#[derive(Debug)]
pub enum KnownError {
    ActionGated(String),
    ActionNotImplementedYet(&'static str),
    ActionNotPossible(&'static str),
    AddToSteamError(String),
    CommandLineParseError(clap::error::Error),
    ConfigParsing(serde_json::Error),
    ConfigRead(std::io::Error),
    DeckyInstall(DynamicError),
    EmuDeckInstall(DynamicError),
    ErrorDuringRun(&'static str),
    NoAvailableActions(TrickID),
    ProviderNotImplemented(String),
    //RawSystemFailure(std::io::Error),
    RemoteScriptError(String),
    SeriousError(SeriousError),
    LiveSystemCommandKillError(std::io::Error),
    LiveSystemCommandStatusCheckError(std::io::Error),
    SystemCommandFailed(Box<SysCommandResult>),
    SystemCommandParse(String),
    SystemCommandThreadError(String),
    SystemCommandRunFailure(Box<SysCommandRunError>),
    TestError(String),
    UnknownTrickID(TrickID),
}

impl Display for KnownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddToSteamError(msg) => {
                write!(f, "Error adding to Steam: {msg}")
            }
            Self::ConfigParsing(serde_json_err) => {
                write!(f, "Error parsing config: {serde_json_err:?}")
            }
            Self::ConfigRead(cfg_read_err) => write!(f, "Error reading config: {cfg_read_err:?}"),
            Self::CommandLineParseError(cmd_parse_err) => {
                write!(f, "Error parsing command line: {cmd_parse_err:#?}")
            }
            // TODO: merge custom installer errs
            Self::DeckyInstall(decky_install_err) => {
                write!(f, "Error installing Decky: {decky_install_err:#?}")
            }
            Self::EmuDeckInstall(emudeck_install_err) => {
                write!(f, "Error installing EmuDeck: {emudeck_install_err:#?}")
            }
            Self::SeriousError(serious_err) => write!(f, "{serious_err}"),
            Self::SystemCommandParse(sys_parse_err) => {
                write!(f, "Error parsing system command: {sys_parse_err:#?}")
            }
            Self::SystemCommandRunFailure(sys_run_err) => {
                write!(f, "Error running system command: {sys_run_err:#?}")
            }
            Self::SystemCommandThreadError(sys_run_err) => {
                write!(f, "Error in system command thread: {sys_run_err:#?}")
            }
            Self::SystemCommandFailed(output) => {
                write!(f, "System command failed: {output:?}")
            }
            Self::LiveSystemCommandKillError(sys_kill_err) => {
                write!(f, "Error killing live process: {sys_kill_err:#?}")
            }
            Self::LiveSystemCommandStatusCheckError(sys_status_err) => {
                write!(f, "Error checking status of live process: {sys_status_err:#?}")
            }
            Self::RemoteScriptError(output) => {
                write!(f, "Error while fetching remote script: {output}")
            }
            Self::UnknownTrickID(trick_id) => write!(f, "Unknown trick ID: {trick_id}"),
            Self::NoAvailableActions(trick_id) => write!(
                f,
                "No actions available for \"{trick_id}\". This is almost certainly a bug."
            ),
            Self::ActionGated(msg) | Self::ProviderNotImplemented(msg) | Self::TestError(msg) => {
                write!(f, "{msg}")
            }

            Self::ActionNotImplementedYet(msg)
            | Self::ActionNotPossible(msg)
            | Self::ErrorDuringRun(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<&KnownError> for String {
    fn from(e: &KnownError) -> Self {
        format!("{e}")
    }
}

impl From<clap::error::Error> for KnownError {
    fn from(e: clap::error::Error) -> Self {
        Self::CommandLineParseError(e)
    }
}

//impl From<std::io::Error> for KnownError {
//    fn from(e: std::io::Error) -> Self {
//        Self::RawSystemFailure(e)
//    }
//}

#[derive(Debug)]
pub struct DecktricksError {
    pub message: String,
}

impl DecktricksError {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for DecktricksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error has occurred: {}", self.message)
    }
}
impl std::error::Error for DecktricksError {}

#[derive(Debug)]
pub struct SeriousError {
    pub error_type: String,
    pub location: String,
    pub message: String,
}
impl SeriousError {
    #[must_use]
    pub fn new(error_type: &str, location: &str, message: &str) -> Self {
        SeriousError {
            error_type: error_type.into(),
            location: location.into(),
            message: message.into(),
        }
    }

    fn link(&self) -> String {
        let backtrace = Backtrace::capture();
        format!(
            "{}/issues/new?title={}&body={}",
            GITHUB_LINK,
            encode(&format!("[USER] Error report: {}", self.error_type)),
            encode(&format!(
                "Error type: {}\nError location: {}\nError message: {}\nBacktrace: {}

--------------------------------------------
Please give any additional information under this line:
--------------------------------------------

",
                self.error_type, self.location, self.message, backtrace
            ))
        )
    }
}
impl std::fmt::Display for SeriousError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "!!! A very serious error has occurred. Please report this by clicking the following link: {}", self.link())
    }
}
impl std::error::Error for SeriousError {}

#[test]
fn test_serious_error() {
    let se = SeriousError::new(
        "test-error",
        "in the test",
        "i am a very serious lowercase error message",
    );

    assert!(se.link().contains("i%20am"));
}
