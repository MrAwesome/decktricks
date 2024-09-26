use crate::prelude::*;
use crate::run_system_command::system_command_output;

#[derive(Debug)]
pub(crate) enum GeneralAction {
    List { installed: bool },
    UpdateAll,
    SeeAllAvailableActions,
}

impl GeneralAction {
    pub(crate) fn run(&self, loader: &TricksLoader) -> Result<ActionSuccess, KnownError> {
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
                let mut all_available: Vec<String> = vec![];
                let tricks = loader.get_all_tricks();

                for (id, trick) in tricks {
                    let maybe_provider = DynProvider::try_from(trick);

                    // TODO: better error logging
                    match maybe_provider {
                        Err(KnownError::NotImplemented(_)) => {
                            eprintln!(
                                "INFO: Skipping unimplemented provider \"{}\" for trick \"{}\".",
                                trick.provider_config, id
                            );
                        }
                        provider => {
                            let mut available = vec![];
                            for action in provider?.get_available_actions()? {
                                let action_id = serde_json::to_string::<SpecificActionID>(&action)
                                    .map_err(|e| KnownError::ConfigParsing(e))?;
                                available.push(action_id);
                            }

                            all_available.push(format!("{}:\n  {}", id.clone(), available
                                    .iter()
                                    .map(|s| s.trim_matches(|c| c == '"'))
                                    .collect::<Vec<_>>()
                                    .join("\n  ")));
                        }
                    };
                }

                let output = all_available.join("\n");

                success!(output)
            }
        }
    }
}
