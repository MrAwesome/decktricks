use crate::prelude::*;
use clap::{Parser, Subcommand};

mod general;
mod specific;

pub(crate) use general::*;
pub(crate) use specific::*;

#[derive(Parser)]
#[clap(name = "decktricks")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Action,
}

// This is the canonical list of actions we can take on tricks.
// It is directly parsed by clap for the commandline, and should be
// the entry point for any GUI or downstream lib.
#[derive(Debug, Subcommand, Clone)]
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
    // Note that update can work both globally or for a specific id.
    Update {
        id: Option<String>
    },
    // Items below do not take trick ids, and function differently.
    List {
        #[clap(long)]
        installed: bool,
    },
    SeeAllAvailableActions,
}

impl Action {
    pub fn do_with(&self, loader: &TricksLoader) -> Result<ActionSuccess, KnownError> {
        let typed_action = TypedAction::from(self);
        typed_action.do_with(loader)
    }
}

pub(crate) enum TypedAction {
    Specific(SpecificAction),
    General(GeneralAction),
}

impl From<&Action> for TypedAction {
    fn from(action: &Action) -> Self {
        match action.clone() {
            Action::Run { id } => Self::Specific(SpecificAction::Run { id }),
            Action::Kill { id } => Self::Specific(SpecificAction::Kill { id }),
            Action::Info { id } => Self::Specific(SpecificAction::Info { id }),
            Action::Install { id } => Self::Specific(SpecificAction::Install { id }),
            Action::AddToSteam { name, id } => Self::Specific(SpecificAction::AddToSteam { name, id }),
            Action::Uninstall { id } => Self::Specific(SpecificAction::Uninstall { id }),
            Action::Update { id: Some(id) } => Self::Specific(SpecificAction::Update { id }),

            Action::Update { id: None } => Self::General(GeneralAction::UpdateAll {}),
            Action::List { installed } => Self::General(GeneralAction::List { installed }),
            Action::SeeAllAvailableActions => Self::General(GeneralAction::SeeAllAvailableActions),
        }
    } 
}

impl TypedAction {
    fn do_with(&self, loader: &TricksLoader) -> Result<ActionSuccess, KnownError> {
        match self {
            Self::General(general_action) => general_action.run(loader),
            Self::Specific(specific_action) => specific_action.run(loader),
        }
    }
}

//pub(crate) struct CheckFailure {
//    reason: String,
//}
//
//impl CheckFailure {
//    fn new(reason: String) -> Self {
//        Self { reason }
//    }
//}
//
//
//pub(crate) enum CheckOutcome {
//    Success,
//    Failure(CheckFailure),
//}

pub struct ActionSuccess {
    message: Option<String>,
}

impl ActionSuccess {
    pub fn get_message(&self) -> Option<String> {
        self.message.clone()
    }

    pub fn get_message_or_blank(&self) -> String {
        match self.message.clone() {
            Some(msg) => msg,
            None => "".into(),
        }
    }
}

impl ActionSuccess {
    pub(crate) fn success(msg: Option<impl AsRef<str>>) -> Self {
        Self {
            message: msg.map(|s| s.as_ref().into()),
        }
    }
}

// TODO: or just launch steamtinkerlaunch GUI manually?
pub(crate) struct AddToSteamContext {
    _name: Option<String>,
}

//
//#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
//#[serde(rename_all = "snake_case")]
//pub(crate) enum TrickActionID {
//    Run,
//    Install,
//    Kill,
//    Uninstall,
//    AddToSteam,
//    Info,
//}
//
//#[derive(Debug, Serialize, Eq, PartialEq)]
//#[serde(rename_all = "snake_case")]
//pub(crate) enum GeneralActionID {
//    List,
//}
//
//pub(crate) enum ActionID {
//    Individual(TrickActionID),
//    General(GeneralActionID),
//}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}