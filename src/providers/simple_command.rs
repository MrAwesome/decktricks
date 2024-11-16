use crate::run_system_command::{SysCommand, SysCommandResultChecker, SysCommandRunner};
use crate::{prelude::*, utils::kill_pids};

#[derive(Debug)]
pub struct SimpleCommandProvider {
    pub command: String,
    pub args: Vec<String>,
    pub runner: RunnerRc,
    pub trick_id: TrickID,
    pub running_instances: Vec<ProcessID>,
}

impl SimpleCommandProvider {
    pub(crate) fn new<S: Into<String>>(
        command: S,
        args: Vec<S>,
        runner: RunnerRc,
        trick_id: TrickID,
        running_instances: Vec<ProcessID>,
    ) -> Self {
        Self {
            command: command.into(),
            args: args.into_iter().map(Into::into).collect(),
            runner,
            trick_id,
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

    fn is_installed(&self) -> bool {
        false
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
        // For now, we'll assume these aren't commands people will want to run through Steam
        false
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
        SysCommand::new(&self.command, self.args.iter().collect())
            .env(PID_ENV_STRING, &self.trick_id)
            .run_with(&self.runner)?
            .as_success()
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        kill_pids(&self.runner, &self.running_instances)
    }

    fn update(&self) -> DeckResult<ActionSuccess> {
        not_possible("Simple commands cannot be installed!")
    }

    fn add_to_steam(&self, _ctx: AddToSteamContext) -> DeckResult<ActionSuccess> {
        not_implemented("Simple commands cannot be added to Steam yet.")
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
    use std::sync::Arc;

    #[test]
    fn basic_expectations() {
        let runner = Arc::new(MockTestActualRunner::new());
        let sc = SimpleCommandProvider::new(
            "echo",
            vec!["lol"],
            runner,
            "echo-lol".into(),
            Vec::default(),
        );
        assert!(!sc.is_installable());
        assert!(!sc.is_installed());
        assert!(sc.is_runnable());
        assert!(!sc.is_running());
        assert!(!sc.is_addable_to_steam());
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

        let runner = Arc::new(mock);
        let sc = SimpleCommandProvider::new(cmd, args, runner, "echo-lol".into(), Vec::default());
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
        assert!(matches!(
            sc.add_to_steam(AddToSteamContext::default()),
            Err(KnownError::ActionNotImplementedYet(_))
        ));
    }
}
