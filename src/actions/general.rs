use crate::steam::SteamSubcommand;
use crate::add_to_steam::debug_steam_shortcuts;
use crate::gui::GuiType;
use crate::prelude::*;
use crate::providers::decky_installer::DeckyInstallerGeneralProvider;
use crate::providers::flatpak::FlatpakGeneralProvider;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub(crate) enum GeneralAction {
    Gui {
        gui: GuiType,
    },
    List {
        installed: bool,
    },
    Actions {
        id: Option<String>,
        json: bool,
    },
    UpdateAll,
    GetConfig,
    Version { verbose: bool },
    Steam { steam_subcommand: SteamSubcommand },

    // Internal use:
    GetActionDisplayNameMapping,
    GatherContext,
    RunSystemCommand {
        command: String,
        args: Option<Vec<String>>,
    },
    DebugSteamShortcuts {
        filename: Option<String>,
    },
    AddDecktricksToSteam,
}

impl GeneralAction {
    pub(crate) fn do_with(self, executor: &Executor, current_log_level: LogType, logger: &LoggerRc) -> Vec<DeckResult<ActionSuccess>> {
        let (loader, full_ctx, runner) = executor.get_pieces();
        let general_ctx = GeneralExecutionContext::new(runner.clone(), current_log_level, logger.clone());
        match self {
            Self::List { installed } => {
                let tricks = loader.get_all_tricks();

                let tricks_names: Vec<&str> = if installed {
                    tricks
                        .filter(|name_and_trick| {
                            let trick = name_and_trick.1;
                            let trick_ctx =
                                SpecificExecutionContext::new(trick.clone(), runner.clone(), current_log_level, logger.clone());
                            let provider = DynTrickProvider::new(&trick_ctx, full_ctx);
                            provider.is_installed()
                        })
                        .map(|name_and_trick| name_and_trick.0.as_str())
                        .collect()
                } else {
                    tricks
                        .map(|name_and_trick| name_and_trick.0.as_str())
                        .collect()
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
                    Box::new(FlatpakGeneralProvider::new(general_ctx)),
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
                    executor, &id, current_log_level, logger.clone(), json,
                )]
            }
            Self::Gui { gui } => vec![gui.launch(executor)],
            Self::GatherContext => vec![],
            Self::GetConfig => {
                // TODO: if using live configs, use here
                vec![success!(DEFAULT_CONFIG_CONTENTS)]
            }
            Self::Version { verbose } => {
                let ver_str = env!("CARGO_PKG_VERSION");
                let msg = if verbose {
                    let git_hash = option_env!("/tmp/.decktricks_git_hash").unwrap_or_default();
                    let git_title = option_env!("/tmp/.decktricks_git_title").unwrap_or_default();
                    format!("Version: {ver_str}\nGit Hash: {git_hash}\nTitle: {git_title}")
                } else {
                    ver_str.to_string()
                };
                vec![success!(msg)]
            }
            Self::Steam { steam_subcommand } => {
                // TODO
                vec![]
            }

            Self::GetActionDisplayNameMapping => {
                let display_mapping = SpecificActionID::get_display_name_mapping();
                let maybe_json_display_mapping =
                    serde_json::to_string(&display_mapping).map_err(KnownError::from);
                match maybe_json_display_mapping {
                    Ok(json_display_mapping) => vec![success!(json_display_mapping)],
                    Err(err) => vec![Err(err)],
                }
            }
            Self::RunSystemCommand { command, args } => {
                vec![internal_test_run_system_command(
                    &general_ctx,
                    command,
                    args,
                )]
            }
            Self::DebugSteamShortcuts { filename } => vec![debug_steam_shortcuts(filename)],
            Self::AddDecktricksToSteam => vec![add_to_steam(&AddToSteamTarget::Decktricks)],
        }
    }
}

fn get_all_available_actions_for_all_tricks(
    executor: &Executor,
    current_log_level: LogType,
    logger: &LoggerRc,
) -> Vec<(TrickID, Vec<SpecificActionID>)> {
    let (loader, _full_ctx, _runner) = executor.get_pieces();
    let tricks = loader.get_hashmap();

    let mut name_to_actions = vec![];
    for (id, trick) in tricks {
        let actions = get_all_available_actions_for_trick(executor, trick, current_log_level, logger.clone());

        name_to_actions.push((id.clone(), actions));
    }

    // We sort so that `actions --json` does not change between runs
    // unless the system state has changed
    name_to_actions.sort_by_key(|k| k.0.clone());

    name_to_actions
}

fn get_all_available_actions_for_trick(
    executor: &Executor,
    trick: &Trick,
    current_log_level: LogType,
    logger: LoggerRc,
) -> Vec<SpecificActionID> {
    let (_loader, full_ctx, runner) = executor.get_pieces();
    let ctx = SpecificExecutionContext::new(trick.clone(), runner.clone(), current_log_level, logger);
    let provider = DynTrickProvider::new(&ctx, full_ctx);

    provider.get_available_actions()
}

fn get_all_available_actions(
    executor: &Executor,
    maybe_id: &Option<TrickID>,
    current_log_level: LogType,
    logger: LoggerRc,
    json: bool,
) -> DeckResult<ActionSuccess> {
    let (loader, _full_ctx, _runner) = executor.get_pieces();
    if let Some(id) = maybe_id {
        let trick = loader.get_trick(id.as_ref())?;
        let action_ids = get_all_available_actions_for_trick(executor, trick, current_log_level, logger);

        let mut names = vec![];
        for action_id in action_ids {
            names.push(String::try_from(&action_id)?);
        }

        // TODO: unit test this:
        if json {
            success!(serde_json::to_string(&names).map_err(KnownError::from)?)
        } else {
            success!(names.join("\n"))
        }
    } else {
        let mut all_available = vec![];
        let results = get_all_available_actions_for_all_tricks(executor, current_log_level, &logger);

        // TODO: unit test this:
        let output = if json {
            // NOTE: this is a BTreeMap so that sort order is maintained
            let results_map: std::collections::BTreeMap<_, _> = results.into_iter().collect();
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

fn internal_test_run_system_command(
    ctx: &impl ExecCtx,
    command: String,
    maybe_args: Option<Vec<String>>,
) -> DeckResult<ActionSuccess> {
    let real_args = maybe_args.unwrap_or_default();
    SysCommand::new(ctx, command, real_args)
        .run()?
        .as_success()
}
