use crate::run_system_command::{SysCommand, SysCommandResultChecker, SysCommandRunner};
use crate::{prelude::*, utils::kill_pids};

#[derive(Debug)]
pub struct SimpleCommandProvider {
    pub trick_id: TrickID,
    pub command: String,
    pub args: Vec<String>,
    pub ctx: SpecificExecutionContext,
    pub running_instances: Vec<ProcessID>,
}

impl SimpleCommandProvider {
    pub(crate) fn new<S: Into<String>>(
        trick_id: TrickID,
        command: S,
        args: Vec<S>,
        ctx: SpecificExecutionContext,
        running_instances: Vec<ProcessID>,
    ) -> Self {
        Self {
            trick_id,
            command: command.into(),
            args: args.into_iter().map(Into::into).collect(),
            ctx,
            running_instances,
        }
    }
}

impl TrickProvider for SimpleCommandProvider {}

impl ProviderChecks for SimpleCommandProvider {
    fn is_installable(&self) -> bool {
        // These are meant to be simple system commands which are always known to be installed
        false
    }
    fn is_uninstallable(&self) -> bool {
        false
    }
    // Can use which in gather system context for each known simplecommandprovider,
    // but it's probably better to rely on always_present_on_steamdeck in most cases instead
    fn is_installed(&self) -> bool {
        true
    }
    fn is_runnable(&self) -> bool {
        true
    }
    fn is_running(&self) -> bool {
        !self.running_instances.is_empty()
    }
    fn is_killable(&self) -> bool {
        self.is_running()
    }
    fn is_updateable(&self) -> bool {
        false
    }
    fn is_addable_to_steam(&self) -> bool {
        self.is_installed()
    }
}

impl ProviderActions for SimpleCommandProvider {
    // TODO: generalize these to be default implementations?
    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        not_possible("Simple commands cannot be uninstalled!")
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        not_possible("Simple commands cannot be installed!")
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new(&self.ctx, &self.command, self.args.iter())
            .env(PID_ENV_STRING, &self.trick_id)
            .run()?
            .as_success()
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        kill_pids(&self.ctx, &self.running_instances)
    }

    fn update(&self) -> DeckResult<ActionSuccess> {
        not_possible("Simple commands cannot be installed!")
    }

    fn add_to_steam(&self) -> DeckResult<ActionSuccess> {
        add_to_steam(&AddToSteamTarget::Specific(AddToSteamContext::try_from(
            &self.ctx.trick,
        )?))
    }
}

impl GeneralProvider for SimpleCommandProvider {
    fn update_all(&self) -> DeckResult<ActionSuccess> {
        not_implemented("Simple commands cannot be updated!")
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleCommandProvider;
    use crate::prelude::*;
    use crate::run_system_command::MockTestActualRunner;

    #[test]
    fn basic_expectations() {
        let ctx = SpecificExecutionContext::test(Trick::test());

        let sc =
            SimpleCommandProvider::new("echo-lol".into(), "echo", vec!["lol"], ctx, Vec::default());
        assert!(!sc.is_installable());
        assert!(sc.is_installed());
        assert!(sc.is_runnable());
        assert!(!sc.is_running());
        assert!(sc.is_addable_to_steam());
    }

    #[test]
    fn expected_failures() {
        let cmd = "echo";
        let args = vec!["lol"];
        let mut mock = MockTestActualRunner::new();
        mock.expect_run().times(1).returning(move |_| {
            Ok(SysCommandResult::fake_for_test(
                "echo",
                vec!["lol"],
                0,
                "lol",
                "",
            ))
        });

        let trick = Trick::test();
        let ctx = SpecificExecutionContext::test_with_runner(trick, std::sync::Arc::new(mock));
        let sc = SimpleCommandProvider::new("echo-lol".into(), cmd, args, ctx, Vec::default());
        // TODO: generalize these to be default implementations?

        assert!(matches!(sc.run(), Ok(ActionSuccess { .. })));
        assert!(matches!(
            sc.uninstall(),
            Err(KnownError::ActionNotPossible(_))
        ));
        assert!(matches!(
            sc.install(),
            Err(KnownError::ActionNotPossible(_))
        ));
        assert!(sc.kill().is_ok());
        assert!(matches!(sc.update(), Err(KnownError::ActionNotPossible(_))));
        assert_eq!(
            sc.add_to_steam().unwrap().get_message().unwrap(),
            "Ran in test..."
        );
    }
}
