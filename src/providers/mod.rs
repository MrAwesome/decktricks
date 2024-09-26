use crate::prelude::*;
use std::fmt::Debug;
//use crate::tricks_config::TricksConfig;
//use serde::Serialize;

// XXX
pub mod flatpak;
pub mod simple_command;
//pub mod decky_installer;

pub(crate) type DynProvider = Box<dyn Provider>;
impl TryFrom<&Trick> for DynProvider {
    type Error = KnownError;

    fn try_from(trick: &Trick) -> Result<Self, Self::Error> {
        match &trick.provider_config {
            ProviderConfig::Flatpak(flatpak) => Ok(Box::new(flatpak.clone())),
            ProviderConfig::SimpleCommand(simple_command) => Ok(Box::new(simple_command.clone())),
            _ => unimplemented!(),
        }
    }
}

pub(crate) trait Provider: ProviderChecks + ProviderActions + Debug {
    //    fn get_provider_info(&self) -> ProviderInfo;
    //    fn specific_actions(&self) -> Vec<TrickActionID>;
    //    fn always_allowed_actions(&self) -> Vec<TrickActionID> {
    //        vec![TrickActionID::Info]
    //    }
    //
    //    fn possible(&self) -> Vec<TrickActionID> {
    //        [self.specific_actions(), self.always_allowed_actions()].concat()
    //    }
    //    fn perform(&self, action: &Action) -> Result<ActionOutcome, KnownError> {
    //        let res = self.can(action)?;
    //        match res {
    //            CheckOutcome::Success => {
    //                match action {
    //                    Action::Run { .. } => self.run(),
    //                    Action::Install { .. } => self.install(),
    //                    Action::Kill { .. } => self.kill(),
    //                    Action::Uninstall { .. } => self.uninstall(),
    //                    Action::AddToSteam { .. } => self.add_to_steam(),
    //                    Action::Info { .. } => self.info(),
    //                    Action::List { .. } => todo!("split actions at the type level to avoid this"),
    //                }
    //            },
    //            CheckOutcome::Failure(cf) =>
    //                Ok(ActionOutcome::CheckFailure(cf)),
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

pub(crate) trait ProviderChecks {
    fn can(&self, action: &SpecificAction) -> Result<bool, KnownError> {
        match action {
            // Change these to just be () or the downstream checks should throw?
            SpecificAction::Run { .. } => self.is_runnable(),
            SpecificAction::Install { .. } => self.is_installable(),
            SpecificAction::Kill { .. } => self.is_killable(),
            SpecificAction::Uninstall { .. } => self.is_uninstallable(),
            SpecificAction::AddToSteam { .. } => self.is_addable_to_steam(),
            SpecificAction::Info { .. } => Ok(true),
        }
    }

    fn is_installable(&self) -> Result<bool, KnownError>;
    fn is_uninstallable(&self) -> Result<bool, KnownError>;

    fn is_installed(&self) -> Result<bool, KnownError>;

    fn is_runnable(&self) -> Result<bool, KnownError>;
    fn is_running(&self) -> Result<bool, KnownError>;
    fn is_killable(&self) -> Result<bool, KnownError>;

    fn is_addable_to_steam(&self) -> Result<bool, KnownError>;
}

pub(crate) trait ProviderActions {
    fn run(&self) -> Result<ActionSuccess, KnownError>;
    fn kill(&self) -> Result<ActionSuccess, KnownError>;
    fn install(&self) -> Result<ActionSuccess, KnownError>;
    fn uninstall(&self) -> Result<ActionSuccess, KnownError>;

    // TODO: pop up an interstitial asking for args before running in GUI
    fn add_to_steam(&self, ctx: AddToSteamContext) -> Result<ActionSuccess, KnownError>;

    // This is the version specific to a package. For general updates, maybe make a
    // special-case GeneralProvider<ProviderType> for general actions?
    //fn update(&self) -> Result<ActionSuccess, KnownError>;

    //fn force_reinstall(&self) -> Result<ActionSuccess, KnownError>;

    //fn remove_from_steam(&self) -> Result<ActionSuccess, DynamicError>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        #[derive(Debug)]
        ProviderImpl {}

        impl Provider for ProviderImpl {

        }
        impl ProviderChecks for ProviderImpl {
            fn is_installable(&self) -> Result<bool, KnownError>;
            fn is_uninstallable(&self) -> Result<bool, KnownError>;
            fn is_installed(&self) -> Result<bool, KnownError>;
            fn is_runnable(&self) -> Result<bool, KnownError>;
            fn is_running(&self) -> Result<bool, KnownError>;
            fn is_killable(&self) -> Result<bool, KnownError>;
            fn is_addable_to_steam(&self) -> Result<bool, KnownError>;
        }

        impl ProviderActions for ProviderImpl {
            fn run(&self) -> Result<ActionSuccess, KnownError>;
            fn kill(&self) -> Result<ActionSuccess, KnownError>;
            fn install(&self) -> Result<ActionSuccess, KnownError>;
            fn uninstall(&self) -> Result<ActionSuccess, KnownError>;
            fn add_to_steam(&self, ctx: AddToSteamContext) -> Result<ActionSuccess, KnownError>;
        }
    }

    #[test]
    fn test_can_run() -> Result<(), KnownError> {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_runnable().times(1).returning(|| Ok(true));
        let action = SpecificAction::Run {
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }

    #[test]
    fn test_can_install() -> Result<(), KnownError> {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_installable().times(1).returning(|| Ok(true));
        let action = SpecificAction::Install {
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }

    #[test]
    fn test_can_kill() -> Result<(), KnownError> {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_killable().times(1).returning(|| Ok(true));
        let action = SpecificAction::Kill {
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }

    #[test]
    fn test_can_uninstall() -> Result<(), KnownError> {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_uninstallable()
            .times(1)
            .returning(|| Ok(true));
        let action = SpecificAction::Uninstall {
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }

    #[test]
    fn test_can_add_to_steam() -> Result<(), KnownError> {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_addable_to_steam()
            .times(1)
            .returning(|| Ok(true));
        let action = SpecificAction::AddToSteam {
            name: None,
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }

    #[test]
    fn test_can_info() -> Result<(), KnownError> {
        let mock = MockProviderImpl::new();
        let action = SpecificAction::Info {
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }
}
