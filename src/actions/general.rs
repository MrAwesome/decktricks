use crate::prelude::*;
use crate::run_system_command::system_command_output;

#[derive(Debug)]
pub(crate) enum GeneralAction {
    List { installed: bool },
    UpdateAll,
    SeeAllAvailableActions,
}

impl GeneralAction {
    pub(crate) fn do_with(&self, loader: &TricksLoader, full_ctx: &FullSystemContext) -> DeckResult<ActionSuccess> {
        match self {
            Self::List { installed } => {
                let tricks = loader.get_all_tricks();

                let tricks_names: Vec<&str> = match installed {
                    false => tricks
                        .map(|name_and_trick| name_and_trick.0.as_str())
                        .collect(),
                    true => tricks
                        .filter(|name_and_trick| {
                            let trick = name_and_trick.1;
                            let provider = DynProvider::try_from((trick, full_ctx));
                            provider.is_ok_and(|t| t.is_installed())
                        })
                        .map(|name_and_trick| name_and_trick.0.as_str())
                        .collect(),
                };

                let tricks_newline_delineated = tricks_names.join("\n");
                success!(tricks_newline_delineated)
            }
            Self::UpdateAll => {
                // TODO: in a typesafe way, iterate over all known providers, run their global
                // update logic, and if they don't have global update logic, run per-trick updates
                // for every known trick

                // NOTE!!! for flatpak update -y, you MUST run it twice to remove unused runtimes.
                system_command_output("flatpak", vec!["update", "-y"])?;
                system_command_output("flatpak", vec!["update", "-y"])?;

                todo!("Run global update procedures for all providers: for those that have global, run global (flatpak update). For those that don't, specifically update all installed packages.")
            }
            Self::SeeAllAvailableActions => {
                let mut all_available = vec![];
                let results = get_all_available_actions_for_all_tricks(loader, full_ctx)?;

                // Convert the results from above into a commandline-friendly format
                for (trick_id, maybe_action_ids) in results {
                    let mut available: Vec<String> = vec![];
                    for action in maybe_action_ids {
                        let name = serde_json::to_string::<SpecificActionID>(&action)
                            .map_err(KnownError::ConfigParsing)?
                            .trim_matches(|c| c == '"')
                            .into();

                        available.push(name);
                    }

                    let formatted = format!(
                        "{}:\n  {}",
                        trick_id.clone(),
                        available.join("\n  ")
                    );
                    all_available.push(formatted);
                }

                all_available.sort();
                let output = all_available.join("\n");
                success!(output)
            }
        }
    }
}

fn get_all_available_actions_for_all_tricks(
    loader: &TricksLoader,
    full_ctx: &FullSystemContext,
) -> DeckResult<Vec<(TrickID, Vec<SpecificActionID>)>> {
    let tricks = loader.get_hashmap();

    let mut name_to_actions = vec![];
    for (id, trick) in tricks {
        let option_actions = get_all_available_actions_for_trick(trick, full_ctx)?;

        if let Some(actions) = option_actions {
            name_to_actions.push((id.clone(), actions));
        }
    }

    Ok(name_to_actions)
}

fn get_all_available_actions_for_trick(
    trick: &Trick,
    full_ctx: &FullSystemContext,
) -> DeckResult<Option<Vec<SpecificActionID>>> {
    let maybe_provider = DynProvider::try_from((trick, full_ctx));

    match maybe_provider {
        Err(KnownError::NotImplemented(_)) => {
            info!(
                "Skipping unimplemented provider \"{}\" for trick \"{}\".",
                trick.provider_config, trick.id
            );
            Ok(None)
        }
        Ok(provider) => Ok(Some(provider.get_available_actions())),
        Err(e) => Err(e),
    }
}
