use std::sync::Arc;
use crate::prelude::*;

#[cfg(test)]
pub type Runner = MockTestActualRunner;
#[cfg(not(test))]
pub type Runner = LiveActualRunner;

pub type RunnerRc = Arc<Runner>;

#[derive(Debug, Clone)]
pub(crate) struct SysCommand {
    pub cmd: String,
    pub args: Vec<String>,
}

impl SysCommand {
    pub fn new<S: Into<String>>(cmd: S, args: Vec<S>) -> Self {
        Self {
            cmd: cmd.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }
}

pub(crate) trait SysCommandRunner {
    fn get_cmd(&self) -> &SysCommand;

    fn run_with(&self, runner: &RunnerRc) -> DeckResult<SysCommandResult> {
        let sys_command = self.get_cmd();
        let raw_output = runner.run(sys_command)?;

        Ok(SysCommandResult {
            raw_output,
            sys_command: sys_command.clone(),
        })
    }
}

impl SysCommandRunner for SysCommand {
    fn get_cmd(&self) -> &SysCommand {
        self
    }
}

pub(crate) struct SysCommandResult {
    sys_command: SysCommand,
    raw_output: std::process::Output,
}

pub(crate) trait SysCommandResultChecker {
    fn raw_output(&self) -> &std::process::Output;
    fn sys_command(&self) -> &SysCommand;

    fn ran_successfully(&self) -> bool {
        if is_debug() {
            let cmd = &self.sys_command().cmd;
            let args = &self.sys_command().args;
            let output = self.raw_output();
            let status = output.status;
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            debug!("== EXTERNAL COMMAND STATUS: {cmd} {args:?} {status:?}\nSTDERR:\"{stderr}\"\nSTDOUT:\"{stdout}\"");
        }

        self.raw_output().status.success()
    }

    fn as_success(&self) -> DeckResult<ActionSuccess> {
        if self.ran_successfully() {
            success!(String::from_utf8_lossy(&self.raw_output().stdout))
        } else {
            Err(KnownError::SystemCommandFailed(self.raw_output().clone()))
        }
    }
}

impl SysCommandResultChecker for SysCommandResult {
    fn sys_command(&self) -> &SysCommand {
        &self.sys_command
    }

    fn raw_output(&self) -> &std::process::Output {
        &self.raw_output
    }
}

pub(crate) trait ActualRunner {
    fn run(&self, sys_command: &SysCommand) -> DeckResult<std::process::Output>;
}

#[cfg(test)]
use mockall::mock;
#[cfg(test)]
mock! {
    #[derive(Debug, Clone, Copy)]
    pub(crate) TestActualRunner {}

    impl ActualRunner for TestActualRunner {
        fn run(&self, sys_command: &SysCommand) -> DeckResult<std::process::Output>;
    }
}

#[cfg(not(test))]
#[derive(Debug, Clone, Copy)]
pub struct LiveActualRunner {}

#[cfg(not(test))]
impl LiveActualRunner {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
#[cfg(not(test))]
impl ActualRunner for LiveActualRunner {
    fn run(&self, sys_command: &SysCommand) -> DeckResult<std::process::Output> {
        let cmd = &sys_command.cmd;
        let args = &sys_command.args;
        assert!(
            !am_in_test(),
            "Attempted to run system command {:?} in test!",
            (cmd, args)
        );

        std::process::Command::new(cmd)
            .args(args)
            .output()
            .map_err(KnownError::SystemCommandRun)
    }
}

// For now, return nothing.
// TODO: run in a thread and report back (for gui), run and wait (for CLI)
// TODO: return child process info, store it with provider?
// TODO: pass Result back to calling functions
//
// https://doc.rust-lang.org/std/process/struct.Child.html#method.try_wait
//pub(crate) fn spawn_system_command(cmdname: &str, args: Vec<&str>) {
//    Command::new(cmdname)
//        .args(args)
//        .spawn();
//}
