use crate::system_command_runners::merge_stdouterr::live_log_child_and_wait_with_output;
use super::system_command::*;
use crate::prelude::*;
use std::sync::Arc;

// This file contains the actual logic for running commands.

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
}

use mockall::mock;
mock! {
    #[derive(Debug, Clone, Copy)]
    pub TestActualRunner {}

    impl ActualRunner for TestActualRunner {
        fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult>;
    }
}

// Don't instantiate this directly, use ctx.sys_command() instead
#[derive(Debug, Clone, Default)]
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
    fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult> {
        if cfg!(test) {
            eprintln!("Culprit command: {sys_command:?}");
            panic!("Tried to run real command in tests!");
        }

        let ctx = sys_command.get_ctx();
        let cmd = &sys_command.cmd;
        let args = &sys_command.args;

        // NOTE: Here is where we finally create the actual Command to be run.

        let mut command;
        if sys_command.pty_needed {
            // Because many commands will buffer stdout if they thing they're not running in a
            // terminal, we force a pty with the script command. This is a hacky-hacky-hack,
            // but it fixes the problem for now.
            //
            // TODO: fix this hack, preferably using the portable_pty crate
            command = std::process::Command::new("/usr/bin/script");
            command.args(&["-q", "-e", "--command", format!("{} {}", cmd, args.join(" ")).as_ref(), "/dev/null"]);
        } else {
            command = std::process::Command::new(cmd);
            command.args(args);
        }

        command.stderr(std::process::Stdio::piped());
        command.stdout(std::process::Stdio::piped());

        for (var, val) in &sys_command.desired_env_vars {
            command.env(var, val);
        }

        let mut child_handle = command
            .spawn()
            .map_err(|e| sys_command_error_to_known_error(sys_command, e))?;

        // TODO: determine if this log level check is needed - should be redundant?
        if sys_command.live_logging_desired {
            let cmdline_for_logging = format!("{} {}", cmd, args.join(" "));
            live_log_child_and_wait_with_output(ctx, cmdline_for_logging, &mut child_handle)
                .map_err(|e| sys_command_error_to_known_error(sys_command, e))?;
        }

        let output = child_handle.wait_with_output()
            .map_err(|e| sys_command_error_to_known_error(sys_command, e))?;

        if output.status.success() {
            info!(
                ctx,
                "Command {sys_command:#?} ran successfully with {}output:\n\n{}",
                if sys_command.live_logging_desired { "additional " } else { "" },
                String::from_utf8_lossy(&output.stdout)
            );
        } else {
            let exit_code = output
                .status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "UNKNOWN".into());
            info!(
                ctx,
                "Command {sys_command:#?} exited with non-zero exit code {exit_code} with {}output:\n\n\nSTDOUT:\n\n{}\n\nSTDERR:\n\n{}",
                if sys_command.live_logging_desired { "additional " } else { "" },
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
}
