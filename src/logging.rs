use crate::prelude::StringType;
use crate::prelude::*;
use serde::Serialize;

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
    fn actual_print<S: StringType>(&self, text: S);
    fn actual_print_debug<S: StringType>(&self, text: S);
    fn actual_print_error<S: StringType>(&self, text: S);
    fn actual_print_info<S: StringType>(&self, text: S);
    fn actual_print_warn<S: StringType>(&self, text: S);
    fn store<S: StringType>(&self, ctx: &impl ExecCtx, text: S);

    fn decktricks_print_inner<S: StringType>(
        &self,
        log_type: LogType,
        ctx: &impl ExecCtx,
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

        self.store(ctx, text);
    }
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
    fn actual_print<S: StringType>(&self, text: S) {
        println!("{text}");
    }

    fn actual_print_debug<S: StringType>(&self, text: S) {
        eprintln!("{text}");
    }

    fn actual_print_error<S: StringType>(&self, text: S) {
        eprintln!("{text}");
    }

    fn actual_print_info<S: StringType>(&self, text: S) {
        eprintln!("{text}");
    }

    fn actual_print_warn<S: StringType>(&self, text: S) {
        eprintln!("{text}");
    }

    fn store<S: StringType>(&self, _ctx: &impl ExecCtx, _text: S) {}
}

#[allow(clippy::crate_in_macro_def)] // This is desired, each crate should define its own logger
#[macro_export(local_inner_macros)]
macro_rules! inner_print {
    ($logtype:expr, $channel:expr, $fmt:literal $(, $args:expr )*) => {
        let full_filename = ::std::file!();
        let filename = full_filename.split_once("decktricks")
            .map(|(_, s)| s.split_once("/")
                .map(|(_, ss)| ss)
                .unwrap_or(full_filename))
            .unwrap_or(full_filename);
        crate::CRATE_DECKTRICKS_LOGGER.decktricks_print_inner(
            $logtype.clone(),
            $channel,
            ::std::format!(
                "{}: {}:{} {}",
                filename,
                ::std::line!(),
                ::std::column!(),
                ::std::format!($fmt, $($args)*)
            )
        )
    }
}

#[macro_export(local_inner_macros)]
macro_rules! outer_print {
    ($logtype:expr, $channel:expr, $fmt:literal $(, $args:expr )*) => {
        inner_print!(
            $logtype,
            $channel,
            $fmt
            $(, $args)*)
    };
    ($logtype:expr, $channel:expr, $msg:expr) => {
        inner_print!(
            $logtype,
            $channel,
            "{}",
            $msg)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    ( $( $args:tt )* ) => {
        outer_print!(
            $crate::logging::LogType::Info,
            $($args)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! debug {
    ( $( $args:tt )* ) => {
        outer_print!(
            $crate::logging::LogType::Debug,
            $($args)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
    ( $( $args:tt )* ) => {
        outer_print!(
            $crate::logging::LogType::Warn,
            $($args)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! error {
    ( $( $args:tt )* ) => {
        outer_print!(
            $crate::logging::LogType::Error,
            $($args)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! dt_print {
    ( $( $args:tt )* ) => {
        outer_print!(
            $crate::logging::LogType::Error,
            $($args)*)
    };
}


#[test]
fn test_print_macros() {
    info!(&ExecutionContext::general_for_test(), "{}", "lol");
}
