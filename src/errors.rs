use crate::prelude::*;
use std::{backtrace::Backtrace, fmt::Debug};
use urlencoding::encode;

pub type DynamicError = Box<dyn std::error::Error>;

// NOTE: set to pub(crate) temporarily to find unused values
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum DeckTricksErrorKind {
    InstallError,
}

#[derive(Debug)]
pub struct DeckTricksError {
    pub kind: DeckTricksErrorKind,
    pub desc: String,
    //pub extra_data: Option<Vec<(String, String)>>,
    //pub exit_code: Option<i32>,
}

// TODO: have full list of errors
#[derive(Debug)]
pub struct ActionErrorTEMPORARY {
    pub message: String,
}
impl std::fmt::Display for ActionErrorTEMPORARY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error has occurred: {}", self.message)
    }
}
impl std::error::Error for ActionErrorTEMPORARY {}

#[derive(Debug)]
pub struct SeriousError {
    pub error_type: String,
    pub location: String,
    pub message: String,
}
impl SeriousError {
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

    println!("{}", se);
    panic!()
}
