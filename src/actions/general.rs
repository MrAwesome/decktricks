use crate::prelude::*;
use crate::run_system_command::system_command_output;
use rayon::prelude::*;

#[derive(Debug)]
pub(crate) enum GeneralAction {
    List { installed: bool },
    UpdateAll,
    SeeAllAvailableActions,
}

impl GeneralAction {
    pub(crate) fn do_with(&self, loader: &TricksLoader) -> DeckResult<ActionSuccess> {
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
                            let provider = DynProvider::try_from(trick);
                            provider.is_ok_and(|t| t.is_installed().is_ok())
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
                let results = get_all_available_actions_for_all_tricks(loader);

                // Convert the results from above into a commandline-friendly format
                for (trick_id, maybe_action_ids) in results {
                    let mut available = vec![];
                    for action in maybe_action_ids? {
                        let name = serde_json::to_string::<SpecificActionID>(&action)
                            .map_err(KnownError::ConfigParsing)?;

                        available.push(name);
                    }
                    let formatted = format!(
                        "{}:\n  {}",
                        trick_id.clone(),
                        available
                            .iter()
                            .map(|s| s.trim_matches(|c| c == '"'))
                            .collect::<Vec<_>>()
                            .join("\n  ")
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
) -> Vec<(TrickID, DeckResult<Vec<SpecificActionID>>)> {
    let tricks = loader.get_hashmap();

    tricks
        .par_iter()
        .filter_map(get_all_available_actions_for_trick)
        .collect::<Vec<(String, Result<Vec<_>, _>)>>()
}

fn get_all_available_actions_for_trick(
    (trick_id, trick): (&String, &Trick),
) -> Option<(String, DeckResult<Vec<SpecificActionID>>)> {
    let maybe_provider = DynProvider::try_from(trick);

    match maybe_provider {
        Err(KnownError::NotImplemented(_)) => {
            eprintln!(
                "INFO: Skipping unimplemented provider \"{}\" for trick \"{}\".",
                trick.provider_config, trick_id
            );
            None
        }
        Ok(provider) => Some((trick_id.clone(), provider.get_available_actions())),
        Err(e) => Some((trick_id.clone(), Err(e))),
    }
}
