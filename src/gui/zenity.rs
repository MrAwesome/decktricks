use crate::prelude::*;
use std::process::{Command, Stdio};

pub(super) fn zenity_launch(executor: &Executor) -> DeckResult<ActionSuccess> {
    let unchecked_results = zenity_run_action(executor)?;

    let output = unchecked_results
        .iter()
        .map(|r| match r {
            Ok(a) => String::from(a),
            Err(e) => String::from(e),
        })
        .collect::<Vec<_>>()
        .join("\n");

    Command::new("zenity")
        .arg("--info")
        .arg("--text")
        .arg(output.clone())
        .status()
        .expect("Failed to execute process");

    success!(output)
}

fn zenity_run_action(executor: &Executor) -> DeckResult<Vec<DeckResult<ActionSuccess>>> {
    let loader: &TricksLoader = &executor.loader;
    let full_ctx: &FullSystemContext = &executor.full_ctx;
    let runner: &RunnerRc = &executor.runner;

    let trick_ids = loader.get_all_tricks().map(|x| x.0);

    let program_output = Command::new("zenity")
        .arg("--list")
        .arg("--title=Select Trick")
        .arg("--column=Program")
        .args(trick_ids)
        .stdout(Stdio::piped())
        .output()
        .map_err(KnownError::from)?;

    let trick_id_untrimmed = String::from_utf8_lossy(&program_output.stdout);
    let trick_id = trick_id_untrimmed.trim();
    let trick = loader.get_trick(trick_id)?;
    let provider = DynProvider::try_from((trick, full_ctx, runner))?;
    let action_ids: Vec<_> = provider
        .get_available_actions()
        .iter()
        .map(String::try_from)
        .collect::<Result<_, _>>()?;

    let action_output = Command::new("zenity")
        .arg("--list")
        .arg("--title=Select Action")
        .arg("--column=Action")
        .args(action_ids)
        .stdout(Stdio::piped())
        .output()
        .map_err(KnownError::from)?;

    let action_id_untrimmed = String::from_utf8_lossy(&action_output.stdout);
    let action_id = action_id_untrimmed.trim();
    let command =
        DeckTricksCommand::try_parse_from(["decktricks", action_id, trick_id]).map_err(KnownError::from)?;
    let action = command.action;

    let results = executor.execute(&action);

    Ok(results)
}
