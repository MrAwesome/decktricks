use crate::prelude::*;

mod general;
mod specific;

pub(crate) use general::*;
pub(crate) use specific::*;

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
            Action::AddToSteam { name, id } => {
                Self::Specific(SpecificAction::AddToSteam { name, id })
            }
            Action::Uninstall { id } => Self::Specific(SpecificAction::Uninstall { id }),
            Action::Update { id: Some(id) } => Self::Specific(SpecificAction::Update { id }),

            Action::Update { id: None } => Self::General(GeneralAction::UpdateAll {}),
            Action::List { installed } => Self::General(GeneralAction::List { installed }),
            Action::Actions { id, json } => {
                Self::General(GeneralAction::Actions { id, json })
            }
            Action::Gui { gui } => {
                Self::General(GeneralAction::Gui { gui })
            }
        }
    }
}

impl TypedAction {
    pub(crate) fn do_with(
        &self,
        executor: &Executor,
    ) -> Vec<DeckResult<ActionSuccess>> {
        match self {
            Self::General(general_action) => general_action.do_with(executor),
            Self::Specific(specific_action) => {
                vec![specific_action.do_with(executor)]
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

// TODO: or just launch steamtinkerlaunch GUI manually?
#[derive(Debug, Default)]
pub(crate) struct AddToSteamContext {
    _name: Option<String>,
}
