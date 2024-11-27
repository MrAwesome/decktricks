use crate::prelude::*;

const KNOWN_CI_ENV_VARS: &[&str] = &["GITHUB_ACTIONS", "TRAVIS", "CIRCLECI", "GITLAB_CI"];

#[must_use]
pub fn running_in_ci_container() -> bool {
    for var in KNOWN_CI_ENV_VARS {
        if std::env::var(var).is_ok_and(|val| !val.is_empty()) {
            return true;
        }
    }
    false
}

#[allow(clippy::unnecessary_wraps)]
#[cfg(test)]
pub(crate) fn run_remote_script(
    _ctx: &ExecutionContext,
    url: &str,
    local_filename: &str,
) -> DeckResult<ActionSuccess> {
    warn!("Not running run_remote_script({url}, {local_filename}) from test...");
    success!()
}

#[cfg(not(test))]
pub(crate) fn run_remote_script(
    ctx: &ExecutionContext,
    url: &str,
    local_filename: &str,
) -> DeckResult<ActionSuccess> {
    use std::fs::File;
    use std::io::copy;
    use std::os::unix::fs::PermissionsExt;

    // TODO: make this and the operations below test-safe
    let response = reqwest::blocking::get(url).map_err(KnownError::from)?;

    // These are in blocks to ensure that files are closed out
    // before attempting to do further changes
    {
        let mut dest = File::create(local_filename)?;
        copy(&mut response.bytes()?.as_ref(), &mut dest)?;
    }

    {
        std::fs::set_permissions(local_filename, std::fs::Permissions::from_mode(0o755))?;
    }

    SysCommand::new_no_args(local_filename)
        .run_with(ctx)?
        .as_success()
}

pub(crate) fn get_homedir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/home/deck".to_string())
}

pub(crate) fn exists_and_executable(ctx: &ExecutionContext, path: &str) -> bool {
    // Using this instead of rust-native code to piggyback on the test-friendliness of SysCommand
    let res = SysCommand::new("/bin/test", ["-x", path]).run_with(ctx);

    match res {
        Ok(cmdres) => cmdres.ran_successfully(),
        Err(_) => false,
    }
}

pub(crate) fn get_running_pids_exact(
    ctx: &ExecutionContext,
    binary_name: &str,
) -> DeckResult<Vec<String>> {
    Ok(SysCommand::new("ps", ["-C", binary_name, "-o", "pid="])
        .run_with(ctx)?
        .as_success()?
        .get_message_or_blank()
        .split_whitespace()
        .map(ToString::to_string)
        .collect())
}

pub(crate) fn kill_pids(ctx: &ExecutionContext, pids: &[ProcessID]) -> DeckResult<ActionSuccess> {
    let mut outputs = vec![];
    let string_pids: Vec<String> = pids.iter().map(ToString::to_string).collect();
    for pid in string_pids {
        let res = SysCommand::new("kill", [&pid]).run_with(ctx)?.as_success();

        if res.is_ok() {
            outputs.push(format!("Successfully killed pid '{pid}'"));
        } else {
            warn!("Failed to kill pid '{pid}'!");
        }
    }
    let output = outputs.join("\n");
    success!(output)
}
