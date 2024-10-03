use crate::prelude::*;

pub(super) fn get_running_flatpak_applications(runner: &RunnerRc) -> DeckResult<Vec<String>> {
    // TODO: error handling
    let ps_output = flatpak_ps(runner);

    // XXX
    println!("========== result: {}", &ps_output.as_ref().unwrap().get_message_or_blank());

    match &ps_output {
        Ok(output_obj) => {
            let text = output_obj.get_message_or_blank();
            let lines = text.lines().map(String::from).collect();
            Ok(lines)
        }
        Err(e) => Err(KnownError::SystemCommandParse(err!(
            "Failed to parse 'flatpak ps' output: {e:?}"
        ))),
    }
}

pub(super) fn get_installed_flatpak_applications(runner: &RunnerRc) -> DeckResult<Vec<String>> {
    // TODO: error handling
    let list_output = flatpak_list(runner);

    match &list_output {
        Ok(output_obj) => {
            let text = output_obj.get_message_or_blank();
            let lines = text.lines().map(String::from).collect();
            Ok(lines)
        }
        Err(e) => Err(KnownError::SystemCommandParse(err!(
            "Failed to parse 'flatpak list' output: {e:?}"
        ))),
    }
}

fn flatpak_list(runner: &RunnerRc) -> DeckResult<ActionSuccess> {
    // NOTE: when debugging, to see what this actually sees here, pipe flatpak list to cat.
    SysCommand::new("flatpak", vec!["list", "--app", "--columns=application"]).run_with(runner)?.as_success()
}

fn flatpak_ps(runner: &RunnerRc) -> DeckResult<ActionSuccess> {
    // NOTE: when debugging, to see what this actually sees here, pipe flatpak ps to cat.
    SysCommand::new("flatpak", vec!["ps", "--columns=application"]).run_with(runner)?.as_success()
}
