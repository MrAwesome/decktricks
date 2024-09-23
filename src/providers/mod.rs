use crate::{actions::TrickActionID, prelude::*};
//use crate::tricks_config::TricksConfig;
//use serde::Serialize;

// XXX
//pub mod flatpak;
//pub mod simple_command;
//pub mod decky_installer;

struct ProviderInfo {
    id: String,
    display_name: String,
}

//// TODO: Sort in GUI
//#[derive(Debug, PartialEq, Clone, Serialize)]
//enum Action {
//    Install = 0,
//    Run = 1,
//    Uninstall = 2,
//    AddToSteam = 3,
//    // Numbers >= 50 are hidden behind a menu click
//    Kill = 80,
//    Info = 99,
//}


pub trait Provider: ProviderChecks + ProviderActions {
    fn get_provider_info(&self) -> ProviderInfo;
    fn specific_actions(&self) -> Vec<TrickActionID>;
    fn always_allowed_actions(&self) -> Vec<TrickActionID> {
        vec![TrickActionID::Info]
    }

    fn possible(&self) -> Vec<TrickActionID> {
        [self.specific_actions(), self.always_allowed_actions()].concat()
    }
}

pub trait ProviderChecks {
    fn is_installable(&self) -> Result<(), DynamicError>;
    fn is_installed(&self) -> Result<(), DynamicError>;
    fn is_runnable(&self) -> Result<(), DynamicError>;
    fn is_running(&self) -> Result<(), DynamicError>;
    fn is_addable_to_steam(&self) -> Result<(), DynamicError>;
}

pub trait ProviderActions {
    fn run(&self) -> Result<(), DynamicError>;
    fn kill(&self) -> Result<(), DynamicError>;
    fn install(&self) -> Result<(), DynamicError>;
    fn update(&self) -> Result<(), DynamicError>;
    fn uninstall(&self) -> Result<(), DynamicError>;
    fn force_reinstall(&self) -> Result<(), DynamicError>;
    fn add_to_steam(&self) -> Result<(), DynamicError>;
    //TODO: someday
    //fn remove_from_steam(&self) -> Result<(), DynamicError>>;
}
