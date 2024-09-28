use crate::prelude::*;
use rayon::prelude::*;
use std::fmt::Debug;

pub mod decky_installer;
pub mod flatpak;
mod flatpak_helpers;
pub mod simple_command;

pub(crate) type DynProvider = Box<dyn TrickProvider>;
impl TryFrom<&Trick> for DynProvider {
    type Error = KnownError;

    fn try_from(trick: &Trick) -> Result<Self, Self::Error> {
        match &trick.provider_config {
            ProviderConfig::Flatpak(flatpak) => Ok(Box::new(flatpak.clone())),
            ProviderConfig::SimpleCommand(simple_command) => Ok(Box::new(simple_command.clone())),
            ProviderConfig::DeckyInstaller(decky_installer) => {
                Ok(Box::new(decky_installer.clone()))
            }
            ProviderConfig::Custom => not_implemented(trick),
        }
    }
}

fn not_implemented(trick: &Trick) -> DeckResult<DynProvider> {
    Err(KnownError::NotImplemented(format!(
        "Provider {} not implemented yet for trick: \"{}\"",
        trick.provider_config, trick.id,
    )))
}

pub(crate) trait TrickProvider: ProviderChecks + ProviderActions + Debug + Sync {
    fn get_available_actions(&self) -> DeckResult<Vec<SpecificActionID>> {
        let all_variants = SpecificActionID::all_variants();
        // Go through and perform all system checks in parallel
        let results: Vec<DeckResult<&SpecificActionID>> = all_variants
            .par_iter()
            .filter_map(|id| match self.can_id(id) {
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
    fn can(&self, action: &SpecificAction) -> DeckResult<bool> {
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

    fn can_id(&self, action_id: &SpecificActionID) -> DeckResult<bool> {
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

    fn is_installable(&self) -> DeckResult<bool>;
    fn is_uninstallable(&self) -> DeckResult<bool>;

    fn is_installed(&self) -> DeckResult<bool>;

    fn is_runnable(&self) -> DeckResult<bool>;
    fn is_running(&self) -> DeckResult<bool>;
    fn is_killable(&self) -> DeckResult<bool>;

    fn is_updateable(&self) -> DeckResult<bool>;

    fn is_addable_to_steam(&self) -> DeckResult<bool>;
}

pub(crate) trait ProviderActions {
    fn run(&self) -> DeckResult<ActionSuccess>;
    fn kill(&self) -> DeckResult<ActionSuccess>;
    fn install(&self) -> DeckResult<ActionSuccess>;
    fn uninstall(&self) -> DeckResult<ActionSuccess>;

    // TODO: pop up an interstitial asking for args before running in GUI
    fn add_to_steam(&self, ctx: AddToSteamContext) -> DeckResult<ActionSuccess>;

    // This is the version specific to a package. For general updates, maybe make a
    // special-case GeneralProvider<ProviderType> for general actions?
    fn update(&self) -> DeckResult<ActionSuccess>;

    //fn force_reinstall(&self) -> DeckResult<ActionSuccess>;

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
            fn is_installable(&self) -> DeckResult<bool>;
            fn is_uninstallable(&self) -> DeckResult<bool>;
            fn is_installed(&self) -> DeckResult<bool>;
            fn is_runnable(&self) -> DeckResult<bool>;
            fn is_running(&self) -> DeckResult<bool>;
            fn is_killable(&self) -> DeckResult<bool>;
            fn is_updateable(&self) -> DeckResult<bool>;
            fn is_addable_to_steam(&self) -> DeckResult<bool>;
        }

        impl ProviderActions for ProviderImpl {
            fn run(&self) -> DeckResult<ActionSuccess>;
            fn kill(&self) -> DeckResult<ActionSuccess>;
            fn install(&self) -> DeckResult<ActionSuccess>;
            fn uninstall(&self) -> DeckResult<ActionSuccess>;
            fn update(&self) -> DeckResult<ActionSuccess>;
            fn add_to_steam(&self, ctx: AddToSteamContext) -> DeckResult<ActionSuccess>;
        }
    }

    #[test]
    fn test_can_run() -> DeckResult<()> {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_runnable().times(1).returning(|| Ok(true));
        let action = SpecificAction::Run {
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }

    #[test]
    fn test_can_install() -> DeckResult<()> {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_installable().times(1).returning(|| Ok(true));
        let action = SpecificAction::Install {
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }

    #[test]
    fn test_can_kill() -> DeckResult<()> {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_killable().times(1).returning(|| Ok(true));
        let action = SpecificAction::Kill {
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }

    #[test]
    fn test_can_uninstall() -> DeckResult<()> {
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
    fn test_can_update() -> DeckResult<()> {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_updateable().times(1).returning(|| Ok(true));
        let action = SpecificAction::Update {
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }

    #[test]
    fn test_can_add_to_steam() -> DeckResult<()> {
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
    fn test_can_info() -> DeckResult<()> {
        let mock = MockProviderImpl::new();
        let action = SpecificAction::Info {
            id: "test-id".into(),
        };
        assert!(mock.can(&action)?);
        Ok(())
    }
}
