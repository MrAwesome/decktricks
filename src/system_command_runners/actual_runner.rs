use crate::prelude::*;
use super::system_command::*;
use std::sync::Arc;

// TODO: prevent Command module-wide
// TODO: unit/integration test spawn vs output
// TODO: better indication that a process was spawned vs output?

// TODO: codemod into a dyn testrunner


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

        let TODO = "start here";
        // create mpsc, run the command in thread with a sender, in a loop that sends every second
        // (and has a "we're done here" signal as well)
        todo!()
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

        let output = command.output().map_err(|e| {
            let args = sys_command.args.join(" ");
            error!(
                sys_command.get_ctx(),
                "Error running command \"{} {}\": {e:?}", sys_command.cmd, args
            );
            KnownError::SystemCommandRunFailure(Box::new(SysCommandRunError {
                cmd: sys_command.clone(),
                error: e,
            }))
        })?;

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
            // NOTE: we don't want to warn here, as we run a lot of background commands that we
            // expect to fail, and we'd flood the logs immediately
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
        // NOTE: this is not how you should run commands normally, this is just to test safety
        let r = RealWorldActualRunner {};
        let sys_command = GeneralExecutionContext::test().sys_command_no_args("echo");
        let _ = r.run(&sys_command);
    }

    #[test]
    #[should_panic(expected = "Tried to run live real command in tests!")]
    fn test_real_world_live_command_panics() {
        // NOTE: this is not how you should run commands normally, this is just to test safety
        let r = RealWorldActualRunner {};
        let sys_command = GeneralExecutionContext::test().sys_command_no_args("echo");
        let _ = r.run_live(&sys_command);
    }
}
