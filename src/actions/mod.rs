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
    message: Option<String>
}

// TODO: have full list of errors
#[derive(Debug)]
pub struct ActionErrorTEMPORARY {
    message: String,
}

impl std::fmt::Display for ActionErrorTEMPORARY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error has occurred")
    }
}

impl std::error::Error for ActionErrorTEMPORARY {}

impl Action {
    pub fn run(&self, config: TricksConfig) -> Result<ActionSuccess, Box<dyn std::error::Error>> {
        match &self {
            Action::Run { id } => {
                // TODO: return errors from here, do not treat this as console code
                // don't get the config every time?
                //
                //let all_tricks = config.get_all_tricks();
                //dbg!(&all_tricks);

                let maybe_trick = config.get_trick(id);
                // TODO: provider_from_id

                let trick = match maybe_trick {
                    Some(trick) => trick,
                    None => return Err(Box::new(ActionErrorTEMPORARY { message: format!("Trick not known: {id}") }))
                };

                let provider = provider_from_trick(trick);
                //unimplemented!();

                provider.is_runnable().unwrap().run();

                Ok(ActionSuccess { message: None })

            }
            _ => {
                // XXX
                unimplemented!()
            }
        }
    }
}
