use crate::prelude::*;
use crate::providers::decky_installer::DeckyInstallerProvider;
use crate::providers::emudeck_installer::EmuDeckInstallerProvider;
use crate::providers::flatpak::FlatpakProvider;
use crate::providers::simple_command::SimpleCommandProvider;
use crate::providers::system_context::FullSystemContext;
use std::fmt::Debug;

pub mod decky_installer;
pub mod emudeck_installer;
pub mod flatpak;
mod flatpak_helpers;
pub mod simple_command;
pub mod system_context;

pub(super) const fn not_possible(reason: &'static str) -> DeckResult<ActionSuccess> {
    Err(KnownError::ActionNotPossible(reason))
}

pub(super) const fn not_implemented(reason: &'static str) -> DeckResult<ActionSuccess> {
    Err(KnownError::ActionNotImplementedYet(reason))
}

pub(crate) type DynProvider<'a> = Box<dyn TrickProvider>;
impl<'a> TryFrom<(&Trick, &ExecutionContext, &FullSystemContext)> for DynProvider<'a> {
    type Error = KnownError;

    fn try_from(
        (trick, ctx, full_ctx): (&Trick, &ExecutionContext, &FullSystemContext),
    ) -> Result<Self, Self::Error> {
        let running_instances = full_ctx
            .procs_ctx
            .tricks_to_running_pids
            .get(&trick.id)
            .cloned()
            .unwrap_or_default();

        let trick_id = trick.id.clone();

        match &trick.provider_config {
            ProviderConfig::Flatpak(flatpak) => Ok(Box::new(FlatpakProvider::new(
                trick_id,
                flatpak,
                full_ctx.flatpak_ctx.clone(),
                ctx.clone(),
            ))),
            ProviderConfig::SimpleCommand(simple_command) => {
                Ok(Box::new(SimpleCommandProvider::new(
                    trick_id,
                    simple_command.command.clone(),
                    simple_command.args.clone().unwrap_or_default(),
                    ctx.clone(),
                    running_instances,
                )))
            }
            ProviderConfig::DeckyInstaller(_decky_installer) => Ok(Box::new(
                DeckyInstallerProvider::new(trick_id, ctx.clone(), full_ctx.decky_ctx.clone()),
            )),
            ProviderConfig::EmuDeckInstaller(_emudeck_installer) => Ok(Box::new(
                EmuDeckInstallerProvider::new(trick_id, ctx.clone(), full_ctx.emudeck_ctx.clone()),
            )),
            ProviderConfig::Custom => provider_not_implemented(trick),
        }
    }
}

fn provider_not_implemented(trick: &Trick) -> DeckResult<DynProvider> {
    Err(KnownError::ProviderNotImplemented(format!(
        "Provider {} not implemented yet for trick: \"{}\"",
        trick.provider_config, trick.id,
    )))
}

pub(crate) trait TrickProvider: ProviderChecks + ProviderActions + Debug + Sync {
    fn get_available_actions(&self) -> Vec<SpecificActionID> {
        let all_variants = SpecificActionID::all_variants();

        let results: Vec<&SpecificActionID> =
            all_variants.iter().filter(|x| self.can_id(x)).collect();

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
    fn add_to_steam(&self, steam_ctx: AddToSteamContext) -> DeckResult<ActionSuccess>;

    // This is the version specific to a package. For general updates, maybe make a
    // special-case GeneralProvider<ProviderType> for general actions?
    fn update(&self) -> DeckResult<ActionSuccess>;

    //fn force_reinstall(&self) -> DeckResult<ActionSuccess>;

    //fn remove_from_steam(&self) -> Result<ActionSuccess, DynamicError>>;
}

pub(crate) trait GeneralProvider: Debug + Sync {
    fn update_all(&self) -> DeckResult<ActionSuccess>;
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
            fn add_to_steam(&self, steam_ctx: AddToSteamContext) -> DeckResult<ActionSuccess>;
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
        mock.expect_is_uninstallable().times(1).returning(|| true);
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
