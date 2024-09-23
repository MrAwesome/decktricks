use crate::tricks_config::TrickID;
use crate::prelude::*;
use crate::actions::*;
use crate::providers::Provider;
use crate::tricks_config::TricksConfig;

fn run_action_with_config(action: &Action, config: &TricksConfig) -> Result<ActionSuccess, DynamicError> {
    let maybe_trick_id = action.get_trick_id();
    match maybe_trick_id {
        Some(trick_id) => run_trick_action(trick_id.to_owned(), action, config),
        None => run_general_action(action, config)
    }
}

fn run_general_action(action: &Action, config: &TricksConfig) -> Result<ActionSuccess, DynamicError> {
    match action {
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

            let message = Some(tricks_newline_delineated);
            Ok(ActionSuccess { message })
        },
        Action::Run { .. } | Action::Kill { .. } | Action::Info { .. } | Action::Install { .. } | Action::AddToSteam { .. } | Action::Uninstall { .. } => {
            let error_type = "wrong-action-individual-in-general";
            let location = "run_general_action";
            let message = format!("Individual action type was passed to general type function! Type: {:?}", action);
            Err(Box::new(SeriousError::new(error_type, location, &message)))
        }
    }

}

fn run_trick_action(trick_id: TrickID, action: &Action, config: &TricksConfig) -> Result<ActionSuccess, DynamicError> {
    let trick = config.get_trick(trick_id.as_ref())?;
    let provider: dyn Provider = provider_from_trick(trick)?;
    let possible = provider.possible();

    if !possible.contains(&action.id()) {
        // XXX
        //return Err(format!("Action \"{:?}\", supported actions: ", action.try_into()));
        unimplemented!()
    }

    match action {
        Action::Install { id } => provider.install(),
        Action::Run { id } => provider.run(),
        Action::Uninstall { id } => provider.uninstall(),
        Action::AddToSteam { name, id } => provider.add_to_steam(),
        Action::Kill { id } => provider.kill(),

        // TODO: this is provider-agnostic, just run code here
        Action::Info { id } => unimplemented!(),

        // All general actions should be caught here
        Action::List { .. } => {
            let error_type = "wrong-action-general-in-individual";
            let location = "run_trick_action";
            let message = format!("General action type was passed to individual type function! Type: {:?}", action);
            Err(Box::new(SeriousError::new(error_type, location, &message)))
        }
    }
}
