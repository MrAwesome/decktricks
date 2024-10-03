use crate::prelude::*;
use std::sync::Arc;

pub struct Executor {
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
    pub fn new(command: &Command) -> DeckResult<Self> {
        let maybe_config_path = command.config.as_ref();
        let loader = match maybe_config_path {
            Some(config_path) => TricksLoader::from_config(config_path)?,
            None => TricksLoader::from_default_config()?,
        };

        let runner = Arc::new(Runner::new());

        let full_ctx = FullSystemContext::gather_with(&runner)?;

        Ok(Self::with(loader, full_ctx, runner))
    }

    #[must_use]
    pub fn with(loader: TricksLoader, full_ctx: FullSystemContext, runner: RunnerRc) -> Self {
        Self {
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
        typed_action.do_with(&self.loader, &self.full_ctx, &self.runner)
    }

    //    pub fn reload_config(&mut self) -> DeckResult<()> {
    //        self.loader = TricksLoader::from_disk_config()?;
    //        Ok(())
    //    }

    //    pub fn reload_system_context(&mut self) -> DeckResult<()> {
    //        self.full_ctx = FullSystemContext::gather()?;
    //        Ok(())
    //    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_executor() -> DeckResult<Executor> {
        let loader = TricksLoader::from_default_config()?;

        let mut mock = MockTestActualRunner::new();
        mock.expect_run()
            .returning(|_| Ok(SysCommandResult::fake_success()));

        let runner = Arc::new(mock);

        let ctx = FullSystemContext::gather_with(&runner)?;

        let executor = Executor::with(loader, ctx, runner);
        Ok(executor)
    }

    #[test]
    fn test_top_level_install() -> DeckResult<()> {
        let command = Command {
            action: Action::Install {
                id: "lutris".into(),
            },
            config: None,
        };

        let executor = get_executor()?;
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
    fn test_top_level_incorrect_run() -> DeckResult<()> {
        let command = Command {
            action: Action::Run {
                id: "FAKE_PACKAGE".into(),
            },
            config: None,
        };

        let executor = get_executor()?;
        let results = executor.execute(&command.action);
        assert_eq!(results.len(), 1);
        match &results[0] {
            Ok(action_success) => panic!("unexpected successful installation for nonexistent package: {action_success:?}"),
            Err(KnownError::UnknownTrickID(pkg)) => assert_eq!("FAKE_PACKAGE", pkg),
            Err(e) => panic!("unexpected failure in test: {e:?}"),
        }
        Ok(())
    }
}
