use crate::prelude::*;
use decky_installer::{DeckyInstallerProvider, DeckySystemContext};
use flatpak::{FlatpakProvider, FlatpakSystemContext};
use std::fmt::Debug;

pub mod decky_installer;
pub mod flatpak;
mod flatpak_helpers;
pub mod simple_command;

pub struct FullSystemContext {
    flatpak_ctx: FlatpakSystemContext,
    decky_ctx: DeckySystemContext,
}

impl FullSystemContext {
    // TODO: can be parallelized
    /// # Errors
    ///
    /// Can return system errors from trying to gather system information
    pub fn try_gather() -> DeckResult<Self> {
        let (decky_ctx, flatpak_ctx) = join_all!(
            DeckySystemContext::try_gather,
            FlatpakSystemContext::try_gather
        );

        Ok(Self {
            decky_ctx: decky_ctx?,
            flatpak_ctx: flatpak_ctx?,
        })
    }
}

pub(crate) type DynProvider = Box<dyn TrickProvider>;
impl TryFrom<(&Trick, &FullSystemContext)> for DynProvider {
    type Error = KnownError;

    fn try_from((trick, full_ctx): (&Trick, &FullSystemContext)) -> Result<Self, Self::Error> {
        match &trick.provider_config {
            ProviderConfig::Flatpak(flatpak) => Ok(Box::new(FlatpakProvider::new(
                flatpak,
                full_ctx.flatpak_ctx.clone(),
            ))),
            ProviderConfig::SimpleCommand(simple_command) => Ok(Box::new(simple_command.clone())),
            ProviderConfig::DeckyInstaller(_decky_installer) => Ok(Box::new(
                DeckyInstallerProvider::new(full_ctx.decky_ctx.clone()),
            )),
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
    fn get_available_actions(&self) -> Vec<SpecificActionID> {
        let all_variants = SpecificActionID::all_variants();

        // Was once parallelized with par_iter, but all system checks are now
        //   based on FullSystemContext.
        let results: Vec<&SpecificActionID> =
            all_variants.iter().filter(|x| self.can_id(x)).collect();

        // Filter out any errors and throw them if found
        let mut allowed_actions = vec![];
        for res in results {
            allowed_actions.push(res.clone());
        }

        allowed_actions
    }
}

pub(crate) trait ProviderChecks {
    fn can(&self, action: &SpecificAction) -> bool {
        match action {
            // Change these to just be () or the downstream checks should throw?
            SpecificAction::Run { .. } => self.is_runnable(),
            SpecificAction::Install { .. } => self.is_installable(),
            SpecificAction::Kill { .. } => self.is_killable(),
            SpecificAction::Uninstall { .. } => self.is_uninstallable(),
            SpecificAction::AddToSteam { .. } => self.is_addable_to_steam(),
            SpecificAction::Update { .. } => self.is_updateable(),
            SpecificAction::Info { .. } => true,
        }
    }

    fn can_id(&self, action_id: &SpecificActionID) -> bool {
        match action_id {
            // Change these to just be () or the downstream checks should throw?
            SpecificActionID::Run => self.is_runnable(),
            SpecificActionID::Install => self.is_installable(),
            SpecificActionID::Kill => self.is_killable(),
            SpecificActionID::Uninstall => self.is_uninstallable(),
            SpecificActionID::AddToSteam => self.is_addable_to_steam(),
            SpecificActionID::Update => self.is_updateable(),
            SpecificActionID::Info => true,
        }
    }

    fn is_installable(&self) -> bool;
    fn is_uninstallable(&self) -> bool;

    fn is_installed(&self) -> bool;

    fn is_runnable(&self) -> bool;
    fn is_running(&self) -> bool;
    fn is_killable(&self) -> bool;

    fn is_updateable(&self) -> bool;

    fn is_addable_to_steam(&self) -> bool;
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
            fn is_installable(&self) -> bool;
            fn is_uninstallable(&self) -> bool;
            fn is_installed(&self) -> bool;
            fn is_runnable(&self) -> bool;
            fn is_running(&self) -> bool;
            fn is_killable(&self) -> bool;
            fn is_updateable(&self) -> bool;
            fn is_addable_to_steam(&self) -> bool;
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
    fn test_can_run() {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_runnable().times(1).returning(|| true);
        let action = SpecificAction::Run {
            id: "test-id".into(),
        };
        assert!(mock.can(&action));
    }

    #[test]
    fn test_can_install() {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_installable().times(1).returning(|| true);
        let action = SpecificAction::Install {
            id: "test-id".into(),
        };
        assert!(mock.can(&action));
    }

    #[test]
    fn test_can_kill() {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_killable().times(1).returning(|| true);
        let action = SpecificAction::Kill {
            id: "test-id".into(),
        };
        assert!(mock.can(&action));
    }

    #[test]
    fn test_can_uninstall() {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_uninstallable()
            .times(1)
            .returning(|| true);
        let action = SpecificAction::Uninstall {
            id: "test-id".into(),
        };
        assert!(mock.can(&action));
    }

    #[test]
    fn test_can_update() {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_updateable().times(1).returning(|| true);
        let action = SpecificAction::Update {
            id: "test-id".into(),
        };
        assert!(mock.can(&action));
    }

    #[test]
    fn test_can_add_to_steam() {
        let mut mock = MockProviderImpl::new();
        mock.expect_is_addable_to_steam()
            .times(1)
            .returning(|| true);
        let action = SpecificAction::AddToSteam {
            name: None,
            id: "test-id".into(),
        };
        assert!(mock.can(&action));
    }

    #[test]
    fn test_can_info() {
        let mock = MockProviderImpl::new();
        let action = SpecificAction::Info {
            id: "test-id".into(),
        };
        assert!(mock.can(&action));
    }
}
