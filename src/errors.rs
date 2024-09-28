use crate::prelude::*;
use std::{backtrace::Backtrace, fmt::Debug};
use urlencoding::encode;

pub type DeckResult<T> = Result<T, KnownError>;
pub type DynamicError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub enum KnownError {
    ConfigParsing(serde_json::Error),
    NotImplemented(String),
    SeriousError(SeriousError),
    SystemCommandParse(DynamicError),
    SystemCommandRun(std::io::Error),
    DeckyInstall(DynamicError),
    UnknownTrickID(DynamicError),
    TestError(String),
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
