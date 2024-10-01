use crate::prelude::*;

pub struct Executor {
    pub loader: TricksLoader,
    pub full_ctx: FullSystemContext,
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

        let full_ctx = FullSystemContext::gather()?;

        Ok(Self { loader, full_ctx })
    }

    #[must_use]
    pub fn with(loader: TricksLoader, full_ctx: FullSystemContext) -> Self {
        Self { loader, full_ctx }
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
        typed_action.do_with(&self.loader, &self.full_ctx)
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
