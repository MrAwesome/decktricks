#![warn(clippy::pedantic)]

pub mod actions;
pub mod errors;
//#[allow(refining_impl_trait)]
pub mod prelude;
pub mod providers;
#[macro_use]
pub mod macros;
pub mod run_system_command;
pub mod tricks_config;
