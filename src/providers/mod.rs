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
    fn can(&self, action: &Action) -> Result<bool, KnownError> {
        match action {
            Action::Run { .. } => self.is_runnable(),
            Action::Install { .. } => self.is_installable(),
            Action::Kill { .. } => self.is_killable(),
            Action::Uninstall { .. } => self.is_uninstallable(),
            Action::AddToSteam { .. } => self.is_addable_to_steam(),
            Action::Info { .. } => Ok(true),
            Action::List { .. } => Ok(true),
        }
    }
//
//    fn perform(&self, action: &Action) -> Result<bool, KnownError> {
//        match action {
//            Action::Run { .. } => self.is_runnable(),
//            Action::Install { .. } => self.is_installable(),
//            Action::Kill { .. } => self.is_killable(),
//            Action::Uninstall { .. } => self.is_uninstallable(),
//            Action::AddToSteam { .. } => self.is_addable_to_steam(),
//            Action::Info { .. } => Ok(true),
//            Action::List { .. } => Ok(true),
//        }
//    }
}

// struct Killable;
// impl Action {
//     fn to_token(&self) {
//         match self {
//             Self::Killable => Killable;
//         }
//     }
// }
//
// ... is_killable() -> Result<Some(Killable), KnownError>;

//struct Killable;

pub trait ProviderChecks {
    fn is_installable(&self) -> Result<bool, KnownError>;
    fn is_uninstallable(&self) -> Result<bool, KnownError>;

    fn is_installed(&self) -> Result<bool, KnownError>;

    fn is_runnable(&self) -> Result<bool, KnownError>;
    fn is_running(&self) -> Result<bool, KnownError>;
    fn is_killable(&self) -> Result<bool, KnownError>;

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
