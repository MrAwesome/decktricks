use std::sync::Arc;
use std::fmt::Debug;
use crate::prelude::*;

#[macro_export(local_inner_macros)]
macro_rules! decktricks_logging_init {
    ($min_log_level:expr) => {
        decktricks_logging_init!($min_log_level, $crate::logging::DecktricksConsoleLogger);
    };
    ($min_log_level:expr, $logger:ty) => {
        use std::sync::{Arc, LazyLock, RwLock};
        use $crate::prelude::LogType;

        pub static CRATE_DECKTRICKS_DEFAULT_LOGGER: LazyLock<Arc<$logger>> = LazyLock::new(|| Arc::new(<$logger>::new()));

        pub const CRATE_DECKTRICKS_DEFAULT_LOG_LEVEL: LogType = $min_log_level;

        pub static CRATE_DECKTRICKS_CURRENT_LOG_LEVEL: LazyLock<Arc<RwLock<LogType>>> =
            LazyLock::new(|| Arc::new(RwLock::new($min_log_level)));
    };
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LogChannel {
    General,
    TrickID(String),
    IgnoreCompletelyAlways,
}

impl LogChannel {
    #[must_use]
    pub fn to_key(&self) -> String {
        match self {
            Self::General => "general".into(),
            // Respect any special-case names
            Self::TrickID(trick_id) => match trick_id.as_ref() {
                "general" => "trick-general".into(),
                "ignore" => "trick-ignore".into(),
                "all" => "trick-all".into(),
                other => other.to_string(),
            },
            Self::IgnoreCompletelyAlways => "ignore".into(),
        }
    }
}

pub type LoggerRc = Arc<dyn DecktricksLogger>;

pub trait DecktricksLogger: Send + Sync + Debug {
    fn get_log_level(&self) -> LogType;
    fn set_log_level(&mut self, log_type: LogType);
    fn actual_print(&self, text: String);
    fn actual_print_debug(&self, text: String);
    fn actual_print_error(&self, text: String);
    fn actual_print_info(&self, text: String);
    fn actual_print_warn(&self, text: String);
    fn store(&self, ctx: ExecutionContext, text: String);

    fn decktricks_print_inner(
        &self,
        log_type: LogType,
        ctx: ExecutionContext,
        text: String,
    ) {
        let to_print = text.clone();
        match log_type {
            LogType::Log => self.actual_print(to_print),
            LogType::Debug => self.actual_print_debug(to_print),
            LogType::Error => self.actual_print_error(to_print),
            LogType::Info => self.actual_print_info(to_print),
            LogType::Warn => self.actual_print_warn(to_print),
        };

        self.store(ctx, text);
    }
}

#[derive(Debug, Clone)]
pub struct DecktricksConsoleLogger {
    log_level: LogType,
}

impl Default for DecktricksConsoleLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl DecktricksConsoleLogger {
    #[must_use]
    pub fn new() -> Self {
        Self {
            log_level: crate::CRATE_DECKTRICKS_DEFAULT_LOG_LEVEL,
        }
    }
}

impl DecktricksLogger for DecktricksConsoleLogger {
    fn get_log_level(&self) -> LogType {
        self.log_level
    }

    fn set_log_level(&mut self, log_type: LogType) {
        self.log_level = log_type;
    }

    fn actual_print(&self, text: String) {
        println!("{text}");
    }

    fn actual_print_debug(&self, text: String) {
        eprintln!("{text}");
    }

    fn actual_print_error(&self, text: String) {
        eprintln!("{text}");
    }

    fn actual_print_info(&self, text: String) {
        eprintln!("{text}");
    }

    fn actual_print_warn(&self, text: String) {
        eprintln!("{text}");
    }

    fn store(&self, _ctx: ExecutionContext, _text: String) {}
}

#[allow(clippy::crate_in_macro_def)] // This is desired, each crate should define its own logger
#[macro_export(local_inner_macros)]
macro_rules! inner_print {
    ($logtype:expr, $ctx:expr, $fmt:literal $(, $args:expr )*) => {{
        if $logtype <= $ctx.get_current_log_level() {
            let ctx = $ctx;
            let prefix = $logtype.get_prefix_for();
            let channel = ctx.get_log_channel();
            let full_filename = ::std::file!();
            let filename = full_filename.split_once("decktricks")
                .map(|(_, s)| s.split_once("/")
                    .map(|(_, ss)| ss)
                    .unwrap_or(full_filename))
                .unwrap_or(full_filename);
            $ctx.get_logger().decktricks_print_inner(
                $logtype.clone(),
                $ctx.as_ctx(),
                ::std::format!(
                    "{} <{}> {}: {}:{} {}",
                    prefix,
                    channel.to_key(),
                    filename,
                    ::std::line!(),
                    ::std::column!(),
                    ::std::format!($fmt, $($args)*)
                )
            )
        }
    }}
}

#[macro_export(local_inner_macros)]
macro_rules! outer_print {
    ($logtype:expr, $ctx:expr, $fmt:literal $(, $args:expr )*) => {
        inner_print!(
            $logtype,
            $ctx,
            $fmt
            $(, $args)*)
    };
    ($logtype:expr, $ctx:expr, $msg:expr) => {
        inner_print!(
            $logtype,
            $ctx,
            "{}",
            $msg)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    ( $( $args:tt )* ) => {
        outer_print!(
            $crate::prelude::LogType::Info,
            $($args)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! debug {
    ( $( $args:tt )* ) => {
        outer_print!(
            $crate::prelude::LogType::Debug,
            $($args)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
    ( $( $args:tt )* ) => {
        outer_print!(
            $crate::prelude::LogType::Warn,
            $($args)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! error {
    ( $( $args:tt )* ) => {
        outer_print!(
            $crate::prelude::LogType::Error,
            $($args)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! log {
    ( $( $args:tt )* ) => {
        outer_print!(
            $crate::prelude::LogType::Log,
            $($args)*)
    };
}

#[test]
fn test_print_macros() {
    let ctx = ExecutionContext::general_for_test();
    info!(&ctx, "{}", "lol");
}

#[test]
fn test_special_logchannels() {
    assert_eq!(LogChannel::General.to_key(), "general".to_string());
    assert_eq!(
        LogChannel::TrickID("harblgarbl".to_string()).to_key(),
        "harblgarbl".to_string()
    );
    assert_eq!(
        LogChannel::IgnoreCompletelyAlways.to_key(),
        "ignore".to_string()
    );

    // Verify that we're respecting any special-case names
    assert_ne!(
        LogChannel::TrickID("general".to_string()).to_key(),
        "general".to_string()
    );
    assert_ne!(
        LogChannel::TrickID("ignore".to_string()).to_key(),
        "ignore".to_string()
    );
    assert_ne!(
        LogChannel::TrickID("all".to_string()).to_key(),
        "all".to_string()
    );
}
