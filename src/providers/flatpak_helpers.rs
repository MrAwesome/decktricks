use crate::prelude::*;

pub(super) fn get_running_flatpak_applications(ctx: &impl ExecutionContextTrait) -> DeckResult<Vec<String>> {
    // TODO: error handling
    let ps_output = flatpak_ps(ctx);

    match &ps_output {
        Ok(output_obj) => {
            let text = output_obj.get_message_or_blank();
            let lines = text.lines().map(String::from).collect();
            Ok(lines)
        }
        Err(e) => Err(KnownError::SystemCommandParse(dterr!(
            "Failed to parse 'flatpak ps' output: {e:?}"
        ))),
    }
}

pub(super) fn get_installed_flatpak_applications(ctx: &impl ExecutionContextTrait) -> DeckResult<Vec<String>> {
    // TODO: error handling
    let list_output = flatpak_list(ctx);

    match &list_output {
        Ok(output_obj) => {
            let text = output_obj.get_message_or_blank();
            let lines = text.lines().map(String::from).collect();
            Ok(lines)
        }
        Err(e) => Err(KnownError::SystemCommandParse(dterr!(
            "Failed to parse 'flatpak list' output: {e:?}"
        ))),
    }
}

fn flatpak_list(ctx: &impl ExecutionContextTrait) -> DeckResult<ActionSuccess> {
    // NOTE: when debugging, to see what this actually sees here, pipe flatpak list to cat.
    SysCommand::new("flatpak", ["list", "--app", "--columns=application"]).run_with(ctx)?.as_success()
}

fn flatpak_ps(ctx: &impl ExecutionContextTrait) -> DeckResult<ActionSuccess> {
    // NOTE: when debugging, to see what this actually sees here, pipe flatpak ps to cat.
    SysCommand::new("flatpak", ["ps", "--columns=application"]).run_with(ctx)?.as_success()
}
