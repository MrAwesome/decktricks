use crate::prelude::*;
use decky_installer::DeckySystemContext;
use flatpak::FlatpakSystemContext;
use std::collections::HashMap;

// TODO: test
pub struct FullSystemContext {
    pub flatpak_ctx: FlatpakSystemContext,
    pub decky_ctx: DeckySystemContext,
    pub procs_ctx: RunningProgramSystemContext,
}

impl FullSystemContext {
    /// # Errors
    ///
    /// Can return system errors from trying to gather system information
    pub fn gather_with(runner: &RunnerRc) -> DeckResult<Self> {
        let (decky_ctx, flatpak_ctx, procs_ctx) = join_all!(
            || DeckySystemContext::gather_with(runner),
            || FlatpakSystemContext::gather_with(runner),
            || RunningProgramSystemContext::gather_with(runner)
        );

        Ok(Self {
            decky_ctx: decky_ctx?,
            flatpak_ctx: flatpak_ctx?,
            procs_ctx: procs_ctx?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RunningProgramSystemContext {
    pub tricks_to_running_pids: HashMap<TrickID, Vec<ProcessID>>,
}

impl Default for RunningProgramSystemContext {
    fn default() -> Self {
        Self {
            tricks_to_running_pids: Default::default()
        }
    }
}

impl RunningProgramSystemContext {
    /// # Errors
    ///
    /// Returns errors relating to finding, reading, and parsing files in /proc
    pub fn gather_with(runner: &RunnerRc) -> DeckResult<Self> {
        let mut tricks_to_running_pids: HashMap<TrickID, Vec<ProcessID>> = HashMap::new();
        // This can be stored in an "initial system context" if needed

        let proc_dirs_for_user = get_proc_dirs_for_user(runner)?;

        for dir in proc_dirs_for_user {
            let filename = format!("{dir}/environ");
            let maybe_filetext = get_file_contents(runner, &filename);

            // We expect some failures to read certain files (environ owned by root, disappeared),
            // but we can safely skip those.
            if let Ok(filetext) = maybe_filetext {
                if let Some((trick_id, pid)) = get_pid_and_trick_id_for_environ(&dir, &filetext) {
                    tricks_to_running_pids.entry(trick_id).or_default().push(pid);
                }
            }
        }

        Ok(Self {
            tricks_to_running_pids,
        })
    }
}

fn get_proc_dirs_for_user(runner: &RunnerRc) -> DeckResult<Vec<String>> {
        let username = SysCommand::new("whoami", vec![])
            .run_with(runner)?
            .as_success()?
            .get_message()
            .map_or(DEFAULT_USER.into(), |x| x.trim().to_string());

        // TODO: instead of find via SysCommand, make a test-safe file finder?
        let res = SysCommand::new(
            "find",
            vec!["/proc", "-maxdepth", "1", "-type", "d", "-user", &username],
        )
        .run_with(runner)?;

        // Just attempt to use whatever find gives us
        Ok(String::from_utf8_lossy(&res.raw_output().stdout)
            .lines()
            .map(String::from)
            .collect())
}

fn get_file_contents(runner: &RunnerRc, filename: &str) -> DeckResult<String> {
    // TODO: instead of cat via SysCommand, make a test-safe file reader
    Ok(SysCommand::new("cat", vec![filename])
        .run_with(runner)?
        .as_success()?
        .get_message().unwrap_or(String::new()))
}

fn get_pid_and_trick_id_for_environ(dir: &str, filetext: &str) -> Option<(TrickID, ProcessID)> {
    let envs = filetext.split('\0');
    for env in envs {
        if env.starts_with(PID_ENV_STRING) {
            if let Some(trick_id) = env.split('=').last() {
                let pidstr = dir.trim_start_matches("/proc/").trim_end_matches('/');

                match pidstr.parse() {
                    Ok(pid) => return Some((trick_id.into(), pid)),
                    Err(err) => warn!("Failed to parse pid: '{pidstr}' for '{env}' in '{dir}': {err:?}"),
                }
            } else {
                warn!("Error parsing env '{env}' in '{dir}'");
            }
            break;
        }
    }
    None
}

#[test]
fn test_environ_file_parsing() {
    let muh_trick_id = "muh-trick-id";
    let desired_pid: isize = 12345;
    let dirname = format!("/proc/{desired_pid}");
    let filetext = format!("SOME_ENV=JKFLDJS\0OTHER_ENV=1231231\0{PID_ENV_STRING}={muh_trick_id}\0ENNNVVVV=848297");
    let res = get_pid_and_trick_id_for_environ(&dirname, &filetext);
    assert_eq!(res, Some((muh_trick_id.into(), desired_pid)));
}
