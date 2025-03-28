use crate::prelude::*;

pub(super) fn get_running_flatpak_applications(ctx: &impl ExecCtx) -> DeckResult<Vec<String>> {
    let ps_output = flatpak_ps(ctx);

    match &ps_output {
        Ok(output_obj) => {
            let text = output_obj.get_message_or_blank();
            let lines = text.lines().map(String::from).collect();
            Ok(lines)
        }
        Err(e) => Err(KnownError::SystemCommandParse(format!(
            "Failed to parse 'flatpak ps' output: {e:?}"
        ))),
    }
}

pub(super) fn get_installed_flatpak_applications(ctx: &impl ExecCtx) -> DeckResult<Vec<String>> {
    let list_output = flatpak_list(ctx);

    match &list_output {
        Ok(output_obj) => {
            let text = output_obj.get_message_or_blank();
            let lines = text.lines().map(String::from).collect();
            Ok(lines)
        }
        Err(e) => Err(KnownError::SystemCommandParse(format!(
            "Failed to parse 'flatpak list' output: {e:?}"
        ))),
    }
}

fn flatpak_list(ctx: &impl ExecCtx) -> DeckResult<ActionSuccess> {
    // NOTE: when debugging, to see what this actually sees here, pipe flatpak list to cat.
    SysCommand::new(ctx, "flatpak", ["list", "--app", "--columns=application"]).run()?.as_success()
}

fn flatpak_ps(ctx: &impl ExecCtx) -> DeckResult<ActionSuccess> {
    // NOTE: when debugging, to see what this actually sees here, pipe flatpak ps to cat.
    SysCommand::new(ctx, "flatpak", ["ps", "--columns=application"]).run()?.as_success()
}
