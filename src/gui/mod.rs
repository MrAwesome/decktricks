use clap::Subcommand;
use crate::prelude::*;

pub mod zenity;

#[derive(Debug, Clone, Subcommand)]
pub enum GuiType {
    Zenity,
}

impl GuiType {
    /// # Errors
    ///
    /// Can return any error from running any of our actions.
    pub fn launch(
        &self,
        executor: &Executor,
    ) -> DeckResult<ActionSuccess> {
        match self {
            Self::Zenity => {
                zenity::zenity_launch(executor)
            }
        }
    }
}
