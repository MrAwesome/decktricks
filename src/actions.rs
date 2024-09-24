use clap::{Parser, Subcommand};
use serde::Serialize;

#[derive(Parser)]
#[clap(name = "decktricks")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Action,
}

// This is the canonical list of actions we can take on tricks.
// It is directly parsed by clap for the commandline, and should be
// the entry point for any GUI or downstream lib.
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
    AddToSteam {
        #[clap(long)]
        name: Option<String>,
        id: String,
    },
    Info {
        id: String,
    },
    // Items below do not take trick ids, and function differently.
    List {
        #[clap(long)]
        installed: bool,
    },
}

impl Action {
    pub fn get_trick_id(&self) -> Option<&str> {
        match self {
            Action::Run { id }
            | Action::Kill { id }
            | Action::Info { id }
            | Action::Install { id }
            | Action::AddToSteam { id, .. }
            | Action::Uninstall { id } => Some(id),
            Action::List { .. } => None,
        }
    }

//    pub(crate) fn id(&self) -> ActionID {
//        match self {
//            Self::Run { .. } => ActionID::Individual(TrickActionID::Run),
//            Self::Install { .. } => ActionID::Individual(TrickActionID::Install),
//            Self::Kill { .. } => ActionID::Individual(TrickActionID::Kill),
//            Self::Uninstall { .. } => ActionID::Individual(TrickActionID::Uninstall),
//            Self::AddToSteam { .. } => ActionID::Individual(TrickActionID::AddToSteam),
//            Self::Info { .. } => ActionID::Individual(TrickActionID::Info),
//
//            Self::List { .. } => ActionID::General(GeneralActionID::List),
//        }
//    }
}

pub struct ActionSuccess {
    pub message: Option<String>,
}

impl ActionSuccess {
    pub fn new<S: Into<String>>(msg: Option<S>) -> Self {
        Self {
            message: msg.map(|s| s.into())
        }
    }
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
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
