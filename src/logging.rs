use crate::prelude::StringType;
use serde::Serialize;

#[derive(Debug, Clone, Hash)]
pub enum LogChannel {
    General,
    TrickID(String),
    IgnoreCompletelyAlways,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum LogType {
    Print,
    Debug,
    Error,
    Info,
    Warn,
}

pub trait DecktricksLogger {
    fn actual_print<S: StringType>(&self, string: S);
    fn actual_print_debug<S: StringType>(&self, string: S);
    fn actual_print_error<S: StringType>(&self, string: S);
    fn actual_print_info<S: StringType>(&self, string: S);
    fn actual_print_warn<S: StringType>(&self, string: S);
    fn decktricks_print_inner<S: StringType>(
        &self,
        log_type: LogType,
        _channel: &LogChannel,
        text: S,
    ) {
        let to_print = text;
        match log_type {
            LogType::Print => self.actual_print(to_print),
            LogType::Debug => self.actual_print_debug(to_print),
            LogType::Error => self.actual_print_error(to_print),
            LogType::Info => self.actual_print_info(to_print),
            LogType::Warn => self.actual_print_warn(to_print),
        };
    }
}

pub trait DecktricksLoggerWithChannels: DecktricksLogger {
    fn decktricks_print_inner<S: StringType>(
        &self,
        log_type: LogType,
        channel: &LogChannel,
        text: S,
    ) {
        let to_print = text.clone();
        match log_type {
            LogType::Print => self.actual_print(to_print),
            LogType::Debug => self.actual_print_debug(to_print),
            LogType::Error => self.actual_print_error(to_print),
            LogType::Info => self.actual_print_info(to_print),
            LogType::Warn => self.actual_print_warn(to_print),
        };

        self.store_in_channel(channel, text);
    }

    fn store_in_channel<S: StringType>(&self, channel: &LogChannel, string: S);
}

pub struct DecktricksConsoleLogger {}

impl Default for DecktricksConsoleLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl DecktricksConsoleLogger {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl DecktricksLogger for DecktricksConsoleLogger {
    fn actual_print<S: StringType>(&self, string: S) {
        println!("{string}");
    }
    fn actual_print_debug<S: StringType>(&self, string: S) {
        eprintln!("{string}");
    }
    fn actual_print_error<S: StringType>(&self, string: S) {
        eprintln!("{string}");
    }
    fn actual_print_info<S: StringType>(&self, string: S) {
        eprintln!("{string}");
    }
    fn actual_print_warn<S: StringType>(&self, string: S) {
        eprintln!("{string}");
    }
}

#[allow(clippy::crate_in_macro_def)] // This is desired, each crate should define its own logger
#[macro_export(local_inner_macros)]
macro_rules! inner_print {
    ($logtype:expr, $channel:expr, $fmt:literal, $($arg:expr),*) => {
        crate::CRATE_DECKTRICKS_LOGGER.decktricks_print_inner(
            $logtype,
            $channel,
            ::std::format!(
                "[INFO] {}: {}:{} {}",
                ::std::file!(),
                ::std::line!(),
                ::std::column!(),
                ::std::format!($fmt, $($arg)*)
            )
        )
    }
}

#[macro_export(local_inner_macros)]
macro_rules! dt_info {
    ($channel:expr, $fmt:literal, $($arg:expr),*) => {
        inner_print!(
            $crate::logging::LogType::Info,
            $channel,
            $fmt,
            $($arg)*)
    };
    ($channel:expr, $msg:expr) => {
        inner_print!(
            $crate::logging::LogType::Info,
            $channel,
            "{}",
            $msg)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! decktricks_logging_init {
    () => {
        decktricks_logging_init!($crate::logging::DecktricksConsoleLogger);
    };
    ($logger:ty) => {
        static CRATE_DECKTRICKS_LOGGER: std::sync::LazyLock<std::sync::Arc<$logger>> =
            std::sync::LazyLock::new(|| std::sync::Arc::new(<$logger>::new()));
    };
}

#[test]
fn test_print_macros() {
    dt_info!(&LogChannel::General, "{}", "lol");
}
