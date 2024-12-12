use crate::prelude::*;
use crate::providers::system_context::FullSystemContext;
use std::sync::Arc;

pub trait ExecCtx: Clone + Send + Sync {
    fn as_ctx(&self) -> ExecutionContext;
    fn get_runner(&self) -> &RunnerRc;
    fn get_log_channel(&self) -> &LogChannel;
    fn get_current_log_level(&self) -> LogType;
    fn get_logger(&self) -> LoggerRc;
}

#[derive(Debug, Clone)]
pub enum ExecutionContext {
    General(GeneralExecutionContext),
    Specific(SpecificExecutionContext),
}

impl From<&ExecutionContext> for ExecutionContext {
    fn from(val: &ExecutionContext) -> Self {
        val.clone()
    }
}

impl ExecCtx for ExecutionContext {
    fn get_runner(&self) -> &RunnerRc {
        match self {
            Self::General(x) => x.get_runner(),
            Self::Specific(x) => x.get_runner(),
        }
    }

    fn get_log_channel(&self) -> &LogChannel {
        match self {
            Self::General(x) => x.get_log_channel(),
            Self::Specific(x) => x.get_log_channel(),
        }
    }

    fn get_current_log_level(&self) -> LogType {
        match self {
            Self::General(x) => x.get_current_log_level(),
            Self::Specific(x) => x.get_current_log_level(),
        }
    }

    fn get_logger(&self) -> LoggerRc {
        match self {
            Self::General(x) => x.get_logger(),
            Self::Specific(x) => x.get_logger(),
        }
    }

    fn as_ctx(&self) -> ExecutionContext {
        self.clone()
    }
}

impl ExecCtx for &ExecutionContext {
    fn get_runner(&self) -> &RunnerRc {
        match self {
            ExecutionContext::General(x) => x.get_runner(),
            ExecutionContext::Specific(x) => x.get_runner(),
        }
    }

    fn get_log_channel(&self) -> &LogChannel {
        match self {
            ExecutionContext::General(x) => x.get_log_channel(),
            ExecutionContext::Specific(x) => x.get_log_channel(),
        }
    }

    fn get_current_log_level(&self) -> LogType {
        match self {
            ExecutionContext::General(x) => x.get_current_log_level(),
            ExecutionContext::Specific(x) => x.get_current_log_level(),
        }
    }

    fn get_logger(&self) -> LoggerRc {
        match self {
            ExecutionContext::General(x) => x.get_logger(),
            ExecutionContext::Specific(x) => x.get_logger(),
        }
    }

    fn as_ctx(&self) -> ExecutionContext {
        (*self).clone()
    }
}

impl ExecutionContext {
    // TODO: code smell
    #[must_use]
    pub fn internal_get_for_logging(current_log_level: LogType, logger: LoggerRc) -> Self {
        let runner = Arc::new(Runner::new());
        Self::General(GeneralExecutionContext::new(runner, current_log_level, logger))
    }
}

// Export for unit tests in Godot
#[cfg(test)]
impl ExecutionContext {
    pub(crate) fn general_for_test_with(runner: RunnerRc) -> Self {
        Self::General(GeneralExecutionContext::new(runner, LogType::Warn, Arc::new(DecktricksConsoleLogger::new())))
    }

    pub(crate) fn general_for_test() -> Self {
        Self::General(GeneralExecutionContext::test())
    }
}

impl From<SpecificExecutionContext> for ExecutionContext {
    fn from(val: SpecificExecutionContext) -> Self {
        Self::Specific(val)
    }
}

impl From<&SpecificExecutionContext> for ExecutionContext {
    fn from(val: &SpecificExecutionContext) -> Self {
        Self::Specific(val.clone())
    }
}

impl From<GeneralExecutionContext> for ExecutionContext {
    fn from(val: GeneralExecutionContext) -> Self {
        Self::General(val)
    }
}

impl From<&GeneralExecutionContext> for ExecutionContext {
    fn from(val: &GeneralExecutionContext) -> Self {
        Self::General(val.clone())
    }
}

#[derive(Debug, Clone)]
pub struct SpecificExecutionContext {
    // TODO: can default to "none", reserve that as special, and use to
    // still track all procs we have launched?
    //pub trick_id: TrickID,
    pub log_channel: LogChannel,
    pub current_log_level: LogType,
    pub runner: RunnerRc,
    pub trick: Trick,
    pub logger: LoggerRc,
}

#[derive(Debug, Clone)]
pub struct GeneralExecutionContext {
    // TODO: can default to "none", reserve that as special, and use to
    // still track all procs we have launched?
    //pub trick_id: TrickID,
    pub log_channel: LogChannel,
    pub current_log_level: LogType,
    pub runner: RunnerRc,
    pub logger: LoggerRc,
}

impl GeneralExecutionContext {
    #[must_use]
    pub fn new(runner: RunnerRc, current_log_level: LogType, logger: LoggerRc) -> Self {
        Self {
            log_channel: LogChannel::General,
            current_log_level,
            runner,
            logger,
        }
    }

    // Export for unit tests in Godot
    #[cfg(any(test, feature = "test"))]
    #[must_use]
    pub fn test() -> Self {
        Self {
            current_log_level: LogType::Warn,
            log_channel: LogChannel::General,
            runner: Arc::new(MockTestActualRunner::new()),
            logger: Arc::new(DecktricksConsoleLogger::new()),
        }
    }

    #[must_use]
    pub fn test_with_runner(runner: RunnerRc) -> Self {
        Self {
            current_log_level: LogType::Warn,
            log_channel: LogChannel::General,
            runner,
            logger: Arc::new(DecktricksConsoleLogger::new()),
        }
    }
}

impl ExecCtx for GeneralExecutionContext {
    fn as_ctx(&self) -> ExecutionContext {
        ExecutionContext::General(self.clone())
    }

    fn get_runner(&self) -> &RunnerRc {
        &self.runner
    }

    fn get_log_channel(&self) -> &LogChannel {
        &self.log_channel
    }

    fn get_current_log_level(&self) -> LogType {
        self.current_log_level
    }

    fn get_logger(&self) -> LoggerRc {
        self.logger.clone()
    }
}

impl ExecCtx for SpecificExecutionContext {
    fn as_ctx(&self) -> ExecutionContext {
        ExecutionContext::Specific(self.clone())
    }

    fn get_runner(&self) -> &RunnerRc {
        &self.runner
    }

    fn get_log_channel(&self) -> &LogChannel {
        &self.log_channel
    }

    fn get_current_log_level(&self) -> LogType {
        self.current_log_level
    }

    fn get_logger(&self) -> LoggerRc {
        self.logger.clone()
    }
}

impl SpecificExecutionContext {
    #[must_use]
    pub fn new(trick: Trick, runner: RunnerRc, current_log_level: LogType, logger: LoggerRc) -> Self {
        Self {
            log_channel: LogChannel::TrickID(trick.id.clone()),
            current_log_level,
            runner,
            trick,
            logger,
        }
    }

    #[cfg(test)]
    pub(crate) fn test(trick: Trick) -> Self {
        Self {
            log_channel: LogChannel::TrickID(trick.id.clone()),
            current_log_level: LogType::Warn,
            runner: Arc::new(MockTestActualRunner::new()),
            trick,
            logger: Arc::new(DecktricksConsoleLogger::new()),
        }
    }

    #[cfg(test)]
    pub(crate) fn test_with_runner(trick: Trick, runner: Arc<MockTestActualRunner>) -> Self {
        Self {
            log_channel: LogChannel::TrickID(trick.id.clone()),
            current_log_level: LogType::Warn,
            runner,
            trick,
            logger: Arc::new(DecktricksConsoleLogger::new()),
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
    current_log_level: LogType,
}

impl Executor {
    // In the context of this function, Command is used as "global action context"
    //
    /// # Errors
    ///
    /// Any errors that might arise from parsing the config
    /// or from gathering system resources.
    ///
    pub fn create_with_gather(
        mode: ExecutorMode,
        command: &DecktricksCommand,
        current_log_level: LogType,
        logger: LoggerRc,
    ) -> DeckResult<Self> {
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
            let gather_execution_ctx =
                GeneralExecutionContext::new(runner.clone(), current_log_level, logger);
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

        Ok(Self::with(mode, loader, full_ctx, runner, current_log_level))
    }

    #[must_use]
    pub fn with(
        mode: ExecutorMode,
        loader: TricksLoader,
        full_ctx: FullSystemContext,
        runner: RunnerRc,
        current_log_level: LogType,
    ) -> Self {
        Self {
            mode,
            loader,
            full_ctx,
            runner,
            current_log_level,
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
    pub fn execute(&self, command: &DecktricksCommand, logger: LoggerRc) -> Vec<DeckResult<ActionSuccess>> {
        let typed_action = TypedAction::from(&command.action);
        let current_log_level = command.log_level.unwrap_or(self.current_log_level);
        typed_action.do_with(self, current_log_level, logger)
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

        let ctx = ExecutionContext::general_for_test_with(runner.clone());
        let full_ctx = FullSystemContext::gather_with(&ctx)?;

        let executor = Executor::with(ExecutorMode::OnceOff, loader, full_ctx, runner, LogType::Warn);
        Ok(executor)
    }

    #[test]
    fn top_level_install() -> DeckResult<()> {
        let command = DecktricksCommand::new(Action::Install {
            id: "lutris".into(),
        });

        let executor = get_executor(None)?;
        let logger = crate::CRATE_DECKTRICKS_DEFAULT_LOGGER.clone();
        let results = executor.execute(&command, logger);
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
        let logger = crate::CRATE_DECKTRICKS_DEFAULT_LOGGER.clone();
        let results = executor.execute(&command, logger);
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
        let logger = crate::CRATE_DECKTRICKS_DEFAULT_LOGGER.clone();
        let results = executor.execute(&command, logger);
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
        let arg = SysCommand::new(&ExecutionContext::general_for_test(), cmd, args);
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
        let logger = crate::CRATE_DECKTRICKS_DEFAULT_LOGGER.clone();
        let results = executor.execute(&command, logger);
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
