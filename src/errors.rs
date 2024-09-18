pub type DynamicError = Box<dyn std::error::Error>;

// NOTE: set to pub(crate) temporarily to find unused values
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum DeckTricksErrorKind {
    InstallError
}

#[derive(Debug)]
pub struct DeckTricksError {
    pub kind: DeckTricksErrorKind,
    pub desc: String,
    //pub extra_data: Option<Vec<(String, String)>>,
    //pub exit_code: Option<i32>,
}
