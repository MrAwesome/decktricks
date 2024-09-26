use crate::prelude::*;
use rayon::prelude::*;
use std::fmt::Debug;
//use crate::tricks_config::TricksConfig;
//use serde::Serialize;

// XXX
pub mod flatpak;
pub mod simple_command;
//pub mod decky_installer;

pub(crate) type DynProvider = Box<dyn TrickProvider>;
impl TryFrom<&Trick> for DynProvider {
    type Error = KnownError;

    fn try_from(trick: &Trick) -> Result<Self, Self::Error> {
        match &trick.provider_config {
            ProviderConfig::Flatpak(flatpak) => Ok(Box::new(flatpak.clone())),
            ProviderConfig::SimpleCommand(simple_command) => Ok(Box::new(simple_command.clone())),
            _ => Err(KnownError::NotImplemented(format!(
                "Provider {} not implemented yet for trick: \"{}\"",
                trick.provider_config, trick.id,
            ))),
        }
    }
}

pub(crate) trait TrickProvider: ProviderChecks + ProviderActions + Debug + Sync {
    fn get_available_actions(&self) -> Result<Vec<SpecificActionID>, KnownError> {
        let all_variants = SpecificActionID::all_variants();
        // Go through and perform all system checks in parallel
        let results: Vec<Result<&SpecificActionID, KnownError>> = all_variants
            .par_iter()
            .filter_map(|id| match self.can_id(&id) {
                Ok(true) => Some(Ok(id)),
                Ok(false) => None,
                Err(e) => Some(Err(e)),
            })
            .collect();

        // Filter out any errors and throw them if found
        let mut allowed_actions = vec![];
        for res in results {
            allowed_actions.push(res?.clone());
        }

        Ok(allowed_actions)
    }
}

pub(crate) trait ProviderChecks {
    fn can(&self, action: &SpecificAction) -> Result<bool, KnownError> {
        match action {
            // Change these to just be () or the downstream checks should throw?
            SpecificAction::Run { .. } => self.is_runnable(),
            SpecificAction::Install { .. } => self.is_installable(),
            SpecificAction::Kill { .. } => self.is_killable(),
            SpecificAction::Uninstall { .. } => self.is_uninstallable(),
            SpecificAction::AddToSteam { .. } => self.is_addable_to_steam(),
            SpecificAction::Update { .. } => self.is_updateable(),
            SpecificAction::Info { .. } => Ok(true),
        }
    }

    fn can_id(&self, action_id: &SpecificActionID) -> Result<bool, KnownError> {
        match action_id {
            // Change these to just be () or the downstream checks should throw?
            SpecificActionID::Run => self.is_runnable(),
            SpecificActionID::Install => self.is_installable(),
            SpecificActionID::Kill => self.is_killable(),
            SpecificActionID::Uninstall => self.is_uninstallable(),
            SpecificActionID::AddToSteam => self.is_addable_to_steam(),
            SpecificActionID::Update => self.is_updateable(),
            SpecificActionID::Info => Ok(true),
        }
    }

    fn is_installable(&self) -> Result<bool, KnownError>;
    fn is_uninstallable(&self) -> Result<bool, KnownError>;

    fn is_installed(&self) -> Result<bool, KnownError>;

    fn is_runnable(&self) -> Result<bool, KnownError>;
    fn is_running(&self) -> Result<bool, KnownError>;
    fn is_killable(&self) -> Result<bool, KnownError>;

    fn is_updateable(&self) -> Result<bool, KnownError>;

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
    fn update(&self) -> Result<ActionSuccess, KnownError>;

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

        impl TrickProvider for ProviderImpl {

        }
        impl ProviderChecks for ProviderImpl {
            fn is_installable(&self) -> Result<bool, KnownError>;
            fn is_uninstallable(&self) -> Result<bool, KnownError>;
            fn is_installed(&self) -> Result<bool, KnownError>;
            fn is_runnable(&self) -> Result<bool, KnownError>;
            fn is_running(&self) -> Result<bool, KnownError>;
            fn is_killable(&self) -> Result<bool, KnownError>;
            fn is_updateable(&self) -> Result<bool, KnownError>;
            fn is_addable_to_steam(&self) -> Result<bool, KnownError>;
        }

        impl ProviderActions for ProviderImpl {
            fn run(&self) -> Result<ActionSuccess, KnownError>;
            fn kill(&self) -> Result<ActionSuccess, KnownError>;
            fn install(&self) -> Result<ActionSuccess, KnownError>;
            fn uninstall(&self) -> Result<ActionSuccess, KnownError>;
            fn update(&self) -> Result<ActionSuccess, KnownError>;
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
    fn test_can_update() -> Result<(), KnownError> {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_updateable().times(1).returning(|| Ok(true));
        let action = SpecificAction::Update {
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
