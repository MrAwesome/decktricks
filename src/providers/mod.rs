use crate::prelude::*;
use std::fmt::Debug;
//use crate::tricks_config::TricksConfig;
//use serde::Serialize;

// XXX
pub mod flatpak;
pub mod simple_command;
//pub mod decky_installer;

pub trait Provider: ProviderChecks + ProviderActions + Debug {
    //    fn get_provider_info(&self) -> ProviderInfo;
    //    fn specific_actions(&self) -> Vec<TrickActionID>;
    //    fn always_allowed_actions(&self) -> Vec<TrickActionID> {
    //        vec![TrickActionID::Info]
    //    }
    //
    //    fn possible(&self) -> Vec<TrickActionID> {
    //        [self.specific_actions(), self.always_allowed_actions()].concat()
    //    }
}

pub trait ProviderChecks {
    fn is_installable(&self) -> Result<bool, KnownError>;
    fn is_installed(&self) -> Result<bool, KnownError>;
    fn is_runnable(&self) -> Result<bool, KnownError>;
    fn is_running(&self) -> Result<bool, KnownError>;
    fn is_addable_to_steam(&self) -> Result<bool, KnownError>;
}

pub trait ProviderActions {
    fn run(&self) -> Result<ActionSuccess, KnownError>;
    fn kill(&self) -> Result<ActionSuccess, KnownError>;
    fn install(&self) -> Result<ActionSuccess, KnownError>;
    fn update(&self) -> Result<ActionSuccess, KnownError>;
    fn uninstall(&self) -> Result<ActionSuccess, KnownError>;
    fn force_reinstall(&self) -> Result<ActionSuccess, KnownError>;
    fn add_to_steam(&self) -> Result<ActionSuccess, KnownError>;
    //TODO: someday
    //fn remove_from_steam(&self) -> Result<ActionSuccess, DynamicError>>;
}
