use crate::prelude::*;
use crate::providers::system_context::FullSystemContext;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SpecificExecutionContext {
    // TODO: can default to "none", reserve that as special, and use to
    // still track all procs we have launched?
    //pub trick_id: TrickID,
    pub log_channel: LogChannel,
    pub runner: RunnerRc,
    pub trick: Trick,
}

#[derive(Debug, Clone)]
pub struct GeneralExecutionContext {
    // TODO: can default to "none", reserve that as special, and use to
    // still track all procs we have launched?
    //pub trick_id: TrickID,
    pub log_channel: LogChannel,
    pub runner: RunnerRc,
}

pub trait ExecutionContextTrait: Send + Sync + Sized {
    fn get_runner(&self) -> &RunnerRc;
    fn get_log_channel(&self) -> &LogChannel;
}

impl ExecutionContextTrait for GeneralExecutionContext {
    fn get_runner(&self) -> &RunnerRc {
        &self.runner
    }

    fn get_log_channel(&self) -> &LogChannel {
        &self.log_channel
    }
}

impl ExecutionContextTrait for SpecificExecutionContext {
    fn get_runner(&self) -> &RunnerRc {
        &self.runner
    }

    fn get_log_channel(&self) -> &LogChannel {
        &self.log_channel
    }
}

impl SpecificExecutionContext {
    #[must_use]
    pub fn new(trick: Trick, runner: RunnerRc) -> Self {
        Self {
            log_channel: LogChannel::TrickID(trick.id.clone()),
            runner,
            trick,
        }
    }

    #[cfg(test)]
    pub(crate) fn test(trick: Trick) -> Self {
        Self {
            log_channel: LogChannel::TrickID(trick.id.clone()),
            runner: Arc::new(MockTestActualRunner::new()),
            trick,
        }
    }
}

impl GeneralExecutionContext {
    #[must_use]
    pub fn new(runner: RunnerRc) -> Self {
        Self {
            log_channel: LogChannel::General,
            runner,
        }
    }

    #[cfg(test)]
    pub(crate) fn _test() -> Self {
        Self {
            log_channel: LogChannel::General,
            runner: Arc::new(MockTestActualRunner::new()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ExecutorMode {
    Continuous,
    OnceOff,
}

#[derive(Debug, Clone)]
pub struct Executor {
    pub mode: ExecutorMode,
    pub loader: TricksLoader,
    pub full_ctx: FullSystemContext,
    pub runner: RunnerRc,
}

impl Executor {
    // In the context of this function, Command is used as "global action context"
    //
    /// # Errors
    ///
    /// Any errors that might arise from parsing the config
    /// or from gathering system resources.
    ///
    pub fn new(mode: ExecutorMode, command: &DecktricksCommand) -> DeckResult<Self> {
        let maybe_config_path = command.config.as_ref();
        let loader = match maybe_config_path {
            Some(config_path) => TricksLoader::from_config(config_path)?,
            // To fall back to default config:
            //            match TricksLoader::from_config(config_path) {
            //                Ok(config) => config,
            //                Err(err) => {
            //                    error!("Failed to load config from path '{config_path}'. Will fall back to default config. Error was: {err:?}");
            //                    TricksLoader::from_default_config()?
            //            }},
            None => TricksLoader::from_default_config()?,
        };

        let runner = Arc::new(Runner::new());

        // TODO: unit test
        //
        // If we're running in CLI mode, we're only going to run a single time
        //      and so we need to know *here* whether or not the desired action
        //      needs to bother querying system state.
        // If we're running in GUI mode, we're planning to reuse this executor
        //      multiple times with the same system context and so we always
        //      want to gather context.
        let full_ctx = {
            let gather_execution_ctx = GeneralExecutionContext::new(runner.clone());
            if matches!(mode, ExecutorMode::OnceOff) {
                let do_not_gather = command
                    .action
                    .does_not_need_system_context(command.gather_context_on_specific_actions);
                if do_not_gather {
                    FullSystemContext::default()
                } else {
                    FullSystemContext::gather_with(&gather_execution_ctx)?
                }
            } else {
                FullSystemContext::gather_with(&gather_execution_ctx)?
            }
        };

        Ok(Self::with(mode, loader, full_ctx, runner))
    }

    #[must_use]
    pub fn with(
        mode: ExecutorMode,
        loader: TricksLoader,
        full_ctx: FullSystemContext,
        runner: RunnerRc,
    ) -> Self {
        Self {
            mode,
            loader,
            full_ctx,
            runner,
        }
    }

    // NOTE: if the initial full system check is too slow, you can have Specific check types do the
    // gather only for their own provider type
    //
    #[must_use = "this is the result of an action taken"]
    /// # Errors
    ///
    /// Almost any `KnownError` can happen by this point, as this is the entry point to most of our
    /// program logic.
    pub fn execute(&self, action: &Action) -> Vec<DeckResult<ActionSuccess>> {
        let typed_action = TypedAction::from(action);
        typed_action.do_with(self)
    }

    //    pub fn reload_config(&mut self) -> DeckResult<()> {
    //        self.loader = TricksLoader::from_disk_config()?;
    //        Ok(())
    //    }

    //    pub fn reload_system_context(&mut self) -> DeckResult<()> {
    //        self.full_ctx = FullSystemContext::gather()?;
    //        Ok(())
    //    }

    #[must_use]
    pub fn get_pieces(&self) -> (&TricksLoader, &FullSystemContext, &RunnerRc) {
        (&self.loader, &self.full_ctx, &self.runner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    fn get_executor(maybe_mock: Option<MockTestActualRunner>) -> DeckResult<Executor> {
        let loader = TricksLoader::from_default_config()?;

        let mock = match maybe_mock {
            None => {
                let mut mock = MockTestActualRunner::new();
                mock.expect_run()
                    .returning(|_| Ok(SysCommandResult::fake_success()));
                mock
            }
            Some(mock) => mock,
        };

        let runner = Arc::new(mock);

        let ctx = GeneralExecutionContext::new(runner.clone());
        let full_ctx = FullSystemContext::gather_with(&ctx)?;

        let executor = Executor::with(ExecutorMode::OnceOff, loader, full_ctx, runner);
        Ok(executor)
    }

    #[test]
    fn top_level_install() -> DeckResult<()> {
        let command = DecktricksCommand::new(Action::Install {
            id: "lutris".into(),
        });

        let executor = get_executor(None)?;
        let results = executor.execute(&command.action);
        assert_eq!(results.len(), 1);
        match &results[0] {
            Ok(action_success) => assert_eq!(
                "\"net.lutris.Lutris\" installed successfully.",
                action_success.get_message().unwrap_or_default()
            ),
            Err(e) => panic!("failed installation in test: {e:?}"),
        }
        Ok(())
    }

    #[test]
    fn top_level_incorrect_run() -> DeckResult<()> {
        let command = DecktricksCommand::new(Action::Run {
            id: "FAKE_PACKAGE".into(),
        });

        let executor = get_executor(None)?;
        let results = executor.execute(&command.action);
        assert_eq!(results.len(), 1);
        match &results[0] {
            Ok(action_success) => panic!(
                "unexpected successful installation for nonexistent package: {action_success:?}"
            ),
            Err(KnownError::UnknownTrickID(pkg)) => assert_eq!("FAKE_PACKAGE", pkg),
            Err(e) => panic!("unexpected failure in test: {e:?}"),
        }
        Ok(())
    }

    #[test]
    fn top_level_general_list() -> DeckResult<()> {
        let command = DecktricksCommand::new(Action::List { installed: false });

        let executor = get_executor(None)?;
        let results = executor.execute(&command.action);
        assert_eq!(results.len(), 1);
        let res = &results[0];
        assert!(res
            .as_ref()
            .unwrap()
            .get_message()
            .unwrap()
            .lines()
            .any(|l| l == "protonup-qt"));
        Ok(())
    }

    #[test]
    fn top_level_general_list_installed() -> DeckResult<()> {
        let command = DecktricksCommand::new(Action::List { installed: true });

        let mut mock = MockTestActualRunner::new();

        let cmd = "flatpak";
        let args = vec!["list", "--app", "--columns=application"];
        let returned_args = args.clone();
        let arg = SysCommand::new(cmd, args);
        mock.expect_run()
            .with(predicate::eq(arg))
            .returning(move |_| {
                Ok(SysCommandResult::fake_for_test(
                    cmd,
                    returned_args.clone(),
                    0,
                    "net.lutris.Lutris",
                    "dooker",
                ))
            });

        mock.expect_run()
            .returning(|_| Ok(SysCommandResult::fake_success()));

        let executor = get_executor(Some(mock))?;
        let results = executor.execute(&command.action);
        assert_eq!(results.len(), 1);
        let res = &results[0];
        assert!(res
            .as_ref()
            .unwrap()
            .get_message()
            .unwrap()
            .lines()
            .any(|l| l == "lutris"));

        Ok(())
    }
}
