use std::fmt::Debug;
use crate::actions::ActionSuccess;
use crate::prelude::*;
//use crate::tricks_config::TricksConfig;
//use serde::Serialize;

// XXX
pub mod flatpak;
//pub mod simple_command;
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
    fn is_installable(&self) -> Result<(), DynamicError>;
    fn is_installed(&self) -> Result<(), DynamicError>;
    fn is_runnable(&self) -> Result<(), DynamicError>;
    fn is_running(&self) -> Result<(), DynamicError>;
    fn is_addable_to_steam(&self) -> Result<(), DynamicError>;
}

pub trait ProviderActions {
    fn run(&self) -> Result<ActionSuccess, DynamicError>;
    fn kill(&self) -> Result<ActionSuccess, DynamicError>;
    fn install(&self) -> Result<ActionSuccess, DynamicError>;
    fn update(&self) -> Result<ActionSuccess, DynamicError>;
    fn uninstall(&self) -> Result<ActionSuccess, DynamicError>;
    fn force_reinstall(&self) -> Result<ActionSuccess, DynamicError>;
    fn add_to_steam(&self) -> Result<ActionSuccess, DynamicError>;
    //TODO: someday
    //fn remove_from_steam(&self) -> Result<ActionSuccess, DynamicError>>;
}
