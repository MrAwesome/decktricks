use clap::Subcommand;
use crate::prelude::*;

#[derive(Debug, Clone, Subcommand)]
pub enum GuiType {
}

impl GuiType {
    /// # Errors
    ///
    /// Can return any error from running any of our actions.
    pub fn launch(
        &self,
        _executor: &Executor,
    ) -> DeckResult<ActionSuccess> {
        success!("No GUIs implemented through this flow.")
    }
}
