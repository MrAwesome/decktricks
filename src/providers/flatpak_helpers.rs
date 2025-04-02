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

pub(super) fn get_has_user_flathub(ctx: &impl ExecCtx) -> DeckResult<bool> {
    let remotes_output = flatpak_remotes(ctx);

    match &remotes_output {
        Ok(output_obj) => {
            let text = output_obj.get_message_or_blank();
            let lines: Vec<String> = text.lines().map(String::from).collect();

            let has_user_flathub =
                lines
                    .iter()
                    .map(|s| s.trim().split_whitespace())
                    .any(|mut words| {
                        words.next().is_some_and(|s| s == "flathub")
                            && words.next().is_some_and(|s| s == "user")
                    });

            Ok(has_user_flathub)
        }
        Err(e) => Err(KnownError::SystemCommandParse(format!(
            "Failed to parse 'flatpak list' output: {e:?}"
        ))),
    }
}

fn flatpak_remotes(ctx: &impl ExecCtx) -> DeckResult<ActionSuccess> {
    SysCommand::new(ctx, "flatpak", ["remotes", "--columns=name,options"])
        .run()?
        .as_success()
}

fn flatpak_list(ctx: &impl ExecCtx) -> DeckResult<ActionSuccess> {
    // NOTE: when debugging, to see what this actually sees here, pipe flatpak list to cat.
    SysCommand::new(ctx, "flatpak", ["list", "--app", "--columns=application"])
        .run()?
        .as_success()
}

fn flatpak_ps(ctx: &impl ExecCtx) -> DeckResult<ActionSuccess> {
    // NOTE: when debugging, to see what this actually sees here, pipe flatpak ps to cat.
    SysCommand::new(ctx, "flatpak", ["ps", "--columns=application"])
        .run()?
        .as_success()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;
    use std::sync::Arc;

    #[test]
    fn test_flatpak_remotes() -> DeckResult<()> {
        for (mocked_stdout, should_pass) in [
            ("flathub system\nflathub user", true),
            ("flathub user", true),
            ("flathub system\nflathub system", false),
            ("flthub system\nflathub usr", false),
            ("", false),
        ] {
            assert_eq!(test_flatpak_remotes_inner(mocked_stdout)?, should_pass);
        }

        Ok(())
    }

    fn test_flatpak_remotes_inner(mocked_stdout: &'static str) -> DeckResult<bool> {
        let mut mock = MockTestActualRunner::new();

        let cmd = "flatpak";
        let args = vec!["remotes", "--columns=name,options"];
        let returned_args = args.clone();
        let arg = SysCommand::new(&ExecutionContext::general_for_test(), cmd, args);
        mock.expect_run()
            .with(predicate::eq(arg))
            .returning(move |_| {
                Ok(SysCommandResult::fake_for_test(
                    cmd,
                    returned_args.clone(),
                    0,
                    mocked_stdout,
                    "",
                ))
            });

        mock.expect_run()
            .returning(|_| Ok(SysCommandResult::fake_success()));

        let runner = Arc::new(mock);
        let ctx = ExecutionContext::general_for_test_with(runner.clone());
        let retval = get_has_user_flathub(&ctx)?;

        Ok(retval)
    }
}
