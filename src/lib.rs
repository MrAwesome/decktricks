#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]

// Entry point for all of our custom logging:
decktricks_logging_init!(LogType::Warn);

pub mod actions;
pub mod add_to_steam;
pub mod controller_layout;
pub mod errors;
//#[allow(refining_impl_trait)]
pub mod prelude;
pub mod providers;
#[macro_use]
pub mod macros;
pub mod command;
pub mod executor;
pub mod gui;
#[macro_use]
pub mod logging;
pub mod system_command_runners;
pub mod tricks_config;
pub mod tricks_status;
pub mod utils;
pub mod steam;

pub use rayon;
