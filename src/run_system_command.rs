use crate::prelude::*;
use std::sync::Arc;

// TODO: prevent Command module-wide

#[cfg(test)]
use std::os::unix::process::ExitStatusExt;

#[cfg(test)]
pub type Runner = MockTestActualRunner;
#[cfg(not(test))]
pub type Runner = LiveActualRunner;

pub type RunnerRc = Arc<Runner>;

#[derive(Debug)]
pub struct SysCommandRunError {
    pub cmd: SysCommand,
    pub error: std::io::Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysCommand {
    pub cmd: String,
    pub args: Vec<String>,
    pub desired_env_vars: Vec<(String, String)>
}

impl SysCommand {
    pub fn new<S: Into<String>>(cmd: S, args: Vec<S>) -> Self {
        Self {
            cmd: cmd.into(),
            args: args.into_iter().map(Into::into).collect(),
            desired_env_vars: Vec::default(),
        }
    }
}

pub(crate) trait SysCommandRunner {
    fn get_cmd(&self) -> &SysCommand;

    fn env(&mut self, varname: &str, value: &str) -> &Self;

    fn run_with(&self, ctx: &ExecutionContext) -> DeckResult<SysCommandResult> {
        let sys_command = self.get_cmd();
        ctx.runner.run(sys_command)
    }
}

impl SysCommandRunner for SysCommand {
    fn get_cmd(&self) -> &SysCommand {
        self
    }

    fn env(&mut self, varname: &str, value: &str) -> &Self {
        self.desired_env_vars.push((varname.into(), value.into()));
        self
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SysCommandResult {
    sys_command: SysCommand,
    raw_output: std::process::Output,
}

#[cfg(not(test))]
impl SysCommandResult {
    fn new(cmd: String, args: Vec<String>, env: Vec<(String, String)>, output: std::process::Output) -> Self {
        Self {
            sys_command: SysCommand {
                cmd,
                args,
                // NOTE: this is incorrect
                desired_env_vars: env,
            },
            raw_output: output,
        }
    }
}

impl std::fmt::Debug for SysCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
impl SysCommandResult {
    pub(crate) fn fake_success() -> Self {
        Self {
            sys_command: SysCommand::new(
                "nothingburger".to_string(),
                vec!["you should not care about this".into()],
            ),
            raw_output: std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: b"".to_vec(),
                stderr: b"".to_vec(),
            },
        }
    }
    pub(crate) fn fake_for_test(
        cmd: &str,
        args: Vec<&str>,
        code: i32,
        stdout: &str,
        stderr: &str,
    ) -> Self {
        Self {
            sys_command: SysCommand::new(
                cmd.to_string(),
                args.into_iter().map(String::from).collect(),
            ),

            raw_output: std::process::Output {
                status: std::process::ExitStatus::from_raw(code),
                stdout: stdout.as_bytes().to_vec(),
                stderr: stderr.as_bytes().to_vec(),
            },
        }
    }

    pub(crate) fn success_output(
        stdout: &str,
    ) -> Self {
        Self {
            sys_command: SysCommand::new(
                "nothingburger".to_string(),
                vec!["you should not care about this".into()],
            ),
            raw_output: std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: stdout.as_bytes().to_vec(),
                stderr: b"".to_vec(),
            },
        }
    }
}

pub(crate) trait SysCommandResultChecker {
    fn raw_output(&self) -> &std::process::Output;
    fn sys_command(&self) -> &SysCommand;
    fn as_concrete(&self) -> SysCommandResult;

    fn ran_successfully(&self) -> bool {
        if is_debug() {
            debug!("== EXTERNAL COMMAND STATUS: {}", self.as_string());
        }

        self.raw_output().status.success()
    }

    fn as_success(&self) -> DeckResult<ActionSuccess> {
        if self.ran_successfully() {
            success!(String::from_utf8_lossy(&self.raw_output().stdout))
        } else {
            Err(KnownError::SystemCommandFailed(self.as_concrete()))
        }
    }

    fn as_string(&self) -> String {
        let cmd = &self.sys_command().cmd;
        let args = &self.sys_command().args;
        let output = self.raw_output();
        let status = output.status;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        format!("{cmd} {args:?}\nSTATUS: {status:?}\nSTDOUT:\"{stdout}\"\nSTDERR:\"{stderr}\"")
    }
}

impl SysCommandResultChecker for SysCommandResult {
    fn sys_command(&self) -> &SysCommand {
        &self.sys_command
    }

    fn raw_output(&self) -> &std::process::Output {
        &self.raw_output
    }

    fn as_concrete(&self) -> SysCommandResult {
        self.clone()
    }
}

pub(crate) trait ActualRunner {
    fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult>;
}

#[cfg(test)]
use mockall::mock;
#[cfg(test)]
mock! {
    #[derive(Debug, Clone, Copy)]
    pub TestActualRunner {}

    impl ActualRunner for TestActualRunner {
        fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult>;
    }
}

#[cfg(not(test))]
#[derive(Debug, Clone, Copy, Default)]
pub struct LiveActualRunner {}

#[cfg(not(test))]
impl LiveActualRunner {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(not(test))]
impl ActualRunner for LiveActualRunner {
    #[cfg(test)]
    fn run(&self, sys_command: &SysCommand) -> DeckResult<std::process::Output> {
        panic!("Attempted to run system command {:?} in test!", sys_command)
    }

    #[cfg(not(test))]
    fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult> {
        let cmd = &sys_command.cmd;
        let args = &sys_command.args;

        let mut command = std::process::Command::new(cmd);
        command.args(args);

        for (var, val) in &sys_command.desired_env_vars {
            command.env(var, val);
        }

        let output = command
            .output()
            .map_err(|e| {
                KnownError::SystemCommandRunFailure(SysCommandRunError {
                    cmd: sys_command.clone(),
                    error: e,
                })
            })?;

        Ok(SysCommandResult::new(cmd.clone(), args.clone(), sys_command.desired_env_vars.clone(), output))
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
