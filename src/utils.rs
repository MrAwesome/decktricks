use crate::prelude::*;
use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

static HOMEDIR: LazyLock<String> = LazyLock::new(||
    std::env::var("HOME").unwrap_or_else(|_| "/home/deck".to_string())
);

const KNOWN_CI_ENV_VARS: &[&str] = &["CI", "GITHUB_ACTIONS", "TRAVIS", "CIRCLECI", "GITLAB_CI"];

#[must_use]
pub fn check_log_level_env_var() -> Option<LogType> {
    std::env::var("DECKTRICKS_LOG_LEVEL")
        .ok()?
        .parse::<u8>()
        .ok()
        .map(From::from)
}

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
#[must_use]
pub(crate) fn fetch_and_prep_remote_executable(
    ctx: &impl ExecCtx,
    url: &str,
    local_filename: &str,
) -> DeckResult<SysCommand> {
    warn!(
        ExecutionContext::general_for_test(),
        "Not running run_remote_executable({url}, {local_filename}) from test..."
    );
    Ok(ctx.sys_command_no_args("echo"))
}

#[cfg(not(test))]
#[must_use]
pub(crate) fn fetch_and_prep_remote_executable(
    ctx: &impl ExecCtx,
    url: &str,
    local_filename: &str,
) -> DeckResult<SysCommand> {
    use std::fs::File;
    use std::os::unix::fs::PermissionsExt;
    use ureq;

    // TODO: make this and the operations below test-safe
    let data = ureq::get(url)
        .call()
        .map_err(|e| {
            KnownError::RemoteScriptError(format!("Failed downloading local script file: {e:#?}"))
        })?;
    // let response = reqwest::blocking::get(url).map_err(KnownError::from)?;
    // let data = response.bytes()?.as_ref();

    // These are in blocks to ensure that files are closed out
    // before attempting to do further changes
    {

        let mut dest = File::create(local_filename).map_err(|e| {
            KnownError::RemoteScriptError(format!("Failed to create local script file: {e:#?}"))
        })?;

        std::io::copy(&mut data.into_reader(), &mut dest).map_err(|e| {
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

    let mut sys_command = ctx.sys_command_no_args(local_filename);
    sys_command
        // TODO: is force_pty needed for geforce now to have live logs?
        .force_pty()
        .enable_live_logging();

    Ok(sys_command)
}

pub fn get_homedir() -> &'static str {
    HOMEDIR.as_str()
}

pub fn get_decktricks_dir() -> PathBuf {
    Path::join(Path::new(&get_homedir()), ".local/share/decktricks/")
}

pub fn get_decktricks_update_log_file_location() -> PathBuf {
    Path::join(&get_decktricks_dir(), "logs/decktricks-update.log")
}

pub fn exists_and_executable(ctx: &impl ExecCtx, path: &str) -> bool {
    // Using this instead of rust-native code to piggyback on the test-friendliness of SysCommand
    let res = ctx.sys_command("/bin/test", ["-x", path]).run();

    match res {
        Ok(cmdres) => cmdres.ran_successfully(),
        Err(_) => false,
    }
}

// NOTE: this only works with binary files! for scripts,
pub(crate) fn get_running_pids_exact(
    ctx: &impl ExecCtx,
    binary_name: &str,
) -> DeckResult<Vec<String>> {
    Ok(ctx
        .sys_command("ps", ["-C", binary_name, "-o", "pid="])
        .run()?
        .as_success()?
        .get_message_or_blank()
        .split_whitespace()
        .map(ToString::to_string)
        .collect())
}

pub(crate) fn pgrep(
    ctx: &impl ExecCtx,
    pattern: &str,
) -> DeckResult<Vec<String>> {
    Ok(ctx
        .sys_command("pgrep", ["-f", pattern])
        .run()?
        .as_success()?
        .get_message_or_blank()
        .split_whitespace()
        .map(ToString::to_string)
        .collect())
}

pub(crate) fn kill_pids(ctx: &impl ExecCtx, pids: &[ProcessID]) -> DeckResult<ActionSuccess> {
    let mut outputs = vec![];
    let string_pids: Vec<String> = pids.iter().map(ToString::to_string).collect();
    for pid in string_pids {
        let res = ctx.sys_command("kill", [&pid]).run()?.as_success();

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
