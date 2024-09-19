use crate::prelude::*;
use crate::providers::types::*;
use crate::tricks_config::TricksConfig;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "decktricks")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Action,
}

#[derive(Subcommand)]
pub enum Action {
    Run {
        id: String,
    },
    Install {
        id: String,
    },
    Kill {
        id: String,
    },
    #[clap(alias = "remove")]
    Uninstall {
        id: String,
    },
    List {
        #[clap(long)]
        installed: bool,
    },
    AddToSteam {
        #[clap(long)]
        name: Option<String>,
        id: String,
    },
}

pub struct ActionSuccess {
    pub message: Option<String>
}

// TODO: have full list of errors
#[derive(Debug)]
pub struct ActionErrorTEMPORARY {
    pub message: String,
}

impl std::fmt::Display for ActionErrorTEMPORARY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error has occurred: {}", self.message)
    }
}

impl std::error::Error for ActionErrorTEMPORARY {}

impl Action {
    pub fn run_action(&self, config: TricksConfig) -> Result<ActionSuccess, DynamicError> {
        match self {
            Action::Run { id } => {
                let trick = config.get_trick(id)?;
                let provider = provider_from_trick(trick)?;

                provider.is_runnable()?.run()?;

                Ok(ActionSuccess { message: None })
            },
            Action::Install { id } => {
                let trick = config.get_trick(id)?;
                let provider = provider_from_trick(trick)?;

                provider.is_installable()?.install()?;

                let message = Some(format!("Trick \"{id}\" installed successfully!"));
                Ok(ActionSuccess { message })
            },
            Action::Uninstall { id } => {
                let trick = config.get_trick(id)?;
                let provider = provider_from_trick(trick)?;

                provider.is_installed()?.remove()?;

                let message = Some(format!("Trick \"{id}\" removed successfully!"));
                Ok(ActionSuccess { message })
            },
            Action::List { installed } => {
                let tricks = config.get_all_tricks();

                let tricks_names: Vec<&str> = 
                    match installed {
                        false => tricks.map(|nat| nat.0.as_str()).collect(),
                        true => tricks.filter(|nat| 
                            provider_from_trick(nat.1).is_ok_and(|t| t.is_installed().is_ok())
                        ).map(|nat| nat.0.as_str()).collect()
                    };

                let tricks_newline_delineated = tricks_names.join("\n");

                let message = Some(format!("{tricks_newline_delineated}"));
                Ok(ActionSuccess { message })
            },
            Action::Kill { id } => {
                let trick = config.get_trick(id)?;
                let provider = provider_from_trick(trick)?;

                provider.is_running()?.kill()?;

                Ok(ActionSuccess { message: None })
            }
            Action::AddToSteam { name: _, id: _ } => {
                unimplemented!()
            }


        }
    }
}
