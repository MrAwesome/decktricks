use crate::prelude::*;

#[cfg(not(test))]
use crate::run_system_command::system_command_output;

pub(super) fn get_running_flatpak_applications() -> DeckResult<Vec<String>> {
    // TODO: error handling
    let ps_output = flatpak_ps();

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

#[cfg(not(test))]
pub(super) fn flatpak_ps() -> DeckResult<ActionSuccess> {
    // NOTE: to see what this actually sees here, pipe it to cat.
    system_command_output("flatpak", vec!["ps", "--columns=application"])
}

#[cfg(test)]
pub(super) fn flatpak_ps() -> DeckResult<ActionSuccess> {
    // TODO: test failures here
    //Err(KnownError::SystemCommandParse(err!("jlkfdsjfd")))

    success!("running_package\nrunning_package2")
}
