use crate::prelude::*;

use std::sync::Arc;

// This is not the logic for actually running system commands, just the representation of what
// those commands are and the context within which they run.

#[derive(Debug)]
pub struct SysCommandRunError {
    pub cmd: SysCommand,
    pub error: std::io::Error,
}

#[derive(Debug, Clone)]
pub struct SysCommand {
    pub ctx: Arc<ExecutionContext>,
    pub cmd: String,
    pub args: Vec<String>,
    pub desired_env_vars: Vec<(String, String)>,
    pub live_logging_desired: bool,
    pub pty_needed: bool,
}

impl PartialEq for SysCommand {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd
            && self.args == other.args
            && self.desired_env_vars == other.desired_env_vars
    }
}

impl SysCommand {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<S, SS, I>(ctx: &impl ExecCtx, cmd: S, args: I) -> Self
    where
        I: IntoIterator<Item = SS>,
        S: StringType,
        SS: StringType,
    {
        Self {
            ctx: Arc::new(ctx.as_ctx()),
            cmd: cmd.to_string(),
            args: args.into_iter().map(|x| x.to_string()).collect(),
            desired_env_vars: Vec::default(),
            live_logging_desired: false,
            pty_needed: false,
        }
    }

    pub fn force_pty(&mut self) -> &mut Self {
        self.pty_needed = true;
        self
    }

    pub fn enable_live_logging(&mut self) -> &mut Self {
        self.live_logging_desired = true;
        self
    }
}

pub trait SysCommandRunner {
    fn get_cmd(&self) -> &SysCommand;
    fn get_ctx(&self) -> &ExecutionContext;

    fn env(&mut self, varname: &str, value: &str) -> &mut Self;

    fn run(&self) -> DeckResult<SysCommandResult> {
        let sys_command = self.get_cmd();
        self.get_ctx().get_runner().run(sys_command)
    }
}

impl SysCommandRunner for SysCommand {
    fn get_ctx(&self) -> &ExecutionContext {
        &self.ctx
    }

    fn get_cmd(&self) -> &SysCommand {
        self
    }

    fn env(&mut self, varname: &str, value: &str) -> &mut Self {
        self.desired_env_vars.push((varname.into(), value.into()));
        self
    }
}

#[derive(Clone, PartialEq)]
pub struct SysCommandResult {
    sys_command: SysCommand,
    raw_output: std::process::Output,
}

impl SysCommandResult {
    pub(super) fn new(sys_command: SysCommand, output: std::process::Output) -> Self {
        Self {
            sys_command,
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
        use std::os::unix::process::ExitStatusExt;

        Self {
            sys_command: ExecutionContext::general_for_test().sys_command(
                "nothingburger".to_string(),
                ["you should not care about this"],
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
        use std::os::unix::process::ExitStatusExt;

        Self {
            sys_command: ExecutionContext::general_for_test().sys_command(cmd.to_string(), args),

            raw_output: std::process::Output {
                status: std::process::ExitStatus::from_raw(code),
                stdout: stdout.as_bytes().to_vec(),
                stderr: stderr.as_bytes().to_vec(),
            },
        }
    }

    pub(crate) fn success_output(stdout: &str) -> Self {
        use std::os::unix::process::ExitStatusExt;

        Self {
            sys_command: ExecutionContext::general_for_test().sys_command(
                "nothingburger".to_string(),
                ["you should not care about this"],
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
            debug!(
                self.sys_command().get_ctx(),
                "== EXTERNAL COMMAND STATUS: {}",
                self.as_string()
            );
        }

        self.raw_output().status.success()
    }

    fn as_success(&self) -> DeckResult<ActionSuccess> {
        if self.ran_successfully() {
            success!(String::from_utf8_lossy(&self.raw_output().stdout))
        } else {
            Err(KnownError::SystemCommandFailed(Box::new(
                self.as_concrete(),
            )))
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
