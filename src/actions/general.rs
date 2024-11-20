use crate::gui::GuiType;
use crate::prelude::*;
use crate::providers::decky_installer::DeckyInstallerGeneralProvider;
use crate::providers::system_context::FullSystemContext;
use crate::providers::flatpak::FlatpakGeneralProvider;
use rayon::prelude::*;

#[derive(Debug)]
pub(crate) enum GeneralAction {
    Gui { gui: GuiType },
    List { installed: bool },
    Actions { id: Option<String>, json: bool },
    UpdateAll,
    GetConfig,
    GetActionDisplayNameMapping,
    GatherContext,
}

impl GeneralAction {
    pub(crate) fn do_with(
        &self,
        executor: &Executor,
    ) -> Vec<DeckResult<ActionSuccess>> {
        let (loader, full_ctx, runner) = executor.get_pieces();
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
                            let provider = DynProvider::try_from((trick, full_ctx, runner));
                            provider.is_ok_and(|t| t.is_installed())
                        })
                        .map(|name_and_trick| name_and_trick.0.as_str())
                        .collect(),
                };

                let tricks_newline_delineated = tricks_names.join("\n");
                vec![success!(tricks_newline_delineated)]
            }
            Self::UpdateAll => {
                // TODO: in a typesafe way, iterate over all known providers, run their global
                // update logic, and if they don't have global update logic, run per-trick updates
                // for every known trick?
                //
                // decktricks update <trick-id>
                // decktricks update_all [provtype]
                // decktricks update -> decktricks update_all

                let general_providers: Vec<Box<dyn GeneralProvider>> = vec![
                    Box::new(FlatpakGeneralProvider::new(runner.clone())),
                    Box::new(DeckyInstallerGeneralProvider),
                ];
                let mut results: Vec<DeckResult<ActionSuccess>> = general_providers
                    .par_iter()
                    .map(|p| p.update_all())
                    .collect();

                let any_failures = results.iter().any(Result::is_err);

                // TODO: print successes even if there are failures
                if !any_failures {
                    results.push(success!("All updates completed successfully!"));
                }

                results
            }
            Self::Actions { id, json } => {
                vec![get_all_available_actions(
                    loader, full_ctx, runner, id, *json,
                )]
            }
            Self::Gui { gui } => vec![gui.launch(executor)],
            Self::GatherContext => vec![],
            Self::GetConfig => {
                // TODO: if using live configs, use here
                vec![success!(DEFAULT_CONFIG_CONTENTS)]
            },
            Self::GetActionDisplayNameMapping => {
                let display_mapping = SpecificActionID::get_display_name_mapping();
                let maybe_json_display_mapping = serde_json::to_string(&display_mapping).map_err(KnownError::from);
                match maybe_json_display_mapping {
                    Ok(json_display_mapping) => vec![success!(json_display_mapping)],
                    Err(err) => vec![Err(err)],
                }
            }
        }
    }
}

fn get_all_available_actions_for_all_tricks(
    loader: &TricksLoader,
    full_ctx: &FullSystemContext,
    runner: &RunnerRc,
) -> DeckResult<Vec<(TrickID, Vec<SpecificActionID>)>> {
    let tricks = loader.get_hashmap();

    let mut name_to_actions = vec![];
    for (id, trick) in tricks {
        let maybe_actions = get_all_available_actions_for_trick(trick, full_ctx, runner)?;

        if let Some(actions) = maybe_actions {
            name_to_actions.push((id.clone(), actions));
        }
    }

    Ok(name_to_actions)
}

fn get_all_available_actions_for_trick(
    trick: &Trick,
    full_ctx: &FullSystemContext,
    runner: &RunnerRc,
) -> DeckResult<Option<Vec<SpecificActionID>>> {
    let maybe_provider = DynProvider::try_from((trick, full_ctx, runner));

    match maybe_provider {
        Err(KnownError::ProviderNotImplemented(_)) => {
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

fn get_all_available_actions(
    loader: &TricksLoader,
    full_ctx: &FullSystemContext,
    runner: &RunnerRc,
    maybe_id: &Option<TrickID>,
    json: bool,
) -> DeckResult<ActionSuccess> {
    if let Some(id) = maybe_id {
        let trick = loader.get_trick(id.as_ref())?;
        let maybe_actions = get_all_available_actions_for_trick(trick, full_ctx, runner)?;

        if let Some(action_ids) = maybe_actions {
            let mut names = vec![];
            for action_id in action_ids {
                names.push(String::try_from(&action_id)?);
            }

            // TODO: unit test this
            if json {
                success!(serde_json::to_string(&names).map_err(KnownError::from)?)
            } else {
                success!(names.join("\n"))
            }
        } else {
            Err(KnownError::NoAvailableActions(format!(
                "No actions available for \"{id}\"."
            )))
        }
    } else {
        let mut all_available = vec![];
        let results = get_all_available_actions_for_all_tricks(loader, full_ctx, runner)?;

        // TODO: unit test this
        let output = if json {
            let results_map: std::collections::HashMap<_, _> = results.into_iter().collect();
            serde_json::to_string(&results_map).map_err(KnownError::from)?
        } else {
            // Convert the results from above into a commandline-friendly format
            for (trick_id, maybe_action_ids) in results {
                let mut available: Vec<String> = vec![];
                for action_id in maybe_action_ids {
                    let name = String::try_from(&action_id)?;
                    available.push(name);
                }

                let formatted = format!("{}:\n  {}", trick_id.clone(), available.join("\n  "));
                all_available.push(formatted);
            }

            all_available.sort();
            all_available.join("\n")
        };
        success!(output)
    }
}
