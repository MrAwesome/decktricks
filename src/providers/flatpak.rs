use super::flatpak_helpers::{
    get_installed_flatpak_applications, get_running_flatpak_applications,
};
use crate::prelude::*;

const FLATPAK_SYSTEM_COMMAND: &str = "flatpak";

type FlatpakID = String;
#[derive(Debug)]
pub(crate) struct FlatpakProvider {
    id: FlatpakID,
    flatpak_ctx: FlatpakSystemContext,
    ctx: SpecificExecutionContext,
}

impl FlatpakProvider {
    pub(crate) fn new(
        flatpak: &Flatpak,
        flatpak_ctx: FlatpakSystemContext,
        ctx: SpecificExecutionContext,
    ) -> Self {
        let id = flatpak.id.clone();
        Self {
            id,
            flatpak_ctx,
            ctx,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FlatpakSystemContext {
    pub running: Vec<FlatpakID>,
    pub installed: Vec<FlatpakID>,
}

impl FlatpakSystemContext {
    // TODO: parallelize this
    pub(crate) fn gather_with(ctx: &impl ExecCtx) -> DeckResult<Self> {
        let (running, installed) = join_all!(|| get_running_flatpak_applications(ctx), || {
            get_installed_flatpak_applications(ctx)
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
        self.flatpak_ctx.installed.contains(&self.id)
    }

    fn is_pkg_running(&self) -> bool {
        self.flatpak_ctx.running.contains(&self.id)
    }
}

impl FlatpakProvider {
    // NOTE: Can handle/track child pid status here, but
    // `flatpak ps` gives us that easily and authoritatively.
    fn flatpak_run(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new(&self.ctx, FLATPAK_SYSTEM_COMMAND, ["run", &self.id])
            .run()?
            .as_success()
    }

    fn flatpak_install(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new(
            &self.ctx,
            FLATPAK_SYSTEM_COMMAND,
            ["install", "--user", "-y", &self.id],
        )
        .env("DECKTRICKS_IS_INSTALLING", self.ctx.trick.id.as_ref())
        .run()?
        .as_success()
    }

    fn flatpak_uninstall(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new(
            &self.ctx,
            FLATPAK_SYSTEM_COMMAND,
            ["uninstall", "-y", &self.id],
        )
        .run()?
        .as_success()
    }

    fn flatpak_kill(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new(&self.ctx, FLATPAK_SYSTEM_COMMAND, ["kill", &self.id])
            .run()?
            .as_success()
    }

    fn flatpak_update(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new(&self.ctx, FLATPAK_SYSTEM_COMMAND, ["update", &self.id])
            .run()?
            .as_success()
    }
}

// TODO: remove all test blocks for checks

impl ProviderChecks for FlatpakProvider {
    fn get_execution_context(&self) -> &SpecificExecutionContext {
        &self.ctx
    }

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
        self.is_installed() && !self.is_running()
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

    fn add_to_steam(&self) -> DeckResult<ActionSuccess> {
        add_to_steam(&AddToSteamTarget::Specific(AddToSteamContext::try_from(
            &self.ctx.trick,
        )?))
    }
}

#[derive(Debug)]
pub(crate) struct FlatpakGeneralProvider {
    ctx: GeneralExecutionContext,
}

impl FlatpakGeneralProvider {
    pub(crate) fn new(ctx: GeneralExecutionContext) -> Self {
        Self { ctx }
    }
}

impl GeneralProvider for FlatpakGeneralProvider {
    fn update_all(&self) -> DeckResult<ActionSuccess> {
        // TODO: when running in parallel, collect errors for each portion

        // IMPORTANT: for global flatpak update -y, you MUST run it twice to remove unused runtimes.
        SysCommand::new(&self.ctx, FLATPAK_SYSTEM_COMMAND, ["update", "-y"]).run()?;
        SysCommand::new(&self.ctx, FLATPAK_SYSTEM_COMMAND, ["update", "-y"]).run()?;

        success!("Flatpak update run successfully!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_system_command::MockTestActualRunner;
    use mockall::*;

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

    fn get_execution_context() -> SpecificExecutionContext {
        SpecificExecutionContext::test(Trick::test())
    }

    fn fpak_prov(id: &str, ctx: SpecificExecutionContext) -> FlatpakProvider {
        let flatpak_ctx = get_system_context();
        FlatpakProvider::new(&Flatpak::new(id), flatpak_ctx, ctx)
    }

    #[test]
    fn test_new_flatpak_provider() {
        let ctx = get_execution_context();
        let provider = fpak_prov("test_pkg", ctx);
        assert_eq!(provider.id, "test_pkg");
    }

    #[test]
    fn test_is_pkg_installed_true() {
        let ctx = get_execution_context();
        let provider = fpak_prov("installed_package", ctx);
        assert!(provider.is_installed());
        let ctx = get_execution_context();
        let provider = fpak_prov("package_not_installed", ctx);
        assert!(!provider.is_installed());
    }

    #[test]
    fn test_installable() {
        let ctx = get_execution_context();
        let provider = fpak_prov("RANDOM_NAME_FROM_NOWHERE", ctx);
        assert!(provider.is_installable());
    }

    #[test]
    fn test_updateable() {
        let ctx = get_execution_context();
        let provider = fpak_prov("installed_package", ctx);
        assert!(provider.is_updateable());
        let ctx = get_execution_context();
        let provider = fpak_prov("test_pkg_not_installed", ctx);
        assert!(!provider.is_updateable());
    }

    #[test]
    fn test_is_pkg_running() {
        let ctx = get_execution_context();
        let provider = fpak_prov("running_package", ctx);
        assert!(provider.is_running());
        let ctx = get_execution_context();
        let provider = fpak_prov("not_running_package", ctx);
        assert!(!provider.is_running());
    }

    #[test]
    fn test_can_install_pkg() {
        let cmd = FLATPAK_SYSTEM_COMMAND;
        let args = vec!["install", "--user", "-y", "RANDOM_PACKAGE"];
        let returned_args = args.clone();
        let mut mock = MockTestActualRunner::new();

        let mut expected_sys_command = SysCommand::new(
            &ExecutionContext::specific_for_test(),
            FLATPAK_SYSTEM_COMMAND,
            args,
        );
        expected_sys_command.env(INSTALLING_ENV_STRING, "trick_for_test");

        mock.expect_run()
            .times(1)
            .with(predicate::eq(expected_sys_command))
            .returning(move |_| {
                Ok(SysCommandResult::fake_for_test(
                    cmd,
                    returned_args.clone(),
                    0,
                    "",
                    "",
                ))
            });

        let ctx =
            SpecificExecutionContext::test_with_runner(Trick::test(), std::sync::Arc::new(mock));
        let provider = fpak_prov("RANDOM_PACKAGE", ctx);
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
        let cmd = FLATPAK_SYSTEM_COMMAND;
        let args = vec!["install", "--user", "-y", "RANDOM_PACKAGE"];
        let failure = SysCommandResult::fake_for_test(cmd, args.clone(), 1, "FAILED LOL", "");
        let expected_failure = failure.clone();
        let mut expected_sys_command = SysCommand::new(
            &ExecutionContext::specific_for_test(),
            FLATPAK_SYSTEM_COMMAND,
            args,
        );
        expected_sys_command.env(INSTALLING_ENV_STRING, "trick_for_test");

        let mut mock = MockTestActualRunner::new();
        mock.expect_run()
            .times(1)
            .with(predicate::eq(expected_sys_command))
            .returning(move |_| Ok(failure.clone()));

        let ctx =
            SpecificExecutionContext::test_with_runner(Trick::test(), std::sync::Arc::new(mock));

        let provider = fpak_prov("RANDOM_PACKAGE", ctx);
        match provider.install() {
            Err(KnownError::SystemCommandFailed(output)) => {
                assert_eq!(output, Box::new(expected_failure));
            }
            Err(e) => panic!("package installation in test failed in unexpected way: {e:?}"),
            Ok(action_success) => panic!(
                "package installation in test succeeded but should not have: {action_success:?}"
            ),
        }
    }
}
