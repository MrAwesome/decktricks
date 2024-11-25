#[derive(Debug, Clone)]
pub enum LogChannel {
    All,
    TrickID(String),
    IgnoreCompletelyAlways,
}
