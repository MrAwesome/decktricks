use crate::prelude::*;
use crate::providers::emudeck_installer::EmuDeckSystemContext;
use crate::providers::systemd_run::SystemdRunUnitsContext;
use crate::utils::running_in_ci_container;
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
    pub systemd_run_ctx: SystemdRunUnitsContext,
}

// TODO: gather optimistically, don't fail the whole gather if some particular error is encountered
// [] just log gather errors with error! and continue
impl FullSystemContext {
    /// # Errors
    ///
    /// Can return system errors from trying to gather system information
    pub fn gather_with(ctx: &impl ExecCtx, tricks_loader: &TricksLoader) -> DeckResult<Self> {
        let (decky_ctx, flatpak_ctx, procs_ctx, emudeck_ctx, systemd_run_ctx) = join_all!(
            || DeckySystemContext::gather_with(&ctx.clone())
                .map_err(|e| {
                    error!(ctx, "Error gathering Decky context: {}", e);
                    e
                })
                .unwrap_or_default(),
            || FlatpakSystemContext::gather_with(&ctx.clone())
                .map_err(|e| {
                    error!(ctx, "Error gathering Flatpak context: {}", e);
                    e
                })
                .unwrap_or_default(),
            || RunningProgramSystemContext::gather_with(&ctx.clone())
                .map_err(|e| {
                    error!(ctx, "Error gathering running program context: {}", e);
                    e
                })
                .unwrap_or_default(),
            || EmuDeckSystemContext::gather_with(&ctx.clone())
                .map_err(|e| {
                    error!(ctx, "Error gathering EmuDeck context: {}", e);
                    e
                })
                .unwrap_or_default(),
            || SystemdRunUnitsContext::gather_with(&ctx.clone(), tricks_loader)
                .map_err(|e| {
                    error!(ctx, "Error gathering systemd-run context: {}", e);
                    e
                })
                .unwrap_or_default()
        );

        Ok(Self {
            flatpak_ctx,
            decky_ctx,
            emudeck_ctx,
            procs_ctx,
            systemd_run_ctx,
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
    pub fn gather_with(ctx: &impl ExecCtx) -> DeckResult<Self> {
        // This can be stored in an "initial system context" if needed

        let desired_string = format!("{PID_ENV_STRING}=");
        let res = get_procs_with_env(ctx);

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
                                ctx,
                                "Expected pid, but did not find one. Command line: '''{line}'''"
                            );
                        }
                    } else {
                        error!(
                            ctx,
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

fn get_procs_with_env(ctx: &impl ExecCtx) -> Option<String> {
    // XXX: NOTE: we do not run this inside of containers in CI, as ps eww errors there.
    if running_in_ci_container() {
        return None;
    }

    let run_res = SysCommand::new(ctx, "/bin/ps", ["eww"]).run();

    match run_res {
        Ok(res) => match res.as_success() {
            Ok(succ) => succ.get_message(),
            Err(err) => {
                error!(
                    ctx,
                    "Program error running 'ps eww' to find running programs! Will fallback to blank output. Error: {err}");
                None
            }
        },
        Err(err) => {
            error!(
                ctx,
                "Run error running 'ps eww' to find running programs! Will fallback to blank output. Error: {err}");
            None
        }
    }
}

#[test]
fn gather_procs() -> DeckResult<()> {
    // NOTE!! This test does not run in CI, so local errors should be taken especially seriously.
    if running_in_ci_container() {
        return Ok(());
    }

    let mut mock = MockTestActualRunner::new();
    let desired_pid = "432151";

    let pseww_output = format!("{desired_pid} pts/1    Sl+    0:01 systemsettings BOOBOO=1234 HARBLGARBL=jklsja jdskaf {PID_ENV_STRING}=systemsettings LOL=jfkda");

    mock.expect_run()
        .returning(move |_| Ok(SysCommandResult::success_output(&pseww_output)));
    let ctx = GeneralExecutionContext::test_with_runner(std::sync::Arc::new(mock));
    let proc_ctx = RunningProgramSystemContext::gather_with(&ctx)?;
    assert_eq!(
        desired_pid,
        proc_ctx
            .tricks_to_running_pids
            .get("systemsettings")
            .unwrap()
            .first()
            .unwrap()
    );

    Ok(())
}
