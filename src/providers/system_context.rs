use crate::prelude::*;
use crate::providers::emudeck_installer::EmuDeckSystemContext;
use decky_installer::DeckySystemContext;
use flatpak::FlatpakSystemContext;
use std::collections::HashMap;

// TODO: test
#[derive(Debug, Clone, Default)]
pub struct FullSystemContext {
    pub flatpak_ctx: FlatpakSystemContext,
    pub decky_ctx: DeckySystemContext,
    pub emudeck_ctx: EmuDeckSystemContext,
    pub procs_ctx: RunningProgramSystemContext,
}

impl FullSystemContext {
    /// # Errors
    ///
    /// Can return system errors from trying to gather system information
    pub fn gather_with(runner: &RunnerRc) -> DeckResult<Self> {
        let (decky_ctx, flatpak_ctx, procs_ctx, emudeck_ctx) = join_all!(
            || DeckySystemContext::gather_with(runner),
            || FlatpakSystemContext::gather_with(runner),
            || RunningProgramSystemContext::gather_with(runner),
            || EmuDeckSystemContext::gather_with(runner)
        );

        Ok(Self {
            decky_ctx: decky_ctx?,
            flatpak_ctx: flatpak_ctx?,
            procs_ctx: procs_ctx?,
            emudeck_ctx: emudeck_ctx?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct RunningProgramSystemContext {
    pub tricks_to_running_pids: HashMap<TrickID, Vec<ProcessID>>,
}

impl RunningProgramSystemContext {
    /// # Errors
    ///
    /// Returns errors relating to finding, reading, and parsing files in /proc
    pub fn gather_with(runner: &RunnerRc) -> DeckResult<Self> {
        // This can be stored in an "initial system context" if needed

        let desired_string = format!("{PID_ENV_STRING}=");
        let res = SysCommand::new("/bin/ps", vec!["eww"])
            .run_with(runner)?
            .as_success()?
            .get_message();

        let mut tricks_to_running_pids: HashMap<TrickID, Vec<ProcessID>> = HashMap::new();
        if let Some(output) = res {
            for line in output.lines() {
                if line.contains(&desired_string) {
                    let maybe_trick_id = line
                        .split_whitespace()
                        .find_map(|s| s.strip_prefix(&desired_string).map(ToString::to_string));

                    let maybe_pid = line.split_whitespace().next();
                    if let Some(trick_id) = maybe_trick_id {
                        if let Some(pid) = maybe_pid {
                            tricks_to_running_pids
                                .entry(trick_id)
                                .or_default()
                                .push(pid.into());
                        } else {
                            error!(
                                "Expected pid, but did not find one. Command line: '''{line}'''"
                            );
                        }
                    } else {
                        error!(
                            "Expected trick ID, but did not find one. Command line: '''{line}'''"
                        );
                    }
                }
            }
        }

        Ok(Self {
            tricks_to_running_pids,
        })
    }
}



#[test]
fn gather_procs() -> DeckResult<()> {
    use std::sync::Arc;
    let mut mock = MockTestActualRunner::new();
    let desired_pid = "432151";

    let pseww_output = format!("{desired_pid} pts/1    Sl+    0:01 systemsettings BOOBOO=1234 HARBLGARBL=jklsja jdskaf {PID_ENV_STRING}=systemsettings LOL=jfkda");

    mock.expect_run()
        .returning(move |_| Ok(SysCommandResult::success_output(&pseww_output)));
    let runner = Arc::new(mock);
    let ctx = RunningProgramSystemContext::gather_with(&runner)?;
    assert_eq!(desired_pid, ctx.tricks_to_running_pids.get("systemsettings").unwrap().first().unwrap());

    Ok(())
}
