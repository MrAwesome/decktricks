use crate::prelude::*;

// For the master list of available actions, see command.rs

mod general;
mod specific;

pub(crate) use general::*;
pub use specific::SpecificActionID;
pub use specific::SpecificAction;
pub(crate) use specific::*;

#[derive(Debug, Clone)]
pub enum TypedAction {
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
            Action::AddToSteam { id } => Self::Specific(SpecificAction::AddToSteam { id }),
            Action::Uninstall { id } => Self::Specific(SpecificAction::Uninstall { id }),
            Action::Update { id: Some(id) } => Self::Specific(SpecificAction::Update { id }),

            Action::Update { id: None } => Self::General(GeneralAction::UpdateAll {}),
            Action::List { installed } => Self::General(GeneralAction::List { installed }),
            Action::Actions { id, json } => Self::General(GeneralAction::Actions { id, json }),
            Action::Gui { gui } => Self::General(GeneralAction::Gui { gui }),
            Action::GetConfig => Self::General(GeneralAction::GetConfig),
            Action::Version { verbose } => Self::General(GeneralAction::Version { verbose }),
            Action::Steam { _steam_subcommand } => Self::General(GeneralAction::Steam { _steam_subcommand }),

            // Internal use:
            Action::GetActionDisplayNameMapping => {
                Self::General(GeneralAction::GetActionDisplayNameMapping)
            }
            Action::GatherContext => Self::General(GeneralAction::GatherContext),
            Action::RunSystemCommand { command, args } => {
                Self::General(GeneralAction::RunSystemCommand { command, args })
            }
            Action::DebugSteamShortcuts { filename } => {
                Self::General(GeneralAction::DebugSteamShortcuts { filename })
            }
            Action::AddDecktricksToSteam => Self::General(GeneralAction::AddDecktricksToSteam),
        }
    }
}

impl TypedAction {
    pub fn do_with(
        self,
        executor: &Executor,
        current_log_level: LogType,
        logger: LoggerRc,
    ) -> Vec<DeckResult<ActionSuccess>> {
        match self {
            Self::General(general_action) => {
                general_action.do_with(executor, current_log_level, &logger)
            }
            Self::Specific(specific_action) => {
                vec![specific_action.do_with(executor, current_log_level, logger)]
            }
        }
    }
}

#[derive(Debug)]
pub struct ActionSuccess {
    message: Option<String>,
}

impl ActionSuccess {
    #[must_use]
    pub fn get_message(&self) -> Option<String> {
        self.message.clone()
    }

    #[must_use]
    pub fn get_message_or_blank(&self) -> String {
        self.message.clone().unwrap_or_default()
    }
}

impl From<&ActionSuccess> for String {
    fn from(value: &ActionSuccess) -> Self {
        value.get_message_or_blank()
    }
}

impl ActionSuccess {
    pub(crate) fn success(msg: Option<impl AsRef<str>>) -> Self {
        Self {
            message: msg.map(|s| s.as_ref().into()),
        }
    }
}
