use std::process::{Command, Stdio};
use std::thread;
use crate::prelude::*;
use super::system_command::*;
use std::sync::mpsc;
use std::sync::Arc;

pub fn get_runner() -> Arc<dyn ActualRunner> {
    if cfg!(test) {
        Arc::new(MockTestActualRunner::new())
    } else {
        Arc::new(RealWorldActualRunner::new())
    }
}

pub type RunnerRc = Arc<dyn ActualRunner>;

pub trait ActualRunner: Send + Sync + std::fmt::Debug {
    fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult>;
    fn run_live(&self, sys_command: &SysCommand) -> DeckResult<LiveSysCommandWatcher>;
}

use mockall::mock;
mock! {
    #[derive(Debug, Clone, Copy)]
    pub TestActualRunner {}

    impl ActualRunner for TestActualRunner {
        fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult>;
        fn run_live(&self, sys_command: &SysCommand) -> DeckResult<LiveSysCommandWatcher>;
    }
}

// Don't instantiate this directly, use ctx.sys_command() instead
#[derive(Debug, Clone, Copy, Default)]
pub struct RealWorldActualRunner {}

impl RealWorldActualRunner {
    #[must_use]
    pub fn new() -> Self {
        if cfg!(test) {
            panic!("Tried to create real world command runner in tests!");
        }
        Self::default()
    }
}

impl ActualRunner for RealWorldActualRunner {
    fn run_live(&self, sys_command: &SysCommand) -> DeckResult<LiveSysCommandWatcher> {
        if cfg!(test) {
            eprintln!("Culprit command: {sys_command:?}");
            panic!("Tried to run live real command in tests!");
        }

        let (rx, tx) = mpsc::channel::<LiveRunMessage>();

        thread::spawn(move || {
            let ctx = sys_command.ctx;

            let command = Command::new(sys_command.cmd);
            command.args(sys_command.args);
            command.envs(sys_command.desired_env_vars);
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());

            let child_res = command.spawn();
            let child = match child_res {
                Ok(child) => child,
                Err(err) => {
                    let known_error = sys_command_error_to_known_error(sys_command, err);
                    rx.send(LiveRunMessage::Error(known_error));
                    return;
                }
            };

            let stdout = child.stdout.take();
            let stderr = child.stderr.take();

            if stdout.is_none() {
                warn!(
                    ctx,
                    "Live command stdout could not be taken! Command: {sys_command:?}"
                )
            }

            if stderr.is_none() {
                warn!(
                    ctx,
                    "Live command stderr could not be taken! Command: {sys_command:?}"
                )
            }

            // TODO: use child.kill() instead of kludgy ps tracking elsewhere
            child.kill
        });

        // STARTHERE
        // TODO:
        // [] open channel
        // [] spawn command in another thread
        // [] watch command stdout/stderr
        // [] report back liveness periodically if needed?
        // [] return a SysCommandResult back across the channel when done (if large amounts of text
        // over channel won't cause issues)
        // [] on final loop, use child.wait_with_output() to get output for use with SysCommandResult
        //
        // ENDHERE
    }

    fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult> {
        if cfg!(test) {
            eprintln!("Culprit command: {sys_command:?}");
            panic!("Tried to run real command in tests!");
        }

        let cmd = &sys_command.cmd;
        let args = &sys_command.args;

        // NOTE: Here is where we finally create the actual Command to be run.
        let mut command = std::process::Command::new(cmd);
        command.args(args);

        for (var, val) in &sys_command.desired_env_vars {
            command.env(var, val);
        }

        let output = command.output().map_err(|e| sys_command_error_to_known_error(sys_command, e))?;

        if output.status.success() {
            info!(
                sys_command.get_ctx(),
                "Command {sys_command:#?} ran successfully with output:\n\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        } else {
            let exit_code = output
                .status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "UNKNOWN".into());
            info!(
                sys_command.get_ctx(),
                "Command {sys_command:#?} exited with non-zero exit code {exit_code} with:\n\n\nSTDOUT:\n\n{}\n\nSTDERR:\n\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(SysCommandResult::new(sys_command.clone(), output))
    }
}

fn sys_command_error_to_known_error(sys_command: &SysCommand, e: std::io::Error) -> KnownError {
    let args = sys_command.args.join(" ");
    error!(
        sys_command.get_ctx(),
        "Error running command \"{} {}\": {e:?}", sys_command.cmd, args
    );
    KnownError::SystemCommandRunFailure(Box::new(SysCommandRunError {
        cmd: sys_command.clone(),
        error: e,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Tried to create real world command runner in tests!")]
    fn test_real_world_creation_panics() {
        let _ = RealWorldActualRunner::new();
    }

    #[test]
    #[should_panic(expected = "Tried to run real command in tests!")]
    fn test_real_world_command_panics() {
        let r = RealWorldActualRunner {};
        let sys_command = GeneralExecutionContext::test().sys_command_no_args("echo");
        let _ = r.run(&sys_command);
    }

    #[test]
    #[should_panic(expected = "Tried to run live real command in tests!")]
    fn test_real_world_live_command_panics() {
        let r = RealWorldActualRunner {};
        let sys_command = GeneralExecutionContext::test().sys_command_no_args("echo");
        let _ = r.run_live(&sys_command);
    }
}
