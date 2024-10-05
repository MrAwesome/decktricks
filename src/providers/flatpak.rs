use super::flatpak_helpers::{
    get_installed_flatpak_applications, get_running_flatpak_applications,
};
use crate::prelude::*;

type FlatpakID = String;

#[derive(Debug)]
pub(crate) struct FlatpakProvider {
    id: FlatpakID,
    ctx: FlatpakSystemContext,
    runner: RunnerRc,
}

impl FlatpakProvider {
    pub(crate) fn new(flatpak: &Flatpak, ctx: FlatpakSystemContext, runner: RunnerRc) -> Self {
        let id = flatpak.id.clone();
        Self { id, ctx, runner }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FlatpakSystemContext {
    running: Vec<FlatpakID>,
    installed: Vec<FlatpakID>,
}

impl FlatpakSystemContext {
    // TODO: parallelize this
    pub(crate) fn gather_with(runner: &RunnerRc) -> DeckResult<Self> {
        let (running, installed) = join_all!(|| get_running_flatpak_applications(runner), || {
            get_installed_flatpak_applications(runner)
        });

        Ok(Self {
            running: running?,
            installed: installed?,
        })
    }
}

impl TrickProvider for FlatpakProvider {}

impl FlatpakProvider {
    fn is_pkg_installed(&self) -> bool {
        self.ctx.installed.contains(&self.id)
    }

    fn is_pkg_running(&self) -> bool {
        self.ctx.running.contains(&self.id)
    }
}

impl FlatpakProvider {
    // NOTE: Can handle/track child pid status here, but
    // `flatpak ps` gives us that easily and authoritatively.
    fn flatpak_run(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new("flatpak", vec!["run", &self.id])
            .run_with(&self.runner)?
            .as_success()
    }

    fn flatpak_install(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new("flatpak", vec!["install", "-y", &self.id])
            .run_with(&self.runner)?
            .as_success()
    }

    fn flatpak_uninstall(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new("flatpak", vec!["uninstall", "-y", &self.id])
            .run_with(&self.runner)?
            .as_success()
    }

    fn flatpak_kill(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new("flatpak", vec!["kill", &self.id])
            .run_with(&self.runner)?
            .as_success()
    }

    fn flatpak_update(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new("flatpak", vec!["update", &self.id])
            .run_with(&self.runner)?
            .as_success()
    }
}

// TODO: remove all test blocks for checks

impl ProviderChecks for FlatpakProvider {
    fn is_installable(&self) -> bool {
        !self.is_installed()
    }

    fn is_uninstallable(&self) -> bool {
        self.is_installed()
    }

    fn is_installed(&self) -> bool {
        self.is_pkg_installed()
    }

    fn is_runnable(&self) -> bool {
        self.is_installed()
    }

    fn is_running(&self) -> bool {
        self.is_pkg_running()
    }

    fn is_killable(&self) -> bool {
        self.is_running()
    }

    fn is_updateable(&self) -> bool {
        self.is_installed()
    }

    fn is_addable_to_steam(&self) -> bool {
        self.is_installed()
    }
}

impl ProviderActions for FlatpakProvider {
    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_uninstall()?;
        success!("\"{}\" uninstalled successfully.", self.id)
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_install()?;
        success!("\"{}\" installed successfully.", self.id)
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_run()
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_kill()?;
        success!()
    }

    fn update(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_update()?;
        success!()
    }

    fn add_to_steam(&self, _ctx: AddToSteamContext) -> DeckResult<ActionSuccess> {
        not_implemented("Add to steam is not yet implemented for flatpak!")
    }
}

#[derive(Debug)]
pub(crate) struct FlatpakGeneralProvider {
    runner: RunnerRc,
}

impl FlatpakGeneralProvider {
    pub(crate) fn new(runner: RunnerRc) -> Self {
        Self { runner }
    }
}

impl GeneralProvider for FlatpakGeneralProvider {
    fn update_all(&self) -> DeckResult<ActionSuccess> {
        // TODO: when running in parallel, collect errors for each portion

        // IMPORTANT: for global flatpak update -y, you MUST run it twice to remove unused runtimes.
        SysCommand::new("flatpak", vec!["update", "-y"]).run_with(&self.runner)?;
        SysCommand::new("flatpak", vec!["update", "-y"]).run_with(&self.runner)?;

        success!("Flatpak update run successfully!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_system_command::MockTestActualRunner;
    use mockall::*;
    use std::sync::Arc;

    impl Flatpak {
        pub(crate) fn new<S: Into<String>>(id: S) -> Self {
            Flatpak { id: id.into() }
        }
    }

    fn get_system_context() -> FlatpakSystemContext {
        FlatpakSystemContext {
            installed: vec!["installed_package".into(), "installed_package2".into()],
            running: vec!["running_package".into(), "running_package2".into()],
        }
    }

    fn fpak_prov(id: &str, runner: RunnerRc) -> FlatpakProvider {
        let ctx = get_system_context();
        FlatpakProvider::new(&Flatpak::new(id), ctx, runner)
    }

    #[test]
    fn test_new_flatpak_provider() {
        let runner = Arc::new(MockTestActualRunner::new());
        let provider = fpak_prov("test_pkg", runner);
        assert_eq!(provider.id, "test_pkg");
    }

    #[test]
    fn test_is_pkg_installed_true() {
        let runner = Arc::new(MockTestActualRunner::new());
        let provider = fpak_prov("installed_package", runner);
        assert!(provider.is_installed());
        let runner = Arc::new(MockTestActualRunner::new());
        let provider = fpak_prov("package_not_installed", runner);
        assert!(!provider.is_installed());
    }

    #[test]
    fn test_installable() {
        let runner = Arc::new(MockTestActualRunner::new());
        let provider = fpak_prov("RANDOM_NAME_FROM_NOWHERE", runner);
        assert!(provider.is_installable());
    }

    #[test]
    fn test_updateable() {
        let runner = Arc::new(MockTestActualRunner::new());
        let provider = fpak_prov("installed_package", runner);
        assert!(provider.is_updateable());
        let runner = Arc::new(MockTestActualRunner::new());
        let provider = fpak_prov("test_pkg_not_installed", runner);
        assert!(!provider.is_updateable());
    }

    #[test]
    fn test_is_pkg_running() {
        let runner = Arc::new(MockTestActualRunner::new());
        let provider = fpak_prov("running_package", runner);
        assert!(provider.is_running());
        let runner = Arc::new(MockTestActualRunner::new());
        let provider = fpak_prov("not_running_package", runner);
        assert!(!provider.is_running());
    }

    #[test]
    fn test_can_install_pkg() {
        let cmd = "flatpak";
        let args = vec!["install", "-y", "RANDOM_PACKAGE"];
        let returned_args = args.clone();
        let mut mock = MockTestActualRunner::new();
        mock.expect_run()
            .times(1)
            .with(predicate::eq(SysCommand::new(cmd, args)))
            .returning(move |_| Ok(SysCommandResult::fake_for_test(cmd, returned_args.clone(), 0, "", "")));

        let runner = Arc::new(mock);
        let provider = fpak_prov("RANDOM_PACKAGE", runner);
        match provider.install() {
            Ok(action_success) => assert_eq!(
                action_success.get_message_or_blank(),
                "\"RANDOM_PACKAGE\" installed successfully."
            ),
            Err(e) => panic!("package installation in test failed: {e:?}"),
        }
    }

    #[test]
    fn test_failed_to_install_pkg() {
        let cmd = "flatpak";
        let args = vec!["install", "-y", "RANDOM_PACKAGE"];
        let failure = SysCommandResult::fake_for_test(cmd, args, 1, "FAILED LOL", "");
        let expected_failure = failure.clone();

        let mut mock = MockTestActualRunner::new();
        mock.expect_run()
            .times(1)
            .with(predicate::eq(SysCommand::new(
                "flatpak",
                vec!["install", "-y", "RANDOM_PACKAGE"],
            )))
            .returning(move |_| Ok(failure.clone()));

        let runner = Arc::new(mock);
        let provider = fpak_prov("RANDOM_PACKAGE", runner);
        match provider.install() {
            Err(KnownError::SystemCommandFailed(output)) => assert_eq!(output, expected_failure),
            Err(e) => panic!("package installation in test failed in unexpected way: {e:?}"),
            Ok(action_success) => panic!(
                "package installation in test succeeded but should not have: {action_success:?}"
            ),
        }
    }
}
