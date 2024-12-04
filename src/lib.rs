#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]

#[cfg(test)]
decktricks_logging_init!();

pub mod actions;
pub mod add_to_steam;
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
pub mod run_system_command;
pub mod tricks_config;
pub mod utils;

pub use rayon;
