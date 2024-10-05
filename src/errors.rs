use crate::prelude::*;
use std::fmt::Display;
use std::{backtrace::Backtrace, fmt::Debug};
use urlencoding::encode;

pub type DeckResult<T> = Result<T, KnownError>;
pub type DynamicError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub enum KnownError {
    ActionGated(String),
    ActionNotImplementedYet(&'static str),
    ActionNotPossible(&'static str),
    CommandLineParseError(clap::error::Error),
    ConfigParsing(serde_json::Error),
    ConfigRead(std::io::Error),
    DeckyInstall(DynamicError),
    ErrorDuringRun(&'static str),
    LoggerInitializationFail(log::SetLoggerError),
    NoAvailableActions(TrickID),
    ProviderNotImplemented(String),
    SeriousError(SeriousError),
    SystemCommandFailed(SysCommandResult),
    SystemCommandParse(DynamicError),
    SystemCommandRun(std::io::Error),
    TestError(String),
    UnknownTrickID(TrickID),
}

impl Display for KnownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigParsing(serde_json_err) => {
                write!(f, "Error parsing config: {serde_json_err:?}")
            }
            Self::ConfigRead(cfg_read_err) => write!(f, "Error reading config: {cfg_read_err:?}"),
            Self::CommandLineParseError(cmd_parse_err) => {
                write!(f, "Error parsing command line: {cmd_parse_err:#?}")
            }
            Self::DeckyInstall(decky_install_err) => {
                write!(f, "Error installing Decky: {decky_install_err:#?}")
            }
            Self::LoggerInitializationFail(logger_err) => {
                write!(f, "Logger initialization failure: {logger_err:#?}")
            }
            Self::SeriousError(serious_err) => write!(f, "{serious_err}"),
            Self::SystemCommandParse(sys_parse_err) => {
                write!(f, "Error parsing system command: {sys_parse_err:#?}")
            }
            Self::SystemCommandRun(sys_run_err) => {
                write!(f, "Error running system command: {sys_run_err:#?}")
            }
            Self::SystemCommandFailed(output) => {
                write!(f, "System command failed to run: {output:?}")
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

impl From<std::io::Error> for KnownError {
    fn from(e: std::io::Error) -> Self {
        Self::SystemCommandRun(e)
    }
}

#[derive(Debug)]
pub struct DeckTricksError {
    pub message: String,
}

impl DeckTricksError {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for DeckTricksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error has occurred: {}", self.message)
    }
}
impl std::error::Error for DeckTricksError {}

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
