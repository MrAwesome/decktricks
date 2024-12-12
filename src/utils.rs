use crate::prelude::*;
use std::fmt::Display;

const KNOWN_CI_ENV_VARS: &[&str] = &["CI", "GITHUB_ACTIONS", "TRAVIS", "CIRCLECI", "GITLAB_CI"];

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
    _ctx: &impl ExecCtx,
    url: &str,
    local_filename: &str,
) -> DeckResult<ActionSuccess> {
    warn!(ExecutionContext::general_for_test(), "Not running run_remote_script({url}, {local_filename}) from test...");
    success!()
}

#[cfg(not(test))]
pub(crate) fn run_remote_script(
    ctx: &impl ExecCtx,
    url: &str,
    local_filename: &str,
) -> DeckResult<ActionSuccess> {
    use std::fs::File;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use ureq;

    // TODO: make this and the operations below test-safe
    let data = ureq::get(url)
        .call()
        .map_err(|e| {
            KnownError::RemoteScriptError(format!("Failed downloading local script file: {e:#?}"))
        })?
        .into_string()
        .map_err(|e| {
            KnownError::RemoteScriptError(format!("Failed stringifying local script file: {e:#?}"))
        })?;
    // let response = reqwest::blocking::get(url).map_err(KnownError::from)?;
    // let data = response.bytes()?.as_ref();

    // These are in blocks to ensure that files are closed out
    // before attempting to do further changes
    {
        let mut dest = File::create(local_filename).map_err(|e| {
            KnownError::RemoteScriptError(format!("Failed to create local script file: {e:#?}"))
        })?;
        write!(&mut dest, "{data}").map_err(|e| {
            KnownError::RemoteScriptError(format!("Failed to write local script file: {e:#?}"))
        })?;
        //copy(&mut response.into_reader().take(10_000_000).?, &mut dest)?;
    };

    {
        std::fs::set_permissions(local_filename, std::fs::Permissions::from_mode(0o755)).map_err(
            |e| {
                KnownError::RemoteScriptError(format!(
                    "Failed to set permissions for local script file: {e:#?}"
                ))
            },
        )?;
    }

    SysCommand::new_no_args(ctx, local_filename)
        .run()?
        .as_success()
}

pub(crate) fn get_homedir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/home/deck".to_string())
}

pub(crate) fn exists_and_executable(ctx: &impl ExecCtx, path: &str) -> bool {
    // Using this instead of rust-native code to piggyback on the test-friendliness of SysCommand
    let res = SysCommand::new(ctx, "/bin/test", ["-x", path]).run();

    match res {
        Ok(cmdres) => cmdres.ran_successfully(),
        Err(_) => false,
    }
}

pub(crate) fn get_running_pids_exact(
    ctx: &impl ExecCtx,
    binary_name: &str,
) -> DeckResult<Vec<String>> {
    Ok(SysCommand::new(ctx, "ps", ["-C", binary_name, "-o", "pid="])
        .run()?
        .as_success()?
        .get_message_or_blank()
        .split_whitespace()
        .map(ToString::to_string)
        .collect())
}

pub(crate) fn kill_pids(
    ctx: &impl ExecCtx,
    pids: &[ProcessID],
) -> DeckResult<ActionSuccess> {
    let mut outputs = vec![];
    let string_pids: Vec<String> = pids.iter().map(ToString::to_string).collect();
    for pid in string_pids {
        let res = SysCommand::new(ctx, "kill", [&pid]).run()?.as_success();

        if res.is_ok() {
            outputs.push(format!("Successfully killed pid '{pid}'"));
        } else {
            warn!(ctx, "Failed to kill pid '{pid}'!");
        }
    }
    let output = outputs.join("\n");
    success!(output)
}

#[cfg(test)]
pub fn which<T: AsRef<std::ffi::OsStr> + Display>(binary_name: T) -> DeckResult<String> {
    Ok(format!("/FAKE/PATH/IN/TEST/{binary_name}"))
}

#[cfg(not(test))]
pub fn which<T: AsRef<std::ffi::OsStr> + Display>(binary_name: T) -> DeckResult<String> {
    Ok(which::which(&binary_name)
        .map_err(|e| {
            KnownError::AddToSteamError(format!(
                "Failed to find a binary matching {binary_name} in $PATH! Error: {e}"
            ))
        })?
        .to_string_lossy()
        .to_string())
}
