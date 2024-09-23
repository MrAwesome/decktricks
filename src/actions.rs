use serde::Serialize;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "decktricks")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Action,
}

#[derive(Debug, Subcommand)]
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
    Info {
        id: String,
    },
}

impl Action {
    pub fn get_trick_id(&self) -> Option<&str> {
        match self {
            Action::Run { id } | Action::Kill { id } | Action::Info { id } | Action::Install { id } | Action::AddToSteam { id, .. } | Action::Uninstall { id } => Some(id),
            Action::List { .. } => None,
        }
    }

    pub fn id(&self) -> ActionID {
        match self {
            Self::Run { .. } => ActionID::Individual(TrickActionID::Run),
            Self::Install { .. } => ActionID::Individual(TrickActionID::Install),
            Self::Kill { .. } => ActionID::Individual(TrickActionID::Kill),
            Self::Uninstall { .. } => ActionID::Individual(TrickActionID::Uninstall),
            Self::AddToSteam { .. } => ActionID::Individual(TrickActionID::AddToSteam),
            Self::Info { .. } => ActionID::Individual(TrickActionID::Info),
            Self::List { .. } => ActionID::General(GeneralActionID::List),
        }
    }
}

pub struct ActionSuccess {
    pub message: Option<String>
}


#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TrickActionID {
    Run,
    Install,
    Kill,
    Uninstall,
    AddToSteam,
    Info,
}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GeneralActionID {
    List,
}

pub enum ActionID {
    Individual(TrickActionID),
    General(GeneralActionID),
}

#[test]
fn action_name() {
    let action = GeneralActionID::List;
    assert_eq!("install", format!("{:?}", action));
}

//const fn action_to_id(action: &Action) -> ActionID {
//    match action {
//        Action::Run { .. } => ActionID::Run,
//        _ => ActionID::Info
//    }
//}
//

//const DEFAULT_ACTIONS: [Action; 1] = [Action::Info];

//#[derive(Subcommand)]
//pub enum Action {
//    Run {
//        id: String,
//    },
//    Install {
//        id: String,
//    },
//    Kill {
//        id: String,
//    },
//    #[clap(alias = "remove")]
//    Uninstall {
//        id: String,
//    },
//    List {
//        #[clap(long)]
//        installed: bool,
//    },
//    AddToSteam {
//        #[clap(long)]
//        name: Option<String>,
//        id: String,
//    },
//}
//
//impl Action {
//    pub fn run_action(&self, config: TricksConfig) -> Result<ActionSuccess, DynamicError> {
//        match self {
//            Action::Run { id } => {
//                let trick = config.get_trick(id)?;
//                let provider = provider_from_trick(trick)?;
//
//                provider.is_runnable()?.run()?;
//
//                Ok(ActionSuccess { message: None })
//            },
//            Action::Install { id } => {
//                let trick = config.get_trick(id)?;
//                let provider = provider_from_trick(trick)?;
//
//                provider.is_installable()?.install()?;
//
//                let message = Some(format!("Trick \"{id}\" installed successfully!"));
//                Ok(ActionSuccess { message })
//            },
//            Action::Uninstall { id } => {
//                let trick = config.get_trick(id)?;
//                let provider = provider_from_trick(trick)?;
//
//                provider.is_installed()?.remove()?;
//
//                let message = Some(format!("Trick \"{id}\" removed successfully!"));
//                Ok(ActionSuccess { message })
//            },
//            Action::List { installed } => {
//                let tricks = config.get_all_tricks();
//
//                let tricks_names: Vec<&str> = 
//                    match installed {
//                        false => tricks.map(|nat| nat.0.as_str()).collect(),
//                        true => tricks.filter(|nat| 
//                            provider_from_trick(nat.1).is_ok_and(|t| t.is_installed().is_ok())
//                        ).map(|nat| nat.0.as_str()).collect()
//                    };
//
//                let tricks_newline_delineated = tricks_names.join("\n");
//
//                let message = Some(tricks_newline_delineated);
//                Ok(ActionSuccess { message })
//            },
//            Action::Kill { id } => {
//                let trick = config.get_trick(id)?;
//                let provider = provider_from_trick(trick)?;
//
//                provider.is_running()?.kill()?;
//
//                Ok(ActionSuccess { message: None })
//            }
//            Action::AddToSteam { name: _, id: _ } => {
//                unimplemented!()
//            }
//
//
//        }
//    }
//}
